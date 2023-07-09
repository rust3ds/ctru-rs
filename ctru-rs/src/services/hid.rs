//! HID service
//!
//! The HID service provides access to user input such as button presses, touch screen presses,
//! and circle pad information. It also provides information from the sound volume slider,
//! the accelerometer, and the gyroscope.

use crate::error::ResultCode;
bitflags::bitflags! {
    /// A set of flags corresponding to the button and directional pad
    /// inputs on the 3DS
    pub struct KeyPad: u32 {
        /// A button.
        const A             = ctru_sys::KEY_A;
        /// B button.
        const B             = ctru_sys::KEY_B;
        /// Select button.
        const SELECT        = ctru_sys::KEY_SELECT;
        /// Start button.
        const START         = ctru_sys::KEY_START;
        /// D-Pad Right.
        const DPAD_RIGHT        = ctru_sys::KEY_DRIGHT;
        /// D-Pad Left.
        const DPAD_LEFT         = ctru_sys::KEY_DLEFT;
        /// D-Pad Up.
        const DPAD_UP           = ctru_sys::KEY_DUP;
        /// D-Pad Down.
        const DPAD_DOWN         = ctru_sys::KEY_DDOWN;
        /// R button.
        const R             = ctru_sys::KEY_R;
        /// L button.
        const L             = ctru_sys::KEY_L;
        /// X button.
        const X             = ctru_sys::KEY_X;
        /// Y button.
        const Y             = ctru_sys::KEY_Y;
        /// ZL button.
        const ZL            = ctru_sys::KEY_ZL;
        /// ZR button.
        const ZR            = ctru_sys::KEY_ZR;
        /// Touchscreen.
        const TOUCH         = ctru_sys::KEY_TOUCH;
        /// C-Stick Right.
        const CSTICK_RIGHT  = ctru_sys::KEY_CSTICK_RIGHT;
        /// C-Stick Left.
        const CSTICK_LEFT   = ctru_sys::KEY_CSTICK_LEFT;
        /// C-Stick Up.
        const CSTICK_UP     = ctru_sys::KEY_CSTICK_UP;
        /// C-Stick Down.
        const CSTICK_DOWN   = ctru_sys::KEY_CSTICK_DOWN;
        /// CirclePad Right.
        const CPAD_RIGHT    = ctru_sys::KEY_CPAD_RIGHT;
        /// CirclePad Left.
        const CPAD_LEFT     = ctru_sys::KEY_CPAD_LEFT;
        /// CirclePad Up.
        const CPAD_UP       = ctru_sys::KEY_CPAD_UP;
        /// CirclePad Down.
        const CPAD_DOWN     = ctru_sys::KEY_CPAD_DOWN;

        // Convenience catch-all for the D-Pad and the C-Pad

        /// Direction Up (either D-Pad or C-Pad).
        const UP    = KeyPad::DPAD_UP.bits()    | KeyPad::CPAD_UP.bits();
        /// Direction Down (either D-Pad or C-Pad).
        const DOWN  = KeyPad::DPAD_DOWN.bits()  | KeyPad::CPAD_DOWN.bits();
        /// Direction Left (either D-Pad or C-Pad).
        const LEFT  = KeyPad::DPAD_LEFT.bits()  | KeyPad::CPAD_LEFT.bits();
        /// Direction Right (either D-Pad or C-Pad)..
        const RIGHT = KeyPad::DPAD_RIGHT.bits() | KeyPad::CPAD_RIGHT.bits();
    }
}

/// A reference-counted handle to the HID service. The service is closed
/// when all instances of this struct fall out of scope.
///
/// This service requires no special permissions to use.
pub struct Hid(());

impl Hid {
    /// Initializes the HID service.
    ///
    /// # Errors
    ///
    /// This function will return an error if the service was unable to be initialized.
    /// Since this service requires no special or elevated permissions, errors are
    /// rare in practice.
    #[doc(alias = "hidInit")]
    pub fn new() -> crate::Result<Hid> {
        unsafe {
            ResultCode(ctru_sys::hidInit())?;
            Ok(Hid(()))
        }
    }

    /// Scans the HID service for all user input occurring on the current
    /// frame. This function should be called on every frame when polling
    /// for user input.
    #[doc(alias = "hidScanInput")]
    pub fn scan_input(&mut self) {
        unsafe { ctru_sys::hidScanInput() };
    }

    /// Returns a bitflag struct representing which buttons have just been pressed
    /// on the current frame (and were not pressed on the previous frame).
    #[doc(alias = "hidKeysDown")]
    pub fn keys_down(&self) -> KeyPad {
        unsafe {
            let keys = ctru_sys::hidKeysDown();
            KeyPad::from_bits_truncate(keys)
        }
    }

    /// Returns a bitflag struct representing which buttons have been held down
    /// during the current frame.
    #[doc(alias = "hidKeysHeld")]
    pub fn keys_held(&self) -> KeyPad {
        unsafe {
            let keys = ctru_sys::hidKeysHeld();
            KeyPad::from_bits_truncate(keys)
        }
    }

    /// Returns a bitflag struct representing which buttons have just been released on
    /// the current frame.
    #[doc(alias = "hidKeysUp")]
    pub fn keys_up(&self) -> KeyPad {
        unsafe {
            let keys = ctru_sys::hidKeysUp();
            KeyPad::from_bits_truncate(keys)
        }
    }

    /// Returns the current touch position in pixels (x, y).
    ///
    /// # Notes
    ///
    /// (0, 0) represents the top left corner of the screen.
    #[doc(alias = "hidTouchRead")]
    pub fn touch_position(&mut self) -> (u16, u16) {
        let mut res = ctru_sys::touchPosition { px: 0, py: 0 };

        unsafe {
            ctru_sys::hidTouchRead(&mut res);
        }
        (res.px, res.py)
    }

    /// Returns the current circle pad position in relative (x, y).
    ///
    /// # Notes
    ///
    /// (0, 0) represents the center of the circle pad.
    #[doc(alias = "hidCircleRead")]
    pub fn circlepad_position(&mut self) -> (i16, i16) {
        let mut res = ctru_sys::circlePosition { dx: 0, dy: 0 };

        unsafe {
            ctru_sys::hidCircleRead(&mut res);
        }
        (res.dx, res.dy)
    }
}

impl Drop for Hid {
    #[doc(alias = "hidExit")]
    fn drop(&mut self) {
        unsafe { ctru_sys::hidExit() };
    }
}
