//! HID service
//!
//! The HID service provides access to user input such as button presses, touch screen presses,
//! and circle pad information. It also provides information from the sound volume slider,
//! the accelerometer, and the gyroscope.

bitflags! {
    /// A set of flags corresponding to the button and directional pad
    /// inputs on the 3DS
    #[derive(Default)]
    pub struct KeyPad: u32 {
        const KEY_A             = 1u32 << 0;
        const KEY_B             = 1u32 << 1;
        const KEY_SELECT        = 1u32 << 2;
        const KEY_START         = 1u32 << 3;
        const KEY_DRIGHT        = 1u32 << 4;
        const KEY_DLEFT         = 1u32 << 5;
        const KEY_DUP           = 1u32 << 6;
        const KEY_DDOWN         = 1u32 << 7;
        const KEY_R             = 1u32 << 8;
        const KEY_L             = 1u32 << 9;
        const KEY_X             = 1u32 << 10;
        const KEY_Y             = 1u32 << 11;
        const KEY_ZL            = 1u32 << 14;
        const KEY_ZR            = 1u32 << 15;
        const KEY_TOUCH         = 1u32 << 20;
        const KEY_CSTICK_RIGHT  = 1u32 << 24;
        const KEY_CSTICK_LEFT   = 1u32 << 25;
        const KEY_CSTICK_UP     = 1u32 << 26;
        const KEY_CSTICK_DOWN   = 1u32 << 27;
        const KEY_CPAD_RIGHT    = 1u32 << 28;
        const KEY_CPAD_LEFT     = 1u32 << 29;
        const KEY_CPAD_UP       = 1u32 << 30;
        const KEY_CPAD_DOWN     = 1u32 << 31;
        // convenience catch-all for the dpad and cpad
        const KEY_UP    = KeyPad::KEY_DUP.bits    | KeyPad::KEY_CPAD_UP.bits;
        const KEY_DOWN  = KeyPad::KEY_DDOWN.bits  | KeyPad::KEY_CPAD_DOWN.bits;
        const KEY_LEFT  = KeyPad::KEY_DLEFT.bits  | KeyPad::KEY_CPAD_LEFT.bits;
        const KEY_RIGHT = KeyPad::KEY_DRIGHT.bits | KeyPad::KEY_CPAD_RIGHT.bits;
    }
}

/// A reference-counted handle to the HID service. The service is closed
/// when all instances of this struct fall out of scope.
///
/// This service requires no special permissions to use.
pub struct Hid(());

/// Represents user input to the touchscreen.
pub struct TouchPosition(::libctru::touchPosition);

/// Represents the current position of the 3DS circle pad.
pub struct CirclePosition(::libctru::circlePosition);

/// Initializes the HID service.
///
/// # Errors
///
/// This function will return an error if the service was unable to be initialized.
/// Since this service requires no special or elevated permissions, errors are
/// rare in practice.
impl Hid {
    pub fn init() -> ::Result<Hid> {
        unsafe {
            let r = ::libctru::hidInit();
            if r < 0 {
                Err(r.into())
            } else {
                Ok(Hid(()))
            }
        }
    }

    /// Scans the HID service for all user input occurring on the current
    /// frame. This function should be called on every frame when polling
    /// for user input.
    pub fn scan_input(&self) {
        unsafe { ::libctru::hidScanInput() };
    }

    /// Returns a bitflag struct representing which buttons have just been pressed
    /// on the current frame (and were not pressed on the previous frame).
    pub fn keys_down(&self) -> KeyPad {
        unsafe {
            let keys = ::libctru::hidKeysDown();
            KeyPad::from_bits_truncate(keys)
        }
    }

    /// Returns a bitflag struct representing which buttons have been held down
    /// during the current frame.
    pub fn keys_held(&self) -> KeyPad {
        unsafe {
            let keys = ::libctru::hidKeysHeld();
            KeyPad::from_bits_truncate(keys)
        }
    }

    /// Returns a bitflag struct representing which buttons have just been released on
    /// the current frame.
    pub fn keys_up(&self) -> KeyPad {
        unsafe {
            let keys = ::libctru::hidKeysUp();
            KeyPad::from_bits_truncate(keys)
        }
    }
}

impl TouchPosition {
    /// Create a new TouchPosition instance.
    pub fn new() -> Self {
        TouchPosition(::libctru::touchPosition { px: 0, py: 0 })
    }

    /// Returns the current touch position in pixels.
    pub fn get(&mut self) -> (u16, u16) {
        unsafe {
            ::libctru::hidTouchRead(&mut self.0);
        }
        (self.0.px, self.0.py)
    }
}

impl CirclePosition {
    /// Create a new CirclePosition instance.
    pub fn new() -> Self {
        CirclePosition(::libctru::circlePosition { dx: 0, dy: 0 })
    }

    /// Returns the current circle pad position in (x, y) form.
    pub fn get(&mut self) -> (i16, i16) {
        unsafe {
            ::libctru::hidCircleRead(&mut self.0);
        }
        (self.0.dx, self.0.dy)
    }
}

impl Drop for Hid {
    fn drop(&mut self) {
        unsafe { ::libctru::hidExit() };
    }
}
