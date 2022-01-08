// TODO: Implement remaining functions

pub struct SslC(());

impl SslC {
    /// Initialize sslc
    pub fn init() -> crate::Result<Self> {
        unsafe {
            let r = ctru_sys::sslcInit(0);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(SslC(()))
            }
        }
    }

    /// Fill `buf` with `buf.len()` random bytes
    pub fn generate_random_data(&self, buf: &mut [u8]) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::sslcGenerateRandomData(buf.as_ptr() as _, buf.len() as u32);
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
        unsafe { ctru_sys::sslcExit() };
    }
}
