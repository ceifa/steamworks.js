use napi_derive::napi;

#[napi]
pub mod leaderboards {
    use lazy_static::lazy_static;
    use napi::bindgen_prelude::{BigInt, Error};
    use std::collections::HashMap;
    use std::sync::Mutex;
    use tokio::sync::oneshot;

    // Cache leaderboard handles by name. Steam returns a Leaderboard handle from
    // FindOrCreate / Find that all subsequent operations require, but the JS API
    // is keyed by name for ergonomics. We resolve once and reuse.
    lazy_static! {
        static ref LEADERBOARD_CACHE: Mutex<HashMap<String, steamworks::Leaderboard>> =
            Mutex::new(HashMap::new());
    }

    #[derive(Debug)]
    #[napi]
    pub enum LeaderboardSortMethod {
        Ascending,
        Descending,
    }

    impl From<LeaderboardSortMethod> for steamworks::LeaderboardSortMethod {
        fn from(val: LeaderboardSortMethod) -> Self {
            match val {
                LeaderboardSortMethod::Ascending => steamworks::LeaderboardSortMethod::Ascending,
                LeaderboardSortMethod::Descending => steamworks::LeaderboardSortMethod::Descending,
            }
        }
    }

    #[derive(Debug)]
    #[napi]
    pub enum LeaderboardDisplayType {
        Numeric,
        TimeSeconds,
        TimeMilliSeconds,
    }

    impl From<LeaderboardDisplayType> for steamworks::LeaderboardDisplayType {
        fn from(val: LeaderboardDisplayType) -> Self {
            match val {
                LeaderboardDisplayType::Numeric => steamworks::LeaderboardDisplayType::Numeric,
                LeaderboardDisplayType::TimeSeconds => {
                    steamworks::LeaderboardDisplayType::TimeSeconds
                }
                LeaderboardDisplayType::TimeMilliSeconds => {
                    steamworks::LeaderboardDisplayType::TimeMilliSeconds
                }
            }
        }
    }

    #[derive(Debug)]
    #[napi]
    pub enum LeaderboardDataRequest {
        Global,
        GlobalAroundUser,
        Friends,
    }

    impl From<LeaderboardDataRequest> for steamworks::LeaderboardDataRequest {
        fn from(val: LeaderboardDataRequest) -> Self {
            match val {
                LeaderboardDataRequest::Global => steamworks::LeaderboardDataRequest::Global,
                LeaderboardDataRequest::GlobalAroundUser => {
                    steamworks::LeaderboardDataRequest::GlobalAroundUser
                }
                LeaderboardDataRequest::Friends => steamworks::LeaderboardDataRequest::Friends,
            }
        }
    }

    #[derive(Debug)]
    #[napi]
    pub enum UploadScoreMethod {
        KeepBest,
        ForceUpdate,
    }

    impl From<UploadScoreMethod> for steamworks::UploadScoreMethod {
        fn from(val: UploadScoreMethod) -> Self {
            match val {
                UploadScoreMethod::KeepBest => steamworks::UploadScoreMethod::KeepBest,
                UploadScoreMethod::ForceUpdate => steamworks::UploadScoreMethod::ForceUpdate,
            }
        }
    }

    #[napi(object)]
    pub struct LeaderboardEntry {
        /// SteamId64 of the entry's owner.
        pub steam_id: BigInt,
        /// 1-indexed global rank.
        pub global_rank: i32,
        pub score: i32,
        /// Game-defined metadata attached at upload (up to 64 i32s).
        pub details: Vec<i32>,
    }

    #[napi(object)]
    pub struct LeaderboardScoreUploaded {
        pub score: i32,
        pub score_changed: bool,
        pub global_rank_new: i32,
        pub global_rank_previous: i32,
    }

    /// Find an existing leaderboard or create one with the supplied config.
    /// The handle is cached internally; subsequent calls by the same name
    /// reuse the cached handle without round-tripping to Steam.
    #[napi]
    pub async fn find_or_create_leaderboard(
        name: String,
        sort_method: LeaderboardSortMethod,
        display_type: LeaderboardDisplayType,
    ) -> Result<bool, Error> {
        ensure_leaderboard(&name, sort_method, display_type).await?;
        Ok(true)
    }

