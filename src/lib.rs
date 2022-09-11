use napi::bindgen_prelude::Error;
use napi_derive::napi;
use steamworks::AppId;
use steamworks::Client;

pub mod client;

#[macro_use]
extern crate lazy_static;

#[napi]
pub fn init(app_id: u32) -> Result<(), Error> {
    if client::has_client() {
        let initialized_app_id = client::get_client().utils().app_id().0;
        if initialized_app_id != app_id {
            return Err(Error::from_reason(format!(
                "Client already initialized for app id {}",
                app_id
            )));
        } else {
            return Ok(());
        }
    }

    let result = Client::init_app(app_id);
    match result {
        Ok((steam_client, steam_single)) => {
            steam_client.user_stats().request_current_stats();

            client::set_client(steam_client);
            client::set_single(steam_single);
            Ok(())
        }
        Err(e) => Err(Error::from_reason(e.to_string())),
    }
}

#[napi]
pub fn restart_app_if_necessary(app_id: u32) -> bool {
    steamworks::restart_app_if_necessary(AppId(app_id))
}

#[napi_derive::napi]
pub fn run_callbacks() {
    client::get_single().run_callbacks();
}

pub mod api;
