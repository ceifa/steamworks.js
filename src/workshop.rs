use napi_derive::napi;

#[napi]
pub mod workshop {
    use napi::bindgen_prelude::{BigInt, Error};
    use std::path::Path;
    use steamworks::{FileType, PublishedFileId};
    use tokio::sync::oneshot;

    #[napi(object)]
    pub struct UgcResult {
        pub item_id: BigInt,
        pub needs_to_accept_agreement: bool,
    }

    #[napi(object)]
    pub struct UgcUpdate {
        pub title: Option<String>,
        pub description: Option<String>,
        pub change_note: Option<String>,
        pub preview_path: Option<String>,
        pub content_path: Option<String>,
        pub tags: Option<Vec<String>>,
    }

    #[napi(object)]
    pub struct InstallInfo {
        pub folder: String,
        pub size_on_disk: BigInt,
        pub timestamp: u32,
    }

    #[napi(object)]
    pub struct DownloadInfo {
        pub current: BigInt,
        pub total: BigInt,
    }

    #[napi]
    pub async fn create_item() -> Result<UgcResult, Error> {
        let client = crate::client::get_client();
        let appid = client.utils().app_id();

        let (tx, rx) = oneshot::channel();

        client
            .ugc()
            .create_item(appid, FileType::Community, |result| {
                tx.send(result).unwrap();
            });

        let result = rx.await.unwrap();
        match result {
            Ok((item_id, needs_to_accept_agreement)) => Ok(UgcResult {
                item_id: BigInt {
                    sign_bit: false,
                    words: vec![item_id.0],
                },
                needs_to_accept_agreement,
            }),
            Err(e) => Err(Error::from_reason(e.to_string())),
        }
    }

    #[napi]
    pub async fn update_item(
        item_id: BigInt,
        update_details: UgcUpdate,
    ) -> Result<UgcResult, Error> {
        let client = crate::client::get_client();
        let appid = client.utils().app_id();

        let (tx, rx) = oneshot::channel();

        {
            let mut update = client
                .ugc()
                .start_item_update(appid, PublishedFileId(item_id.get_u64().1));

            if let Some(title) = update_details.title {
                update = update.title(title.as_str());
            }

            if let Some(description) = update_details.description {
                update = update.description(description.as_str());
            }

            if let Some(preview_path) = update_details.preview_path {
                update = update.preview_path(Path::new(&preview_path));
            }

            if let Some(tags) = update_details.tags {
                update = update.tags(tags);
            }

            if let Some(content_path) = update_details.content_path {
                update = update.content_path(Path::new(&content_path));
            }

            let change_note = update_details.change_note.as_deref();

            update.submit(change_note, |result| {
                tx.send(result).unwrap();
            });
        }

        let result = rx.await.unwrap();
        match result {
            Ok((item_id, needs_to_accept_agreement)) => Ok(UgcResult {
                item_id: BigInt {
                    sign_bit: false,
                    words: vec![item_id.0],
                },
                needs_to_accept_agreement,
            }),
            Err(e) => Err(Error::from_reason(e.to_string())),
        }
    }

    /// Subscribe to a workshop item. It will be downloaded and installed as soon as possible.
    /// https://partner.steamgames.com/doc/api/ISteamUGC#SubscribeItem
    #[napi]
    pub async fn subscribe(item_id: BigInt) -> Result<(), Error> {
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();

        client
            .ugc()
            .subscribe_item(PublishedFileId(item_id.get_u64().1), |result| {
                tx.send(result).unwrap();
            });

        let result = rx.await.unwrap();
        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(Error::from_reason(e.to_string())),
        }
    }

    /// Unsubscribe from a workshop item. This will result in the item being removed after the game quits.
    /// https://partner.steamgames.com/doc/api/ISteamUGC#UnsubscribeItem
    #[napi]
    pub async fn unsubscribe(item_id: BigInt) -> Result<(), Error> {
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();

        client
            .ugc()
            .unsubscribe_item(PublishedFileId(item_id.get_u64().1), |result| {
                tx.send(result).unwrap();
            });

        let result = rx.await.unwrap();
        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(Error::from_reason(e.to_string())),
        }
    }

    /// Gets the current state of a workshop item on this client. States can be combined.
    /// https://partner.steamgames.com/doc/api/ISteamUGC#GetItemState
    /// https://partner.steamgames.com/doc/api/ISteamUGC#EItemState
    #[napi]
    pub fn state(item_id: BigInt) -> u32 {
        let client = crate::client::get_client();
        let result = client
            .ugc()
            .item_state(PublishedFileId(item_id.get_u64().1));

        result.bits()
    }

    /// Gets info about currently installed content on the disc for workshop item.
    /// https://partner.steamgames.com/doc/api/ISteamUGC#GetItemInstallInfo
    #[napi]
    pub fn install_info(item_id: BigInt) -> Option<InstallInfo> {
        let client = crate::client::get_client();
        let result = client
            .ugc()
            .item_install_info(PublishedFileId(item_id.get_u64().1));

        match result {
            Some(install_info) => Some(InstallInfo {
                folder: install_info.folder,
                size_on_disk: BigInt {
                    sign_bit: false,
                    words: vec![install_info.size_on_disk],
                },
                timestamp: install_info.timestamp,
            }),
            None => None,
        }
    }

    #[napi]
    pub fn download_info(item_id: BigInt) -> Option<DownloadInfo> {
        let client = crate::client::get_client();
        let result = client
            .ugc()
            .item_download_info(PublishedFileId(item_id.get_u64().1));

        match result {
            Some(download_info) => Some(DownloadInfo {
                current: BigInt {
                    sign_bit: false,
                    words: vec![download_info.0],
                },
                total: BigInt {
                    sign_bit: false,
                    words: vec![download_info.1],
                },
            }),
            None => None,
        }
    }

    #[napi]
    pub fn download(item_id: BigInt, high_priority: bool) -> bool {
        let client = crate::client::get_client();
        client
            .ugc()
            .download_item(PublishedFileId(item_id.get_u64().1), high_priority)
    }
}
