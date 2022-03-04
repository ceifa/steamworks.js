#[path = "client.rs"]
pub mod client;

#[napi_derive::napi]
pub fn get_stat_i32(name: String) -> i32 {
    let client = client::get_client();
    let result = client.user_stats().get_stat_i32(&name);
    result.unwrap()
}