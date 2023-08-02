use napi_derive::napi;

#[napi]
pub mod utils {
    use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
    use steamworks::FloatingGamepadTextInputMode as kFloatingGamepadTextInputMode;

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

    #[napi]
    pub enum FloatingGamepadTextInputMode {
        SingleLine,
        MultipleLines,
        Email,
        Numeric,
    }

    #[napi]
    pub fn show_floating_gamepad_text_input(
        keyboard_mode: FloatingGamepadTextInputMode,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> bool {
        let client = crate::client::get_client();
        let dismissed_cb = || {};
        client.utils().show_floating_gamepad_text_input(
            match keyboard_mode {
                FloatingGamepadTextInputMode::SingleLine => {
                    kFloatingGamepadTextInputMode::SingleLine
                }
                FloatingGamepadTextInputMode::MultipleLines => {
                    kFloatingGamepadTextInputMode::MultipleLines
                }
                FloatingGamepadTextInputMode::Email => kFloatingGamepadTextInputMode::Email,
                FloatingGamepadTextInputMode::Numeric => kFloatingGamepadTextInputMode::Numeric,
            },
            x,
            y,
            width,
            height,
            dismissed_cb,
        )
    }
}
