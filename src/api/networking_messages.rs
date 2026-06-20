use napi_derive::napi;

#[napi]
pub mod networking_messages {
    use std::{
        ffi::{c_void, CStr},
        os::raw::c_char,
    };

    use napi::{
        bindgen_prelude::{BigInt, Buffer, Error},
        threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
        JsFunction,
    };
    use steamworks::{
        networking_types::{NetworkingIdentity, SendFlags},
        sys, Callback, CallbackHandle, SteamId,
    };

    use crate::api::localplayer::PlayerSteamId;

    const DEFAULT_CHANNEL: u32 = 0;
    const DEFAULT_MAX_MESSAGES: u32 = 10;

    #[napi(object)]
    pub struct Message {
        pub data: Buffer,
        pub steam_id: Option<PlayerSteamId>,
    }

    #[napi(object)]
    pub struct ConnectionInfo {
        pub steam_id: Option<PlayerSteamId>,
        pub user_data: BigInt,
        pub listen_socket: Option<u32>,
        pub state: i32,
        pub state_name: String,
        pub end_reason: Option<i32>,
        pub end_debug: String,
        pub connection_description: String,
        pub flags: i32,
    }

    #[napi(object)]
    pub struct ConnectionQuickStatus {
        pub state: i32,
        pub state_name: String,
        pub ping: i32,
        pub connection_quality_local: f64,
        pub connection_quality_remote: f64,
        pub out_packets_per_second: f64,
        pub out_bytes_per_second: f64,
        pub in_packets_per_second: f64,
        pub in_bytes_per_second: f64,
        pub send_rate_bytes_per_second: i32,
        pub pending_unreliable: i32,
        pub pending_reliable: i32,
        pub sent_unacked_reliable: i32,
        pub queue_time_microseconds: BigInt,
    }

    #[napi(object)]
    pub struct SessionConnectionInfo {
        pub state: i32,
        pub state_name: String,
        pub connection_info: ConnectionInfo,
        pub quick_status: ConnectionQuickStatus,
    }

    #[napi(object)]
    pub struct SessionRequest {
        pub steam_id: Option<PlayerSteamId>,
    }

    #[napi(object)]
    pub struct SessionFailed {
        pub connection_info: ConnectionInfo,
    }

    #[napi]
    pub struct SessionRequestCallbackHandle {
        handle: Option<CallbackHandle>,
    }

    #[napi]
    impl SessionRequestCallbackHandle {
        #[napi]
        pub fn disconnect(&mut self) {
            if let Some(handle) = self.handle.take() {
                handle.disconnect();
            }
        }
    }

    #[napi]
    pub struct SessionFailedCallbackHandle {
        handle: Option<CallbackHandle>,
    }

    #[napi]
    impl SessionFailedCallbackHandle {
        #[napi]
        pub fn disconnect(&mut self) {
            if let Some(handle) = self.handle.take() {
                handle.disconnect();
            }
        }
    }

    struct NetworkingMessagesSessionRequest {
        remote: NetworkingIdentity,
    }

    struct NetworkingMessagesSessionFailed {
        connection_info: ConnectionInfo,
    }

    unsafe impl Callback for NetworkingMessagesSessionRequest {
        const ID: i32 = sys::SteamNetworkingMessagesSessionRequest_t_k_iCallback as _;
        const SIZE: i32 = std::mem::size_of::<sys::SteamNetworkingMessagesSessionRequest_t>() as _;

        unsafe fn from_raw(raw: *mut c_void) -> Self {
            let request = *(raw as *mut sys::SteamNetworkingMessagesSessionRequest_t);
            Self {
                remote: request.m_identityRemote.into(),
            }
        }
    }

    unsafe impl Callback for NetworkingMessagesSessionFailed {
        const ID: i32 = sys::SteamNetworkingMessagesSessionFailed_t_k_iCallback as _;
        const SIZE: i32 = std::mem::size_of::<sys::SteamNetworkingMessagesSessionFailed_t>() as _;

        unsafe fn from_raw(raw: *mut c_void) -> Self {
            let failed = *(raw as *mut sys::SteamNetworkingMessagesSessionFailed_t);
            Self {
                connection_info: connection_info_from_raw(failed.m_info),
            }
        }
    }

    fn networking_identity_from_steam_id64(steam_id64: BigInt) -> NetworkingIdentity {
        NetworkingIdentity::new_steam_id(SteamId::from_raw(steam_id64.get_u64().1))
    }

