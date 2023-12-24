use napi_derive::napi;

#[napi]
pub mod workshop {
    use napi::bindgen_prelude::{BigInt, Error, FromNapiValue, ToNapiValue};
    use steamworks::{AccountId, PublishedFileId, UGCQuery};
    use tokio::sync::oneshot;

    use crate::api::localplayer::PlayerSteamId;

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

    impl From<UGCQueryType> for steamworks::UGCQueryType {
        fn from(query_type: UGCQueryType) -> Self {
            match query_type {
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

    impl From<UGCType> for steamworks::UGCType {
        fn from(ugc_type: UGCType) -> Self {
            match ugc_type {
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

    impl From<UserListType> for steamworks::UserList {
        fn from(list_type: UserListType) -> Self {
            match list_type {
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

    impl From<UserListOrder> for steamworks::UserListOrder {
        fn from(sort_order: UserListOrder) -> Self {
            match sort_order {
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
        pub banned: bool,
        pub accepted_for_use: bool,
        pub tags: Vec<String>,
        pub tags_truncated: bool,
        pub url: String,
        pub num_upvotes: u32,
        pub num_downvotes: u32,
        pub num_children: u32,
        pub preview_url: Option<String>,
    }

    impl WorkshopItem {
        fn from_query(result: steamworks::QueryResult, preview_url: Option<String>) -> Self {
            Self {
                published_file_id: BigInt::from(result.published_file_id.0),
                creator_app_id: result.creator_app_id.map(|id| id.0),
                consumer_app_id: result.consumer_app_id.map(|id| id.0),
                title: result.title,
                description: result.description,
                owner: PlayerSteamId::from_steamid(result.owner),
                time_created: result.time_created,
                time_updated: result.time_updated,
                banned: result.banned,
                accepted_for_use: result.accepted_for_use,
                tags: result.tags,
                tags_truncated: result.tags_truncated,
                url: result.url,
                num_upvotes: result.num_upvotes,
                num_downvotes: result.num_downvotes,
                num_children: result.num_children,
                preview_url,
            }
        }
    }

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
    }

    fn handle_query_config<Manager, T: UGCQuery<Manager>>(
        mut query_handle: T,
        query_config: Option<WorkshopItemQueryConfig>,
    ) -> T {
        // Apply additional query parameters if provided
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

            query_handle.fetch(|result| {
                tx.send(result.map(|result| {
                    result
                        .get(0)
                        .map(|item| WorkshopItem::from_query(item, result.preview_url(0)))
                }))
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
    ) -> Result<Vec<Option<WorkshopItem>>, Error> {
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

            query_handle.fetch(|result| {
                tx.send(result.map(|result| {
                    result
                        .iter()
                        .enumerate()
                        .map(|(i, item)| {
                            item.map(|item| {
                                WorkshopItem::from_query(item, result.preview_url(i as u32))
                            })
                        })
                        .collect()
                }))
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
    ) -> Result<Vec<Option<WorkshopItem>>, Error> {
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

            query_handle.fetch(|result| {
                tx.send(result.map(|result| {
                    result
                        .iter()
                        .enumerate()
                        .map(|(i, item)| {
                            item.map(|item| {
                                WorkshopItem::from_query(item, result.preview_url(i as u32))
                            })
                        })
                        .collect()
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
    ) -> Result<Vec<Option<WorkshopItem>>, Error> {
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

            query_handle.fetch(|result| {
                tx.send(result.map(|result| {
                    result
                        .iter()
                        .enumerate()
                        .map(|(i, item)| {
                            item.map(|item| {
                                WorkshopItem::from_query(item, result.preview_url(i as u32))
                            })
                        })
                        .collect()
                }))
                .unwrap();
            });
        }

        rx.await
            .unwrap()
            .map_err(|e| Error::from_reason(e.to_string()))
    }
}
