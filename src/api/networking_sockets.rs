use napi_derive::napi;

#[napi]
pub mod networking_sockets {
    use napi::{bindgen_prelude::{BigInt, Buffer}, Error};
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::net::{
      Ipv4Addr, SocketAddr
    };
    use steamworks::{
        ClientManager as Manager,
        SteamId,
        networking_types::{
          SendFlags,
          NetworkingIdentity,
          ListenSocketEvent
        },
        networking_sockets::{
          ListenSocket, NetConnection,
        },
    };
    use crate::api::localplayer::PlayerSteamId;

    lazy_static! {
        static ref LISTEN_P2P: Mutex<Option<ListenSocket<Manager>>> = Mutex::new(None);
        static ref LISTEN_IP: Mutex<Option<ListenSocket<Manager>>> = Mutex::new(None);
        static ref CONNECTIONS: Mutex<HashMap<SteamId, NetConnection<Manager>>> = Mutex::new(HashMap::new());
        static ref ACCEPT_NEW_REQUESTS: Mutex<bool> = Mutex::new(true);
        static ref AM_I_SERVER: Mutex<bool> = Mutex::new(false);
    }

    // used to wait for new connections
    #[napi]
    pub fn create_listen_socket_p2p(local_virtual_port: Option<i32>) -> Result<bool, Error> {
        let client = crate::client::get_client();
        let port = local_virtual_port.unwrap_or(0);

        let handle = client.networking_sockets().create_listen_socket_p2p(port, None);
        
        match handle {
          Ok(socket) => {
              let mut listen_p2p = LISTEN_P2P.lock().unwrap();
              *listen_p2p = Some(socket);
              Ok(true)
          }
          Err(_) => Err(Error::from_reason("Failed to create listen socket")),
        }
    }

