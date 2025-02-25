use napi::bindgen_prelude::Error;
use napi_derive::napi;
use steamworks::AppId;
use steamworks::Client;
use steamworks::SteamAPIInitError;

pub mod client;

#[macro_use]
extern crate lazy_static;

#[napi]
pub fn init(app_id: Option<u32>, networking: Option<bool>) -> Result<(), Error> {
    if client::has_client() {
        client::drop_client();
    }

    let steam_client = app_id
        .map(|app_id| Client::init_app(AppId(app_id)))
        .unwrap_or_else(Client::init)
        .map_err(|e| match e {
            SteamAPIInitError::FailedGeneric(msg)
            | SteamAPIInitError::NoSteamClient(msg)
            | SteamAPIInitError::VersionMismatch(msg) => Error::from_reason(msg),
        })?;

    steam_client.user_stats().request_current_stats();

    client::set_client(steam_client);

    Ok(())
}

#[napi]
pub fn restart_app_if_necessary(app_id: u32) -> bool {
    steamworks::restart_app_if_necessary(AppId(app_id))
}

#[napi]
pub fn run_callbacks() {
    let c = client::get_client();

    // https://github.com/Noxime/steamworks-rs/blob/master/examples/networking-messages/src/main.rs
    // networking message acceptance is handled differently than regular callbacks and it cannot be serialized
    // at most we allow you to control accepting or rejecting a session request at the rust level

    c.networking_messages().session_request_callback(move |req| {
      println!("Accepting session request from {:?}", req.remote());
      // assert!(req.accept());
      assert!(req.accept());
      // mimicing the assert! causes it to crash
    });

    c.networking_messages().session_failed_callback(|info| {
      println!("Session failed: {:?}", info);
    });

    c.run_callbacks();
}

pub mod api;
