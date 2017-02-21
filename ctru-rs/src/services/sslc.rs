use libctru::services::sslc::*;
use Result;

// TODO: Implement remaining functions

pub struct SslC(());

impl SslC {
    /// Initialize sslc
    pub fn init() -> Result<Self> {
        unsafe {
            let r = sslcInit(0);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(SslC(()))
            }
        }
    }

    /// Fill `buf` with `buf.len()` random bytes
    pub fn generate_random_data(&self, buf: &mut [u8]) -> Result<()> {
        unsafe {
            let r = sslcGenerateRandomData(buf.as_ptr() as _, buf.len() as u32);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }
}

impl Drop for SslC {
    fn drop(&mut self) {
        unsafe { sslcExit() };
    }
}
