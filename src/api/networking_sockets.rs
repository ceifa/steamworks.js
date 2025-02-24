use napi_derive::napi;

#[napi]
pub mod networking_sockets {
    use napi::{
        bindgen_prelude::{Array, BigInt, Buffer, Object},
        Error, JsObject,
    };

    use std::net::SocketAddr;
    use std::iter;

    use steamworks::{
      networking_sockets::{
        ListenSocket
      },
      networking_types::{
        NetworkingConfigEntry
      }
    };

    #[napi]
    pub fn create_listen_socket_ip(
      local_address: u16
    ) -> Result<bool, Error>
    {
      let local_address = SocketAddr::from(([0, 0, 0, 0], local_address));

      let client = crate::client::get_client();
      let handle = client.networking_sockets().create_listen_socket_ip(
        local_address,
        iter::empty::<NetworkingConfigEntry>()
      );

      Ok(handle.is_ok())
    }
}