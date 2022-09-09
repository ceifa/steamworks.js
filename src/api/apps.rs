use napi_derive::napi;

#[napi]
pub mod apps {
    use steamworks::AppId;

    #[napi]
    pub fn is_subscribed_app(app_id: u32) -> bool {
        let client = crate::client::get_client();
        client.apps().is_subscribed_app(AppId(app_id))
    }
}
