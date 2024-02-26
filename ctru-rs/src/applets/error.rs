//! Error applet
//!
//! This applet displays error text as a pop-up message on the lower screen.
use crate::services::{apt::Apt, gfx::Gfx};

use ctru_sys::errorConf;

/// Configuration struct to set up the Error applet.
#[doc(alias = "errorConf")]
pub struct PopUp {
    state: Box<errorConf>,
}

/// The kind of error applet to display.
#[doc(alias = "errorType")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Kind {
    /// Error text is centered in the error applet window.
    Center = ctru_sys::ERROR_TEXT,
    /// Error text starts at the top of the error applet window.
    Top = ctru_sys::ERROR_TEXT_WORD_WRAP,
}

/// Error returned by an unsuccessful [`PopUp::launch()`].
#[doc(alias = "errorReturnCode")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum Error {
    /// Unknown error occurred.
    Unknown = ctru_sys::ERROR_UNKNOWN,
    /// Operation not supported.
    NotSupported = ctru_sys::ERROR_NOT_SUPPORTED,
    /// Home button pressed while [`PopUp`] was running.
    HomePressed = ctru_sys::ERROR_HOME_BUTTON,
    /// Power button pressed while [`PopUp`] was running.
    PowerPressed = ctru_sys::ERROR_POWER_BUTTON,
    /// Reset button pressed while [`PopUp`] was running.
    ResetPressed = ctru_sys::ERROR_SOFTWARE_RESET,
}

impl PopUp {
    /// Initialize the error applet with the provided text kind.
    #[doc(alias = "errorInit")]
    pub fn new(kind: Kind) -> Self {
        let mut state = Box::<errorConf>::default();

        unsafe { ctru_sys::errorInit(state.as_mut(), kind as _, 0) };

        Self { state }
    }

    /// Sets the error text to display.
    #[doc(alias = "errorText")]
    pub fn set_text(&mut self, text: &str) {
        for (idx, code_unit) in text
            .encode_utf16()
            .chain(std::iter::once(0))
            .take(self.state.Text.len() - 1)
            .enumerate()
        {
            self.state.Text[idx] = code_unit;
        }
    }

    /// Launches the error applet.
    #[doc(alias = "errorDisp")]
    pub fn launch(&mut self, _apt: &Apt, _gfx: &Gfx) -> Result<(), Error> {
        unsafe { ctru_sys::errorDisp(self.state.as_mut()) }

        match self.state.returnCode {
            ctru_sys::ERROR_NONE | ctru_sys::ERROR_SUCCESS => Ok(()),
            ctru_sys::ERROR_NOT_SUPPORTED => Err(Error::NotSupported),
            ctru_sys::ERROR_HOME_BUTTON => Err(Error::HomePressed),
            ctru_sys::ERROR_POWER_BUTTON => Err(Error::PowerPressed),
            ctru_sys::ERROR_SOFTWARE_RESET => Err(Error::ResetPressed),
            _ => Err(Error::Unknown),
        }
    }

    /// Launches the error applet without requiring an [`Apt`] or [`Gfx`] handle.
    ///
    /// # Safety
    ///
    /// Causes undefined behavior if the aforementioned services are not actually active when the applet launches.
    unsafe fn launch_unchecked(&mut self) {
        unsafe { ctru_sys::errorDisp(self.state.as_mut()) };
    }
}

/// Sets a custom panic hook that uses the error applet to display panic messages. You can also choose to have the
/// previously registered panic hook called along with the error applet message, which can be useful if you want
/// to use input redirection to display panic messages over `3dslink` or `GDB`.
///
/// If the `Gfx` service is not initialized during a panic, the error applet will not be displayed and the old
/// panic hook will be called.
pub fn set_panic_hook(call_old_hook: bool) {
    use crate::services::gfx::GFX_ACTIVE;
    use std::sync::TryLockError;

    let old_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic_info| {
        let thread = std::thread::current();

        let name = thread.name().unwrap_or("<unnamed>");

        // If we get a `WouldBlock` error, we know that the `Gfx` service has been initialized.
        // Otherwise fallback to printing over stderr.
        if let (Err(TryLockError::WouldBlock), Ok(_apt)) = (GFX_ACTIVE.try_lock(), Apt::new()) {
            if call_old_hook {
                old_hook(panic_info);
            }

            let payload = format!("thread '{name}' {panic_info}");

            let mut popup = PopUp::new(Kind::Top);

            popup.set_text(&payload);

            unsafe {
                popup.launch_unchecked();
            }
        } else {
            old_hook(panic_info);
        }
    }));
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotSupported => write!(f, "operation not supported"),
            Self::HomePressed => write!(f, "home button pressed while error applet was running"),
            Self::PowerPressed => write!(f, "power button pressed while error applet was running"),
            Self::ResetPressed => write!(f, "reset button pressed while error applet was running"),
            Self::Unknown => write!(f, "an unknown error occurred"),
        }
    }
}

impl std::error::Error for Error {}
