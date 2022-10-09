use crate::error::LibCtruError;

pub struct Apt(());

impl Apt {
    pub fn init() -> crate::Result<Apt> {
        unsafe {
            LibCtruError(ctru_sys::aptInit())?;
            Ok(Apt(()))
        }
    }

    pub fn main_loop(&self) -> bool {
        unsafe { ctru_sys::aptMainLoop() }
    }

    pub fn set_app_cpu_time_limit(&self, percent: u32) -> crate::Result<()> {
        unsafe {
            LibCtruError(ctru_sys::APT_SetAppCpuTimeLimit(percent))?;
            Ok(())
        }
    }
}

impl Drop for Apt {
    fn drop(&mut self) {
        unsafe { ctru_sys::aptExit() };
    }
}