    /// Find an existing leaderboard. Resolves to `false` if it doesn't exist.
    /// Use `findOrCreateLeaderboard` if creation-on-miss is desired.
    #[napi]
    pub async fn find_leaderboard(name: String) -> Result<bool, Error> {
        match find_leaderboard_inner(&name).await {
            Ok(_) => Ok(true),
            Err(e) if e.reason == "leaderboard not found" => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Upload a score. The leaderboard is found-or-created with the supplied
    /// config (idempotent after the first call). `details` is up to 64 i32s
    /// of arbitrary game-defined metadata returned with each entry on download.
    #[napi]
    pub async fn upload_leaderboard_score(
        name: String,
        sort_method: LeaderboardSortMethod,
        display_type: LeaderboardDisplayType,
        method: UploadScoreMethod,
        score: i32,
        details: Option<Vec<i32>>,
    ) -> Result<LeaderboardScoreUploaded, Error> {
        let lb = ensure_leaderboard(&name, sort_method, display_type).await?;
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();
        let details_vec = details.unwrap_or_default();
        client.user_stats().upload_leaderboard_score(
            &lb,
            method.into(),
            score,
            &details_vec,
            move |result| {
                let _ = tx.send(result);
            },
        );
        let result = rx.await.map_err(|e| Error::from_reason(e.to_string()))?;
        match result {
            Ok(Some(uploaded)) => Ok(LeaderboardScoreUploaded {
                score: uploaded.score,
                score_changed: uploaded.was_changed,
                global_rank_new: uploaded.global_rank_new,
                global_rank_previous: uploaded.global_rank_previous,
            }),
            Ok(None) => Err(Error::from_reason(
                "leaderboard score upload failed".to_string(),
            )),
            Err(e) => Err(Error::from_reason(e.to_string())),
        }
    }

    /// Download leaderboard entries in the given range. The leaderboard must
    /// already exist (call `findOrCreateLeaderboard` first if needed).
    #[napi]
    pub async fn download_leaderboard_entries(
        name: String,
        request: LeaderboardDataRequest,
        range_start: i32,
        range_end: i32,
        max_details: Option<u32>,
    ) -> Result<Vec<LeaderboardEntry>, Error> {
        let lb = find_leaderboard_inner(&name).await?;
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();
        let max_details_len = max_details.unwrap_or(64) as usize;
        client.user_stats().download_leaderboard_entries(
            &lb,
            request.into(),
            range_start as usize,
            range_end as usize,
            max_details_len,
            move |result| {
                let _ = tx.send(result);
            },
        );
        let result = rx.await.map_err(|e| Error::from_reason(e.to_string()))?;
        match result {
            Ok(entries) => Ok(entries
                .into_iter()
                .map(|e| LeaderboardEntry {
                    steam_id: BigInt::from(e.user.raw()),
                    global_rank: e.global_rank,
                    score: e.score,
                    details: e.details,
                })
                .collect()),
            Err(e) => Err(Error::from_reason(e.to_string())),
        }
    }

    /// Total entry count for the leaderboard. The leaderboard must already
    /// exist (call `findOrCreateLeaderboard` first if needed).
    #[napi]
    pub async fn get_leaderboard_entry_count(name: String) -> Result<i32, Error> {
        let lb = find_leaderboard_inner(&name).await?;
        let client = crate::client::get_client();
        Ok(client.user_stats().get_leaderboard_entry_count(&lb))
    }

    // ---- internal helpers ----

    async fn ensure_leaderboard(
        name: &str,
        sort_method: LeaderboardSortMethod,
        display_type: LeaderboardDisplayType,
    ) -> Result<steamworks::Leaderboard, Error> {
        if let Some(lb) = LEADERBOARD_CACHE.lock().unwrap().get(name) {
            return Ok(lb.clone());
        }
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();
        client.user_stats().find_or_create_leaderboard(
            name,
            sort_method.into(),
            display_type.into(),
            move |result| {
                let _ = tx.send(result);
            },
        );
        let result = rx.await.map_err(|e| Error::from_reason(e.to_string()))?;
        match result {
            Ok(Some(lb)) => {
                LEADERBOARD_CACHE
                    .lock()
                    .unwrap()
                    .insert(name.to_string(), lb.clone());
                Ok(lb)
            }
            Ok(None) => Err(Error::from_reason("leaderboard not found".to_string())),
            Err(e) => Err(Error::from_reason(e.to_string())),
        }
    }

    async fn find_leaderboard_inner(name: &str) -> Result<steamworks::Leaderboard, Error> {
        if let Some(lb) = LEADERBOARD_CACHE.lock().unwrap().get(name) {
            return Ok(lb.clone());
        }
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();
        client.user_stats().find_leaderboard(name, move |result| {
            let _ = tx.send(result);
        });
        let result = rx.await.map_err(|e| Error::from_reason(e.to_string()))?;
        match result {
            Ok(Some(lb)) => {
                LEADERBOARD_CACHE
                    .lock()
                    .unwrap()
                    .insert(name.to_string(), lb.clone());
                Ok(lb)
            }
            Ok(None) => Err(Error::from_reason("leaderboard not found".to_string())),
            Err(e) => Err(Error::from_reason(e.to_string())),
        }
    }
}
