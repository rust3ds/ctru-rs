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

/// Sets a custom panic hook that uses the error applet to display panic messages. You can also choose to have
/// messages printed over stderr along with the pop-up display.
///
/// # Safety
///
/// The error applet requires that both the [`Apt`] and [`Gfx`] services are active whenever it launches.
/// By calling this function, you promise that you will keep those services alive until either the program ends or
/// you unregister this hook with [`std::panic::take_hook`](https://doc.rust-lang.org/std/panic/fn.take_hook.html).
pub unsafe fn set_panic_hook(use_stderr: bool) {
    std::panic::set_hook(Box::new(move |panic_info| {
        let mut popup = PopUp::new(Kind::Top);

        let thread = std::thread::current();

        let name = thread.name().unwrap_or("<unnamed>");

        let payload = format!("thread '{name}' {panic_info}");

        popup.set_text(&payload);

        if use_stderr {
            eprintln!("{payload}");
        }

        unsafe {
            popup.launch_unchecked();
        }
    }))
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
