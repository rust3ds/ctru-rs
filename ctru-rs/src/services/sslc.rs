// TODO: Implement remaining functions

use crate::error::LibCtruResult;

pub struct SslC(());

impl SslC {
    /// Initialize sslc
    pub fn init() -> crate::Result<Self> {
        unsafe {
            LibCtruResult(ctru_sys::sslcInit(0))?;
            Ok(SslC(()))
        }
    }

    /// Fill `buf` with `buf.len()` random bytes
    pub fn generate_random_data(&self, buf: &mut [u8]) -> crate::Result<()> {
        unsafe {
            LibCtruResult(ctru_sys::sslcGenerateRandomData(
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