    fn raw_networking_identity_from_steam_id64(steam_id64: BigInt) -> sys::SteamNetworkingIdentity {
        let mut identity = unsafe { std::mem::zeroed() };
        unsafe {
            sys::SteamAPI_SteamNetworkingIdentity_Clear(&mut identity);
            sys::SteamAPI_SteamNetworkingIdentity_SetSteamID64(
                &mut identity,
                steam_id64.get_u64().1,
            );
        }
        identity
    }

    fn networking_messages_ptr() -> *mut sys::ISteamNetworkingMessages {
        let _client = crate::client::get_client();
        unsafe { sys::SteamAPI_SteamNetworkingMessages_SteamAPI_v002() }
    }

    fn connection_state_name(state: sys::ESteamNetworkingConnectionState) -> String {
        match state {
            sys::ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_None => "None",
            sys::ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_Connecting => {
                "Connecting"
            }
            sys::ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_FindingRoute => {
                "FindingRoute"
            }
            sys::ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_Connected => {
                "Connected"
            }
            sys::ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_ClosedByPeer => {
                "ClosedByPeer"
            }
            sys::ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_ProblemDetectedLocally => {
                "ProblemDetectedLocally"
            }
            sys::ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_FinWait => {
                "FinWait"
            }
            sys::ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_Linger => {
                "Linger"
            }
            sys::ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_Dead => "Dead",
            sys::ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState__Force32Bit => {
                "Force32Bit"
            }
            _ => "Unknown",
        }
        .to_string()
    }

    fn c_char_array_to_string(data: &[c_char]) -> String {
        unsafe { CStr::from_ptr(data.as_ptr()).to_string_lossy().into_owned() }
    }

    fn connection_info_from_raw(info: sys::SteamNetConnectionInfo_t) -> ConnectionInfo {
        let identity = NetworkingIdentity::from(info.m_identityRemote);
        let listen_socket = if info.m_hListenSocket == sys::k_HSteamListenSocket_Invalid {
            None
        } else {
            Some(info.m_hListenSocket)
        };
        let end_reason = if info.m_eEndReason
            == sys::ESteamNetConnectionEnd::k_ESteamNetConnectionEnd_Invalid as i32
        {
            None
        } else {
            Some(info.m_eEndReason)
        };

        ConnectionInfo {
            steam_id: identity.steam_id().map(PlayerSteamId::from_steamid),
            user_data: BigInt::from(info.m_nUserData),
            listen_socket,
            state: info.m_eState as i32,
            state_name: connection_state_name(info.m_eState),
            end_reason,
            end_debug: c_char_array_to_string(&info.m_szEndDebug),
            connection_description: c_char_array_to_string(&info.m_szConnectionDescription),
            flags: info.m_nFlags,
        }
    }

    fn quick_status_from_raw(
        status: sys::SteamNetConnectionRealTimeStatus_t,
    ) -> ConnectionQuickStatus {
        ConnectionQuickStatus {
            state: status.m_eState as i32,
            state_name: connection_state_name(status.m_eState),
            ping: status.m_nPing,
            connection_quality_local: f64::from(status.m_flConnectionQualityLocal),
            connection_quality_remote: f64::from(status.m_flConnectionQualityRemote),
            out_packets_per_second: f64::from(status.m_flOutPacketsPerSec),
            out_bytes_per_second: f64::from(status.m_flOutBytesPerSec),
            in_packets_per_second: f64::from(status.m_flInPacketsPerSec),
            in_bytes_per_second: f64::from(status.m_flInBytesPerSec),
            send_rate_bytes_per_second: status.m_nSendRateBytesPerSecond,
            pending_unreliable: status.m_cbPendingUnreliable,
            pending_reliable: status.m_cbPendingReliable,
            sent_unacked_reliable: status.m_cbSentUnackedReliable,
            queue_time_microseconds: BigInt::from(status.m_usecQueueTime),
        }
    }

    #[napi]
    pub fn send_message_to_user(
        steam_id64: BigInt,
        data: Buffer,
        channel: Option<u32>,
    ) -> Result<(), Error> {
        let client = crate::client::get_client();
        let identity = networking_identity_from_steam_id64(steam_id64);

        client
            .networking_messages()
            .send_message_to_user(
                identity,
                SendFlags::RELIABLE_NO_NAGLE | SendFlags::AUTO_RESTART_BROKEN_SESSION,
                &data,
                channel.unwrap_or(DEFAULT_CHANNEL),
            )
            .map_err(|e| Error::from_reason(format!("Failed to send message: {}", e)))
    }

    #[napi]
    pub fn receive_messages_on_channel(
        channel: Option<u32>,
        max_messages: Option<u32>,
    ) -> Vec<Message> {
        let client = crate::client::get_client();

        client
            .networking_messages()
            .receive_messages_on_channel(
                channel.unwrap_or(DEFAULT_CHANNEL),
                max_messages.unwrap_or(DEFAULT_MAX_MESSAGES) as usize,
            )
            .iter()
            .map(|message| Message {
                data: message.data().into(),
                steam_id: message
                    .identity_peer()
                    .steam_id()
                    .map(PlayerSteamId::from_steamid),
            })
            .collect()
    }

