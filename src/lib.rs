use steamworks::Client;
pub mod client;

#[napi_derive::napi]
fn init(app_id: u32) {
    let (client, single) = Client::init_app(app_id).unwrap();
    
    unsafe {
        client::STEAM_CLIENT = Some(client);
        client::STEAM_SINGLE = Some(single);
    }    
}    

#[napi_derive::napi]
fn run_callbacks() {
    unsafe {
        match &client::STEAM_SINGLE {
            Some(single) => single.run_callbacks(),
            None => panic!("Steam client not initialized")
        }
    }
}

// other apis
pub mod achievement;