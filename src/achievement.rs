use super::*;

#[napi_derive::napi]
pub fn activate_achievement(achievement: String) -> bool {
    let client = client::get_client();
    let result = client
        .user_stats()
        .achievement(&achievement)
        .set()
        .and_then(|_| client.user_stats().store_stats());

    result.is_ok()
}
