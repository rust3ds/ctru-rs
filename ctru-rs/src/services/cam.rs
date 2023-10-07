//! Camera service.
//!
//! The CAM service provides access to the built-in cameras. [`Camera`]s can return images
//! in the form of byte vectors which can be displayed to the screen or used in other ways.
#![doc(alias = "camera")]

use crate::error::{Error, ResultCode};
use crate::services::gspgpu::FramebufferFormat;
use crate::services::ServiceReference;
use ctru_sys::Handle;

use std::sync::Mutex;
use std::time::Duration;

static CAM_ACTIVE: Mutex<()> = Mutex::new(());

/// Handle to the Camera service.
pub struct Cam {
    _service_handler: ServiceReference,
    /// Inside-facing camera.
    pub inner_cam: InwardCam,
    /// Outside-facing right camera.
    pub outer_right_cam: OutwardRightCam,
    /// Outside-facing left camera.
    pub outer_left_cam: OutwardLeftCam,
    /// Both outside-facing cameras (mainly used for 3D photos).
    pub both_outer_cams: BothOutwardCam,
}

/// Different kinds of flip modes.
///
/// See [`Camera::flip_image()`] to learn how to use this.
#[doc(alias = "CAMU_Flip")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum FlipMode {
    /// No flip.
    None = ctru_sys::FLIP_NONE,
    /// Horizontal flip.
    Horizontal = ctru_sys::FLIP_HORIZONTAL,
    /// Vertical flip.
    Vertical = ctru_sys::FLIP_VERTICAL,
    /// Both vertical and horizontal flip.
    Reverse = ctru_sys::FLIP_REVERSE,
}

/// Size of the camera view.
///
/// See [`Camera::set_view_size()`] to learn how to use this.
#[doc(alias = "CAMU_Size")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ViewSize {
    /// Size of the 3DS' top screen. (400 × 240)
    ///
    /// Useful if the image is meant to be displayed immediately.
    TopLCD = ctru_sys::SIZE_CTR_TOP_LCD,
    /// Size of the 3DS' bottom screen. (320 × 240)
    ///
    /// Equivalent to QVga.
    BottomLCD = ctru_sys::SIZE_CTR_BOTTOM_LCD,
    /// VGA display size. (640 × 480)
    Vga = ctru_sys::SIZE_VGA,
    /// QQVGA display size. (160 × 120)
    QQVga = ctru_sys::SIZE_QQVGA,
    /// CIF display size. (352 × 288)
    Cif = ctru_sys::SIZE_CIF,
    /// QCIF display size. (176 × 144)
    QCif = ctru_sys::SIZE_QCIF,
    /// Nintendo DS Screen size. (256 × 192)
    DS = ctru_sys::SIZE_DS_LCD,
    /// Nintendo DS Screen size x4. (512 × 384)
    DSX4 = ctru_sys::SIZE_DS_LCDx4,
}

/// Framerate settings.
///
/// See [`Camera::set_frame_rate()`] to learn how to use this.
#[doc(alias = "CAMU_FramRate")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum FrameRate {
    /// 15 FPS.
    Fps15 = ctru_sys::FRAME_RATE_15,
    /// 15 to 5 FPS.
    Fps15To5 = ctru_sys::FRAME_RATE_15_TO_5,
    /// 15 to 2 FPS.
    Fps15To2 = ctru_sys::FRAME_RATE_15_TO_2,
    /// 10 FPS.
    Fps10 = ctru_sys::FRAME_RATE_10,
    /// 8.5 FPS.
    Fps8_5 = ctru_sys::FRAME_RATE_8_5,
    /// 5 FPS.
    Fps5 = ctru_sys::FRAME_RATE_5,
    /// 20 FPS.
    Fps20 = ctru_sys::FRAME_RATE_20,
    /// 20 to 5 FPS.
    Fps20To5 = ctru_sys::FRAME_RATE_20_TO_5,
    /// 30 FPS.
    Fps30 = ctru_sys::FRAME_RATE_30,
    /// 30 to 5 FPS.
    Fps30To5 = ctru_sys::FRAME_RATE_30_TO_5,
    /// 15 to 10 FPS.
    Fps15To10 = ctru_sys::FRAME_RATE_15_TO_10,
    /// 20 to 10 FPS.
    Fps20To10 = ctru_sys::FRAME_RATE_20_TO_10,
    /// 30 to 10 FPS.
    Fps30To10 = ctru_sys::FRAME_RATE_30_TO_10,
}

