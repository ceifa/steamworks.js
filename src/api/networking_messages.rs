use napi_derive::napi;

#[napi]
pub mod networking_messages {
    use napi::bindgen_prelude::{BigInt, Buffer, Error};
    use steamworks::{
        networking_types::{NetworkingIdentity, SendFlags},
        SteamId,
    };

    use crate::api::localplayer::PlayerSteamId;

    const DEFAULT_CHANNEL: u32 = 0;
    const DEFAULT_MAX_MESSAGES: u32 = 10;

    #[napi(object)]
    pub struct Message {
        pub data: Buffer,
        pub steam_id: Option<PlayerSteamId>,
    }

    #[napi]
    pub fn send_message_to_user(
        steam_id64: BigInt,
        data: Buffer,
        channel: Option<u32>,
    ) -> Result<(), Error> {
        let client = crate::client::get_client();
        let steam_id = SteamId::from_raw(steam_id64.get_u64().1);
        let identity = NetworkingIdentity::new_steam_id(steam_id);

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
}
