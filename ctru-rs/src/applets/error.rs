//! Error applet.
//!
//! This applet displays error text as a pop-up message on the lower screen.

use crate::Utf16Writer;
use crate::services::{apt::Apt, gfx::Gfx};

use ctru_sys::{errorConf, errorDisp, errorInit};

/// Configuration struct to set up the Error applet.
#[doc(alias = "errorConf")]
pub struct PopUp {
    state: Box<errorConf>,
}

/// Determines whether the Error applet will use word wrapping when displaying a message.
#[doc(alias = "errorType")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum WordWrap {
    /// Error text is centered in the error applet window and does not use word wrapping.
    Disabled = ctru_sys::ERROR_TEXT,
    /// Error text starts at the top of the error applet window and uses word wrapping.
    Enabled = ctru_sys::ERROR_TEXT_WORD_WRAP,
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
    /// Initializes the error applet with the provided word wrap setting.
    #[doc(alias = "errorInit")]
    pub fn new(word_wrap: WordWrap) -> Self {
        let mut state = Box::<errorConf>::default();

        unsafe { errorInit(state.as_mut(), word_wrap as _, 0) };

        Self { state }
    }

    /// Returns a [`Utf16Writer`] that writes its output to the [`PopUp`]'s internal text buffer.
    ///
    /// # Notes
    ///
    /// The input will be converted to UTF-16 for display with the applet, and the message will be
    /// truncated if it exceeds 1900 UTF-16 code units in length after conversion.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # fn main() {
    /// #
    /// use ctru::applets::error::{PopUp, WordWrap};
    /// use std::fmt::Write;
    ///
    /// let mut popup = PopUp::new(WordWrap::Enabled);
    ///
    /// let _ = write!(popup.writer(), "Look mom, I'm a custom error message!");
    /// #
    /// # }
    /// ```
    #[doc(alias = "errorText")]
    pub fn writer(&mut self) -> Utf16Writer<'_> {
        Utf16Writer::new(&mut self.state.Text)
    }

    /// Launches the error applet.
    #[doc(alias = "errorDisp")]
    pub fn launch(&mut self, _apt: &Apt, _gfx: &Gfx) -> Result<(), Error> {
        unsafe { self.launch_unchecked() }
    }

    /// Launches the error applet without requiring an [`Apt`] or [`Gfx`] handle.
    ///
    /// # Safety
    ///
    /// Potentially leads to undefined behavior if the aforementioned services are not actually active when the applet launches.
    unsafe fn launch_unchecked(&mut self) -> Result<(), Error> {
        unsafe { ctru_sys::errorDisp(self.state.as_mut()) };

        match self.state.returnCode {
            ctru_sys::ERROR_NONE | ctru_sys::ERROR_SUCCESS => Ok(()),
            ctru_sys::ERROR_NOT_SUPPORTED => Err(Error::NotSupported),
            ctru_sys::ERROR_HOME_BUTTON => Err(Error::HomePressed),
            ctru_sys::ERROR_POWER_BUTTON => Err(Error::PowerPressed),
            ctru_sys::ERROR_SOFTWARE_RESET => Err(Error::ResetPressed),
            _ => Err(Error::Unknown),
        }
    }
}

pub(crate) fn set_panic_hook(call_old_hook: bool) {
    use crate::services::gfx::GFX_ACTIVE;
    use std::fmt::Write;
    use std::sync::{Mutex, TryLockError};

    static ERROR_CONF: Mutex<errorConf> = unsafe { Mutex::new(std::mem::zeroed()) };

    let mut lock = ERROR_CONF.lock().unwrap();

    unsafe { errorInit(&mut *lock, WordWrap::Enabled as _, 0) };

    let old_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic_info| {
        // If we get a `WouldBlock` error, we know that the `Gfx` service has been initialized.
        // Otherwise fallback to using the old panic hook.
        if let (Err(TryLockError::WouldBlock), Ok(_apt)) = (GFX_ACTIVE.try_lock(), Apt::new()) {
            if call_old_hook {
                old_hook(panic_info);
            }

            let mut lock = ERROR_CONF.lock().unwrap();

            let error_conf = &mut *lock;

            let mut writer = Utf16Writer::new(&mut error_conf.Text);

            let thread = std::thread::current();

            let name = thread.name().unwrap_or("<unnamed>");

            let _ = write!(writer, "thread '{name}' {panic_info}");

            unsafe {
                errorDisp(error_conf);
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
