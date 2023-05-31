use napi_derive::napi;

#[napi]
pub mod overlay {
    use napi::bindgen_prelude::{BigInt, FromNapiValue, ToNapiValue};
    use std::fmt;
    use steamworks::OverlayToStoreFlag;

    #[napi]
    pub enum Dialog {
        Friends,
        Community,
        Players,
        Settings,
        OfficialGameGroup,
        Stats,
        Achievements,
    }

    impl fmt::Display for Dialog {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Dialog::Friends => write!(f, "friends"),
                Dialog::Community => write!(f, "community"),
                Dialog::Players => write!(f, "players"),
                Dialog::Settings => write!(f, "settings"),
                Dialog::OfficialGameGroup => write!(f, "officialgamegroup"),
                Dialog::Stats => write!(f, "stats"),
                Dialog::Achievements => write!(f, "achievements"),
            }
        }
    }

    #[napi]
    pub enum StoreFlag {
        None,
        AddToCart,
        AddToCartAndShow,
    }

    #[napi]
    pub fn activate_dialog(dialog: Dialog) {
        let client = crate::client::get_client();
        client.friends().activate_game_overlay(&dialog.to_string())
    }

    #[napi]
    pub fn activate_dialog_to_user(dialog: Dialog, steam_id64: BigInt) {
        let client = crate::client::get_client();
        client.friends().activate_game_overlay_to_user(
            &dialog.to_string(),
            steamworks::SteamId::from_raw(steam_id64.get_u64().1),
        )
    }

    #[napi]
    pub fn activate_invite_dialog(lobby_id: BigInt) {
        let client = crate::client::get_client();
        client
            .friends()
            .activate_invite_dialog(steamworks::LobbyId::from_raw(lobby_id.get_u64().1))
    }

    #[napi]
    pub fn activate_to_web_page(url: String) {
        let client = crate::client::get_client();
        client.friends().activate_game_overlay_to_web_page(&url)
    }

    #[napi]
    pub fn activate_to_store(app_id: u32, flag: StoreFlag) {
        let client = crate::client::get_client();
        client.friends().activate_game_overlay_to_store(
            steamworks::AppId(app_id),
            match flag {
                StoreFlag::None => OverlayToStoreFlag::None,
                StoreFlag::AddToCart => OverlayToStoreFlag::AddToCart,
                StoreFlag::AddToCartAndShow => OverlayToStoreFlag::AddToCartAndShow,
            },
        )
    }
}
