#[path = "client.rs"]
pub mod client;

#[napi_derive::napi]
pub fn get_level() -> u32 {
    let client = client::get_client();
    client.user().level()
}