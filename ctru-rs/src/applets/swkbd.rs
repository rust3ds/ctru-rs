use bitflags::bitflags;
use ctru_sys::{
    self, swkbdInit, swkbdInputText, swkbdSetButton, swkbdSetFeatures, swkbdSetHintText, SwkbdState,
};
use libc;
use std::iter::once;
use std::str;

/// An instance of the software keyboard.
#[doc(alias = "SwkbdState")]
#[derive(Clone)]
pub struct Swkbd {
    state: Box<SwkbdState>,
}

/// The kind of keyboard to be initialized.
#[doc(alias = "SwkbdType")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Kind {
    /// Normal keyboard composed of several pages (QWERTY, accents, symbols, mobile).
    Normal = ctru_sys::SWKBD_TYPE_NORMAL,
    /// Only QWERTY keyboard.
    Qwerty = ctru_sys::SWKBD_TYPE_QWERTY,
    /// Only number pad.
    Numpad = ctru_sys::SWKBD_TYPE_NUMPAD,
    /// On JPN systems: a keyboard without japanese input capablities.
    ///
    /// On any other region: same as [`Normal`](Kind::Normal).
    Western = ctru_sys::SWKBD_TYPE_WESTERN,
}

/// Represents which button the user pressed to close the software keyboard.
#[doc(alias = "SwkbdButton")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Button {
    /// Left button. Usually corresponds to "Cancel".
    Left = ctru_sys::SWKBD_BUTTON_LEFT,
    /// Middle button. Usually corresponds to "I Forgot".
    Middle = ctru_sys::SWKBD_BUTTON_MIDDLE,
    /// Right button. Usually corresponds to "OK".
    Right = ctru_sys::SWKBD_BUTTON_RIGHT,
}

/// Error type for the software keyboard.
#[doc(alias = "SwkbdResult")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum Error {
    /// Invalid parameters inputted in the Software Keyboard.
    InvalidInput = ctru_sys::SWKBD_INVALID_INPUT,
    /// Out of memory.
    OutOfMem = ctru_sys::SWKBD_OUTOFMEM,
    /// Home button was pressed during execution.
    HomePressed = ctru_sys::SWKBD_HOMEPRESSED,
    /// Reset button was pressed during execution.
    ResetPressed = ctru_sys::SWKBD_RESETPRESSED,
    /// Power button was pressed during execution.
    PowerPressed = ctru_sys::SWKBD_POWERPRESSED,
    /// The parental PIN was correct.
    ParentalOk = ctru_sys::SWKBD_PARENTAL_OK,
    /// The parental PIN was incorrect.
    ParentalFail = ctru_sys::SWKBD_PARENTAL_FAIL,
    /// Input triggered the filter.
    BannedInput = ctru_sys::SWKBD_BANNED_INPUT,
}

/// Restrictions on keyboard input.
#[doc(alias = "SwkbdValidInput")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ValidInput {
    /// All inputs are accepted.
    Anything = ctru_sys::SWKBD_ANYTHING,
    /// Empty inputs are not accepted.
    NotEmpty = ctru_sys::SWKBD_NOTEMPTY,
    /// Blank (consisting only of whitespaces) inputs are not accepted.
    NotBlank = ctru_sys::SWKBD_NOTBLANK,
    /// Neither empty inputs nor blank inputs are accepted.
    NotEmptyNotBlank = ctru_sys::SWKBD_NOTEMPTY_NOTBLANK,
    /// Input must have a fixed length. Maximum length can be specified with [`Swkbd::set_max_text_len`];
    FixedLen = ctru_sys::SWKBD_FIXEDLEN,
}

