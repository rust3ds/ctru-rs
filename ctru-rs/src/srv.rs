use std::lazy::SyncLazy;
use std::sync::Mutex;

use crate::services::ServiceHandler;

#[non_exhaustive]
pub struct Srv {
    _service_handler: ServiceHandler,
}

static SRV_ACTIVE: SyncLazy<Mutex<usize>> = SyncLazy::new(|| Mutex::new(0));

impl Srv {
    pub fn init() -> crate::Result<Self> {
        let _service_handler = ServiceHandler::new(
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

        let lock = *SRV_ACTIVE.lock().unwrap();

        assert_eq!(lock, 1);

        drop(_srv);

        let lock = *SRV_ACTIVE.lock().unwrap();

        assert_eq!(lock, 0);
    }
}
