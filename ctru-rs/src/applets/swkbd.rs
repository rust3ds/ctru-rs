use bitflags::bitflags;
use ctru_sys::{
    self, swkbdInit, swkbdInputText, swkbdSetButton, swkbdSetFeatures, swkbdSetHintText, SwkbdState,
};
use std::iter::once;
use std::str;

/// An instance of the software keyboard.
pub struct Swkbd {
    state: Box<SwkbdState>,
}

/// The kind of keyboard to be initialized.
///
/// Normal is the full keyboard with several pages (QWERTY/accents/symbol/mobile)
/// Qwerty is a QWERTY-only keyboard.
/// Numpad is a number pad.
/// Western is a text keyboard without japanese symbols (only applies to JPN systems). For other
/// systems it's the same as a Normal keyboard.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum Kind {
    Normal = ctru_sys::SWKBD_TYPE_NORMAL,
    Qwerty = ctru_sys::SWKBD_TYPE_QWERTY,
    Numpad = ctru_sys::SWKBD_TYPE_NUMPAD,
    Western = ctru_sys::SWKBD_TYPE_WESTERN,
}

/// Represents which button the user pressed to close the software keyboard.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum Button {
    Left = ctru_sys::SWKBD_BUTTON_LEFT,
    Middle = ctru_sys::SWKBD_BUTTON_MIDDLE,
    Right = ctru_sys::SWKBD_BUTTON_RIGHT,
}

/// Error type for the software keyboard.
#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum Error {
    InvalidInput = ctru_sys::SWKBD_INVALID_INPUT,
    OutOfMem = ctru_sys::SWKBD_OUTOFMEM,
    HomePressed = ctru_sys::SWKBD_HOMEPRESSED,
    ResetPressed = ctru_sys::SWKBD_RESETPRESSED,
    PowerPressed = ctru_sys::SWKBD_POWERPRESSED,
    ParentalOk = ctru_sys::SWKBD_PARENTAL_OK,
    ParentalFail = ctru_sys::SWKBD_PARENTAL_FAIL,
    BannedInput = ctru_sys::SWKBD_BANNED_INPUT,
}

/// Restrictions on keyboard input
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ValidInput {
    Anything = ctru_sys::SWKBD_ANYTHING,
    NotEmpty = ctru_sys::SWKBD_NOTEMPTY,
    NotEmptyNotBlank = ctru_sys::SWKBD_NOTEMPTY_NOTBLANK,
    NotBlank = ctru_sys::SWKBD_NOTBLANK,
    FixedLen = ctru_sys::SWKBD_FIXEDLEN,
}

bitflags! {
    /// Keyboard feature flags
    pub struct Features: u32 {
        const PARENTAL_PIN      = ctru_sys::SWKBD_PARENTAL;
        const DARKEN_TOP_SCREEN  = ctru_sys::SWKBD_DARKEN_TOP_SCREEN;
        const PREDICTIVE_INPUT  = ctru_sys::SWKBD_PREDICTIVE_INPUT;
        const MULTILINE        = ctru_sys::SWKBD_MULTILINE;
        const FIXED_WIDTH       = ctru_sys::SWKBD_FIXED_WIDTH;
        const ALLOW_HOME        = ctru_sys::SWKBD_ALLOW_HOME;
        const ALLOW_RESET       = ctru_sys::SWKBD_ALLOW_RESET;
        const ALLOW_POWER       = ctru_sys::SWKBD_ALLOW_POWER;
        const DEFAULT_QWERTY    = ctru_sys::SWKBD_DEFAULT_QWERTY;
    }

    /// Keyboard input filtering flags
    pub struct Filters: u32 {
        const DIGITS    = ctru_sys::SWKBD_FILTER_DIGITS;
        const AT        = ctru_sys::SWKBD_FILTER_AT;
        const PERCENT   = ctru_sys::SWKBD_FILTER_PERCENT;
        const BACKSLASH = ctru_sys::SWKBD_FILTER_BACKSLASH;
        const PROFANITY = ctru_sys::SWKBD_FILTER_PROFANITY;
        const CALLBACK  = ctru_sys::SWKBD_FILTER_CALLBACK;
    }
}

impl Swkbd {
    /// Initializes a software keyboard of the specified type and the chosen number of buttons
    /// (from 1-3).
    pub fn init(keyboard_type: Kind, num_buttons: i32) -> Self {
        unsafe {
            let mut state = Box::<SwkbdState>::default();
            swkbdInit(state.as_mut(), keyboard_type.into(), num_buttons, -1);
            Swkbd { state }
        }
    }

