// TODO: Implement remaining functions

use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::Error;

#[non_exhaustive]
pub struct SslC(());

static SSLC_ACTIVE: AtomicBool = AtomicBool::new(false);

impl SslC {
    /// Initialize sslc
    pub fn init() -> crate::Result<Self> {
        match SSLC_ACTIVE.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => {
                let r = unsafe { ctru_sys::sslcInit(0) };
                if r < 0 {
                    Err(r.into())
                } else {
                    Ok(Self(()))
                }
            }
            Err(_) => Err(Error::ServiceAlreadyActive("SslC")),
        }
    }

    /// Fill `buf` with `buf.len()` random bytes
    pub fn generate_random_data(&self, buf: &mut [u8]) -> crate::Result<()> {
        let r = unsafe { ctru_sys::sslcGenerateRandomData(buf.as_ptr() as _, buf.len() as u32) };
        if r < 0 {
            Err(r.into())
        } else {
            Ok(())
        }
    }
}

impl Drop for SslC {
    fn drop(&mut self) {
        unsafe { ctru_sys::sslcExit() };

        SSLC_ACTIVE.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sslc_duplicate() {
        let _sslc = SslC::init().unwrap();

        assert!(SslC::init().is_err());
    }
}

