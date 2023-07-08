//! Applet service.
//!
//! The APT service handles integration with some higher level OS features such as Sleep mode, the Home Menu and application switching.
//!
//! It also handles running applets, small programs made available by the OS to streamline specific functionality. Those are implemented in the [`applets`](crate::applets) module.

use crate::error::ResultCode;

/// Handle to the Applet service.
pub struct Apt(());

impl Apt {
    /// Initialize a new handle.
    #[doc(alias = "aptInit")]
    pub fn new() -> crate::Result<Apt> {
        unsafe {
            ResultCode(ctru_sys::aptInit())?;
            Ok(Apt(()))
        }
    }

    /// Returns `true` if the application is running in the foreground as normal.
    ///
    /// # Notes
    ///
    /// This function is called as such since it automatically handles all checks for Home Menu switching, Sleep mode and other events that could take away control from the application.
    /// For this reason, its main use is as the condition of a while loop that controls the main logic for your program.
    #[doc(alias = "aptMainLoop")]
    pub fn main_loop(&self) -> bool {
        unsafe { ctru_sys::aptMainLoop() }
    }

    /// Sets (in percentage) the amount of time to lend to the application thread spawned on the syscore (core #1).
    ///
    /// # Notes
    ///
    /// It is necessary to set a time limit before spawning threads on the syscore.
    /// The percentage value must be withing 5% and 89%, though it is suggested to use lower values (around 30-45%) to avoid slowing down the OS processes.
    #[doc(alias = "APT_SetAppCpuTimeLimit")]
    pub fn set_app_cpu_time_limit(&mut self, percent: u32) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::APT_SetAppCpuTimeLimit(percent))?;
            Ok(())
        }
    }
}

impl Drop for Apt {
    #[doc(alias = "aptExit")]
    fn drop(&mut self) {
        unsafe { ctru_sys::aptExit() };
    }
}
