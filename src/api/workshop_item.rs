use napi_derive::napi;

#[napi]
pub mod workshop {
    use napi::bindgen_prelude::{BigInt, Error, FromNapiValue, ToNapiValue};
    use steamworks::{AccountId, PublishedFileId};
    use tokio::sync::oneshot;

    use crate::api::localplayer::PlayerSteamId;
    use crate::api::workshop::workshop::UgcItemVisibility;

    #[napi]
    pub enum UGCQueryType {
        RankedByVote,
        RankedByPublicationDate,
        AcceptedForGameRankedByAcceptanceDate,
        RankedByTrend,
        FavoritedByFriendsRankedByPublicationDate,
        CreatedByFriendsRankedByPublicationDate,
        RankedByNumTimesReported,
        CreatedByFollowedUsersRankedByPublicationDate,
        NotYetRated,
        RankedByTotalVotesAsc,
        RankedByVotesUp,
        RankedByTextSearch,
        RankedByTotalUniqueSubscriptions,
        RankedByPlaytimeTrend,
        RankedByTotalPlaytime,
        RankedByAveragePlaytimeTrend,
        RankedByLifetimeAveragePlaytime,
        RankedByPlaytimeSessionsTrend,
        RankedByLifetimePlaytimeSessions,
        RankedByLastUpdatedDate,
    }

    impl Into<steamworks::UGCQueryType> for UGCQueryType {
        fn into(self: UGCQueryType) -> steamworks::UGCQueryType {
            match self {
                UGCQueryType::RankedByVote => steamworks::UGCQueryType::RankedByVote,
                UGCQueryType::RankedByPublicationDate => {
                    steamworks::UGCQueryType::RankedByPublicationDate
                }
                UGCQueryType::AcceptedForGameRankedByAcceptanceDate => {
                    steamworks::UGCQueryType::AcceptedForGameRankedByAcceptanceDate
                }
                UGCQueryType::RankedByTrend => steamworks::UGCQueryType::RankedByTrend,
                UGCQueryType::FavoritedByFriendsRankedByPublicationDate => {
                    steamworks::UGCQueryType::FavoritedByFriendsRankedByPublicationDate
                }
                UGCQueryType::CreatedByFriendsRankedByPublicationDate => {
                    steamworks::UGCQueryType::CreatedByFriendsRankedByPublicationDate
                }
                UGCQueryType::RankedByNumTimesReported => {
                    steamworks::UGCQueryType::RankedByNumTimesReported
                }
                UGCQueryType::CreatedByFollowedUsersRankedByPublicationDate => {
                    steamworks::UGCQueryType::CreatedByFollowedUsersRankedByPublicationDate
                }
                UGCQueryType::NotYetRated => steamworks::UGCQueryType::NotYetRated,
                UGCQueryType::RankedByTotalVotesAsc => {
                    steamworks::UGCQueryType::RankedByTotalVotesAsc
                }
                UGCQueryType::RankedByVotesUp => steamworks::UGCQueryType::RankedByVotesUp,
                UGCQueryType::RankedByTextSearch => steamworks::UGCQueryType::RankedByTextSearch,
                UGCQueryType::RankedByTotalUniqueSubscriptions => {
                    steamworks::UGCQueryType::RankedByTotalUniqueSubscriptions
                }
                UGCQueryType::RankedByPlaytimeTrend => {
                    steamworks::UGCQueryType::RankedByPlaytimeTrend
                }
                UGCQueryType::RankedByTotalPlaytime => {
                    steamworks::UGCQueryType::RankedByTotalPlaytime
                }
                UGCQueryType::RankedByAveragePlaytimeTrend => {
                    steamworks::UGCQueryType::RankedByAveragePlaytimeTrend
                }
                UGCQueryType::RankedByLifetimeAveragePlaytime => {
                    steamworks::UGCQueryType::RankedByLifetimeAveragePlaytime
                }
                UGCQueryType::RankedByPlaytimeSessionsTrend => {
                    steamworks::UGCQueryType::RankedByPlaytimeSessionsTrend
                }
                UGCQueryType::RankedByLifetimePlaytimeSessions => {
                    steamworks::UGCQueryType::RankedByLifetimePlaytimeSessions
                }
                UGCQueryType::RankedByLastUpdatedDate => {
                    steamworks::UGCQueryType::RankedByLastUpdatedDate
                }
            }
        }
    }

