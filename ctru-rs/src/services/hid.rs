//! Human Interface Device service.
//!
//! The HID service provides read access to user input such as [button presses](Hid::keys_down), [touch screen presses](Hid::touch_position),
//! and [circle pad information](Hid::circlepad_position). It also provides information from the [volume slider](Hid::volume_slider()),
//! the [accelerometer](Hid::accelerometer_vector()), and the [gyroscope](Hid::gyroscope_rate()).
#![doc(alias = "input")]
#![doc(alias = "controller")]
#![doc(alias = "gamepad")]

use std::sync::Mutex;

use crate::error::ResultCode;
use crate::services::ServiceReference;

use bitflags::bitflags;

static HID_ACTIVE: Mutex<()> = Mutex::new(());

bitflags! {
    /// A set of flags corresponding to the button and directional pad inputs present on the 3DS.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
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

        // Convenience catch-all for the D-Pad and the CirclePad

        /// Direction Up (either D-Pad or CirclePad).
        const UP    = KeyPad::DPAD_UP.bits()    | KeyPad::CPAD_UP.bits();
        /// Direction Down (either D-Pad or CirclePad).
        const DOWN  = KeyPad::DPAD_DOWN.bits()  | KeyPad::CPAD_DOWN.bits();
        /// Direction Left (either D-Pad or CirclePad).
        const LEFT  = KeyPad::DPAD_LEFT.bits()  | KeyPad::CPAD_LEFT.bits();
        /// Direction Right (either D-Pad or CirclePad).
        const RIGHT = KeyPad::DPAD_RIGHT.bits() | KeyPad::CPAD_RIGHT.bits();
    }
}

/// Error enum for generic errors within the [`Hid`] service.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// An attempt was made to access the accelerometer while disabled.
    UnavailableAccelerometer,
    /// An attempt was made to access the gyroscope while disabled.
    UnavailableGyroscope,
}

/// Handle to the HID service.
pub struct Hid {
    active_accelerometer: bool,
    active_gyroscope: bool,
    _service_handler: ServiceReference,
}

impl Hid {
    /// Initialize a new service handle.
    ///
    /// # Errors
    ///
    /// This function will return an error if the service was unable to be initialized.
    /// Since this service requires no special or elevated permissions, errors are rare in practice.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::hid::Hid;
    ///
    /// let hid = Hid::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "hidInit")]
    pub fn new() -> crate::Result<Hid> {
        let handler = ServiceReference::new(
            &HID_ACTIVE,
            || {
                ResultCode(unsafe { ctru_sys::hidInit() })?;

                Ok(())
            },
            || unsafe {
                let _ = ctru_sys::HIDUSER_DisableGyroscope();
                let _ = ctru_sys::HIDUSER_DisableAccelerometer();

                ctru_sys::hidExit();
            },
        )?;

        Ok(Self {
            active_accelerometer: false,
            active_gyroscope: false,
            _service_handler: handler,
        })
    }

    /// Scan the HID service for all user input occurring on the current frame.
    ///
    /// This function should be called on every frame when polling
    /// for user input.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::hid::Hid;
    /// let mut hid = Hid::new()?;
    ///
    /// hid.scan_input();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "hidScanInput")]
    pub fn scan_input(&mut self) {
        unsafe { ctru_sys::hidScanInput() };
    }

