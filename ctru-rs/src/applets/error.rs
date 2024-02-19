//! Error applet
//!
//! This applet displays error text as a pop-up message on the lower screen.
#![doc(alias = "Error")]

use crate::services::{apt::Apt, gfx::Gfx};

use ctru_sys::errorConf;

/// Configuration struct to set up the Error applet.
#[doc(alias = "errorConf")]
pub struct ErrorApplet {
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

impl ErrorApplet {
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
    pub fn launch(&mut self, _apt: &Apt, _gfx: &Gfx) {
        unsafe { ctru_sys::errorDisp(self.state.as_mut()) }
    }
}
