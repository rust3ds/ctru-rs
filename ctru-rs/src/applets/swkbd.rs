use std::convert::TryInto;
use std::iter::once;
use std::mem::MaybeUninit;
use std::str;

use libctru::{
    self, swkbdInit, swkbdInputText, swkbdSetButton, swkbdSetFeatures, swkbdSetHintText, SwkbdState,
};

use libc;

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
pub enum Kind {
    Normal,
    Qwerty,
    Numpad,
    Western,
}

/// Represents which button the user pressed to close the software keyboard.
#[derive(Copy, Clone, Debug)]
pub enum Button {
    Left,
    Middle,
    Right,
}

/// Error type for the software keyboard.
#[derive(Copy, Clone, Debug)]
pub enum Error {
    InvalidInput,
    OutOfMem,
    HomePressed,
    ResetPressed,
    PowerPressed,
    ParentalOk,
    ParentalFail,
    BannedInput,
}

/// Restrictions on keyboard input
#[derive(Copy, Clone, Debug)]
pub enum ValidInput {
    Anything,
    NotEmpty,
    NotEmptyNotBlank,
    NotBlank,
    FixedLen,
}

bitflags! {
    /// Keyboard feature flags
    pub struct Features: u32 {
        const PARENTAL_PIN      = 1 << 0;
        const DARKEN_TOP_SCREEN = 1 << 1;
        const PREDICTIVE_INPUT  = 1 << 2;
        const MULTILINE         = 1 << 3;
        const FIXED_WIDTH       = 1 << 4;
        const ALLOW_HOME        = 1 << 5;
        const ALLOW_RESET       = 1 << 6;
        const ALLOW_POWER       = 1 << 7;
        const DEFAULT_QWERTY    = 1 << 8;
    }
}

bitflags! {
    /// Keyboard input filtering flags
    pub struct Filters: u32 {
        const DIGITS    = 1 << 0;
        const AT        = 1 << 1;
        const PERCENT   = 1 << 2;
        const BACKSLASH = 1 << 3;
        const PROFANITY = 1 << 4;
        const CALLBACK  = 1 << 5;
    }
}

impl Swkbd {
    /// Initializes a software keyboard of the specified type and the chosen number of buttons
    /// (from 1-3).
    pub fn init(keyboard_type: Kind, num_buttons: i32) -> Self {
        unsafe {
            let mut state = MaybeUninit::<SwkbdState>::uninit();
            swkbdInit(state.as_mut_ptr(), keyboard_type as u32, num_buttons, -1);
            Swkbd {
                state: Box::new(state.assume_init()),
            }
        }
    }

    /// Gets input from this keyboard and appends it to the provided string.
    ///
    /// The text received from the keyboard will be truncated if it is greater than 2048 bytes
    /// in length.
    pub fn get_utf8(&mut self, buf: &mut String) -> Result<Button, Error> {
        // Unfortunately the libctru API doesn't really provide a way to get the exact length
        // of the string that it receieves from the software keyboard. Instead it expects you
        // to pass in a buffer and hope that it's big enough to fit the entire string, so
        // you have to set some upper limit on the potential size of the user's input.
        const MAX_BYTES: usize = 2048;
        let mut tmp = [0u8; MAX_BYTES];
        let button = self.get_bytes(&mut tmp)?;

        // libctru does, however, seem to ensure that the buffer will always contain a properly
        // terminated UTF-8 sequence even if the input has to be truncated, so these operations
        // should be safe.
        let len = unsafe { libc::strlen(tmp.as_ptr()) };
        let utf8 = unsafe { str::from_utf8_unchecked(&tmp[..len]) };

        // Copy the input into the user's `String`
        *buf += utf8;
        Ok(button)
    }

    /// Fills the provided buffer with a UTF-8 encoded, NUL-terminated sequence of bytes from
    /// this software keyboard.
    ///
    /// If the buffer is too small to contain the entire sequence received from the keyboard,
    /// the output will be truncated but should still be well-formed UTF-8
    pub fn get_bytes(&mut self, buf: &mut [u8]) -> Result<Button, Error> {
        unsafe {
            match swkbdInputText(
                self.state.as_mut(),
                buf.as_mut_ptr(),
                buf.len().try_into().unwrap(),
            ) {
                libctru::SWKBD_BUTTON_NONE => Err(self.parse_swkbd_error()),
                libctru::SWKBD_BUTTON_LEFT => Ok(Button::Left),
                libctru::SWKBD_BUTTON_MIDDLE => Ok(Button::Middle),
                libctru::SWKBD_BUTTON_RIGHT => Ok(Button::Right),
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
        self.state.valid_input = validation as i32;
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
                button as u32,
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
            libctru::SWKBD_INVALID_INPUT => Error::InvalidInput,
            libctru::SWKBD_OUTOFMEM => Error::OutOfMem,
            libctru::SWKBD_HOMEPRESSED => Error::HomePressed,
            libctru::SWKBD_RESETPRESSED => Error::ResetPressed,
            libctru::SWKBD_POWERPRESSED => Error::PowerPressed,
            libctru::SWKBD_PARENTAL_OK => Error::ParentalOk,
            libctru::SWKBD_PARENTAL_FAIL => Error::ParentalFail,
            libctru::SWKBD_BANNED_INPUT => Error::BannedInput,
            _ => unreachable!(),
        }
    }
}

impl Default for Swkbd {
    fn default() -> Self {
        Swkbd::init(Kind::Normal, 2)
    }
}
