use std::path::Path;

use super::*;
use steamworks::{FileType, PublishedFileId};
use tokio::sync::oneshot;

#[napi_derive::napi(object)]
pub struct UgcResult {
    pub item_id: BigInt,
    pub needs_to_accept_agreement: bool,
}

#[napi_derive::napi(object)]
pub struct UgcUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub change_note: Option<String>,
    pub preview_path: Option<String>,
    pub content_path: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[napi_derive::napi]
pub async fn create_community_item() -> Result<UgcResult> {
    let client = client::get_client();
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
        Err(e) => Err(Error::new(Status::GenericFailure, e.to_string())),
    }
}

#[napi_derive::napi]
pub async fn update_community_item(
    item_id: BigInt,
    update_details: UgcUpdate,
) -> Result<UgcResult> {
    let client = client::get_client();
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
        Err(e) => Err(Error::new(Status::GenericFailure, e.to_string())),
    }
}

#[napi_derive::napi]
pub async fn ugc_subscribe_item(
    item_id: BigInt
) -> Result<()> {
    let client = client::get_client();
    let (tx, rx) = oneshot::channel();

    client
        .ugc()
        .subscribe_item(PublishedFileId(item_id.get_u64().1), |result| {
            tx.send(result).unwrap();
        });

    let result = rx.await.unwrap();
    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(Error::new(Status::GenericFailure, e.to_string())),
    }
}

#[napi_derive::napi]
pub async fn ugc_unsubscribe_item(
    item_id: BigInt
) -> Result<()> {
    let client = client::get_client();
    let (tx, rx) = oneshot::channel();

    client
        .ugc()
        .unsubscribe_item(PublishedFileId(item_id.get_u64().1), |result| {
            tx.send(result).unwrap();
        });

    let result = rx.await.unwrap();
    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(Error::new(Status::GenericFailure, e.to_string())),
    }
}