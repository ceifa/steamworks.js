use napi_derive::napi;

#[napi]
pub mod matchmaking {
    use napi::bindgen_prelude::{BigInt, Error, ToNapiValue};
    use steamworks::LobbyId;
    use tokio::sync::oneshot;

    #[napi]
    pub enum LobbyType {
        Private,
        FriendsOnly,
        Public,
        Invisible,
    }

    #[napi]
    pub struct Lobby {
        pub id: BigInt,
        lobby_id: LobbyId,
    }

    #[napi]
    impl Lobby {
        #[napi]
        pub async fn join(&self) -> Result<(), Error> {
            match join_jobby(self.id.clone()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }

        #[napi]
        pub fn leave(&self) {
            let client = crate::client::get_client();
            client.matchmaking().leave_lobby(self.lobby_id);
        }

        #[napi]
        pub fn open_invite_dialog(&self) {
            let client = crate::client::get_client();
            client.friends().activate_invite_dialog(self.lobby_id);
        }

        #[napi]
        pub fn get_member_count(&self) -> usize {
            let client = crate::client::get_client();
            client.matchmaking().lobby_member_count(self.lobby_id)
        }

        #[napi]
        pub fn get_member_limit(&self) -> Option<usize> {
            let client = crate::client::get_client();
            client.matchmaking().lobby_member_limit(self.lobby_id)
        }

        #[napi]
        pub fn get_members(&self) {
            let client = crate::client::get_client();
            client.matchmaking().lobby_members(self.lobby_id);
        }

        #[napi]
        pub fn get_owner(&self) -> u64 {
            let client = crate::client::get_client();
            client.matchmaking().lobby_owner(self.lobby_id).raw()
        }

        #[napi]
        pub fn set_joinable(&self, joinable: bool) -> bool {
            let client = crate::client::get_client();
            client
                .matchmaking()
                .set_lobby_joinable(self.lobby_id, joinable)
        }
    }

    #[napi]
    pub async fn create_lobby(lobby_type: LobbyType, max_members: u32) -> Result<Lobby, Error> {
        let client = crate::client::get_client();

        let (tx, rx) = oneshot::channel();

        client.matchmaking().create_lobby(
            match lobby_type {
                LobbyType::Private => steamworks::LobbyType::Private,
                LobbyType::FriendsOnly => steamworks::LobbyType::FriendsOnly,
                LobbyType::Public => steamworks::LobbyType::Public,
                LobbyType::Invisible => steamworks::LobbyType::Invisible,
            },
            max_members,
            |result| {
                tx.send(result).unwrap();
            },
        );

        let result = rx.await.unwrap();
        match result {
            Ok(lobby_id) => Ok(Lobby {
                id: BigInt::from(lobby_id.raw()),
                lobby_id,
            }),
            Err(e) => Err(Error::from_reason(e.to_string())),
        }
    }

    #[napi]
    pub async fn join_jobby(lobby_id: BigInt) -> Result<Lobby, Error> {
        let client = crate::client::get_client();

        let (tx, rx) = oneshot::channel();

        client.matchmaking().join_lobby(
            steamworks::LobbyId::from_raw(lobby_id.get_u64().1),
            |result| {
                tx.send(result).unwrap();
            },
        );

        let result = rx.await.unwrap();
        match result {
            Ok(lobby_id) => Ok(Lobby {
                id: BigInt::from(lobby_id.raw()),
                lobby_id,
            }),
            Err(_) => Err(Error::from_reason("Failed to join lobby".to_string())),
        }
    }

    #[napi]
    pub async fn get_lobbies() -> Result<Vec<Lobby>, Error> {
        let client = crate::client::get_client();

        let (tx, rx) = oneshot::channel();

        client.matchmaking().request_lobby_list(|lobbies| {
            tx.send(lobbies).unwrap();
        });

        let lobbies = rx.await.unwrap();

        match lobbies {
            Ok(lobbies) => Ok(lobbies
                .iter()
                .map(|lobby_id| Lobby {
                    id: BigInt::from(lobby_id.raw()),
                    lobby_id: *lobby_id,
                })
                .collect()),
            Err(e) => Err(Error::from_reason(e.to_string())),
        }
    }
}
