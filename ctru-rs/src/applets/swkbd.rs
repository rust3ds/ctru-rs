//! Software Keyboard applet.
//!
//! This applet opens a virtual keyboard on the console's bottom screen which lets the user write UTF-16 valid text.
// TODO: Implement remaining functionality (password mode, filter callbacks, etc.). Also improve "max text length" API. Improve `number of buttons` API when creating a new SoftwareKeyboard.
// TODO: Split the Parental PIN lock operations into a different type.
#[doc(alias = "keyboard")]

use bitflags::bitflags;
use ctru_sys::{
    self, swkbdInit, swkbdInputText, swkbdSetButton, swkbdSetFeatures, swkbdSetHintText, SwkbdState,
};
use libc;
use std::fmt::Display;
use std::iter::once;
use std::str;

/// Configuration structure to setup the Software Keyboard applet.
#[doc(alias = "SwkbdState")]
#[derive(Clone)]
pub struct SoftwareKeyboard {
    state: Box<SwkbdState>,
}

/// The type of keyboard used by the [`SoftwareKeyboard`].
///
/// Can be set with [`SoftwareKeyboard::new()`]
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
    /// On JPN systems: a keyboard without japanese input capabilities.
    ///
    /// On any other region: same as [`Normal`](Kind::Normal).
    Western = ctru_sys::SWKBD_TYPE_WESTERN,
}

/// Represents which button the user pressed to close the [`SoftwareKeyboard`].
///
/// Button text and behaviour can be customized with [`SoftwareKeyboard::configure_button()`].
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

/// Error returned by an unsuccessful [`SoftwareKeyboard::get_string()`].
#[doc(alias = "SwkbdResult")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum Error {
    /// Invalid parameters given to the [`SoftwareKeyboard`] configuration.
    InvalidParameters = ctru_sys::SWKBD_INVALID_INPUT,
    /// [`SoftwareKeyboard`] ran out of memory.
    OutOfMem = ctru_sys::SWKBD_OUTOFMEM,
    /// Home button was pressed while [`SoftwareKeyboard`] was running.
    HomePressed = ctru_sys::SWKBD_HOMEPRESSED,
    /// Reset button was pressed while [`SoftwareKeyboard`] was running.
    ResetPressed = ctru_sys::SWKBD_RESETPRESSED,
    /// Power button was pressed while [`SoftwareKeyboard`] was running.
    PowerPressed = ctru_sys::SWKBD_POWERPRESSED,
    /// The parental lock PIN was correct.
    ///
    /// While this variant isn't *technically* considerable an error
    /// the result of a Parental PIN operation won't return a string to the program, thus it's still exceptional behaviour.
    ParentalOk = ctru_sys::SWKBD_PARENTAL_OK,
    /// The parental lock PIN was incorrect.
    ParentalFail = ctru_sys::SWKBD_PARENTAL_FAIL,
    /// Input triggered the filter.
    ///
    /// You can have a look at [`Filters`] to activate custom filters.
    BannedInput = ctru_sys::SWKBD_BANNED_INPUT,
}

/// Restrictions to enforce rules on the keyboard input.
///
/// See [`SoftwareKeyboard::set_validation()`]
#[doc(alias = "SwkbdValidInput")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ValidInput {
    /// All inputs are accepted.
    Anything = ctru_sys::SWKBD_ANYTHING,
    /// Empty inputs are not accepted.
    NotEmpty = ctru_sys::SWKBD_NOTEMPTY,
    /// Blank inputs (consisting only of whitespaces) are not accepted.
    NotBlank = ctru_sys::SWKBD_NOTBLANK,
    /// Neither empty inputs nor blank inputs are accepted.
    NotEmptyNotBlank = ctru_sys::SWKBD_NOTEMPTY_NOTBLANK,
    /// Input must have a fixed length. Maximum length can be specified with [`SoftwareKeyboard::set_max_text_len()`];
    FixedLen = ctru_sys::SWKBD_FIXEDLEN,
}