/// White balance settings.
///
/// See [`Camera::set_white_balance()`] and [`Camera::set_white_balance_without_base_up()`] to learn how to use this.
#[doc(alias = "CAMU_WhiteBalance")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WhiteBalance {
    /// Automatic white balance.
    Auto = ctru_sys::WHITE_BALANCE_AUTO,
    /// Tungsten.
    Temp3200K = ctru_sys::WHITE_BALANCE_3200K,
    /// Fluorescent Light.
    Temp4150K = ctru_sys::WHITE_BALANCE_4150K,
    /// Daylight.
    Temp5200K = ctru_sys::WHITE_BALANCE_5200K,
    /// Cloudy/Horizon.
    Temp6000K = ctru_sys::WHITE_BALANCE_6000K,
    /// Shade.
    Temp7000K = ctru_sys::WHITE_BALANCE_7000K,
}

/// Photo mode settings.
///
/// See [`Camera::set_photo_mode()`] to learn how to use this.
#[doc(alias = "CAMU_PhotoMode")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PhotoMode {
    /// Normal mode.
    Normal = ctru_sys::PHOTO_MODE_NORMAL,
    /// Portrait mode.
    Portrait = ctru_sys::PHOTO_MODE_PORTRAIT,
    /// Landscape mode.
    Landscape = ctru_sys::PHOTO_MODE_LANDSCAPE,
    /// NightView mode.
    NightView = ctru_sys::PHOTO_MODE_NIGHTVIEW,
    /// Letter mode.
    Letter = ctru_sys::PHOTO_MODE_LETTER,
}

/// Special camera effects.
///
/// See [`Camera::set_effect()`] to learn how to use this.
#[doc(alias = "CAMU_Effect")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Effect {
    /// No effects.
    None = ctru_sys::EFFECT_NONE,
    /// Mono effect.
    Mono = ctru_sys::EFFECT_MONO,
    /// Sepia effect.
    Sepia = ctru_sys::EFFECT_SEPIA,
    /// Negative effect.
    Negative = ctru_sys::EFFECT_NEGATIVE,
    /// Negative film effect.
    Negafilm = ctru_sys::EFFECT_NEGAFILM,
    /// Sepia effect.
    ///
    /// The difference between this and [`Sepia`](Effect::Sepia) is unknown.
    Sepia01 = ctru_sys::EFFECT_SEPIA01,
}

/// Contrast settings.
///
/// See [`Camera::set_contrast()`] to learn how to use this.
#[doc(alias = "CAMU_Contrast")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Contrast {
    /// Low contrast.
    Low = ctru_sys::CONTRAST_LOW,
    /// Brightness ratio: 70.
    Normal = ctru_sys::CONTRAST_NORMAL,
    /// Brightness ratio: 90.
    High = ctru_sys::CONTRAST_HIGH,
}

/// Lens correction settings.
///
/// See [`Camera::set_lens_correction()`] to learn how to use this.
#[doc(alias = "CAMU_LensCorrection")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum LensCorrection {
    /// No lens correction.
    Off = ctru_sys::LENS_CORRECTION_DARK,
    /// Normal lens correction.
    Normal = ctru_sys::LENS_CORRECTION_NORMAL,
    /// Bright lens correction.
    Bright = ctru_sys::LENS_CORRECTION_BRIGHT,
}

/// Image output format.
///
/// See [`Camera::set_output_format()`] to learn how to use this.
#[doc(alias = "CAMU_OutputFormat")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum OutputFormat {
    /// YUV422 output format. 16 bits per pixel.
    Yuv422 = ctru_sys::OUTPUT_YUV_422,
    /// RGB565 output format. 16 bits per pixel.
    Rgb565 = ctru_sys::OUTPUT_RGB_565,
}

/// Playable shutter sounds.
///
/// See [`Cam::play_shutter_sound()`] to learn how to use this.
#[doc(alias = "CAMU_ShutterSoundType")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ShutterSound {
    /// Photo shutter sound.
    Normal = ctru_sys::SHUTTER_SOUND_TYPE_NORMAL,
    /// Shutter sound to begin a movie recording.
    Movie = ctru_sys::SHUTTER_SOUND_TYPE_MOVIE,
    /// Shutter sound to finish a movie recording.
    MovieEnd = ctru_sys::SHUTTER_SOUND_TYPE_MOVIE_END,
}

