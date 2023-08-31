use napi_derive::napi;

#[napi]
pub mod utils {
    use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
    use steamworks::FloatingGamepadTextInputMode as kFloatingGamepadTextInputMode;
    use steamworks::GamepadTextInputLineMode as kGamepadTextInputLineMode;
    use steamworks::GamepadTextInputMode as kGamepadTextInputMode;

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
    pub enum GamepadTextInputMode {
        Normal,
        Password,
    }

    #[napi]
    pub enum GamepadTextInputLineMode {
        SingleLine,
        MultipleLines,
    }

    #[napi]
    pub fn show_gamepad_text_input(
        input_mode: GamepadTextInputMode,
        input_line_mode: GamepadTextInputLineMode,
        description: String,
        max_characters: u32,
        existing_text: Option<String>,
    ) -> bool {
        let client = crate::client::get_client();
        let dismissed_cb = |_| {};
        client.utils().show_gamepad_text_input(
            match input_mode {
                GamepadTextInputMode::Normal => kGamepadTextInputMode::Normal,
                GamepadTextInputMode::Password => kGamepadTextInputMode::Password,
            },
            match input_line_mode {
                GamepadTextInputLineMode::SingleLine => kGamepadTextInputLineMode::SingleLine,
                GamepadTextInputLineMode::MultipleLines => kGamepadTextInputLineMode::MultipleLines,
            },
            &description,
            max_characters,
            existing_text.as_deref(),
            dismissed_cb,
        )
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
