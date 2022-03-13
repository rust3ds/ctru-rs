use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::services::ServiceReference;

#[non_exhaustive]
pub struct Srv {
    _service_handler: ServiceReference,
}

static SRV_ACTIVE: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

impl Srv {
    pub fn init() -> crate::Result<Self> {
        let _service_handler = ServiceReference::new(
            &SRV_ACTIVE,
            true,
            || {
                let r = unsafe { ctru_sys::srvInit() };
                if r < 0 {
                    return Err(r.into());
                }

                Ok(())
            },
            || unsafe {
                ctru_sys::srvExit();
            },
        )?;

        Ok(Self { _service_handler })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn srv_duplicate() {
        let _srv = Srv::init().unwrap();

        let value = *SRV_ACTIVE.lock().unwrap();

        assert_eq!(value, 1);

        drop(_srv);

        let value = *SRV_ACTIVE.lock().unwrap();

        assert_eq!(value, 0);
    }
}