    #[napi]
    pub enum UGCType {
        Items,
        ItemsMtx,
        ItemsReadyToUse,
        Collections,
        Artwork,
        Videos,
        Screenshots,
        AllGuides,
        WebGuides,
        IntegratedGuides,
        UsableInGame,
        ControllerBindings,
        GameManagedItems,
        All,
    }

    impl Into<steamworks::UGCType> for UGCType {
        fn into(self) -> steamworks::UGCType {
            match self {
                UGCType::Items => steamworks::UGCType::Items,
                UGCType::ItemsMtx => steamworks::UGCType::ItemsMtx,
                UGCType::ItemsReadyToUse => steamworks::UGCType::ItemsReadyToUse,
                UGCType::Collections => steamworks::UGCType::Collections,
                UGCType::Artwork => steamworks::UGCType::Artwork,
                UGCType::Videos => steamworks::UGCType::Videos,
                UGCType::Screenshots => steamworks::UGCType::Screenshots,
                UGCType::AllGuides => steamworks::UGCType::AllGuides,
                UGCType::WebGuides => steamworks::UGCType::WebGuides,
                UGCType::IntegratedGuides => steamworks::UGCType::IntegratedGuides,
                UGCType::UsableInGame => steamworks::UGCType::UsableInGame,
                UGCType::ControllerBindings => steamworks::UGCType::ControllerBindings,
                UGCType::GameManagedItems => steamworks::UGCType::GameManagedItems,
                UGCType::All => steamworks::UGCType::All,
            }
        }
    }

    #[napi]
    pub enum UserListType {
        Published,
        VotedOn,
        VotedUp,
        VotedDown,
        // Deprecated: WillVoteLater,
        Favorited,
        Subscribed,
        UsedOrPlayed,
        Followed,
    }

    impl Into<steamworks::UserList> for UserListType {
        fn into(self) -> steamworks::UserList {
            match self {
                UserListType::Published => steamworks::UserList::Published,
                UserListType::VotedOn => steamworks::UserList::VotedOn,
                UserListType::VotedUp => steamworks::UserList::VotedUp,
                UserListType::VotedDown => steamworks::UserList::VotedDown,
                // UserListType::WillVoteLater => steamworks::UserList::WillVoteLater, // Deprecated
                UserListType::Favorited => steamworks::UserList::Favorited,
                UserListType::Subscribed => steamworks::UserList::Subscribed,
                UserListType::UsedOrPlayed => steamworks::UserList::UsedOrPlayed,
                UserListType::Followed => steamworks::UserList::Followed,
            }
        }
    }

    #[napi]
    pub enum UserListOrder {
        CreationOrderAsc,
        CreationOrderDesc,
        TitleAsc,
        LastUpdatedDesc,
        SubscriptionDateDesc,
        VoteScoreDesc,
        ForModeration,
    }

    impl Into<steamworks::UserListOrder> for UserListOrder {
        fn into(self: UserListOrder) -> steamworks::UserListOrder {
            match self {
                UserListOrder::CreationOrderAsc => steamworks::UserListOrder::CreationOrderAsc,
                UserListOrder::CreationOrderDesc => steamworks::UserListOrder::CreationOrderDesc,
                UserListOrder::TitleAsc => steamworks::UserListOrder::TitleAsc,
                UserListOrder::LastUpdatedDesc => steamworks::UserListOrder::LastUpdatedDesc,
                UserListOrder::SubscriptionDateDesc => {
                    steamworks::UserListOrder::SubscriptionDateDesc
                }
                UserListOrder::VoteScoreDesc => steamworks::UserListOrder::VoteScoreDesc,
                UserListOrder::ForModeration => steamworks::UserListOrder::ForModeration,
            }
        }
    }

