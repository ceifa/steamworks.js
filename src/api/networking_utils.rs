use napi_derive::napi;

#[napi]
pub mod networking_utils {
  use napi::{
    bindgen_prelude::{BigInt, Buffer},
    Error
  };

  #[napi]
  pub fn init_relay_network_access() {
    let client = crate::client::get_client();
    client.networking_utils().init_relay_network_access()
  }

  #[napi]
  pub fn detailed_relay_network_status() -> Result<String, Error> {
    let client = crate::client::get_client();
    let status = client.networking_utils().detailed_relay_network_status();
    Ok(status.debugging_message().to_string())
  }
}