    /// Gets input from this keyboard and appends it to the provided string.
    ///
    /// The text received from the keyboard will be truncated if it is greater than 2048 bytes
    /// in length.
    pub fn write_to_string(&mut self) -> Result<(String, Button), Error> {
        // Unfortunately the libctru API doesn't really provide a way to get the exact length
        // of the string that it receieves from the software keyboard. Instead it expects you
        // to pass in a buffer and hope that it's big enough to fit the entire string, so
        // you have to set some upper limit on the potential size of the user's input.
        const MAX_BYTES: usize = 2048;
        let mut buf = vec![0u8; MAX_BYTES];
        let button = self.write_bytes(&mut buf)?;

        // libctru does, however, seem to ensure that the buffer will always contain a properly
        // terminated UTF-8 sequence even if the input has to be truncated, so this operation
        // should be safe.
        let res = String::from_utf8(buf).unwrap();

        Ok((res, button))
    }

    /// Fills the provided buffer with a UTF-8 encoded, NUL-terminated sequence of bytes from
    /// this software keyboard.
    ///
    /// If the buffer is too small to contain the entire sequence received from the keyboard,
    /// the output will be truncated but should still be well-formed UTF-8.
    pub fn write_bytes(&mut self, buf: &mut [u8]) -> Result<Button, Error> {
        unsafe {
            match swkbdInputText(self.state.as_mut(), buf.as_mut_ptr(), buf.len()) {
                ctru_sys::SWKBD_BUTTON_NONE => Err(self.parse_swkbd_error()),
                ctru_sys::SWKBD_BUTTON_LEFT => Ok(Button::Left),
                ctru_sys::SWKBD_BUTTON_MIDDLE => Ok(Button::Middle),
                ctru_sys::SWKBD_BUTTON_RIGHT => Ok(Button::Right),
                _ => unreachable!(),
            }
        }
    }

    /// Sets special features for this keyboard
    pub fn set_features(&mut self, features: Features) {
        unsafe { swkbdSetFeatures(self.state.as_mut(), features.bits) }
    }

    /// Configures input validation for this keyboard
    pub fn set_validation(&mut self, validation: ValidInput, filters: Filters) {
        self.state.valid_input = validation.into();
        self.state.filter_flags = filters.bits;
    }

    /// Configures the maximum number of digits that can be entered in the keyboard when the
    /// `Filters::DIGITS` flag is enabled
    pub fn set_max_digits(&mut self, digits: u16) {
        self.state.max_digits = digits;
    }

    /// Sets the hint text for this software keyboard (that is, the help text that is displayed
    /// when the textbox is empty)
    pub fn set_hint_text(&mut self, text: &str) {
        unsafe {
            let nul_terminated: String = text.chars().chain(once('\0')).collect();
            swkbdSetHintText(self.state.as_mut(), nul_terminated.as_ptr());
        }
    }

    /// Configures the look and behavior of a button for this keyboard.
    ///
    /// `button` is the `Button` to be configured
    /// `text` configures the display text for the button
    /// `submit` configures whether pressing the button will accept the keyboard's input or
    /// discard it.
    pub fn configure_button(&mut self, button: Button, text: &str, submit: bool) {
        unsafe {
            let nul_terminated: String = text.chars().chain(once('\0')).collect();
            swkbdSetButton(
                self.state.as_mut(),
                button.into(),
                nul_terminated.as_ptr(),
                submit,
            );
        }
    }

    /// Configures the maximum number of UTF-16 code units that can be entered into the software
    /// keyboard. By default the limit is 0xFDE8 code units.
    ///
    /// Note that keyboard input is converted from UTF-16 to UTF-8 before being handed to Rust,
    /// so this code point limit does not necessarily equal the max number of UTF-8 code points
    /// receivable by the `get_utf8` and `get_bytes` functions.
    pub fn set_max_text_len(&mut self, len: u16) {
        self.state.max_text_len = len;
    }

    fn parse_swkbd_error(&self) -> Error {
        match self.state.result {
            ctru_sys::SWKBD_INVALID_INPUT => Error::InvalidInput,
            ctru_sys::SWKBD_OUTOFMEM => Error::OutOfMem,
            ctru_sys::SWKBD_HOMEPRESSED => Error::HomePressed,
            ctru_sys::SWKBD_RESETPRESSED => Error::ResetPressed,
            ctru_sys::SWKBD_POWERPRESSED => Error::PowerPressed,
            ctru_sys::SWKBD_PARENTAL_OK => Error::ParentalOk,
            ctru_sys::SWKBD_PARENTAL_FAIL => Error::ParentalFail,
            ctru_sys::SWKBD_BANNED_INPUT => Error::BannedInput,
            _ => unreachable!(),
        }
    }
}

impl Default for Swkbd {
    fn default() -> Self {
        Swkbd::init(Kind::Normal, 2)
    }
}

from_type_to_u32!(Kind);
from_type_to_u32!(Button);
from_type_to_u32!(Error);
from_type_to_i32!(ValidInput);