bitflags! {
    /// Keyboard feature flags.
    pub struct Features: u32 {
        /// Parental PIN mode.
        const PARENTAL_PIN      = ctru_sys::SWKBD_PARENTAL;
        /// Darken top screen while the Software Keyboard is active.
        const DARKEN_TOP_SCREEN  = ctru_sys::SWKBD_DARKEN_TOP_SCREEN;
        /// Enable predictive input (necessary for Kanji on JPN consoles).
        const PREDICTIVE_INPUT  = ctru_sys::SWKBD_PREDICTIVE_INPUT;
        /// Enable multiline input.
        const MULTILINE        = ctru_sys::SWKBD_MULTILINE;
        /// Enable fixed-width mode.
        const FIXED_WIDTH       = ctru_sys::SWKBD_FIXED_WIDTH;
        /// Allow the usage of the Home Button while the Software Keyboard is active.
        const ALLOW_HOME        = ctru_sys::SWKBD_ALLOW_HOME;
        /// Allow the usage of the Reset Button while the Software Keyboard is active.
        const ALLOW_RESET       = ctru_sys::SWKBD_ALLOW_RESET;
        /// Allow the usage of the Power Button while the Software Keyboard is active.
        const ALLOW_POWER       = ctru_sys::SWKBD_ALLOW_POWER;
        /// Default to the QWERTY page when the Software Keyboard is shown.
        const DEFAULT_QWERTY    = ctru_sys::SWKBD_DEFAULT_QWERTY;
    }

    /// Keyboard input filtering flags
    pub struct Filters: u32 {
        /// Disallows the usage of numerical digits.
        const DIGITS    = ctru_sys::SWKBD_FILTER_DIGITS;
        /// Disallows the usage of the "at" (@) sign.
        const AT        = ctru_sys::SWKBD_FILTER_AT;
        /// Disallows the usage of the "percent" (%) sign.
        const PERCENT   = ctru_sys::SWKBD_FILTER_PERCENT;
        /// Disallows the usage of the "backslash" (\) sign.
        const BACKSLASH = ctru_sys::SWKBD_FILTER_BACKSLASH;
        /// Disallows the use of profanity via Nintendo's profanity filter.
        const PROFANITY = ctru_sys::SWKBD_FILTER_PROFANITY;
        /// Use a custom callback in order to filter the input.
        const CALLBACK  = ctru_sys::SWKBD_FILTER_CALLBACK;
    }
}

impl Swkbd {
    /// Initializes a software keyboard of the specified type and the chosen number of buttons
    /// (from 1-3).
    #[doc(alias = "swkbdInit")]
    pub fn new(keyboard_type: Kind, num_buttons: i32) -> Self {
        unsafe {
            let mut state = Box::<SwkbdState>::default();
            swkbdInit(state.as_mut(), keyboard_type.into(), num_buttons, -1);
            Swkbd { state }
        }
    }

    /// Gets input from this keyboard and appends it to the provided string.
    ///
    /// The text received from the keyboard will be truncated if it is longer than `max_bytes`.
    #[doc(alias = "swkbdInputText")]
    pub fn get_string(&mut self, max_bytes: usize) -> Result<(String, Button), Error> {
        // Unfortunately the libctru API doesn't really provide a way to get the exact length
        // of the string that it receieves from the software keyboard. Instead it expects you
        // to pass in a buffer and hope that it's big enough to fit the entire string, so
        // you have to set some upper limit on the potential size of the user's input.
        let mut tmp = vec![0u8; max_bytes];
        let button = self.write_exact(&mut tmp)?;

        // libctru does, however, seem to ensure that the buffer will always contain a properly
        // terminated UTF-8 sequence even if the input has to be truncated, so these operations
        // should be safe.
        let len = unsafe { libc::strlen(tmp.as_ptr()) };
        tmp.truncate(len);

        let res = unsafe { String::from_utf8_unchecked(tmp) };

        Ok((res, button))
    }

    /// Fills the provided buffer with a UTF-8 encoded, NUL-terminated sequence of bytes from
    /// this software keyboard.
    ///
    /// If the buffer is too small to contain the entire sequence received from the keyboard,
    /// the output will be truncated but should still be well-formed UTF-8.
    #[doc(alias = "swkbdInputText")]
    pub fn write_exact(&mut self, buf: &mut [u8]) -> Result<Button, Error> {
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
    #[doc(alias = "swkbdSetFeatures")]
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
    #[doc(alias = "swkbdSetHintText")]
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
    #[doc(alias = "swkbdSetButton")]
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
        Swkbd::new(Kind::Normal, 2)
    }
}

from_impl!(Kind, ctru_sys::SwkbdType);
from_impl!(Button, ctru_sys::SwkbdButton);
from_impl!(Error, ctru_sys::SwkbdResult);
from_impl!(ValidInput, i32);
