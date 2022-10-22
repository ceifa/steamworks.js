use napi_derive::napi;

#[napi]
pub mod networking {
    use napi::{
        bindgen_prelude::{Buffer, ToNapiValue},
        Error,
    };
    use steamworks::SteamId;

    use crate::api::localplayer::PlayerSteamId;

    #[napi(object)]
    pub struct P2PPacket {
        pub data: Buffer,
        pub size: i32,
        pub steam_id: PlayerSteamId,
    }

    #[napi]
    /// The method used to send a packet
    pub enum SendType {
        /// Send the packet directly over udp.
        ///
        /// Can't be larger than 1200 bytes
        Unreliable,
        /// Like `Unreliable` but doesn't buffer packets
        /// sent before the connection has started.
        UnreliableNoDelay,
        /// Reliable packet sending.
        ///
        /// Can't be larger than 1 megabyte.
        Reliable,
        /// Like `Reliable` but applies the nagle
        /// algorithm to packets being sent
        ReliableWithBuffering,
    }

    #[napi]
    pub fn send_p2p_packet(
        steam_id64: String,
        send_type: SendType,
        data: Buffer,
    ) -> Result<bool, Error> {
        let client = crate::client::get_client();
        if let Ok(steam_id64) = steam_id64.parse::<u64>() {
            let result = client.networking().send_p2p_packet(
                SteamId::from_raw(steam_id64),
                match send_type {
                    SendType::Unreliable => steamworks::SendType::Unreliable,
                    SendType::UnreliableNoDelay => steamworks::SendType::UnreliableNoDelay,
                    SendType::Reliable => steamworks::SendType::Reliable,
                    SendType::ReliableWithBuffering => steamworks::SendType::ReliableWithBuffering,
                },
                &data,
            );
            Ok(result)
        } else {
            Err(Error::new(
                napi::Status::GenericFailure,
                format!("Invalid SteamId64: {}", steam_id64),
            ))
        }
    }

    #[napi]
    pub fn is_p2p_packet_available() -> i32 {
        let client = crate::client::get_client();
        client
            .networking()
            .is_p2p_packet_available()
            .unwrap_or_default() as i32
    }

    #[napi]
    pub fn read_p2p_packet(size: i32) -> Result<P2PPacket, Error> {
        let client = crate::client::get_client();
        let mut buffer = vec![0; size as usize];
        if let Some((steam_id, read_size)) = client.networking().read_p2p_packet(&mut buffer) {
            Ok(P2PPacket {
                data: buffer.into(),
                size: read_size as i32,
                steam_id: PlayerSteamId::from_steamid(steam_id),
            })
        } else {
            Err(Error::new(
                napi::Status::GenericFailure,
                "No packet available".to_string(),
            ))
        }
    }

    #[napi]
    pub fn accept_p2p_session(steam_id64: String) -> Result<(), Error> {
        let client = crate::client::get_client();
        if let Ok(steam_id64) = steam_id64.parse::<u64>() {
            client
                .networking()
                .accept_p2p_session(SteamId::from_raw(steam_id64));
            Ok(())
        } else {
            Err(Error::new(
                napi::Status::GenericFailure,
                format!("Invalid SteamId64: {}", steam_id64),
            ))
        }
    }
}
