use napi_derive::napi;
use std::process;
// some credit here goes to https://github.com/apxapob as this was heavily inspired by his work

#[napi]
pub mod networking_messages {
    use napi::{
        bindgen_prelude::{BigInt, Buffer},
        Error
    };
    use steamworks::{
        SteamId,
        SteamError,
        ClientManager,
        networking_types::{
            SendFlags,
            NetworkingIdentity,
            NetworkingMessage,
        },
        networking_messages::SessionRequest,
    };

    use crate::api::localplayer::PlayerSteamId;

    fn err(steam_err:SteamError) -> Result<(), Error> {
      Err(Error::new(
        napi::Status::GenericFailure,
        steam_err.to_string()
      ))
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
    pub fn send_message_to_user(
      steam_id64: BigInt,
      send_type: SendType,
      data: Buffer,
      channel: Option<u32>
    ) -> Result<(), Error> {
      let client = crate::client::get_client();
      let steam_id = SteamId::from_raw(steam_id64.get_u64().1);
      let identity = NetworkingIdentity::new_steam_id(steam_id);

      println!("sending message in thread {}", std::process::id());

      client.networking_messages().send_message_to_user(
        identity,
        match send_type {
          SendType::Unreliable => SendFlags::UNRELIABLE,
          SendType::UnreliableNoDelay => SendFlags::UNRELIABLE_NO_DELAY,
          SendType::ReliableWithBuffering => SendFlags::RELIABLE, // nagle is the new default
          SendType::Reliable => SendFlags::RELIABLE_NO_NAGLE,
        } & SendFlags::AUTO_RESTART_BROKEN_SESSION,
        &data,
        channel.unwrap_or(0)
      ).or_else(err)
    }

    #[napi(object)]
    pub struct Message {
      pub data: Buffer,
      pub steam_id: Option<PlayerSteamId>,
    }

    #[napi]
    pub fn receive_messages_on_channel(
      channel: u32,
      batch_size: Option<u32>
    ) -> Vec<Message> {
      let client = crate::client::get_client();

      println!("receiving messages in thread {}", std::process::id());

      client
        .networking_messages()
        .receive_messages_on_channel(channel, batch_size.unwrap_or(10).try_into().unwrap())
        .iter().map(|m:&NetworkingMessage<ClientManager>| {
          let steam_id = m.identity_peer().steam_id();

          return Message {
            data: m.data().into(),
            steam_id: steam_id.map(|id| PlayerSteamId::from_steamid(id))
          }
        }).collect()
    }
}