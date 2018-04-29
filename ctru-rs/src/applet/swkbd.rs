use std::mem;
use std::str;

use libctru::{self, SwkbdState, swkbdInit, swkbdSetFeatures, swkbdInputText};

use libc;

/// An instance of the software keyboard.
pub struct Swkbd {
    state: Box<SwkbdState>,
}

/// The kind of keyboard to be initialized.
///
/// Normal is the full keyboard with several pages (QUERTY/accents/symbol/mobile)
/// Querty is a QUERTY-only keyboard.
/// Numpad is a number pad.
/// Western is a text keyboard without japanese symbols (only applies to JPN systems). For other
/// systems it's the same as a Normal keyboard.
#[derive(Copy, Clone, Debug)]
pub enum Kind {
    Normal,
    Querty,
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

/// Keyboard feature flags
bitflags! {
    pub struct Features: u32 {
        const PARENTAL_PIN      = 1 << 0;
        const DARKEN_TOP_SCREEN = 1 << 1;
        const PREDICTIVE_INPUT  = 1 << 2;
        const MULTILINE         = 1 << 3;
        const FIXED_WIDTH       = 1 << 4;
        const ALLOW_HOME        = 1 << 5;
        const ALLOW_RESET       = 1 << 6;
        const ALLOW_POWER       = 1 << 7;
        const DEFAULT_QUERTY    = 1 << 8;
    }
}

/// Keyboard input filtering flags
bitflags! {
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
            let mut state = Box::new(mem::uninitialized::<SwkbdState>());
            swkbdInit(state.as_mut(), keyboard_type as u32, num_buttons, -1);
            Swkbd { state }
        }
    }

    /// Gets input from this keyboard and appends it to the provided string.
    ///
    /// The text received from the keyboard can be up to 2048 bytes in length.
    pub fn get_utf8(&mut self, buf: &mut String) -> Result<Button, Error> {
        // Unfortunately the libctru API doesn't really provide a way to get the exact length
        // of the string that it receieves from the software keyboard. Instead it expects you
        // to pass in a buffer and hope that it's big enough to fit the entire string, so
        // you have to set some upper limit on the potential size of the user's input.
        const MAX_BYTES: usize = 2048;
        let mut tmp = [0u8; MAX_BYTES];
        let button = unsafe { self.get_bytes(&mut tmp)? };

        // Make sure we haven't overflowed our buffer. libctru might already check this,
        // but we'll do it here too just in case
        let len = unsafe { libc::strlen(tmp.as_ptr()) };
        assert!(len <= MAX_BYTES);

        // Not sure if this is falliable or not in this stage of the process,
        // but catch any decoding errors to be sure
        let utf8 = match str::from_utf8(&tmp[..len]) {
            Ok(parsed) => parsed,
            Err(_) => return Err(Error::InvalidInput),
        };

        // Finally, copy the validated input into the user's `String`
        *buf += utf8;
        Ok(button)
    }

    /// Fills the provided buffer with a NUL-terminated sequence of bytes from the software
    /// keyboard
    /// 
    /// # Unsafety
    ///
    /// The received bytes are nominally UTF-8 formatted, but the provided buffer must be large
    /// enough to receive both the text from the software keyboard along with a NUL-terminator.
    /// Otherwise undefined behavior can result.
    pub unsafe fn get_bytes(&mut self, buf: &mut [u8]) -> Result<Button, Error> {
        match swkbdInputText(self.state.as_mut(), buf.as_mut_ptr(), buf.len()) {
            libctru::SWKBD_BUTTON_NONE => Err(self.parse_swkbd_error()),
            libctru::SWKBD_BUTTON_LEFT => Ok(Button::Left),
            libctru::SWKBD_BUTTON_MIDDLE => Ok(Button::Middle),
            libctru::SWKBD_BUTTON_RIGHT => Ok(Button::Right),
            _ => unreachable!(),
        }
    }

    /// Sets special features for this keyboard
    pub fn set_features(&mut self, features: Features) {
        unsafe {
            swkbdSetFeatures(self.state.as_mut(), features.bits)
        }
    }

    /// Configures input validation for this keyboard
    pub fn set_validation(&mut self, validation: ValidInput,
                          filters: Filters) {
        self.state.valid_input = validation as i32;
        self.state.filter_flags = filters.bits;
    }

    /// Configures the maximum number of digits that can be entered in the keyboard when the
    /// `Filters::DIGITS` flag is enabled
    pub fn set_max_digits(&mut self, digits: u16) {
        self.state.max_digits = digits;
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
