use napi::bindgen_prelude::*;
use std::io::Read;

#[path = "client.rs"]
pub mod client;

#[napi_derive::napi]
pub fn is_cloud_enabled_for_account() -> bool {
    let client = client::get_client();
    client.remote_storage().is_cloud_enabled_for_account()
}

#[napi_derive::napi]
pub fn is_cloud_enabled_for_app() -> bool {
    let client = client::get_client();
    client.remote_storage().is_cloud_enabled_for_app()
}

#[napi_derive::napi]
pub fn read_file(name: String) -> Result<String> {
    let client = client::get_client();
    let mut buf: String = String::new();
    let size = client.remote_storage().file(&name).read().read_to_string(&mut buf);

    match size {
        Ok(_) => Ok(buf),
        Err(e) => Err(Error::new(Status::GenericFailure, format!("Failed to read file: {}", e))),
    }
}