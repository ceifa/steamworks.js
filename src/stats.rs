use super::*;

#[napi_derive::napi]
pub fn get_stat_int(name: String) -> Option<i32> {
    let client = client::get_client();
    let result = client.user_stats().get_stat_i32(&name);

    match result {
        Ok(stat) => Some(stat),
        Err(()) => None,
    }
}

#[napi_derive::napi]
pub fn set_stat_int(name: String, value: i32) -> bool {
    let client = client::get_client();
    let result = client.user_stats().set_stat_i32(&name, value);
    result.is_ok()
}

#[napi_derive::napi]
pub fn store_stats() -> bool {
    let client = client::get_client();
    let result = client.user_stats().store_stats();
    result.is_ok()
}

#[napi_derive::napi]
pub fn reset_all_stats(achievements_too: bool) -> bool {
    let client = client::get_client();
    let result = client.user_stats().reset_all_stats(achievements_too);
    result.is_ok()
}