/// Configuration to handle image trimming.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trimming {
    /// Trimming configuration based on absolute coordinates of the image.
    Absolute {
        /// Top-left corner coordinates (x,y) of the trimmed area.
        top_left: (i16, i16),
        /// Bottom-right corner coordinates (x,y) of the trimmed area.
        bottom_right: (i16, i16),
    },
    /// Trimming configuration relatively to the center of the image.
    Centered {
        /// Width of the trimmed area.
        width: i16,
        /// Height of the trimmed area.
        height: i16,
    },
    /// Trimming disabled.
    Off,
}

/// Data used by the camera to calibrate image quality for a single camera.
#[doc(alias = "CAMU_ImageQualityCalibrationData")]
#[derive(Default, Clone, Copy, Debug)]
pub struct ImageQualityCalibrationData(pub ctru_sys::CAMU_ImageQualityCalibrationData);

/// Data used by the camera to calibrate image quality when using both outward cameras.
// TODO: Implement Stereo camera calibration.
#[doc(alias = "CAMU_StereoCameraCalibrationData")]
#[derive(Default, Clone, Copy, Debug)]
pub struct StereoCameraCalibrationData(pub ctru_sys::CAMU_StereoCameraCalibrationData);

/// Basic configuration needed to properly use the built-in cameras.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Configuration {
    view_size: ViewSize,
    trimming: Trimming,
}

impl Configuration {
    fn new() -> Self {
        Self {
            view_size: ViewSize::TopLCD,
            trimming: Trimming::Off,
        }
    }
}

/// Inward camera representation (facing the user of the 3DS).
///
/// Usually used for selfies.
#[non_exhaustive]
pub struct InwardCam {
    configuration: Configuration,
}

/// Right-side outward camera representation.
#[non_exhaustive]
pub struct OutwardRightCam {
    configuration: Configuration,
}

/// Left-side outward camera representation.
#[non_exhaustive]
pub struct OutwardLeftCam {
    configuration: Configuration,
}

/// Both outer cameras combined.
///
/// Usually used for 3D photos.
#[non_exhaustive]
pub struct BothOutwardCam {
    configuration: Configuration,
}

impl BothOutwardCam {
    /// Set whether to enable or disable brightness synchronization between the two cameras.
    #[doc(alias = "CAMU_SetBrightnessSynchronization")]
    pub fn set_brightness_synchronization(
        &mut self,
        brightness_synchronization: bool,
    ) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetBrightnessSynchronization(
                brightness_synchronization,
            ))?;
            Ok(())
        }
    }
}

macro_rules! trimming_checks {
    ($trimming:ident, $view_size:expr) => {
        match $trimming {
            Trimming::Absolute {
                top_left,
                bottom_right,
            } => {
                let view_size: (i16, i16) = $view_size;

                // Top left corner is "before" bottom right corner.
                assert!(top_left.0 <= bottom_right.0 && top_left.1 <= bottom_right.1);
                // All coordinates are positive.
                assert!(
                    top_left.0 >= 0
                        && top_left.1 >= 0
                        && bottom_right.0 >= 0
                        && bottom_right.1 >= 0
                );
                // All coordinates are within the view.
                assert!(
                    top_left.0 < view_size.0
                        && bottom_right.0 < view_size.0
                        && top_left.1 < view_size.1
                        && bottom_right.1 < view_size.1
                );
            }
            Trimming::Centered { width, height } => {
                let view_size: (i16, i16) = $view_size;

                // Trim sizes are positive.
                assert!(width >= 0 && height >= 0);
                // Trim sizes are within the view.
                assert!(width <= view_size.0 && height <= view_size.1);
            }
            Trimming::Off => (),
        }
    };
}

impl Camera for InwardCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_IN1
    }

    fn view_size(&self) -> ViewSize {
        self.configuration.view_size
    }

    fn set_view_size(&mut self, size: ViewSize) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetSize(
                self.camera_as_raw(),
                size.into(),
                ctru_sys::CONTEXT_A,
            ))?;
        }

        self.configuration.view_size = size;

        self.set_trimming(Trimming::Off);

        Ok(())
    }

    fn trimming(&self) -> Trimming {
        self.configuration.trimming
    }

    fn set_trimming(&mut self, trimming: Trimming) {
        // Run checks for all trimming possibilities.
        trimming_checks!(trimming, self.view_size().into());

        self.configuration.trimming = trimming;
    }
}

