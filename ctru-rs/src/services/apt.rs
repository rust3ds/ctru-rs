//! Applet service.
//!
//! The APT service handles integration with other applications,
//! including high-level OS features such as Sleep mode, the Home Menu and application switching.
//!
//! It also handles running applets, small programs made available by the OS to streamline specific functionality.
//! Those are implemented in the [`applets`](crate::applets) module.

use crate::error::ResultCode;

/// Handle to the Applet service.
pub struct Apt(());

impl Apt {
    /// Initialize a new service handle.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::apt::Apt;
    ///
    /// let apt = Apt::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// use std::error::Error;
    /// use ctru::services::apt::Apt;
    ///
    /// // In a simple `main` function, the structure should be the following.
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///
    /// let apt = Apt::new()?;
    ///
    /// while apt.main_loop() {
    ///     // Main program logic should be written here.
    /// }
    ///
    /// // Optional clean-ups after running the application should be written after the main loop.
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "aptMainLoop")]
    pub fn main_loop(&self) -> bool {
        unsafe { ctru_sys::aptMainLoop() }
    }

    /// Set (in percentage) the amount of time to lend to the application thread spawned on the syscore (core #1).
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

    /// Set if the console is allowed to enter sleep mode.
    ///
    /// You can check whether the console is allowed to sleep with [Apt::is_sleep_allowed].
    #[doc(alias = "aptSetSleepAllowed")]
    pub fn set_sleep_allowed(&mut self, allowed: bool) {
        unsafe {
            ctru_sys::aptSetSleepAllowed(allowed);
        }
    }

    /// Check if the console is allowed to enter sleep mode.
    ///
    /// You can set whether the console is allowed to sleep with [Apt::set_sleep_allowed].
    #[doc(alias = "aptIsSleepAllowed")]
    pub fn is_sleep_allowed(&self) -> bool {
        unsafe { ctru_sys::aptIsSleepAllowed() }
    }

    /// Set if the console is allowed to enter the home menu.
    ///
    /// You can check whether the console is allowed to enter the home menu with [Apt::is_home_allowed].
    #[doc(alias = "aptSetHomeAllowed")]
    pub fn set_home_allowed(&mut self, allowed: bool) {
        unsafe {
            ctru_sys::aptSetHomeAllowed(allowed);
        }
    }

    /// Check if the console is allowed to enter the home menu.
    ///
    /// You can set whether the console is allowed to enter the home menu with [Apt::set_home_allowed].
    #[doc(alias = "aptIsHomeAllowed")]
    pub fn is_home_allowed(&self) -> bool {
        unsafe { ctru_sys::aptIsHomeAllowed() }
    }

    /// Immediately jumps to the home menu.
    #[doc(alias = "aptIsHomeAllowed")]
    pub fn jump_to_home_menu(&mut self) {
        unsafe { ctru_sys::aptJumpToHomeMenu() }
    }
}

impl Drop for Apt {
    #[doc(alias = "aptExit")]
    fn drop(&mut self) {
        unsafe { ctru_sys::aptExit() };
    }
}

pub struct Chainloader<'a> {
    _apt: &'a Apt,
}

impl<'a> Chainloader<'a> {
    /// Gets a handle to the chainloader
    pub fn new(apt: &'a Apt) -> Self {
        Self { _apt: apt }
    }

    /// Checks if the chainloader is set
    #[doc(alias = "aptIsChainload")]
    pub fn is_set(&mut self) {
        //unsafe { ctru_sys::aptIsChainload() }
    }

    /// Clears the chainloader state.
    #[doc(alias = "aptClearChainloader")]
    pub fn clear(&mut self) {
        unsafe { ctru_sys::aptClearChainloader() }
    }

    /// Configures the chainloader to launch a specific application.
    #[doc(alias = "aptSetChainloader")]
    pub fn set_chainloader(&mut self, title: &super::am::Title<'_>) {
        unsafe { ctru_sys::aptSetChainloader(title.id(), title.media_type() as u8) }
    }

    /// Configures the chainloader to launch the previous application.
    #[doc(alias = "aptSetChainloaderToCaller")]
    pub fn set_chainloader_to_caller(&mut self) {
        unsafe { ctru_sys::aptSetChainloaderToCaller() }
    }

    /// Configures the chainloader to relaunch the current application (i.e. soft-reset)
    #[doc(alias = "aptSetChainloaderToSelf")]
    pub fn set_chainloader_to_self(&mut self) {
        unsafe { ctru_sys::aptSetChainloaderToSelf() }
    }
}
