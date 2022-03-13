use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::services::ServiceReference;

#[non_exhaustive]
pub struct Apt {
    _service_handler: ServiceReference,
}

static APT_ACTIVE: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

impl Apt {
    pub fn init() -> crate::Result<Self> {
        let _service_handler = ServiceReference::new(
            &APT_ACTIVE,
            true,
            || {
                let r = unsafe { ctru_sys::aptInit() };
                if r < 0 {
                    return Err(r.into());
                }

                Ok(())
            },
            || unsafe {
                ctru_sys::aptExit();
            },
        )?;

        Ok(Self { _service_handler })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apt_duplicate() {
        // We don't need to build a `Apt` because the test runner has one already
        let value = *APT_ACTIVE.lock().unwrap();

        assert_eq!(value, 1);
    }
}