bitflags! {
    /// Special features that can be activated via [`SoftwareKeyboard::set_features()`].
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct Features: u32 {
        /// Parental PIN mode.
        ///
        /// # Notes
        ///
        /// Refer to [`Error::ParentalOk`] and [`Error::ParentalFail`] to check whether the Parental PIN lock was successfully opened.
        const PARENTAL_PIN      = ctru_sys::SWKBD_PARENTAL;
        /// Darken top screen while the [`SoftwareKeyboard`] is active.
        const DARKEN_TOP_SCREEN = ctru_sys::SWKBD_DARKEN_TOP_SCREEN;
        /// Enable predictive input (necessary for Kanji on JPN consoles).
        const PREDICTIVE_INPUT  = ctru_sys::SWKBD_PREDICTIVE_INPUT;
        /// Enable multiline input.
        const MULTILINE         = ctru_sys::SWKBD_MULTILINE;
        /// Enable fixed-width mode.
        const FIXED_WIDTH       = ctru_sys::SWKBD_FIXED_WIDTH;
        /// Allow the usage of the Home Button while the [`SoftwareKeyboard`] is running.
        const ALLOW_HOME        = ctru_sys::SWKBD_ALLOW_HOME;
        /// Allow the usage of the Reset Button while the [`SoftwareKeyboard`] is running.
        const ALLOW_RESET       = ctru_sys::SWKBD_ALLOW_RESET;
        /// Allow the usage of the Power Button while the [`SoftwareKeyboard`] is running.
        const ALLOW_POWER       = ctru_sys::SWKBD_ALLOW_POWER;
        /// Default to the QWERTY page when the [`SoftwareKeyboard`] is shown.
        const DEFAULT_QWERTY    = ctru_sys::SWKBD_DEFAULT_QWERTY;
    }

    /// Availble filters to disallow some types of input for the [`SoftwareKeyboard`].
    ///
    /// See [`SoftwareKeyboard::set_validation()`]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct Filters: u32 {
        /// Disallow the usage of numerical digits.
        ///
        /// The maximum number of digits that are allowed to be used while this filter is active
        /// can be configured with [`SoftwareKeyboard::set_max_digits()`] (default is 0).
        const DIGITS    = ctru_sys::SWKBD_FILTER_DIGITS;
        /// Disallow the usage of the "at" (@) sign.
        const AT        = ctru_sys::SWKBD_FILTER_AT;
        /// Disallow the usage of the "percent" (%) sign.
        const PERCENT   = ctru_sys::SWKBD_FILTER_PERCENT;
        /// Disallow the usage of the "backslash" (\) sign.
        const BACKSLASH = ctru_sys::SWKBD_FILTER_BACKSLASH;
        /// Disallow the use of profanity via Nintendo's profanity filter.
        const PROFANITY = ctru_sys::SWKBD_FILTER_PROFANITY;
        /// Use a custom callback in order to filter the input.
        ///
        /// TODO: It's currently impossible to setup a custom filter callback.
        const CALLBACK  = ctru_sys::SWKBD_FILTER_CALLBACK;
    }
}

impl SoftwareKeyboard {
    /// Initialize a new configuration for the Software Keyboard applet depending on how many "exit" buttons are available to the user (1, 2 or 3).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// #
    /// use ctru::applets::swkbd::{SoftwareKeyboard, Kind};
    ///
    /// // Standard keyboard.
    /// let keyboard = SoftwareKeyboard::new(Kind::Normal, 2);
    ///
    /// // Numpad (with only the "confirm" button).
    /// let keyboard = SoftwareKeyboard::new(Kind::Numpad, 1);
    /// #
    /// # }
    #[doc(alias = "swkbdInit")]
    pub fn new(keyboard_type: Kind, num_buttons: i32) -> Self {
        unsafe {
            let mut state = Box::<SwkbdState>::default();
            swkbdInit(state.as_mut(), keyboard_type.into(), num_buttons, -1);
            SoftwareKeyboard { state }
        }
    }

    /// Launches the applet based on the given configuration and returns a string containing the text input.
    ///
    /// # Notes
    ///
    /// The text received from the keyboard will be truncated if it is longer than `max_bytes`.
    ///
    /// TODO: UNSAFE OPERATION, LAUNCHING APPLETS REQUIRES GRAPHICS, WITHOUT AN ACTIVE GFX THIS WILL CAUSE A SEGMENTATION FAULT.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::applets::swkbd::SoftwareKeyboard;
    /// let mut keyboard = SoftwareKeyboard::default();
    ///
    /// let (text, button) = keyboard.get_string(2048)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
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
    /// # Notes
    ///
    /// If the buffer is too small to contain the entire sequence received from the keyboard,
    /// the output will be truncated.
    ///
    /// TODO: UNSAFE OPERATION, LAUNCHING APPLETS REQUIRES GRAPHICS, WITHOUT AN ACTIVE GFX THIS WILL CAUSE A SEGMENTATION FAULT.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::applets::swkbd::SoftwareKeyboard;
    /// let mut keyboard = SoftwareKeyboard::default();
    ///
    /// let mut buffer = vec![0; 100];
    ///
    /// let button = keyboard.write_exact(&mut buffer)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
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

    /// Set special features for this keyboard.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// #
    /// use ctru::applets::swkbd::{SoftwareKeyboard, Features};
    ///
    /// let mut keyboard = SoftwareKeyboard::default();
    ///
    /// let features = Features::DARKEN_TOP_SCREEN & Features::MULTILINE;
    /// keyboard.set_features(features);
    /// #
    /// # }
    #[doc(alias = "swkbdSetFeatures")]
    pub fn set_features(&mut self, features: Features) {
        unsafe { swkbdSetFeatures(self.state.as_mut(), features.bits()) }
    }

    /// Configure input validation for this keyboard.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// #
    /// use ctru::applets::swkbd::{SoftwareKeyboard, ValidInput, Filters};
    /// let mut keyboard = SoftwareKeyboard::default();
    ///
    /// // Disallow empty or blank input.
    /// let validation = ValidInput::NotEmptyNotBlank;
    ///
    /// // Disallow the use of numerical digits and profanity.
    /// let filters = Filters::DIGITS & Filters::PROFANITY;
    /// keyboard.set_validation(validation, filters);
    /// #
    /// # }
    pub fn set_validation(&mut self, validation: ValidInput, filters: Filters) {
        self.state.valid_input = validation.into();
        self.state.filter_flags = filters.bits();
    }

