use napi_derive::napi;

#[napi]
pub mod achievement {
    #[napi]
    pub fn activate(achievement: String) -> bool {
        let client = crate::client::get_client();
        let result = client
            .user_stats()
            .achievement(&achievement)
            .set()
            .and_then(|_| client.user_stats().store_stats());

        result.is_ok()
    }
}
