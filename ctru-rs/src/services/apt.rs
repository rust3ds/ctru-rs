use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::Error;

#[non_exhaustive]
pub struct Apt(());

static APT_ACTIVE: AtomicBool = AtomicBool::new(false);

impl Apt {
    pub fn init() -> crate::Result<Self> {
        match APT_ACTIVE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => {
                let r = unsafe { ctru_sys::aptInit() };
                if r < 0 {
                    Err(r.into())
                } else {
                    Ok(Self(()))
                }
            }
            Err(_) => Err(Error::ServiceAlreadyActive("Apt")),
        }
    }

    pub fn main_loop(&self) -> bool {
        unsafe { ctru_sys::aptMainLoop() }
    }

    pub fn set_app_cpu_time_limit(&self, percent: u32) -> crate::Result<()> {
        let r = unsafe { ctru_sys::APT_SetAppCpuTimeLimit(percent) };
        if r < 0 {
            Err(r.into())
        } else {
            Ok(())
        }
    }
}

impl Drop for Apt {
    fn drop(&mut self) {
        unsafe { ctru_sys::aptExit() };

        APT_ACTIVE.store(false, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apt_duplicate() {
        // We don't need to build a `Apt` because the test runner has one already
        match Apt::init() {
            Err(Error::ServiceAlreadyActive("Apt")) => return,
            _ => panic!(),
        }
    }
}
