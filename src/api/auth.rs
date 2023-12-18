use napi_derive::napi;

#[napi]
pub mod auth {
    use napi::bindgen_prelude::{BigInt, Buffer, Error};
    use steamworks::{AuthSessionTicketResponse, AuthTicket, SteamId, TicketForWebApiResponse};
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

    /// @param steam_id64 - The 64bit SteamId.
    /// @param timeoutSeconds - The number of seconds to wait for the ticket to be validated. Default value is 10 seconds.
    #[napi]
    pub async fn get_session_ticket_with_steam_id(
        steam_id64: BigInt,
        timeout_seconds: Option<u32>,
    ) -> Result<Ticket, Error> {
        let client = crate::client::get_client();
        let (tx, rx) = oneshot::channel();
        let mut tx = Some(tx);

        let (ticket_handle, ticket) = client
            .user()
            .authentication_session_ticket_with_steam_id(SteamId::from_raw(steam_id64.get_u64().1));
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