    /// Configure the maximum number of digits that can be entered in the keyboard when the [`Filters::DIGITS`] flag is enabled.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// #
    /// use ctru::applets::swkbd::{SoftwareKeyboard, ValidInput, Filters};
    /// let mut keyboard = SoftwareKeyboard::default();
    ///
    /// // Disallow empty or blank input.
    /// let validation = ValidInput::NotEmptyNotBlank;
    ///
    /// // Disallow the use of numerical digits. This filter is customizable thanks to `set_max_digits`.
    /// let filters = Filters::DIGITS;
    /// keyboard.set_validation(validation, filters);
    ///
    /// // No more than 3 numbers are accepted.
    /// keyboard.set_max_digits(3);
    /// #
    /// # }
    pub fn set_max_digits(&mut self, digits: u16) {
        self.state.max_digits = digits;
    }

    /// Set the hint text for this software keyboard.
    ///
    /// The hint text is the text shown in gray before any text gets written in the input box.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// #
    /// use ctru::applets::swkbd::SoftwareKeyboard;
    /// let mut keyboard = SoftwareKeyboard::default();
    ///
    /// keyboard.set_hint_text("Write here what you like!");
    /// #
    /// # }
    #[doc(alias = "swkbdSetHintText")]
    pub fn set_hint_text(&mut self, text: &str) {
        unsafe {
            let nul_terminated: String = text.chars().chain(once('\0')).collect();
            swkbdSetHintText(self.state.as_mut(), nul_terminated.as_ptr());
        }
    }

    /// Configure the look and behavior of a button for this keyboard.
    ///
    /// # Arguments
    ///
    /// - `button` - the [`Button`] to be configured based on the position.
    /// - `text` - the text displayed in the button.
    /// - `submit` - whether pressing the button will accept the keyboard's input or discard it.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// #
    /// use ctru::applets::swkbd::{SoftwareKeyboard, Button, Kind};
    ///
    /// // We create a `SoftwareKeyboard` with left and right buttons.
    /// let mut keyboard = SoftwareKeyboard::new(Kind::Normal, 2);
    ///
    /// // Set the left button text to "Cancel" and pressing it will NOT return the user's input.
    /// keyboard.configure_button(Button::Left, "Cancel", false);
    ///
    /// // Set the right button text to "Ok" and pressing it will return the user's input.
    /// keyboard.configure_button(Button::Right, "Ok", true);
    /// #
    /// # }
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

    /// Configure the maximum number of UTF-16 code units that can be entered into the software
    /// keyboard. By default the limit is `65000` code units.
    ///
    /// # Notes
    ///
    /// Keyboard input is converted from UTF-16 to UTF-8 before being handed to Rust,
    /// so this code point limit does not necessarily equal the max number of UTF-8 code points
    /// receivable by [`SoftwareKeyboard::get_string()`] and [`SoftwareKeyboard::write_exact()`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// #
    /// use ctru::applets::swkbd::{SoftwareKeyboard, Button, Kind};
    /// let mut keyboard = SoftwareKeyboard::default();
    ///
    /// // Set the maximum text length to 18 UTF-16 code units.
    /// keyboard.set_max_text_len(18);
    /// #
    /// # }
    pub fn set_max_text_len(&mut self, len: u16) {
        self.state.max_text_len = len;
    }

    fn parse_swkbd_error(&self) -> Error {
        match self.state.result {
            ctru_sys::SWKBD_INVALID_INPUT => Error::InvalidParameters,
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

/// Creates a new [`SoftwareKeyboard`] configuration set to using a [`Kind::Normal`] keyboard and 2 [`Button`]s.
impl Default for SoftwareKeyboard {
    fn default() -> Self {
        SoftwareKeyboard::new(Kind::Normal, 2)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidParameters => write!(
                f,
                "software keyboard was configured with invalid parameters"
            ),
            Self::OutOfMem => write!(f, "software keyboard ran out of memory"),
            Self::HomePressed => {
                write!(f, "home button pressed while software keyboard was running")
            }
            Self::ResetPressed => write!(
                f,
                "reset button pressed while software keyboard was running"
            ),
            Self::PowerPressed => write!(
                f,
                "power button pressed while software keyboard was running"
            ),
            Self::ParentalOk => write!(f, "parental lock pin was correct"),
            Self::ParentalFail => write!(f, "parental lock pin was incorrect"),
            Self::BannedInput => write!(
                f,
                "input given to the software keyboard triggered the active filters"
            ),
        }
    }
}

impl std::error::Error for Error {}

from_impl!(Kind, ctru_sys::SwkbdType);
from_impl!(Button, ctru_sys::SwkbdButton);
from_impl!(Error, ctru_sys::SwkbdResult);
from_impl!(ValidInput, i32);
