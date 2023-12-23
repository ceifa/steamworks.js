use napi_derive::napi;

#[napi]
pub mod auth {
    use std::net::SocketAddr;

    use napi::bindgen_prelude::{BigInt, Buffer, Error};
    use steamworks::{
        networking_types::NetworkingIdentity, AuthSessionTicketResponse, AuthTicket, SteamId,
        TicketForWebApiResponse,
    };
    use tokio::sync::oneshot;

    #[napi]
    pub struct Ticket {
        pub(crate) data: Vec<u8>,
        pub(crate) handle: AuthTicket,
    }

    #[napi]
    impl Ticket {
        #[napi]
        pub fn cancel(&mut self) {
            let client = crate::client::get_client();
            client.user().cancel_authentication_ticket(self.handle);
        }

        #[napi]
        pub fn get_bytes(&self) -> Buffer {
            self.data.clone().into()
        }
    }

    /// @param steamId64 - The user steam id or game server steam id. Use as NetworkIdentity of the remote system that will authenticate the ticket. If it is peer-to-peer then the user steam ID. If it is a game server, then the game server steam ID may be used if it was obtained from a trusted 3rd party
    /// @param timeoutSeconds - The number of seconds to wait for the ticket to be validated. Default value is 10 seconds.
    #[napi]
    pub async fn get_session_ticket_with_steam_id(
        steam_id64: BigInt,
        timeout_seconds: Option<u32>,
    ) -> Result<Ticket, Error> {
        get_session_ticket(
            NetworkingIdentity::new_steam_id(SteamId::from_raw(steam_id64.get_u64().1)),
            timeout_seconds,
        )
        .await
    }

    /// @param ip - The string of IPv4 or IPv6 address. Use as NetworkIdentity of the remote system that will authenticate the ticket.
    /// @param timeoutSeconds - The number of seconds to wait for the ticket to be validated. Default value is 10 seconds.
    #[napi]
    pub async fn get_session_ticket_with_ip(
        ip: String,
        timeout_seconds: Option<u32>,
    ) -> Result<Ticket, Error> {
        match ip.parse::<SocketAddr>() {
            Ok(addr) => get_session_ticket(NetworkingIdentity::new_ip(addr), timeout_seconds).await,
            Err(e) => Err(Error::from_reason(e.to_string())),
        }
    }

    /// @param networkIdentity - The identity of the remote system that will authenticate the ticket. If it is peer-to-peer then the user steam ID. If it is a game server, then the game server steam ID may be used if it was obtained from a trusted 3rd party, otherwise use the IP address. If it is a service, a string identifier of that service if one if provided.
    /// @param timeoutSeconds - The number of seconds to wait for the ticket to be validated. Default value is 10 seconds.
    async fn get_session_ticket(
        network_identity: NetworkingIdentity,
        timeout_seconds: Option<u32>,
    ) -> Result<Ticket, Error> {
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();
        let mut tx = Some(tx);

        let (ticket_handle, ticket) = client
            .user()
            .authentication_session_ticket(network_identity);

        let callback =
            client.register_callback(move |session_ticket_response: AuthSessionTicketResponse| {
                if session_ticket_response.ticket == ticket_handle {
                    if let Some(tx) = tx.take() {
                        tx.send(match session_ticket_response.result {
                            Ok(()) => Ok(()),
                            Err(e) => Err(Error::from_reason(e.to_string())),
                        })
                        .unwrap();
                    }
                }
            });

        let mut ticket = Ticket {
            data: ticket,
            handle: ticket_handle,
        };

        let timeout_seconds = u64::from(timeout_seconds.unwrap_or(10));
        let result =
            tokio::time::timeout(std::time::Duration::from_secs(timeout_seconds), rx).await;

        drop(callback);

        match result {
            Ok(result) => match result {
                Ok(Ok(())) => Ok(ticket),
                Ok(Err(e)) => {
                    ticket.cancel();
                    Err(e)
                }
                Err(e) => {
                    ticket.cancel();
                    Err(Error::from_reason(e.to_string()))
                }
            },
            Err(_) => {
                ticket.cancel();
                Err(Error::from_reason(
                    "Steam didn't validated the ticket in time.",
                ))
            }
        }
    }

    #[napi]
    pub async fn get_auth_ticket_for_web_api(
        identity: String,
        timeout_seconds: Option<u32>,
    ) -> Result<Ticket, Error> {
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();
        let mut tx = Some(tx);

        let ticket_handle = client
            .user()
            .authentication_session_ticket_for_webapi(&identity);

        let callback =
            client.register_callback(move |ticket_for_webapi_response: TicketForWebApiResponse| {
                if ticket_for_webapi_response.ticket_handle == ticket_handle {
                    let mut ticket = ticket_for_webapi_response.ticket;
                    ticket.truncate(ticket_for_webapi_response.ticket_len as usize);

                    if let Some(tx) = tx.take() {
                        tx.send(match ticket_for_webapi_response.result {
                            Ok(()) => Ok(ticket),
                            Err(e) => Err(Error::from_reason(e.to_string())),
                        })
                        .unwrap();
                    }
                }
            });

        let timeout_seconds = u64::from(timeout_seconds.unwrap_or(10));
        let result =
            tokio::time::timeout(std::time::Duration::from_secs(timeout_seconds), rx).await;

        drop(callback);

        match result {
            Ok(result) => match result {
                Ok(Ok(data)) => Ok(Ticket {
                    handle: ticket_handle,
                    data,
                }),
                Ok(Err(e)) => {
                    client.user().cancel_authentication_ticket(ticket_handle);
                    Err(e)
                }
                Err(e) => {
                    client.user().cancel_authentication_ticket(ticket_handle);
                    Err(Error::from_reason(e.to_string()))
                }
            },
            Err(_) => {
                client.user().cancel_authentication_ticket(ticket_handle);
                Err(Error::from_reason(
                    "Steam didn't validated the ticket in time.",
                ))
            }
        }
    }
}
