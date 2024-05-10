use napi_derive::napi;

#[napi]
pub mod cloud {
    use napi::bindgen_prelude::{BigInt, Error};
    use std::io::Read;
    use std::io::Write;

    #[napi]
    pub struct FileInfo {
        pub name: String,
        pub size: BigInt,
    }

    #[napi]
    pub fn is_enabled_for_account() -> bool {
        let client = crate::client::get_client();
        client.remote_storage().is_cloud_enabled_for_account()
    }

    #[napi]
    pub fn is_enabled_for_app() -> bool {
        let client = crate::client::get_client();
        client.remote_storage().is_cloud_enabled_for_app()
    }

    #[napi]
    pub fn read_file(name: String) -> Result<String, Error> {
        let client = crate::client::get_client();
        let mut buf: String = String::new();
        let size = client
            .remote_storage()
            .file(&name)
            .read()
            .read_to_string(&mut buf);

        match size {
            Ok(_) => Ok(buf),
            Err(e) => Err(Error::from_reason(format!("Failed to read file: {}", e))),
        }
    }

    #[napi]
    pub fn write_file(name: String, content: String) -> bool {
        let client = crate::client::get_client();
        let file = client.remote_storage().file(&name);

        let buf = content.as_bytes();
        file.write().write_all(buf).is_ok()
    }

    #[napi]
    pub fn delete_file(name: String) -> bool {
        let client = crate::client::get_client();
        let file = client.remote_storage().file(&name);

        file.delete()
    }

    #[napi]
    pub fn file_exists(name: String) -> bool {
        let client = crate::client::get_client();
        let file = client.remote_storage().file(&name);

        file.exists()
    }

    #[napi]
    pub fn list_files() -> Vec<FileInfo> {
        let client = crate::client::get_client();
        client
            .remote_storage()
            .files()
            .into_iter()
            .map(|identity| FileInfo {
                name: identity.name,
                size: BigInt::from(identity.size),
            })
            .collect()
    }
}