    #[derive(Debug)]
    #[napi(object)]
    pub struct WorkshopItemStatistic {
        pub num_subscriptions: Option<BigInt>, //   0	gets the number of subscriptions.
        pub num_favorites: Option<BigInt>,     //   1	gets the number of favorites.
        pub num_followers: Option<BigInt>,     //   2	gets the number of followers.
        pub num_unique_subscriptions: Option<BigInt>, // 3	gets the number of unique subscriptions.
        pub num_unique_favorites: Option<BigInt>, // 4	gets the number of unique favorites.
        pub num_unique_followers: Option<BigInt>, // 5	gets the number of unique followers.
        pub num_unique_website_views: Option<BigInt>, //  6	gets the number of unique views the item has on its steam workshop page.
        pub report_score: Option<BigInt>, //    7	gets the number of times the item has been reported.
        pub num_seconds_played: Option<BigInt>, //   8	gets the total number of seconds this item has been used across all players.
        pub num_playtime_sessions: Option<BigInt>, //    9	gets the total number of play sessions this item has been used in.
        pub num_comments: Option<BigInt>, //    10	gets the number of comments on the items that steam has on its steam workshop page.
        pub num_seconds_played_during_time_period: Option<BigInt>, //   11	gets the number of seconds this item has been used over the given time period.
        pub num_playtime_sessions_during_time_period: Option<BigInt>, //    12	Gets the number of sessions this item has been used in over the given time period.
    }

    impl WorkshopItemStatistic {
        fn from_query_results(results: &steamworks::QueryResults, index: u32) -> Self {
            Self {
                num_subscriptions: results
                    .statistic(index, steamworks::UGCStatisticType::Subscriptions)
                    .map(|v| BigInt::from(v)),
                num_favorites: results
                    .statistic(index, steamworks::UGCStatisticType::Favorites)
                    .map(|v| BigInt::from(v)),
                num_followers: results
                    .statistic(index, steamworks::UGCStatisticType::Followers)
                    .map(|v| BigInt::from(v)),
                num_unique_subscriptions: results
                    .statistic(index, steamworks::UGCStatisticType::UniqueSubscriptions)
                    .map(|v| BigInt::from(v)),
                num_unique_favorites: results
                    .statistic(index, steamworks::UGCStatisticType::UniqueFavorites)
                    .map(|v| BigInt::from(v)),
                num_unique_followers: results
                    .statistic(index, steamworks::UGCStatisticType::UniqueFollowers)
                    .map(|v| BigInt::from(v)),
                num_unique_website_views: results
                    .statistic(index, steamworks::UGCStatisticType::UniqueWebsiteViews)
                    .map(|v| BigInt::from(v)),
                report_score: results
                    .statistic(index, steamworks::UGCStatisticType::Reports)
                    .map(|v| BigInt::from(v)),
                num_seconds_played: results
                    .statistic(index, steamworks::UGCStatisticType::SecondsPlayed)
                    .map(|v| BigInt::from(v)),
                num_playtime_sessions: results
                    .statistic(index, steamworks::UGCStatisticType::PlaytimeSessions)
                    .map(|v| BigInt::from(v)),
                num_comments: results
                    .statistic(index, steamworks::UGCStatisticType::Comments)
                    .map(|v| BigInt::from(v)),
                num_seconds_played_during_time_period: results
                    .statistic(
                        index,
                        steamworks::UGCStatisticType::SecondsPlayedDuringTimePeriod,
                    )
                    .map(|v| BigInt::from(v)),
                num_playtime_sessions_during_time_period: results
                    .statistic(
                        index,
                        steamworks::UGCStatisticType::PlaytimeSessionsDuringTimePeriod,
                    )
                    .map(|v| BigInt::from(v)),
            }
        }
    }

    #[derive(Debug)]
    #[napi(object)]
    pub struct WorkshopItem {
        pub published_file_id: BigInt,
        pub creator_app_id: Option<u32>,
        pub consumer_app_id: Option<u32>,
        pub title: String,
        pub description: String,
        pub owner: PlayerSteamId,
        /// Time created in unix epoch seconds format
        pub time_created: u32,
        /// Time updated in unix epoch seconds format
        pub time_updated: u32,
        /// Time when the user added the published item to their list (not always applicable), provided in Unix epoch format (time since Jan 1st, 1970).
        pub time_added_to_user_list: u32,
        pub visibility: UgcItemVisibility,
        pub banned: bool,
        pub accepted_for_use: bool,
        pub tags: Vec<String>,
        pub tags_truncated: bool,
        pub url: String,
        pub num_upvotes: u32,
        pub num_downvotes: u32,
        pub num_children: u32,
        pub preview_url: Option<String>,
        pub statistics: WorkshopItemStatistic, // Is it necessary to design this as optional?
    }

