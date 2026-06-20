use napi_derive::napi;

#[napi]
pub mod networking_utils {
    use std::{
        ffi::CStr,
        os::raw::c_char,
    };

    use napi::Error;
    use steamworks::sys;

    #[napi(object)]
    pub struct RelayNetworkStatus {
        pub availability: i32,
        pub availability_name: String,
        pub ping_measurement_in_progress: bool,
        pub network_config: i32,
        pub network_config_name: String,
        pub any_relay: i32,
        pub any_relay_name: String,
        pub debug: String,
    }

    fn networking_utils_ptr() -> *mut sys::ISteamNetworkingUtils {
        let _client = crate::client::get_client();
        unsafe { sys::SteamAPI_SteamNetworkingUtils_SteamAPI_v004() }
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

    fn relay_network_status_from_raw(status: sys::SteamRelayNetworkStatus_t) -> RelayNetworkStatus {
        RelayNetworkStatus {
            availability: status.m_eAvail as i32,
            availability_name: availability_name(status.m_eAvail),
            ping_measurement_in_progress: status.m_bPingMeasurementInProgress != 0,
            network_config: status.m_eAvailNetworkConfig as i32,
            network_config_name: availability_name(status.m_eAvailNetworkConfig),
            any_relay: status.m_eAvailAnyRelay as i32,
            any_relay_name: availability_name(status.m_eAvailAnyRelay),
            debug: c_char_array_to_string(&status.m_debugMsg),
        }
    }

    #[napi]
    pub fn init_relay_network_access() -> Result<RelayNetworkStatus, Error> {
        let networking_utils = networking_utils_ptr();
        if networking_utils.is_null() {
            return Err(Error::from_reason(
                "SteamNetworkingUtils interface is unavailable",
            ));
        }

        unsafe {
            sys::SteamAPI_ISteamNetworkingUtils_InitRelayNetworkAccess(networking_utils);
        }

        get_relay_network_status()
    }

    #[napi]
    pub fn get_relay_network_status() -> Result<RelayNetworkStatus, Error> {
        let networking_utils = networking_utils_ptr();
        if networking_utils.is_null() {
            return Err(Error::from_reason(
                "SteamNetworkingUtils interface is unavailable",
            ));
        }

        let mut status = unsafe { std::mem::zeroed() };
        unsafe {
            sys::SteamAPI_ISteamNetworkingUtils_GetRelayNetworkStatus(
                networking_utils,
                &mut status,
            );
        }

        Ok(relay_network_status_from_raw(status))
    }
}
