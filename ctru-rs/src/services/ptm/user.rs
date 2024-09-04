//! PTM User service.
//!
//! This sub-service of the Power-Time service family icludes getters for various hardware/console states, such as whether
//! the console shell is open or the current battery level.

use std::sync::Mutex;

use crate::error::ResultCode;
use crate::services::ServiceReference;

static PTMU_ACTIVE: Mutex<()> = Mutex::new(());

/// Whether the console's shell is open or closed.
#[repr(u8)]
pub enum ShellState {
    Closed = 0,
    Open = 1,
}

/// Representation of the console's battery charge level.
///
/// These values correspond to the various states the battery is shown to be in the Home Menu UI.
#[repr(u8)]
pub enum BatteryLevel {
    // Battery charge at 0%. System shutdown is imminent.
    Drained = 0,
    // Battery charge between 5-1%
    Critical = 1,
    // Battery charge between 10-6%
    VeryLow = 2,
    // Battery charge between 30-11%
    Low = 3,
    // Battery charge between 60-31%
    Medium = 4,
    // Battery charge between 100-61%
    High = 5,
}

/// Handle to the PTM:User service.
pub struct PTMUser {
    _service_handler: ServiceReference,
}

impl PTMUser {
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
    /// use ctru::services::ptm::PTMUser;
    ///
    /// let ptmu = PTMUser::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ptmuInit")]
    pub fn new() -> crate::Result<Self> {
        let handler = ServiceReference::new(
            &PTMU_ACTIVE,
            || {
                ResultCode(unsafe { ctru_sys::ptmuInit() })?;

                Ok(())
            },
            || unsafe {
                ctru_sys::ptmuExit();
            },
        )?;

        Ok(Self {
            _service_handler: handler,
        })
    }

    /// Returns whether the console's clamshell is closed or open.
    pub fn shell_state() -> crate::Result<ShellState> {
        let mut state: u8 = 0;

        ResultCode(unsafe { ctru_sys::PTMU_GetShellState(&mut state) })?;

        Ok(state.into())
    }

    /// Returns the console's current battery charge level.
    pub fn battery_level() -> crate::Result<BatteryLevel> {
        let mut level: u8 = 0;

        ResultCode(unsafe { ctru_sys::PTMU_GetBatteryLevel(&mut level) })?;

        Ok(level.into())
    }

    /// Returns whether the console is currently charging its battery.
    pub fn is_charging() -> crate::Result<bool> {
        let mut charging: u8 = 0;

        ResultCode(unsafe { ctru_sys::PTMU_GetBatteryChargeState(&mut charging) })?;

        Ok(charging)
    }
}

from_impl!(ShellState, u8);
from_impl!(BatteryLevel, u8);
