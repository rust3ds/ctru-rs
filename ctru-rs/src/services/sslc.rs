// TODO: Implement remaining functions

use std::lazy::SyncLazy;
use std::sync::Mutex;

use crate::services::ServiceHandler;

#[non_exhaustive]
pub struct SslC {
    _service_handler: ServiceHandler,
}

static SSLC_ACTIVE: SyncLazy<Mutex<usize>> = SyncLazy::new(|| Mutex::new(0));

impl SslC {
    /// Initialize sslc
    pub fn init() -> crate::Result<Self> {
        let _service_handler = ServiceHandler::new(
            &SSLC_ACTIVE,
            true,
            || {
                let r = unsafe { ctru_sys::sslcInit(0) };
                if r < 0 {
                    return Err(r.into());
                }

                Ok(())
            },
            // `socExit` returns an error code. There is no documentantion of when errors could happen,
            // but we wouldn't be able to handle them in the `Drop` implementation anyways.
            // Surely nothing bad will happens :D
            || unsafe {
                // The socket buffer is freed automatically by `socExit`
                ctru_sys::sslcExit();
            },
        )?;

        Ok(Self { _service_handler })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sslc_duplicate() {
        let _sslc = SslC::init().unwrap();

        let lock = *SSLC_ACTIVE.lock().unwrap();

        assert_eq!(lock, 1);

        drop(_sslc);

        let lock = *SSLC_ACTIVE.lock().unwrap();

        assert_eq!(lock, 0);
    }
}
