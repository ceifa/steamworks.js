use steamworks::Client;
use steamworks::SingleClient;

static mut STEAM_CLIENT: Option<Client> = None;
static mut STEAM_SINGLE: Option<SingleClient> = None;

pub fn get_client() -> &'static Client {
    unsafe {
        match &STEAM_CLIENT {
            Some(client) => client,
            None => panic!("Steam client not initialized")
        }
    }
}

#[napi_derive::napi]
fn init(app_id: u32) {
    let (client, single) = Client::init_app(app_id).unwrap();
    
    unsafe {
        STEAM_CLIENT = Some(client);
        STEAM_SINGLE = Some(single);
    }    
}    

#[napi_derive::napi]
fn run_callbacks() {
    unsafe {
        match &STEAM_SINGLE {
            Some(single) => single.run_callbacks(),
            None => panic!("Steam client not initialized")
        }
    }
}