    #[napi]
    pub fn register_session_request_callback(
        #[napi(ts_arg_type = "(request: SessionRequest) => void")]
        handler: JsFunction,
    ) -> Result<SessionRequestCallbackHandle, Error> {
        let threadsafe_handler: ThreadsafeFunction<SessionRequest, ErrorStrategy::Fatal> = handler
            .create_threadsafe_function(0, |ctx| Ok(vec![ctx.value]))
            .map_err(|error| {
                Error::from_reason(format!(
                    "Failed to create session request callback: {}",
                    error
                ))
            })?;
        let client = crate::client::get_client();
        let handle = client.register_callback(move |request: NetworkingMessagesSessionRequest| {
            threadsafe_handler.call(
                SessionRequest {
                    steam_id: request.remote.steam_id().map(PlayerSteamId::from_steamid),
                },
                ThreadsafeFunctionCallMode::Blocking,
            );
        });

        Ok(SessionRequestCallbackHandle {
            handle: Some(handle),
        })
    }

    #[napi]
    pub fn register_session_failed_callback(
        #[napi(ts_arg_type = "(failed: SessionFailed) => void")]
        handler: JsFunction,
    ) -> Result<SessionFailedCallbackHandle, Error> {
        let threadsafe_handler: ThreadsafeFunction<SessionFailed, ErrorStrategy::Fatal> = handler
            .create_threadsafe_function(0, |ctx| Ok(vec![ctx.value]))
            .map_err(|error| {
                Error::from_reason(format!(
                    "Failed to create session failed callback: {}",
                    error
                ))
            })?;
        let client = crate::client::get_client();
        let handle = client.register_callback(move |failed: NetworkingMessagesSessionFailed| {
            threadsafe_handler.call(
                SessionFailed {
                    connection_info: failed.connection_info,
                },
                ThreadsafeFunctionCallMode::Blocking,
            );
        });

        Ok(SessionFailedCallbackHandle {
            handle: Some(handle),
        })
    }

    #[napi]
    pub fn accept_session_with_user(steam_id64: BigInt) -> bool {
        let identity = raw_networking_identity_from_steam_id64(steam_id64);
        let networking_messages = networking_messages_ptr();
        if networking_messages.is_null() {
            return false;
        }

        unsafe {
            sys::SteamAPI_ISteamNetworkingMessages_AcceptSessionWithUser(
                networking_messages,
                &identity,
            )
        }
    }

    #[napi]
    pub fn close_session_with_user(steam_id64: BigInt) -> bool {
        let identity = raw_networking_identity_from_steam_id64(steam_id64);
        let networking_messages = networking_messages_ptr();
        if networking_messages.is_null() {
            return false;
        }

        unsafe {
            sys::SteamAPI_ISteamNetworkingMessages_CloseSessionWithUser(
                networking_messages,
                &identity,
            )
        }
    }

    #[napi]
    pub fn close_channel_with_user(steam_id64: BigInt, channel: Option<u32>) -> bool {
        let identity = raw_networking_identity_from_steam_id64(steam_id64);
        let networking_messages = networking_messages_ptr();
        if networking_messages.is_null() {
            return false;
        }

        unsafe {
            sys::SteamAPI_ISteamNetworkingMessages_CloseChannelWithUser(
                networking_messages,
                &identity,
                channel.unwrap_or(DEFAULT_CHANNEL) as i32,
            )
        }
    }

    #[napi]
    pub fn get_session_connection_info(steam_id64: BigInt) -> Result<SessionConnectionInfo, Error> {
        let identity = raw_networking_identity_from_steam_id64(steam_id64);
        let networking_messages = networking_messages_ptr();
        if networking_messages.is_null() {
            return Err(Error::from_reason(
                "SteamNetworkingMessages interface is unavailable",
            ));
        }

        let mut connection_info = unsafe { std::mem::zeroed() };
        let mut quick_status = unsafe { std::mem::zeroed() };
        let state = unsafe {
            sys::SteamAPI_ISteamNetworkingMessages_GetSessionConnectionInfo(
                networking_messages,
                &identity,
                &mut connection_info,
                &mut quick_status,
            )
        };

        Ok(SessionConnectionInfo {
            state: state as i32,
            state_name: connection_state_name(state),
            connection_info: connection_info_from_raw(connection_info),
            quick_status: quick_status_from_raw(quick_status),
        })
    }
}
