#[path = "client.rs"]
pub mod client;

#[napi_derive::napi]
pub fn activate_achievement(achievement: String) -> bool {
    let client = client::get_client();
    let result = client.user_stats().achievement(&achievement).set();

    result.is_ok()
}
