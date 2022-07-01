use napi_derive::napi;

#[napi]
pub mod callback {
    use napi::{
        bindgen_prelude::ToNapiValue,
        threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
        JsFunction,
    };
    use steamworks::{
        CallbackHandle, PersonaStateChange, SteamServerConnectFailure, SteamServersConnected,
        SteamServersDisconnected,
    };

    #[napi]
    pub struct Handle {
        handle: Option<CallbackHandle>,
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
    }

    #[napi(ts_generic_types = "C extends keyof import('./callbacks').CallbackReturns")]
    pub fn register(
        steam_callback: SteamCallback,
        #[napi(ts_arg_type = "(value: import('./callbacks').CallbackReturns[C]) => void")]
        handler: JsFunction,
    ) -> Handle {
        let threadsafe_handler: ThreadsafeFunction<serde_json::Value, ErrorStrategy::Fatal> =
            handler
                .create_threadsafe_function(0, |ctx| Ok(vec![ctx.value]))
                .unwrap();

        let handle = match steam_callback {
            SteamCallback::PersonaStateChange => {
                register_callback::<PersonaStateChange>(threadsafe_handler)
            }
            SteamCallback::SteamServersConnected => {
                register_callback::<SteamServersConnected>(threadsafe_handler)
            }
            SteamCallback::SteamServersDisconnected => {
                register_callback::<SteamServersDisconnected>(threadsafe_handler)
            }
            SteamCallback::SteamServerConnectFailure => {
                register_callback::<SteamServerConnectFailure>(threadsafe_handler)
            }
        };

        Handle {
            handle: Some(handle),
        }
    }

    fn register_callback<C>(
        threadsafe_handler: ThreadsafeFunction<serde_json::Value, ErrorStrategy::Fatal>,
    ) -> CallbackHandle
    where
        C: steamworks::Callback + serde::Serialize,
    {
        let client = crate::client::get_client();
        client.register_callback(move |value: C| {
            let value = serde_json::to_value(&value).unwrap();
            threadsafe_handler.call(
                serde_json::to_value(value).unwrap(),
                ThreadsafeFunctionCallMode::Blocking,
            );
        })
    }
}