impl Camera for OutwardRightCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_OUT1
    }

    fn view_size(&self) -> ViewSize {
        self.configuration.view_size
    }

    fn set_view_size(&mut self, size: ViewSize) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetSize(
                self.camera_as_raw(),
                size.into(),
                ctru_sys::CONTEXT_A,
            ))?;
        }

        self.configuration.view_size = size;

        self.set_trimming(Trimming::Off);

        Ok(())
    }

    fn trimming(&self) -> Trimming {
        self.configuration.trimming
    }

    fn set_trimming(&mut self, trimming: Trimming) {
        // Run checks for all trimming possibilities.
        trimming_checks!(trimming, self.view_size().into());

        self.configuration.trimming = trimming;
    }
}

impl Camera for OutwardLeftCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_OUT2
    }

    fn view_size(&self) -> ViewSize {
        self.configuration.view_size
    }

    fn set_view_size(&mut self, size: ViewSize) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetSize(
                self.camera_as_raw(),
                size.into(),
                ctru_sys::CONTEXT_A,
            ))?;
        }

        self.configuration.view_size = size;

        self.set_trimming(Trimming::Off);

        Ok(())
    }

    fn trimming(&self) -> Trimming {
        self.configuration.trimming
    }

    fn set_trimming(&mut self, trimming: Trimming) {
        // Run checks for all trimming possibilities.
        trimming_checks!(trimming, self.view_size().into());

        self.configuration.trimming = trimming;
    }
}

impl Camera for BothOutwardCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_OUT1_OUT2
    }

    fn port_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::PORT_BOTH
    }

    fn view_size(&self) -> ViewSize {
        self.configuration.view_size
    }

    fn set_view_size(&mut self, size: ViewSize) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetSize(
                self.camera_as_raw(),
                size.into(),
                ctru_sys::CONTEXT_A,
            ))?;
        }

        self.configuration.view_size = size;

        self.set_trimming(Trimming::Off);

        Ok(())
    }

    fn trimming(&self) -> Trimming {
        self.configuration.trimming
    }

    fn set_trimming(&mut self, trimming: Trimming) {
        // Run checks for all trimming possibilities.
        trimming_checks!(trimming, self.view_size().into());

        self.configuration.trimming = trimming;
    }
}

/// Generic functionality common to all cameras.
pub trait Camera {
    /// Returns the raw value of the selected camera.
    fn camera_as_raw(&self) -> ctru_sys::u32_;

    /// Returns view size of the selected camera.
    ///
    /// # Notes
    ///
    /// This view is the full resolution at which the camera will take the photo.
    /// If you are interested in the final image dimension, after all processing and modifications, have a look at [`Camera::final_image_size()`].
    fn view_size(&self) -> ViewSize;

