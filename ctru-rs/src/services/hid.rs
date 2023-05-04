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
        const A             = ctru_sys::KEY_A;
        const B             = ctru_sys::KEY_B;
        const SELECT        = ctru_sys::KEY_SELECT;
        const START         = ctru_sys::KEY_START;
        const DPAD_RIGHT        = ctru_sys::KEY_DRIGHT;
        const DPAD_LEFT         = ctru_sys::KEY_DLEFT;
        const DPAD_UP           = ctru_sys::KEY_DUP;
        const DPAD_DOWN         = ctru_sys::KEY_DDOWN;
        const R             = ctru_sys::KEY_R;
        const L             = ctru_sys::KEY_L;
        const X             = ctru_sys::KEY_X;
        const Y             = ctru_sys::KEY_Y;
        const ZL            = ctru_sys::KEY_ZL;
        const ZR            = ctru_sys::KEY_ZR;
        const TOUCH         = ctru_sys::KEY_TOUCH;
        const CSTICK_RIGHT  = ctru_sys::KEY_CSTICK_RIGHT;
        const CSTICK_LEFT   = ctru_sys::KEY_CSTICK_LEFT;
        const CSTICK_UP     = ctru_sys::KEY_CSTICK_UP;
        const CSTICK_DOWN   = ctru_sys::KEY_CSTICK_DOWN;
        const CPAD_RIGHT    = ctru_sys::KEY_CPAD_RIGHT;
        const CPAD_LEFT     = ctru_sys::KEY_CPAD_LEFT;
        const CPAD_UP       = ctru_sys::KEY_CPAD_UP;
        const CPAD_DOWN     = ctru_sys::KEY_CPAD_DOWN;
        // Convenience catch-all for the dpad and cpad
        const UP    = KeyPad::DPAD_UP.bits()    | KeyPad::CPAD_UP.bits();
        const DOWN  = KeyPad::DPAD_DOWN.bits()  | KeyPad::CPAD_DOWN.bits();
        const LEFT  = KeyPad::DPAD_LEFT.bits()  | KeyPad::CPAD_LEFT.bits();
        const RIGHT = KeyPad::DPAD_RIGHT.bits() | KeyPad::CPAD_RIGHT.bits();
    }
}

/// A reference-counted handle to the HID service. The service is closed
/// when all instances of this struct fall out of scope.
///
/// This service requires no special permissions to use.
pub struct Hid(());

/// Initializes the HID service.
///
/// # Errors
///
/// This function will return an error if the service was unable to be initialized.
/// Since this service requires no special or elevated permissions, errors are
/// rare in practice.
impl Hid {
    pub fn new() -> crate::Result<Hid> {
        unsafe {
            ResultCode(ctru_sys::hidInit())?;
            Ok(Hid(()))
        }
    }

    /// Scans the HID service for all user input occurring on the current
    /// frame. This function should be called on every frame when polling
    /// for user input.
    pub fn scan_input(&mut self) {
        unsafe { ctru_sys::hidScanInput() };
    }

    /// Returns a bitflag struct representing which buttons have just been pressed
    /// on the current frame (and were not pressed on the previous frame).
    pub fn keys_down(&self) -> KeyPad {
        unsafe {
            let keys = ctru_sys::hidKeysDown();
            KeyPad::from_bits_truncate(keys)
        }
    }

    /// Returns a bitflag struct representing which buttons have been held down
    /// during the current frame.
    pub fn keys_held(&self) -> KeyPad {
        unsafe {
            let keys = ctru_sys::hidKeysHeld();
            KeyPad::from_bits_truncate(keys)
        }
    }

    /// Returns a bitflag struct representing which buttons have just been released on
    /// the current frame.
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
    pub fn circlepad_position(&mut self) -> (i16, i16) {
        let mut res = ctru_sys::circlePosition { dx: 0, dy: 0 };

        unsafe {
            ctru_sys::hidCircleRead(&mut res);
        }
        (res.dx, res.dy)
    }
}

impl Drop for Hid {
    fn drop(&mut self) {
        unsafe { ctru_sys::hidExit() };
    }
}