    /// Returns a bitflag struct representing which buttons have just been pressed
    /// on the current frame (and were not pressed on the previous frame).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::hid::{Hid, KeyPad};
    /// let mut hid = Hid::new()?;
    ///
    /// hid.scan_input();
    ///
    /// if hid.keys_down().contains(KeyPad::A) {
    ///     println!("You have pressed the A button!")
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "hidKeysDown")]
    pub fn keys_down(&self) -> KeyPad {
        unsafe {
            let keys = ctru_sys::hidKeysDown();
            KeyPad::from_bits_truncate(keys)
        }
    }

    /// Returns a bitflag struct representing which buttons have been held down
    /// during the current frame.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::hid::{Hid, KeyPad};
    /// let mut hid = Hid::new()?;
    ///
    /// hid.scan_input();
    ///
    /// if hid.keys_held().contains(KeyPad::START) {
    ///     println!("You are holding the START button!")
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "hidKeysHeld")]
    pub fn keys_held(&self) -> KeyPad {
        unsafe {
            let keys = ctru_sys::hidKeysHeld();
            KeyPad::from_bits_truncate(keys)
        }
    }

    /// Returns a bitflag struct representing which buttons have just been released on
    /// the current frame.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::hid::{Hid, KeyPad};
    /// let mut hid = Hid::new()?;
    ///
    /// hid.scan_input();
    ///
    /// if hid.keys_held().contains(KeyPad::B) {
    ///     println!("You have released the B button!")
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
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
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::hid::Hid;
    /// let mut hid = Hid::new()?;
    ///
    /// hid.scan_input();
    ///
    /// let (touch_x, touch_y) = hid.touch_position();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "hidTouchRead")]
    pub fn touch_position(&self) -> (u16, u16) {
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
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::hid::Hid;
    /// let mut hid = Hid::new()?;
    ///
    /// hid.scan_input();
    ///
    /// let (pad_x, pad_y) = hid.circlepad_position();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "hidCircleRead")]
    pub fn circlepad_position(&self) -> (i16, i16) {
        let mut res = ctru_sys::circlePosition { dx: 0, dy: 0 };

        unsafe {
            ctru_sys::hidCircleRead(&mut res);
        }

        (res.dx, res.dy)
    }

    /// Returns the current volume slider position (between 0 and 1).
    ///
    /// # Notes
    ///
    /// The [`ndsp`](crate::services::ndsp) service automatically uses the volume slider's position to handle audio mixing.
    /// As such this method should not be used to programmatically change the volume.
    ///
    /// Its purpose is only to inform the program of the volume slider's position (e.g. checking if the user has muted the audio).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::hid::Hid;
    /// let mut hid = Hid::new()?;
    ///
    /// hid.scan_input();
    ///
    /// let volume = hid.volume_slider();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "HIDUSER_GetSoundVolume")]
    pub fn volume_slider(&self) -> f32 {
        let mut slider = 0;

        unsafe {
            let _ = ctru_sys::HIDUSER_GetSoundVolume(&mut slider);
        }

        (slider as f32) / 63.
    }

    /// Activate/deactivate the console's acceleration sensor.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::hid::Hid;
    /// let mut hid = Hid::new()?;
    ///
    /// // The accelerometer will start to register movements.
    /// hid.set_accelerometer(true);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "HIDUSER_EnableAccelerometer")]
    #[doc(alias = "HIDUSER_DisableAccelerometer")]
    pub fn set_accelerometer(&mut self, enabled: bool) {
        if enabled {
            let _ = unsafe { ctru_sys::HIDUSER_EnableAccelerometer() };
        } else {
            let _ = unsafe { ctru_sys::HIDUSER_DisableAccelerometer() };
        }

        self.active_accelerometer = enabled;
    }

    /// Activate/deactivate the console's gyroscopic sensor.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::hid::Hid;
    /// let mut hid = Hid::new()?;
    ///
    /// // The gyroscope will start to register positions.
    /// hid.set_gyroscope(true);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "HIDUSER_EnableGyroscope")]
    #[doc(alias = "HIDUSER_DisableGyroscope")]
    pub fn set_gyroscope(&mut self, enabled: bool) {
        if enabled {
            let _ = unsafe { ctru_sys::HIDUSER_EnableGyroscope() };
        } else {
            let _ = unsafe { ctru_sys::HIDUSER_DisableGyroscope() };
        }

        self.active_gyroscope = enabled;
    }

    /// Returns the acceleration vector (x,y,z) registered by the accelerometer.
    ///
    /// # Errors
    ///
    /// This function will return an error if the accelerometer was not previously enabled.
    /// Have a look at [`Hid::set_accelerometer()`] to enable the accelerometer.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::hid::Hid;
    /// let mut hid = Hid::new()?;
    ///
    /// // The accelerometer will start to register movements.
    /// hid.set_accelerometer(true);
    ///
    /// // It's necessary to run `scan_input()` to update the accelerometer's readings.
    /// hid.scan_input();
    ///
    /// // This call fails if the accelerometer was not previously enabled.
    /// let acceleration = hid.accelerometer_vector()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "hidAccelRead")]
    pub fn accelerometer_vector(&self) -> Result<(i16, i16, i16), Error> {
        if !self.active_accelerometer {
            return Err(Error::UnavailableAccelerometer);
        }

        let mut res = ctru_sys::accelVector { x: 0, y: 0, z: 0 };

        unsafe {
            ctru_sys::hidAccelRead(&mut res);
        }

        Ok((res.x, res.y, res.z))
    }

    /// Returns the angular rate (roll,pitch,yaw) registered by the gyroscope.
    ///
    /// # Errors
    ///
    /// This function returns an error if the gyroscope was not previously enabled.
    /// Have a look at [`Hid::set_gyroscope()`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::hid::Hid;
    /// let mut hid = Hid::new()?;
    ///
    /// // The gyroscope will start to register positions.
    ///  hid.set_gyroscope(true);
    ///
    /// // It's necessary to run `scan_input()` to update the gyroscope's readings.
    /// hid.scan_input();
    ///
    /// // This call fails if the gyroscope was not previously enabled.
    /// let angular_rate = hid.gyroscope_rate()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "hidGyroRead")]
    pub fn gyroscope_rate(&self) -> Result<(i16, i16, i16), Error> {
        if !self.active_gyroscope {
            return Err(Error::UnavailableGyroscope);
        }

        let mut res = ctru_sys::angularRate { x: 0, y: 0, z: 0 };

        unsafe {
            ctru_sys::hidGyroRead(&mut res);
        }

        Ok((res.x, res.y, res.z))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnavailableAccelerometer => write!(f, "tried using accelerometer while disabled"),
            Self::UnavailableGyroscope => write!(f, "tried using gyroscope while disabled"),
        }
    }
}

impl std::error::Error for Error {}