    /// Returns the raw port of the selected camera.
    fn port_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::PORT_CAM1
    }

    /// Returns `true` if the camera is busy (receiving data).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cam::{Cam, Camera};
    /// let cam = Cam::new()?;
    ///
    /// let inward = &cam.inner_cam;
    ///
    /// // Inward cam is not busy since it is not being used.
    /// assert!(!inward.is_busy()?);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "CAMU_IsBusy")]
    fn is_busy(&self) -> crate::Result<bool> {
        unsafe {
            let mut is_busy = false;
            ResultCode(ctru_sys::CAMU_IsBusy(&mut is_busy, self.port_as_raw()))?;
            Ok(is_busy)
        }
    }

    /// Returns the maximum amount of bytes the final image will occupy based on the view size, trimming, pixel depth and other
    /// modifications set to the camera.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cam::{Cam, Camera};
    /// let cam = Cam::new()?;
    ///
    /// let inward = &cam.inner_cam;
    ///
    /// let transfer_count = inward.max_byte_count();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "CAMU_GetTransferBytes")]
    fn max_byte_count(&self) -> usize {
        let size = self.final_image_size();

        let mut res: usize = (size.0 as usize * size.1 as usize) * std::mem::size_of::<i16>();

        // If we are taking a picture using both outwards cameras, we need to expect 2 images, rather than just 1
        if self.port_as_raw() == ctru_sys::PORT_BOTH {
            res = res * 2;
        }

        res
    }

    /// Returns the dimensions of the final image based on the view size, trimming and other
    /// modifications set to the camera.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cam::{Cam, Camera};
    /// let cam = Cam::new()?;
    ///
    /// let mut inward = &cam.inner_cam;
    ///
    /// inward.set_trimming(Trimming::Centered {
    ///     width: 100,
    ///     height: 100,
    /// });
    ///
    /// // This result will take into account the trimming.
    /// let final_resolution = inward.final_image_size();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn final_image_size(&self) -> (i16, i16) {
        match self.trimming() {
            Trimming::Absolute {
                top_left,
                bottom_right,
            } => (bottom_right.0 - top_left.0, bottom_right.1 - top_left.1),
            Trimming::Centered { width, height } => (width, height),
            Trimming::Off => self.view_size().into(),
        }
    }

    /// Returns the [`Trimming`] configuration currently set.
    fn trimming(&self) -> Trimming;

    /// Set trimming bounds to trim the camera photo.
    #[doc(alias = "CAMU_SetTrimming")]
    fn set_trimming(&mut self, trimming: Trimming);

    /// Returns whether or not trimming is currently enabled for the camera.
    #[doc(alias = "CAMU_IsTrimming")]
    fn is_trimming(&self) -> bool {
        matches!(self.trimming(), Trimming::Off)
    }

    /// Set the exposure level of the camera.
    #[doc(alias = "CAMU_SetExposure")]
    fn set_exposure(&mut self, exposure: i8) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetExposure(self.camera_as_raw(), exposure))?;
            Ok(())
        }
    }

    /// Set the white balance of the camera.
    #[doc(alias = "CAMU_SetWhiteBalance")]
    fn set_white_balance(&mut self, white_balance: WhiteBalance) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetWhiteBalance(
                self.camera_as_raw(),
                white_balance.into(),
            ))?;
            Ok(())
        }
    }

    /// Set the sharpness of the camera.
    #[doc(alias = "CAMU_SetSharpness")]
    fn set_sharpness(&mut self, sharpness: i8) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetSharpness(self.camera_as_raw(), sharpness))?;
            Ok(())
        }
    }

    /// Set whether auto exposure is enabled or disabled for the camera.
    #[doc(alias = "CAMU_SetAutoExposure")]
    fn set_auto_exposure(&mut self, enabled: bool) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetAutoExposure(
                self.camera_as_raw(),
                enabled,
            ))?;
            Ok(())
        }
    }

    /// Returns `true` if auto exposure is enabled for the camera.
    #[doc(alias = "CAMU_IsAutoExposure")]
    fn is_auto_exposure_enabled(&self) -> crate::Result<bool> {
        unsafe {
            let mut enabled = false;
            ResultCode(ctru_sys::CAMU_IsAutoExposure(
                &mut enabled,
                self.camera_as_raw(),
            ))?;
            Ok(enabled)
        }
    }

    /// Set the flip mode of the camera's image.
    #[doc(alias = "CAMU_FlipImage")]
    fn flip_image(&mut self, flip: FlipMode) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_FlipImage(
                self.camera_as_raw(),
                flip.into(),
                ctru_sys::CONTEXT_A,
            ))?;
            Ok(())
        }
    }

    /// Set the view size of the camera.
    ///
    /// # Notes
    ///
    /// Calling this function will reset the trimming configuration.
    #[doc(alias = "CAMU_SetSize")]
    fn set_view_size(&mut self, size: ViewSize) -> crate::Result<()>;

    /// Set the frame rate of the camera.
    #[doc(alias = "CAMU_SetFrameRate")]
    fn set_frame_rate(&mut self, frame_rate: FrameRate) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetFrameRate(
                self.camera_as_raw(),
                frame_rate.into(),
            ))?;
            Ok(())
        }
    }

    /// Set the photo mode of the camera.
    #[doc(alias = "CAMU_SetPhotoMode")]
    fn set_photo_mode(&mut self, photo_mode: PhotoMode) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetPhotoMode(
                self.camera_as_raw(),
                photo_mode.into(),
            ))?;
            Ok(())
        }
    }

    /// Set the effect of the camera.
    ///
    /// # Notes
    ///
    /// This operation will override any previously set [`Effect`]s.
    /// Multiple effects can be set at once by combining the bitflags of [`Effect`].
    #[doc(alias = "CAMU_SetEffect")]
    fn set_effect(&mut self, effect: Effect) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetEffect(
                self.camera_as_raw(),
                effect.into(),
                ctru_sys::CONTEXT_A,
            ))?;
            Ok(())
        }
    }

    /// Set the contrast of the camera.
    #[doc(alias = "CAMU_SetContrast")]
    fn set_contrast(&mut self, contrast: Contrast) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetContrast(
                self.camera_as_raw(),
                contrast.into(),
            ))?;
            Ok(())
        }
    }

    /// Set the lens correction of the camera.
    #[doc(alias = "CAMU_SetLensCorrection")]
    fn set_lens_correction(&mut self, lens_correction: LensCorrection) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetLensCorrection(
                self.camera_as_raw(),
                lens_correction.into(),
            ))?;
            Ok(())
        }
    }

    /// Set the output format of the camera.
    #[doc(alias = "CAMU_SetOutputFormat")]
    fn set_output_format(&mut self, format: OutputFormat) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetOutputFormat(
                self.camera_as_raw(),
                format.into(),
                ctru_sys::CONTEXT_A,
            ))?;
            Ok(())
        }
    }

    /// Set the region in which auto exposure should be based on.
    ///
    /// # Arguments
    ///
    /// * `x` - Starting x coordinate of the window
    /// * `y` - Starting y coordinate of the window
    /// * `width` - Width of the window
    /// * `height` - Height of the window
    #[doc(alias = "CAMU_SetAutoExposureWindow")]
    fn set_auto_exposure_window(
        &mut self,
        x: i16,
        y: i16,
        width: i16,
        height: i16,
    ) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetAutoExposureWindow(
                self.camera_as_raw(),
                x,
                y,
                width,
                height,
            ))?;
            Ok(())
        }
    }

    /// Set the region in which auto white balance should be based on.
    ///
    /// # Arguments
    ///
    /// * `x` - Starting x coordinate of the window
    /// * `y` - Starting y coordinate of the window
    /// * `width` - Width of the window
    /// * `height` - Height of the window
    ///
    /// # Notes
    ///
    /// To activate automatic white balance, you must pass [`WhiteBalance::Auto`] into [`Camera::set_white_balance()`].
    #[doc(alias = "CAMU_SetAutoWhiteBalanceWindow")]
    fn set_auto_white_balance_window(
        &mut self,
        x: i16,
        y: i16,
        width: i16,
        height: i16,
    ) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetAutoWhiteBalanceWindow(
                self.camera_as_raw(),
                x,
                y,
                width,
                height,
            ))?;
            Ok(())
        }
    }

    /// Set whether the noise filter should be enabled or disabled for the camera.
    #[doc(alias = "CAMU_SetNoiseFilter")]
    fn set_noise_filter(&mut self, enabled: bool) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetNoiseFilter(self.camera_as_raw(), enabled))?;
            Ok(())
        }
    }

    /// Set the [`ImageQualityCalibrationData`] for the camera.
    #[doc(alias = "CAMU_SetImageQualityCalibrationData")]
    fn set_image_quality_calibration_data(
        &mut self,
        data: ImageQualityCalibrationData,
    ) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetImageQualityCalibrationData(data.0))?;
            Ok(())
        }
    }

    /// Returns the current [`ImageQualityCalibrationData`] for the camera.
    #[doc(alias = "CAMU_GetImageQualityCalibrationData")]
    fn image_quality_calibration_data(&self) -> crate::Result<ImageQualityCalibrationData> {
        unsafe {
            let mut data = ImageQualityCalibrationData::default();
            ResultCode(ctru_sys::CAMU_GetImageQualityCalibrationData(&mut data.0))?;
            Ok(data)
        }
    }

    /// Request the camera to take a picture and write it in a buffer.
    ///
    /// # Errors
    ///
    /// This function will return an error if the camera is busy or if the timeout duration gets reached.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the desired image
    /// * `height` - Height of the desired image
    /// * `timeout` - Duration to wait for the image
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cam::{Cam, Camera, ViewSize, OutputFormat};
    /// let mut cam = Cam::new()?;
    ///
    /// // We borrow the inward facing `Camera`.
    /// let inward = &mut cam.inner_cam;
    ///
    /// inward.set_view_size(ViewSize::TopLCD)?;
    /// inward.set_output_format(OutputFormat::Rgb565)?;
    /// inward.set_noise_filter(true)?;
    /// inward.set_auto_exposure(true)?;
    /// inward.set_auto_white_balance(true)?;
    ///
    /// // Size of the top screen buffer at 2 bytes per pixel (RGB565).
    /// let mut buffer = vec![0; 400*240*2];
    ///
    /// // Take picture with 3 seconds of timeout.
    /// inward.take_picture(&mut buffer, 400, 240, Duration::from_secs(3));
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn take_picture(&mut self, buffer: &mut [u8], timeout: Duration) -> crate::Result<()> {
        let full_view: (i16, i16) = self.view_size().into();

        // It seems like doing this as the first step gives the option to use trimming and get correct readings for the transfer bytes.
        unsafe {
            ResultCode(ctru_sys::CAMU_Activate(self.camera_as_raw()))?;
        };

        // Obtain the final view size and make the needed modifications to the camera.
        let final_view: (i16, i16) = match self.trimming() {
            Trimming::Absolute {
                top_left,
                bottom_right,
            } => unsafe {
                ResultCode(ctru_sys::CAMU_SetTrimming(self.port_as_raw(), true))?;

                ResultCode(ctru_sys::CAMU_SetTrimmingParams(
                    self.port_as_raw(),
                    top_left.0,
                    top_left.1,
                    bottom_right.0,
                    bottom_right.1,
                ))?;

                (bottom_right.0 - top_left.0, bottom_right.1 - top_left.1)
            },
            Trimming::Centered { width, height } => unsafe {
                ResultCode(ctru_sys::CAMU_SetTrimming(self.port_as_raw(), true))?;

                ResultCode(ctru_sys::CAMU_SetTrimmingParamsCenter(
                    self.port_as_raw(),
                    width,
                    height,
                    full_view.0,
                    full_view.1,
                ))?;

                (width, height)
            },
            Trimming::Off => unsafe {
                ResultCode(ctru_sys::CAMU_SetTrimming(self.port_as_raw(), false))?;

                full_view
            },
        };

        println!("CAMU_GetMaxBytes{:?}", final_view);

        // The transfer unit is NOT the "max number of bytes" or whatever the docs make you think it is...
        let transfer_unit = unsafe {
            let mut transfer_unit = 0;

            ResultCode(ctru_sys::CAMU_GetMaxBytes(
                &mut transfer_unit,
                full_view.0,
                full_view.1,
            ))?;

            transfer_unit
        };

        unsafe {
            ResultCode(ctru_sys::CAMU_SetTransferBytes(
                self.port_as_raw(),
                transfer_unit,
                final_view.0,
                final_view.1,
            ))?;
        };

        // Check whether the input buffer is big enough for the image.
        let max_size = (final_view.0 as usize * final_view.1 as usize) * std::mem::size_of::<i16>();
        if buffer.len() < max_size {
            // We deactivate the camera prematurely.
            //
            // Note that it shouldn't be too important whether the camera closes or not here,
            // since it only starts capturing later.
            unsafe { ResultCode(ctru_sys::CAMU_Activate(ctru_sys::SELECT_NONE))? };

            return Err(Error::BufferTooShort {
                provided: buffer.len(),
                wanted: max_size,
            });
        }

        let receive_event = unsafe {
            let mut completion_handle: Handle = 0;

            ResultCode(ctru_sys::CAMU_SetReceiving(
                &mut completion_handle,
                buffer.as_mut_ptr().cast(),
                self.port_as_raw(),
                max_size as u32,
                transfer_unit.try_into().unwrap(),
            ))?;

            completion_handle
        };

        // Start capturing with the camera.
        unsafe {
            ResultCode(ctru_sys::CAMU_StartCapture(self.port_as_raw()))?;
        };

        unsafe {
            // Panicking without closing an SVC handle causes an ARM exception, we have to handle it carefully (TODO: SVC module)
            let wait_result = ResultCode(ctru_sys::svcWaitSynchronization(
                receive_event,
                timeout.as_nanos().try_into().unwrap(),
            ));

            // We close everything first, then we check for possible errors
            let _ = ctru_sys::svcCloseHandle(receive_event); // We wouldn't return the error even if there was one, so no use of ResultCode is needed

            // Camera state cleanup
            ResultCode(ctru_sys::CAMU_StopCapture(self.port_as_raw()))?;
            ResultCode(ctru_sys::CAMU_ClearBuffer(self.port_as_raw()))?;
            ResultCode(ctru_sys::CAMU_Activate(ctru_sys::SELECT_NONE))?;

            wait_result?;
        };

        Ok(())
    }
}

