use napi_derive::napi;

/// Other users' personas (`ISteamFriends`). Steam delivers the persona of every
/// member of a lobby the local user is in, so none of this needs the web.
#[napi]
pub mod friends {
    use napi::bindgen_prelude::{BigInt, Buffer};
    use steamworks::{FriendState, SteamId};

    #[napi(object)]
    pub struct Persona {
        pub name: String,
        /// `offline`, `online`, `invisible`, `busy`, `away`, `snooze`,
        /// `lookingToTrade` or `lookingToPlay`.
        pub state: String,
        /// The 64x64 avatar as RGBA bytes, or `null` while Steam has not
        /// delivered it yet (a `PersonaStateChange` follows when it does).
        pub medium_avatar: Option<Buffer>,
    }

    /// The variant name with a lower-case first letter (`Online` -> `online`).
    fn state_name(state: FriendState) -> String {
        let name = format!("{:?}", state);
        let mut chars = name.chars();
        chars.next().map_or(String::new(), |first| {
            first.to_lowercase().chain(chars).collect()
        })
    }

    /// What Steam holds for a user: their name, state and avatar.
    /// {@link https://partner.steamgames.com/doc/api/ISteamFriends#GetFriendPersonaName}
    #[napi]
    pub fn get_persona(steam_id64: BigInt) -> Persona {
        let client = crate::client::get_client();
        let friend = client
            .friends()
            .get_friend(SteamId::from_raw(steam_id64.get_u64().1));
        Persona {
            name: friend.name(),
            state: state_name(friend.state()),
            medium_avatar: friend.medium_avatar().map(Buffer::from),
        }
    }

    /// Asks Steam for a user's persona. True when it is being fetched and a
    /// `PersonaStateChange` will follow; false when it is already held.
    /// {@link https://partner.steamgames.com/doc/api/ISteamFriends#RequestUserInformation}
    #[napi]
    pub fn request_user_information(steam_id64: BigInt, name_only: bool) -> bool {
        let client = crate::client::get_client();
        client
            .friends()
            .request_user_information(SteamId::from_raw(steam_id64.get_u64().1), name_only)
    }
}
