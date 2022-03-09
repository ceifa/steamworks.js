use steamworks::Client;
use napi::bindgen_prelude::*;
pub mod client;

#[macro_use]
extern crate lazy_static;

#[napi_derive::napi]
pub fn init(app_id: u32) -> Result<()> {
    let result = Client::init_app(app_id);
    match result {
        Ok((client, single)) => {
            client::set_client(client);
            client::set_single(single);
            Ok(())
        }
        Err(e) => Err(Error::new(Status::GenericFailure, e.to_string()))
    }
}

#[napi_derive::napi]
pub fn run_callbacks() {
    client::get_single().run_callbacks();
}

// other apis
pub mod localplayer;
pub mod achievement;
pub mod cloud;
pub mod stats;