impl Cam {
    /// Initialize a new service handle.
    ///
    /// # Notes
    ///
    /// All cameras default to taking photos with [`ViewSize::TopLCD`] and [`OutputFormat::Yuv422`].
    /// Have a look at [`Camera::set_view_size()`] and [`Camera::set_output_format()`] to change these settings.
    ///
    /// # Errors
    ///
    /// This function will return an error if the service was unable to be initialized.
    /// Since this service requires no special or elevated permissions, errors are
    /// rare in practice.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cam::Cam;
    ///
    /// let cam = Cam::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "camInit")]
    pub fn new() -> crate::Result<Cam> {
        let _service_handler = ServiceReference::new(
            &CAM_ACTIVE,
            || {
                ResultCode(unsafe { ctru_sys::camInit() })?;

                Ok(())
            },
            || unsafe {
                ctru_sys::camExit();
            },
        )?;

        let configuration = Configuration::new();

        let mut inner_cam = InwardCam { configuration };
        let mut outer_right_cam = OutwardRightCam { configuration };
        let mut outer_left_cam = OutwardLeftCam { configuration };
        let mut both_outer_cams = BothOutwardCam { configuration };

        inner_cam.set_view_size(ViewSize::TopLCD)?;
        outer_right_cam.set_view_size(ViewSize::TopLCD)?;
        outer_left_cam.set_view_size(ViewSize::TopLCD)?;
        both_outer_cams.set_view_size(ViewSize::TopLCD)?;

        Ok(Cam {
            _service_handler,
            inner_cam,
            outer_right_cam,
            outer_left_cam,
            both_outer_cams,
        })
    }

