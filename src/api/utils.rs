use napi_derive::napi;

#[napi]
pub mod utils {
    #[napi]
    pub fn get_app_id() -> u32 {
        let client = crate::client::get_client();
        client.utils().app_id().0
    }

    #[napi]
    pub fn get_server_real_time() -> u32 {
        let client = crate::client::get_client();
        client.utils().get_server_real_time()
    }

    #[napi]
    pub fn is_steam_running_on_steam_deck() -> bool {
        let client = crate::client::get_client();
        client.utils().is_steam_running_on_steam_deck()
    }
}
