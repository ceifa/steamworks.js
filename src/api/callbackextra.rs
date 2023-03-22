use serde::{Deserialize, Serialize};
use std::ffi::c_void;
use steamworks::Callback;
use steamworks_sys as sys;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MicroTxnAuthorizationResponse {
    /// The reason we were disconnected from the Steam servers
    pub app_id: u32,
    pub order_id: u64,
    // Authorized: 0, Unauthorized: 1
    pub authorized: u8,
}

unsafe impl Callback for MicroTxnAuthorizationResponse {
    const ID: i32 = 152;
    const SIZE: i32 = std::mem::size_of::<sys::MicroTxnAuthorizationResponse_t>() as i32;

    unsafe fn from_raw(raw: *mut c_void) -> Self {
        let val = &mut *(raw as *mut sys::MicroTxnAuthorizationResponse_t);
        MicroTxnAuthorizationResponse {
            app_id: val.m_unAppID.into(),
            order_id: val.m_ulOrderID.into(),
            authorized: val.m_bAuthorized.into(),
        }
    }
}