    /// Play the specified sound based on the [`ShutterSound`] argument
    ///
    /// # Notes
    ///
    /// Playing the shutter sound does not require a living handle to the [`Ndsp`](crate::services::ndsp::Ndsp) service.
    /// Volume will always be maxed out to ensure everyone within photo range can hear the picture being taken (as by Japanese law).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cam::{Cam, ShutterSound};
    /// let cam = Cam::new()?;
    ///
    /// // We play the shutter sound on the console's speakers!
    /// // (even though we aren't taking a photo :P)
    /// cam.play_shutter_sound(ShutterSound::Normal);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "CAMU_PlayShutterSound")]
    pub fn play_shutter_sound(&self, sound: ShutterSound) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_PlayShutterSound(sound.into()))?;
            Ok(())
        }
    }
}

impl TryFrom<FramebufferFormat> for OutputFormat {
    type Error = ();

    fn try_from(value: FramebufferFormat) -> Result<Self, Self::Error> {
        match value {
            FramebufferFormat::Rgb565 => Ok(OutputFormat::Rgb565),
            _ => Err(()),
        }
    }
}

impl TryFrom<OutputFormat> for FramebufferFormat {
    type Error = ();

    fn try_from(value: OutputFormat) -> Result<Self, Self::Error> {
        match value {
            OutputFormat::Rgb565 => Ok(FramebufferFormat::Rgb565),
            _ => Err(()),
        }
    }
}

