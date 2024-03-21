use napi_derive::napi;

#[napi]
pub mod networking_messages {
  use napi::bindgen_prelude::{BigInt, Buffer, Error};
  use steamworks::SteamError;
  use steamworks::SteamId;
  use crate::api::localplayer::PlayerSteamId;
  use steamworks::ClientManager;
  use steamworks::networking_types::SendFlags;
  use steamworks::networking_types::NetworkingIdentity;
  use steamworks::networking_types::NetworkingMessage;

  #[napi(object)]
  pub struct Message {
    pub data: Buffer,
    pub steam_id: Option<PlayerSteamId>,
  }

  #[napi]
  pub fn send_message_to_user(
    steam_id64: BigInt, 
    data: Buffer
  ) -> Result<(), Error> {
    let client = crate::client::get_client();
    let steam_id = SteamId::from_raw(steam_id64.get_u64().1);
    let identity = NetworkingIdentity::new_steam_id(steam_id);

    fn err(steam_err: SteamError) -> Result<(), Error> {
      let mut str: String = "Can't send message: ".to_owned();
      let borrowed_string: &str = &steam_err.to_string();
      
      str.push_str(borrowed_string);
      Result::Err(
        Error::new(
          napi::Status::GenericFailure,
          str,
        )
      )
    }

    client
      .networking_messages()
      .send_message_to_user(
        identity, 
        SendFlags::RELIABLE_NO_NAGLE & SendFlags::AUTO_RESTART_BROKEN_SESSION, 
        &data, 
        0
      ).or_else(err)
  }

  #[napi]
  pub fn receive_messages_on_channel() -> Vec<Message> {
    let client = crate::client::get_client();
    
    client
      .networking_messages()
      .receive_messages_on_channel(0, 10)
      .iter().map(|m:&NetworkingMessage<ClientManager>| {
        let steam_id = m.identity_peer().steam_id();
        
        if let Some(steam_id) = steam_id {
          let player_steam_id = PlayerSteamId::from_steamid(steam_id);
          
          return Message { data: m.data().into(), steam_id: Some(player_steam_id) };
        } else {
          return Message { data: m.data().into(), steam_id: None };
        }
        
      }).collect()
  }

}