    impl WorkshopItem {
        fn from_query_results(results: &steamworks::QueryResults, index: u32) -> Option<Self> {
            results.get(index).map(|item| Self {
                published_file_id: BigInt::from(item.published_file_id.0),
                creator_app_id: item.creator_app_id.map(|id| id.0),
                consumer_app_id: item.consumer_app_id.map(|id| id.0),
                title: item.title,
                description: item.description,
                owner: PlayerSteamId::from_steamid(item.owner),
                time_created: item.time_created,
                time_updated: item.time_updated,
                time_added_to_user_list: item.time_added_to_user_list,
                visibility: item.visibility.into(),
                banned: item.banned,
                accepted_for_use: item.accepted_for_use,
                tags: item.tags,
                tags_truncated: item.tags_truncated,
                url: item.url,
                num_upvotes: item.num_upvotes,
                num_downvotes: item.num_downvotes,
                num_children: item.num_children,
                preview_url: results.preview_url(index),
                statistics: WorkshopItemStatistic::from_query_results(results, index),
            })
        }
    }

    #[derive(Debug)]
    #[napi(object)]
    pub struct WorkshopPaginatedResult {
        pub items: Vec<Option<WorkshopItem>>,
        pub returned_results: u32,
        pub total_results: u32,
        pub was_cached: bool,
    }

    impl WorkshopPaginatedResult {
        fn from_query_results(query_results: steamworks::QueryResults) -> Self {
            Self {
                items: (0..query_results.returned_results())
                    .map(|i| WorkshopItem::from_query_results(&query_results, i))
                    .collect(),
                returned_results: query_results.returned_results(),
                total_results: query_results.total_results(),
                was_cached: query_results.was_cached(),
            }
        }
    }

    #[derive(Debug)]
    #[napi(object)]
    pub struct WorkshopItemsResult {
        pub items: Vec<Option<WorkshopItem>>,
        pub was_cached: bool,
    }

    impl WorkshopItemsResult {
        fn from_query_results(query_results: steamworks::QueryResults) -> Self {
            Self {
                items: (0..query_results.returned_results())
                    .map(|i| WorkshopItem::from_query_results(&query_results, i))
                    .collect(),
                was_cached: query_results.was_cached(),
            }
        }
    }

    #[derive(Debug)]
    #[napi(object)]
    pub struct WorkshopItemQueryConfig {
        pub cached_response_max_age: Option<u32>,
        pub include_metadata: Option<bool>,
        pub include_long_description: Option<bool>,
        pub include_additional_previews: Option<bool>,
        pub only_ids: Option<bool>,
        pub only_total: Option<bool>,
        pub language: Option<String>,
        pub match_any_tag: Option<bool>,
        pub required_tags: Option<Vec<String>>,
        pub excluded_tags: Option<Vec<String>>,
        pub search_text: Option<String>,
        pub ranked_by_trend_days: Option<u32>,
    }

    fn handle_query_config<Manager>(
        mut query_handle: steamworks::QueryHandle<Manager>,
        query_config: Option<WorkshopItemQueryConfig>,
    ) -> steamworks::QueryHandle<Manager> {
        // Apply statistics query parameters if provided
        if let Some(query_config) = query_config {
            if let Some(cached_response_max_age) = query_config.cached_response_max_age {
                query_handle = query_handle.set_allow_cached_response(cached_response_max_age);
            }
            if let Some(include_metadata) = query_config.include_metadata {
                query_handle = query_handle.set_return_metadata(include_metadata);
            }
            if let Some(include_long_description) = query_config.include_long_description {
                query_handle = query_handle.set_return_long_description(include_long_description);
            }
            if let Some(include_additional_previews) = query_config.include_additional_previews {
                query_handle =
                    query_handle.set_return_additional_previews(include_additional_previews)
            }
            if let Some(only_ids) = query_config.only_ids {
                query_handle = query_handle.set_return_only_ids(only_ids)
            }
            if let Some(only_total) = query_config.only_total {
                query_handle = query_handle.set_return_total_only(only_total)
            }
            if let Some(language) = query_config.language {
                query_handle = query_handle.set_language(&language);
            }
            if let Some(match_any_tag) = query_config.match_any_tag {
                query_handle = query_handle.set_match_any_tag(match_any_tag);
            }
            if let Some(required_tags) = query_config.required_tags {
                for tag in required_tags {
                    query_handle = query_handle.add_required_tag(&tag);
                }
            }
            if let Some(excluded_tags) = query_config.excluded_tags {
                for tag in excluded_tags {
                    query_handle = query_handle.add_excluded_tag(&tag);
                }
            }
            if let Some(search_text) = query_config.search_text {
                query_handle = query_handle.set_search_text(&search_text);
            }
            if let Some(ranked_by_trend_days) = query_config.ranked_by_trend_days {
                query_handle = query_handle.set_ranked_by_trend_days(ranked_by_trend_days);
            }
        }
        return query_handle;
    }

