use napi_derive::napi;

#[napi]
pub mod workshop {
    use napi::bindgen_prelude::{BigInt, Error};
    use steamworks::PublishedFileId;
    use tokio::sync::oneshot;

    use crate::api::localplayer::PlayerSteamId;

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
    pub struct WorkshopItemQuery {
        pub cached_response_max_age: Option<u32>,
        pub include_metadata: Option<bool>,
        pub include_long_description: Option<bool>,
        pub language: Option<String>,
    }

    #[napi]
    pub async fn get_item(
        item: BigInt,
        query: Option<WorkshopItemQuery>,
    ) -> Result<Option<WorkshopItem>, Error> {
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();

        {
            let mut item_query = client
                .ugc()
                .query_item(PublishedFileId(item.get_u64().1))
                .map_err(|e| Error::from_reason(e.to_string()))?;

            if let Some(query) = query {
                if let Some(cached_response_max_age) = query.cached_response_max_age {
                    item_query = item_query.allow_cached_response(cached_response_max_age);
                }

                if let Some(include_metadata) = query.include_metadata {
                    item_query = item_query.include_metadata(include_metadata);
                }

                if let Some(include_long_description) = query.include_long_description {
                    item_query = item_query.include_long_desc(include_long_description);
                }

                if let Some(language) = query.language {
                    item_query = item_query.language(&language);
                }
            }

            item_query.fetch(|result| {
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
        query: Option<WorkshopItemQuery>,
    ) -> Result<Vec<Option<WorkshopItem>>, Error> {
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();

        {
            let mut item_query = client
                .ugc()
                .query_items(
                    items
                        .iter()
                        .map(|id| PublishedFileId(id.get_u64().1))
                        .collect(),
                )
                .map_err(|e| Error::from_reason(e.to_string()))?;

            if let Some(query) = query {
                if let Some(cached_response_max_age) = query.cached_response_max_age {
                    item_query = item_query.allow_cached_response(cached_response_max_age);
                }

                if let Some(include_metadata) = query.include_metadata {
                    item_query = item_query.include_metadata(include_metadata);
                }

                if let Some(include_long_description) = query.include_long_description {
                    item_query = item_query.include_long_desc(include_long_description);
                }

                if let Some(language) = query.language {
                    item_query = item_query.language(&language);
                }
            }

            item_query.fetch(|result| {
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
