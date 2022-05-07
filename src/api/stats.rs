use napi_derive::napi;

#[napi]
pub mod stats {
    #[napi]
    pub fn get_int(name: String) -> Option<i32> {
        let client = crate::client::get_client();
        let result = client.user_stats().get_stat_i32(&name);

        match result {
            Ok(stat) => Some(stat),
            Err(()) => None,
        }
    }

    #[napi]
    pub fn set_int(name: String, value: i32) -> bool {
        let client = crate::client::get_client();
        let result = client.user_stats().set_stat_i32(&name, value);
        result.is_ok()
    }

    #[napi]
    pub fn store() -> bool {
        let client = crate::client::get_client();
        let result = client.user_stats().store_stats();
        result.is_ok()
    }

    #[napi]
    pub fn reset_all(achievements_too: bool) -> bool {
        let client = crate::client::get_client();
        let result = client.user_stats().reset_all_stats(achievements_too);
        result.is_ok()
    }
}
