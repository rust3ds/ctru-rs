//! SSLC (TLS) service

// TODO: Implement remaining functions

use crate::error::ResultCode;

pub struct SslC(());

impl SslC {
    /// Initialize the service
    pub fn new() -> crate::Result<Self> {
        unsafe {
            ResultCode(ctru_sys::sslcInit(0))?;
            Ok(SslC(()))
        }
    }
}

impl Drop for SslC {
    fn drop(&mut self) {
        unsafe { ctru_sys::sslcExit() };
    }
}
