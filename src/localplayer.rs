use super::*;

#[napi_derive::napi]
pub struct LocalSteamId {
    pub steam_id64: String,
    pub steam_id32: String,
    pub account_id: u32,
}

#[napi_derive::napi]
pub fn get_steam_id() -> LocalSteamId {
    let client = client::get_client();
    let steam_id = client.user().steam_id();

    LocalSteamId {
        steam_id64: steam_id.raw().to_string(),
        steam_id32: steam_id.steamid32(),
        account_id: steam_id.account_id().raw(),
    }
}

#[napi_derive::napi]
pub fn get_name() -> String {
    let client = client::get_client();
    client.friends().name()
}

#[napi_derive::napi]
pub fn get_level() -> u32 {
    let client = client::get_client();
    client.user().level()
}

#[napi_derive::napi]
pub fn get_ip_country() -> String {
    let client = client::get_client();
    client.utils().ip_country()
}