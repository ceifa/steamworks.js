use super::*;
use std::io::Read;
use std::io::Write;

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
    let size = client
        .remote_storage()
        .file(&name)
        .read()
        .read_to_string(&mut buf);

    match size {
        Ok(_) => Ok(buf),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Failed to read file: {}", e),
        )),
    }
}

#[napi_derive::napi]
pub fn write_file(name: String, content: String) -> bool {
    let client = client::get_client();
    let file = client.remote_storage().file(&name);

    let mut buf = content.as_bytes();
    file.write().write_all(&mut buf).is_ok()
}

#[napi_derive::napi]
pub fn delete_file(name: String) -> bool {
    let client = client::get_client();
    let file = client.remote_storage().file(&name);

    file.delete()
}