    #[napi]
    pub async fn get_item(
        item: BigInt,
        query_config: Option<WorkshopItemQueryConfig>,
    ) -> Result<Option<WorkshopItem>, Error> {
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();

        {
            let mut query_handle = client
                .ugc()
                .query_item(PublishedFileId(item.get_u64().1))
                .map_err(|e| Error::from_reason(e.to_string()))?;

            query_handle = handle_query_config(query_handle, query_config);

            query_handle.fetch(|fetch_result| {
                tx.send(
                    fetch_result
                        .map(|query_results| WorkshopItem::from_query_results(&query_results, 0)),
                )
                .unwrap();
            });
        }

        rx.await
            .unwrap()
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub async fn get_items(
        items: Vec<BigInt>,
        query_config: Option<WorkshopItemQueryConfig>,
    ) -> Result<WorkshopItemsResult, Error> {
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();

        {
            let mut query_handle = client
                .ugc()
                .query_items(
                    items
                        .iter()
                        .map(|id| PublishedFileId(id.get_u64().1))
                        .collect(),
                )
                .map_err(|e| Error::from_reason(e.to_string()))?;

            query_handle = handle_query_config(query_handle, query_config);

            query_handle.fetch(|fetch_result| {
                tx.send(
                    fetch_result.map(|query_results| {
                        WorkshopItemsResult::from_query_results(query_results)
                    }),
                )
                .unwrap();
            });
        }

        rx.await
            .unwrap()
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub async fn get_all_items(
        page: u32,
        query_type: UGCQueryType,
        item_type: UGCType,
        creator_app_id: u32,
        consumer_app_id: u32,
        query_config: Option<WorkshopItemQueryConfig>,
    ) -> Result<WorkshopPaginatedResult, Error> {
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();

        {
            // Start configuring the query for all items
            let mut query_handle = client
                .ugc()
                .query_all(
                    query_type.into(),
                    item_type.into(),
                    steamworks::AppIDs::Both {
                        creator: steamworks::AppId(creator_app_id),
                        consumer: steamworks::AppId(consumer_app_id),
                    },
                    page,
                )
                .map_err(|e| Error::from_reason(e.to_string()))?;

            query_handle = handle_query_config(query_handle, query_config);

            query_handle.fetch(|fetch_result| {
                tx.send(fetch_result.map(|query_results| {
                    WorkshopPaginatedResult::from_query_results(query_results)
                }))
                .unwrap();
            });
        }

        rx.await
            .unwrap()
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub async fn get_user_items(
        page: u32,
        account_id: u32,
        list_type: UserListType,
        item_type: UGCType,
        sort_order: UserListOrder,
        creator_app_id: u32,
        consumer_app_id: u32,
        query_config: Option<WorkshopItemQueryConfig>,
    ) -> Result<WorkshopPaginatedResult, Error> {
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();

        {
            // Start configuring the query for user items
            let mut query_handle = client
                .ugc()
                .query_user(
                    AccountId::from_raw(account_id),
                    list_type.into(),
                    item_type.into(),
                    sort_order.into(),
                    steamworks::AppIDs::Both {
                        creator: steamworks::AppId(creator_app_id),
                        consumer: steamworks::AppId(consumer_app_id),
                    },
                    page,
                )
                .map_err(|e| Error::from_reason(e.to_string()))?;

            query_handle = handle_query_config(query_handle, query_config);

            query_handle.fetch(|fetch_result| {
                tx.send(fetch_result.map(|query_results| {
                    WorkshopPaginatedResult::from_query_results(query_results)
                }))
                .unwrap();
            });
        }

        rx.await
            .unwrap()
            .map_err(|e| Error::from_reason(e.to_string()))
    }
}
