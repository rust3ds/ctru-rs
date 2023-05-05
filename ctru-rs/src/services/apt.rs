use crate::error::ResultCode;

pub struct Apt(());

impl Apt {
    pub fn new() -> crate::Result<Apt> {
        unsafe {
            ResultCode(ctru_sys::aptInit())?;
            Ok(Apt(()))
        }
    }

    pub fn main_loop(&self) -> bool {
        unsafe { ctru_sys::aptMainLoop() }
    }

    pub fn set_app_cpu_time_limit(&mut self, percent: u32) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::APT_SetAppCpuTimeLimit(percent))?;
            Ok(())
        }
    }
}

impl Drop for Apt {
    fn drop(&mut self) {
        unsafe { ctru_sys::aptExit() };
    }
}