impl From<ViewSize> for (i16, i16) {
    fn from(value: ViewSize) -> Self {
        match value {
            ViewSize::TopLCD => (400, 240),
            ViewSize::BottomLCD => (320, 240),
            ViewSize::Vga => (640, 480),
            ViewSize::QQVga => (160, 120),
            ViewSize::Cif => (352, 288),
            ViewSize::QCif => (176, 144),
            ViewSize::DS => (256, 192),
            ViewSize::DSX4 => (512, 384),
        }
    }
}

from_impl!(FlipMode, ctru_sys::CAMU_Flip);
from_impl!(ViewSize, ctru_sys::CAMU_Size);
from_impl!(FrameRate, ctru_sys::CAMU_FrameRate);
from_impl!(WhiteBalance, ctru_sys::CAMU_WhiteBalance);
from_impl!(PhotoMode, ctru_sys::CAMU_PhotoMode);
from_impl!(Effect, ctru_sys::CAMU_Effect);
from_impl!(Contrast, ctru_sys::CAMU_Contrast);
from_impl!(LensCorrection, ctru_sys::CAMU_LensCorrection);
from_impl!(OutputFormat, ctru_sys::CAMU_OutputFormat);
from_impl!(ShutterSound, ctru_sys::CAMU_ShutterSoundType);
