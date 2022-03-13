// TODO: Implement remaining functions

use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::services::ServiceReference;

#[non_exhaustive]
pub struct SslC {
    _service_handler: ServiceReference,
}

static SSLC_ACTIVE: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

impl SslC {
    /// Initialize sslc
    pub fn init() -> crate::Result<Self> {
        let _service_handler = ServiceReference::new(
            &SSLC_ACTIVE,
            true,
            || {
                let r = unsafe { ctru_sys::sslcInit(0) };
                if r < 0 {
                    return Err(r.into());
                }

                Ok(())
            },
            || unsafe {
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

        let value = *SSLC_ACTIVE.lock().unwrap();

        assert_eq!(value, 1);

        drop(_sslc);

        let value = *SSLC_ACTIVE.lock().unwrap();

        assert_eq!(value, 0);
    }
}
