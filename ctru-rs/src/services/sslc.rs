//! SSLC (TLS) service

// TODO: Implement remaining functions

use crate::error::ResultCode;

pub struct SslC(());

impl SslC {
    /// Initialize the service
    pub fn init() -> crate::Result<Self> {
        unsafe {
            ResultCode(ctru_sys::sslcInit(0))?;
            Ok(SslC(()))
        }
    }

    /// Fill `buf` with `buf.len()` random bytes
    pub fn generate_random_data(&self, buf: &mut [u8]) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::sslcGenerateRandomData(
                buf.as_ptr() as _,
                buf.len() as u32,
            ))?;
            Ok(())
        }
    }
}

impl Drop for SslC {
    fn drop(&mut self) {
        unsafe { ctru_sys::sslcExit() };
    }
}
