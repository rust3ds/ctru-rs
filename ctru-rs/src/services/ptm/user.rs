//! PTM User service.
//!
//! This sub-service of the Power-Time service family includes getters for various hardware/console states, such as whether
//! the console [shell is open](PTMUser::shell_state) or the current [battery level](PTMUser::battery_level).
#[doc(alias = "battery")]
#[doc(alias = "shell")]
use std::sync::Mutex;

use crate::error::{Error, Result, ResultCode};
use crate::services::ServiceReference;

static PTMU_ACTIVE: Mutex<()> = Mutex::new(());

/// Whether the console's shell is open or closed.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShellState {
    /// Clam shell is currently closed.
    Closed = 0,
    /// Clam shell is currently open.
    Open = 1,
}

/// Representation of the console's battery charge level.
///
/// These values correspond to the various states the battery is shown to be in the Home Menu UI.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BatteryLevel {
    /// Battery charge at 0%. System shutdown is imminent.
    Drained = 0,
    /// Battery charge between 5-1%.
    Critical = 1,
    /// Battery charge between 10-6%.
    VeryLow = 2,
    /// Battery charge between 30-11%.
    Low = 3,
    /// Battery charge between 60-31%.
    Medium = 4,
    /// Battery charge between 100-61%.
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
    /// use ctru::services::ptm::user::PTMUser;
    ///
    /// let ptmu = PTMUser::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ptmuInit")]
    pub fn new() -> Result<Self> {
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
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ptm::user::{PTMUser, ShellState};
    ///
    /// let ptmu = PTMUser::new()?;
    ///
    /// let state = ptmu.shell_state()?;
    ///
    /// match state {
    ///     ShellState::Closed => println!("The shell is closed! How are you able to read this?"),
    ///     ShellState::Open => println!("The shell is open! That might seem obvious to you."),
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "PTMU_GetShellState")]
    pub fn shell_state(&self) -> Result<ShellState> {
        let mut state: u8 = 0;

        ResultCode(unsafe { ctru_sys::PTMU_GetShellState(&mut state) })?;

        state.try_into()
    }

    /// Returns the console's current battery charge level.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ptm::user::{PTMUser, BatteryLevel};
    ///
    /// let ptmu = PTMUser::new()?;
    ///
    /// let charge = ptmu.battery_level()?;
    ///
    /// if charge <= BatteryLevel::Low {
    ///     println!("You should put the console to charge!");
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "PTMU_GetBatteryLevel")]
    pub fn battery_level(&self) -> Result<BatteryLevel> {
        let mut level: u8 = 0;

        ResultCode(unsafe { ctru_sys::PTMU_GetBatteryLevel(&mut level) })?;

        level.try_into()
    }

    /// Returns whether the console is currently charging its battery.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ptm::user::PTMUser;
    ///
    /// let ptmu = PTMUser::new()?;
    ///
    /// let is_charging = ptmu.is_charging()?;
    ///
    /// if is_charging {
    ///     println!("That is one juicy power line.");
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "PTMU_GetBatteryChargeState")]
    pub fn is_charging(&self) -> Result<bool> {
        let mut charging: u8 = 0;

        ResultCode(unsafe { ctru_sys::PTMU_GetBatteryChargeState(&mut charging) })?;

        match charging {
            0 => Ok(false),
            1 => Ok(true),
            v => Err(Error::Other(format!(
                "unexpected charging state value: {v}",
            ))),
        }
    }

    /// Returns the console's total step count.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ptm::user::PTMUser;
    ///
    /// let ptmu = PTMUser::new()?;
    ///
    /// let steps = ptmu.step_count()?;
    ///
    /// println!("You accumulated {steps} steps. Don't stop moving!");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "PTMU_GetTotalStepCount")]
    pub fn step_count(&self) -> Result<u32> {
        let mut steps: u32 = 0;

        ResultCode(unsafe { ctru_sys::PTMU_GetTotalStepCount(&mut steps) })?;

        Ok(steps)
    }
}

impl TryFrom<u8> for ShellState {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Self::Closed),
            1 => Ok(Self::Open),
            v => Err(Error::Other(format!("unexpected shell state value: {v}",))),
        }
    }
}

impl TryFrom<u8> for BatteryLevel {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Self::Drained),
            1 => Ok(Self::Critical),
            2 => Ok(Self::VeryLow),
            3 => Ok(Self::Low),
            4 => Ok(Self::Medium),
            5 => Ok(Self::High),
            v => Err(Error::Other(
                format!("unexpected battery level value: {v}",),
            )),
        }
    }
}

from_impl!(ShellState, u8);
from_impl!(BatteryLevel, u8);
