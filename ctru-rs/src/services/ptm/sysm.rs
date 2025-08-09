//! PTM SystemMenu service.
//!
//! This sub-service of the Power-Time service family is able to control shutdown/sleep functionality and how those states are
//! communicated to the user (such as via the notification/battery LED).
#[doc(alias = "sleep")]
#[doc(alias = "shutdown")]
#[doc(alias = "led")]
use std::sync::Mutex;
use std::time::Duration;

use crate::error::{Result, ResultCode};
use crate::services::ServiceReference;

static PTMSYSM_ACTIVE: Mutex<()> = Mutex::new(());

/// Handle to the PTM:SysM service.
pub struct PTMSysM {
    _service_handler: ServiceReference,
}

impl PTMSysM {
    /// Initialize a new service handle.
    ///
    /// # Errors
    ///
    /// This function will return an error if the service was unable to be initialized.
    /// Since this service requires no special or elevated permissions, errors are rare in practice.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ptm::sysm::PTMSysM;
    ///
    /// let ptm_sysm = PTMSysM::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ptmSysmInit")]
    pub fn new() -> Result<Self> {
        let handler = ServiceReference::new(
            &PTMSYSM_ACTIVE,
            || {
                ResultCode(unsafe { ctru_sys::ptmSysmInit() })?;

                Ok(())
            },
            || unsafe {
                ctru_sys::ptmSysmExit();
            },
        )?;

        Ok(Self {
            _service_handler: handler,
        })
    }

    /// Try putting the console in sleep mode.
    ///
    /// # Notes
    ///
    /// This request can be denied for various reasons. This does not "force" the console to sleep.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ptm::sysm::PTMSysM;
    /// use std::time::Duration;
    ///
    /// let ptm_sysm = PTMSysM::new()?;
    ///
    /// // Request the activation of sleep mode.
    /// ptm_sysm.request_sleep().unwrap();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "PTMSYSM_RequestSleep")]
    pub fn request_sleep(&self) -> Result<()> {
        ResultCode(unsafe { ctru_sys::PTMSYSM_RequestSleep() })?;

        Ok(())
    }

    /// Request a system shutdown within the given timeout.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ptm::sysm::PTMSysM;
    /// use std::time::Duration;
    ///
    /// let ptm_sysm = PTMSysM::new()?;
    ///
    /// // Shutdown the system (usually the request succeeds immediately).
    /// ptm_sysm.request_shutdown(Duration::from_nanos(0)).unwrap();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "PTMSYSM_ShutdownAsync")]
    pub fn request_shutdown(&self, timeout: Duration) -> Result<()> {
        let timeout = timeout.as_nanos() as u64;

        ResultCode(unsafe { ctru_sys::PTMSYSM_ShutdownAsync(timeout) })?;

        Ok(())
    }

    /// Request a system reboot within the given timeout.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ptm::sysm::PTMSysM;
    /// use std::time::Duration;
    ///
    /// let ptm_sysm = PTMSysM::new()?;
    ///
    /// // Reboot the system.
    /// ptm_sysm.request_reboot(Duration::from_nanos(0)).unwrap();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "PTMSYSM_RebootAsync")]
    pub fn request_reboot(&self, timeout: Duration) -> Result<()> {
        let timeout = timeout.as_nanos() as u64;

        ResultCode(unsafe { ctru_sys::PTMSYSM_RebootAsync(timeout) })?;

        Ok(())
    }
}
