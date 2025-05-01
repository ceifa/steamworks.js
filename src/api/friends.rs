use napi_derive::napi;

#[napi]
pub mod friends {
    use napi::bindgen_prelude::BigInt;
    use steamworks::SteamId;

    #[napi(ts_args_type="who: bigint")]
    pub fn get_persona_name(who: BigInt) -> String {
        let client = crate::client::get_client();
        let (_, id, _) = who.get_u64();
        let friend = client.friends().get_friend(SteamId::from_raw(id));
        friend.name()
    }

    #[napi(ts_args_type="who: bigint")]
    pub fn request_user_information(who: BigInt) -> () {
        let client = crate::client::get_client();
        let (_, id, _) = who.get_u64();
        let steam_id = SteamId::from_raw(id);
        client.friends().request_user_information(steam_id, true);
    }
}