    // allow for ip as well -- we need this for the local client loopback
    #[napi]
    pub fn create_listen_socket_ip(local_virtual_port: Option<i32>) -> Result<bool, Error> {
        let client = crate::client::get_client();
        let port = local_virtual_port.unwrap_or(0);

        let handle = client.networking_sockets().create_listen_socket_ip(
          SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), port.try_into().unwrap()),
          vec![],
        );

        match handle {
          Ok(socket) => {
              let mut listen_ip = LISTEN_IP.lock().unwrap();
              *listen_ip = Some(socket);
              Ok(true)
          }
          Err(_) => Err(Error::from_reason("Failed to create listen socket")),
        }
    }

    // used to toggle whether locally it is server functionality
    #[napi]
    pub fn set_am_i_server(is_server: bool) {
        *AM_I_SERVER.lock().unwrap() = is_server;
    }

    // used to allow or reject new connections
    #[napi]
    pub fn set_accept_new_p2p_requests(accept: bool) {
        *ACCEPT_NEW_REQUESTS.lock().unwrap() = accept;
    }

    // used to initiate connection
    #[napi]
    pub fn connect_p2p(steam_id64: BigInt, remote_virtual_port: i32) -> Result<bool, Error> {
        // first check if I am server and does the steam_id64 belong to me
        // if it does, we can just skip this step and return true
        let client = crate::client::get_client();
        let steam_id = SteamId::from_raw(steam_id64.get_u64().1);
        
        let local_steam_id = PlayerSteamId::from_steamid(client.user().steam_id());
        if local_steam_id.steam_id64.get_u64().1 == steam_id64.get_u64().1 {
          // then we need to actually hijack and hit the local server via ip

          let handle = client.networking_sockets().connect_by_ip_address(
            SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), remote_virtual_port.try_into().unwrap()),
            None
          );

          match handle {
            Ok(connection) => {
                CONNECTIONS.lock().unwrap().insert(steam_id, connection);
                return Ok(true)
            }
            Err(e) => {
              eprintln!("Failed to connect P2P(by IP): {:?}", e);
              return Err(Error::from_reason("Failed to connect P2P(by IP)"))
            }
          }
        }
        
        let identity = NetworkingIdentity::new_steam_id(steam_id);
        let handle = client.networking_sockets().connect_p2p(identity, remote_virtual_port, None);
        match handle {
            Ok(connection) => {
                CONNECTIONS.lock().unwrap().insert(steam_id, connection);
                Ok(true)
            }
            Err(e) => {
              eprintln!("Failed to connect P2P: {:?}", e);
              Err(Error::from_reason("Failed to connect P2P"))
            } 
        }
    }

    // used to accept incoming connections
    #[napi]
    pub fn process_listen_p2p_events() {
        // Get the socket if it exists
        let guard = LISTEN_P2P.lock().unwrap();
        let socket = if let Some(socket) = guard.as_ref() {
            socket
        } else {
            return;
        };

        // Process all available events for this socket
        while let Some(event) = socket.try_receive_event() {
          match event {
              ListenSocketEvent::Connecting(request) => {
                  // Check if we should accept the connection request
                  if *ACCEPT_NEW_REQUESTS.lock().unwrap() {
                      // Attempt to accept the connection request
                      if let Err(e) = request.accept() {
                          eprintln!("Failed to accept connection: {:?}", e);
                      }
                  }
              }
              ListenSocketEvent::Connected(connected) => {
                  // Grab the steam id of the connected user
                  let steam_id = connected.remote().steam_id().unwrap();
                  // Insert the connection into the CONNECTIONS map
                  CONNECTIONS.lock().unwrap().insert(steam_id, connected.take_connection());
              }
              _ => {
                  // Ignore other event types for now
              }
          }
      }
    }

    // have to accept the connection from own ip, gross but whatever
    #[napi]
    pub fn process_listen_ip_events() {
        // Get the socket if it exists
        let guard = LISTEN_IP.lock().unwrap();
        let socket = if let Some(socket) = guard.as_ref() {
            socket
        } else {
            return;
        };

        // Process all available events for this socket
        while let Some(event) = socket.try_receive_event() {
          match event {
              ListenSocketEvent::Connecting(request) => {
                  // Check if we should accept the connection request
                  if *ACCEPT_NEW_REQUESTS.lock().unwrap() {
                      // Attempt to accept the connection request
                      if let Err(e) = request.accept() {
                          eprintln!("Failed to accept connection: {:?}", e);
                      }
                  }
              }
              ListenSocketEvent::Connected(connected) => {
                  // Grab the steam id of the connected user
                  let steam_id = connected.remote().steam_id().unwrap();
                  // Insert the connection into the CONNECTIONS map
                  CONNECTIONS.lock().unwrap().insert(steam_id, connected.take_connection());
              }
              _ => {
                  // Ignore other event types for now
              }
          }
        }
    }

    // now we need a way to receive all mesages
    #[napi(object)]
    pub struct P2PPacket {
      pub data: Buffer,
      pub steam_id: BigInt,
    }

    #[napi]
    pub fn receive_p2p_messages(
      batch_size: Option<u32>
    ) -> Vec<P2PPacket> {
      let mut messages = Vec::new();
      let mut connections = CONNECTIONS.lock().unwrap();
      
      for (steam_id, connection) in connections.iter_mut() {
        if let Ok(received_messages) = connection.receive_messages(batch_size.unwrap_or(10) as usize) {
          for message in received_messages {
            messages.push(P2PPacket {
              steam_id: BigInt::from(steam_id.raw()),
              data: Buffer::from(message.data()),
            });
          }
        }
      }

      messages
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

    // and a way to send messages
    #[napi]
    pub fn send_p2p_message(
      steam_id64: BigInt,
      data: Buffer,
      send_type: SendType
    ) -> Result<bool, Error> {
      let steam_id = SteamId::from_raw(steam_id64.get_u64().1);

      let mut connections = CONNECTIONS.lock().unwrap();
      if let Some(connection) = connections.get_mut(&steam_id) {
        let result = connection.send_message(
          &data,
          match send_type {
            SendType::Unreliable => SendFlags::UNRELIABLE,
            SendType::UnreliableNoDelay => SendFlags::UNRELIABLE_NO_DELAY,
            SendType::ReliableWithBuffering => SendFlags::RELIABLE, // nagle is the new default
            SendType::Reliable => SendFlags::RELIABLE_NO_NAGLE,
          } & SendFlags::AUTO_RESTART_BROKEN_SESSION,
        );
        return Ok(result.is_ok());
      }
      Err(Error::from_reason("Failed to send message"))
    }
}