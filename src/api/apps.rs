use super::localplayer::PlayerSteamId;
use napi_derive::napi;

#[napi]
pub mod apps {
    use super::PlayerSteamId;
    use steamworks::AppId;

    #[napi]
    pub fn is_subscribed_app(app_id: u32) -> bool {
        let client = crate::client::get_client();
        client.apps().is_subscribed_app(AppId(app_id))
    }
    #[napi]
    pub fn is_app_installed(app_id: u32) -> bool {
        let client = crate::client::get_client();
        client.apps().is_app_installed(AppId(app_id))
    }

    #[napi]
    pub fn is_dlc_installed(app_id: u32) -> bool {
        let client = crate::client::get_client();
        client.apps().is_dlc_installed(AppId(app_id))
    }

    #[napi]
    pub fn is_subscribed_from_free_weekend() -> bool {
        let client = crate::client::get_client();
        client.apps().is_subscribed_from_free_weekend()
    }

    #[napi]
    pub fn is_vac_banned() -> bool {
        let client = crate::client::get_client();
        client.apps().is_vac_banned()
    }

    #[napi]
    pub fn is_cybercafe() -> bool {
        let client = crate::client::get_client();
        client.apps().is_cybercafe()
    }

    #[napi]
    pub fn is_low_violence() -> bool {
        let client = crate::client::get_client();
        client.apps().is_low_violence()
    }

    #[napi]
    pub fn is_subscribed() -> bool {
        let client = crate::client::get_client();
        client.apps().is_subscribed()
    }

    #[napi]
    pub fn app_build_id() -> i32 {
        let client = crate::client::get_client();
        client.apps().app_build_id()
    }

    #[napi]
    pub fn app_install_dir(app_id: u32) -> String {
        let client = crate::client::get_client();
        client.apps().app_install_dir(AppId(app_id))
    }

    #[napi]
    pub fn app_owner() -> PlayerSteamId {
        let client = crate::client::get_client();
        let steam_id = client.apps().app_owner();
        PlayerSteamId::from_steamid(steam_id)
    }

    #[napi]
    pub fn available_game_languages() -> Vec<String> {
        let client = crate::client::get_client();
        client.apps().available_game_languages()
    }

    #[napi]
    pub fn current_game_language() -> String {
        let client = crate::client::get_client();
        client.apps().current_game_language()
    }

    #[napi]
    pub fn current_beta_name() -> Option<String> {
        let client = crate::client::get_client();
        client.apps().current_beta_name()
    }
}
