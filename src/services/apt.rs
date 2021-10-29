use crate::error::Result;
use crate::raw;

pub struct Apt(());

impl Apt {
    pub fn init() -> Result<Apt> {
        unsafe {
            let r = raw::aptInit();
            if r < 0 {
                Err(r.into())
            } else {
                Ok(Apt(()))
            }
        }
    }

    pub fn main_loop(&self) -> bool {
        unsafe {
            raw::aptMainLoop()
        }
    }

    pub fn set_app_cpu_time_limit(&self, percent: u32) -> Result<()> {
        unsafe {
           let r = raw::APT_SetAppCpuTimeLimit(percent);
           if r < 0 {
               Err(r.into())
           } else {
               Ok(())
           }
        }
    }
}

impl Drop for Apt {
    fn drop(&mut self) {
        unsafe { raw::aptExit() };
    }
}
