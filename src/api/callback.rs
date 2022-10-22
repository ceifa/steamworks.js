use napi_derive::napi;

#[napi]
pub mod callback {
    use napi::{
        bindgen_prelude::ToNapiValue,
        threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
        JsFunction,
    };

    #[napi]
    pub struct Handle {
        handle: Option<steamworks::CallbackHandle>,
    }

    #[napi]
    impl Handle {
        #[napi]
        pub fn disconnect(&mut self) {
            self.handle = None;
        }
    }

    #[napi]
    pub enum SteamCallback {
        PersonaStateChange,
        SteamServersConnected,
        SteamServersDisconnected,
        SteamServerConnectFailure,
        LobbyDataUpdate,
        LobbyChatUpdate,
        P2PSessionRequest,
        P2PSessionConnectFail,
    }

    #[napi(ts_generic_types = "C extends keyof import('./callbacks').CallbackReturns")]
    pub fn register(
        #[napi(ts_arg_type = "C")] steam_callback: SteamCallback,
        #[napi(ts_arg_type = "(value: import('./callbacks').CallbackReturns[C]) => void")] handler: JsFunction,
    ) -> Handle {
        let threadsafe_handler: ThreadsafeFunction<serde_json::Value, ErrorStrategy::Fatal> =
            handler
                .create_threadsafe_function(0, |ctx| Ok(vec![ctx.value]))
                .unwrap();

        let handle = match steam_callback {
            SteamCallback::PersonaStateChange => {
                register_callback::<steamworks::PersonaStateChange>(threadsafe_handler)
            }
            SteamCallback::SteamServersConnected => {
                register_callback::<steamworks::SteamServersConnected>(threadsafe_handler)
            }
            SteamCallback::SteamServersDisconnected => {
                register_callback::<steamworks::SteamServersDisconnected>(threadsafe_handler)
            }
            SteamCallback::SteamServerConnectFailure => {
                register_callback::<steamworks::SteamServerConnectFailure>(threadsafe_handler)
            }
            SteamCallback::LobbyDataUpdate => {
                register_callback::<steamworks::LobbyDataUpdate>(threadsafe_handler)
            }
            SteamCallback::LobbyChatUpdate => {
                register_callback::<steamworks::LobbyChatUpdate>(threadsafe_handler)
            }
            SteamCallback::P2PSessionRequest => {
                register_callback::<steamworks::P2PSessionRequest>(threadsafe_handler)
            }
            SteamCallback::P2PSessionConnectFail => {
                register_callback::<steamworks::P2PSessionConnectFail>(threadsafe_handler)
            }
        };

        Handle {
            handle: Some(handle),
        }
    }

    fn register_callback<C>(
        threadsafe_handler: ThreadsafeFunction<serde_json::Value, ErrorStrategy::Fatal>,
    ) -> steamworks::CallbackHandle
    where
        C: steamworks::Callback + serde::Serialize,
    {
        let client = crate::client::get_client();
        client.register_callback(move |value: C| {
            let value = serde_json::to_value(&value).unwrap();
            threadsafe_handler.call(value, ThreadsafeFunctionCallMode::Blocking);
        })
    }
}
