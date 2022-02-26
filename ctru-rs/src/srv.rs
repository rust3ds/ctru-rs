use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::Error;

#[non_exhaustive]
pub struct Srv(());

static SRV_ACTIVE: AtomicBool = AtomicBool::new(false);

impl Srv {
    pub fn init() -> crate::Result<Self> {
        match SRV_ACTIVE.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => {
                let r = unsafe { ctru_sys::srvInit() };
                if r < 0 {
                    Err(r.into())
                } else {
                    Ok(Self(()))
                }
            }
            Err(_) => Err(Error::ServiceAlreadyActive("Srv")),
        }
    }
}

impl Drop for Srv {
    fn drop(&mut self) {
        unsafe { ctru_sys::srvExit() };

        SRV_ACTIVE.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn srv_duplicate() {
        let _srv = Srv::init().unwrap();

        assert!(Srv::init().is_err());
    }
}
