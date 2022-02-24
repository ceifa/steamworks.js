use steamworks::Client;
use steamworks::SingleClient;

pub static mut STEAM_CLIENT: Option<Client> = None;
pub static mut STEAM_SINGLE: Option<SingleClient> = None;

pub fn get_client() -> &'static Client {
    unsafe {
        match &STEAM_CLIENT {
            Some(client) => client,
            None => panic!("Steam client not initialized")
        }
    }
}