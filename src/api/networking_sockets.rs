use napi_derive::napi;

#[napi]
pub mod networking_sockets {
    use std::{
        ffi::CStr,
        os::raw::c_char,
    };

    use napi::Error;
    use steamworks::sys;

    #[napi(object)]
    pub struct AuthenticationStatus {
        pub availability: i32,
        pub availability_name: String,
        pub debug: String,
    }

    fn networking_sockets_ptr() -> *mut sys::ISteamNetworkingSockets {
        let _client = crate::client::get_client();
        unsafe { sys::SteamAPI_SteamNetworkingSockets_SteamAPI_v012() }
    }

    fn availability_name(availability: sys::ESteamNetworkingAvailability) -> String {
        match availability {
            sys::ESteamNetworkingAvailability::k_ESteamNetworkingAvailability_CannotTry => {
                "CannotTry"
            }
            sys::ESteamNetworkingAvailability::k_ESteamNetworkingAvailability_Failed => "Failed",
            sys::ESteamNetworkingAvailability::k_ESteamNetworkingAvailability_Previously => {
                "Previously"
            }
            sys::ESteamNetworkingAvailability::k_ESteamNetworkingAvailability_Retrying => {
                "Retrying"
            }
            sys::ESteamNetworkingAvailability::k_ESteamNetworkingAvailability_NeverTried => {
                "NeverTried"
            }
            sys::ESteamNetworkingAvailability::k_ESteamNetworkingAvailability_Waiting => "Waiting",
            sys::ESteamNetworkingAvailability::k_ESteamNetworkingAvailability_Attempting => {
                "Attempting"
            }
            sys::ESteamNetworkingAvailability::k_ESteamNetworkingAvailability_Current => "Current",
            sys::ESteamNetworkingAvailability::k_ESteamNetworkingAvailability_Unknown => "Unknown",
            sys::ESteamNetworkingAvailability::k_ESteamNetworkingAvailability__Force32bit => {
                "Force32bit"
            }
            _ => "Unknown",
        }
        .to_string()
    }

    fn c_char_array_to_string(data: &[c_char]) -> String {
        unsafe { CStr::from_ptr(data.as_ptr()).to_string_lossy().into_owned() }
    }

    fn authentication_status_from_raw(
        availability: sys::ESteamNetworkingAvailability,
        details: sys::SteamNetAuthenticationStatus_t,
    ) -> AuthenticationStatus {
        AuthenticationStatus {
            availability: availability as i32,
            availability_name: availability_name(availability),
            debug: c_char_array_to_string(&details.m_debugMsg),
        }
    }

    #[napi]
    pub fn init_authentication() -> Result<AuthenticationStatus, Error> {
        let networking_sockets = networking_sockets_ptr();
        if networking_sockets.is_null() {
            return Err(Error::from_reason(
                "SteamNetworkingSockets interface is unavailable",
            ));
        }

        unsafe {
            sys::SteamAPI_ISteamNetworkingSockets_InitAuthentication(networking_sockets);
        }

        get_authentication_status()
    }

    #[napi]
    pub fn get_authentication_status() -> Result<AuthenticationStatus, Error> {
        let networking_sockets = networking_sockets_ptr();
        if networking_sockets.is_null() {
            return Err(Error::from_reason(
                "SteamNetworkingSockets interface is unavailable",
            ));
        }

        let mut details = unsafe { std::mem::zeroed() };
        let availability = unsafe {
            sys::SteamAPI_ISteamNetworkingSockets_GetAuthenticationStatus(
                networking_sockets,
                &mut details,
            )
        };

        Ok(authentication_status_from_raw(availability, details))
    }
}
