//! Camera service.
//!
//! The CAM service provides access to the built-in cameras. [`Camera`]s can return images
//! in the form of byte vectors which can be displayed to the screen or used in other ways.
#![doc(alias = "camera")]

use crate::error::{Error, ResultCode};
use crate::services::ServiceReference;
use crate::services::gspgpu::FramebufferFormat;
use ctru_sys::Handle;
use private::Configuration;

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
#[repr(u8)]
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
#[repr(u8)]
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
#[repr(u8)]
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
/// See [`Camera::set_white_balance()`] to learn how to use this.
#[doc(alias = "CAMU_WhiteBalance")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
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
#[repr(u8)]
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
#[repr(u8)]
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
#[repr(u8)]
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
#[repr(u8)]
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
#[repr(u8)]
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
#[repr(u8)]
pub enum ShutterSound {
    /// Photo shutter sound.
    Normal = ctru_sys::SHUTTER_SOUND_TYPE_NORMAL,
    /// Shutter sound to begin a movie recording.
    Movie = ctru_sys::SHUTTER_SOUND_TYPE_MOVIE,
    /// Shutter sound to finish a movie recording.
    MovieEnd = ctru_sys::SHUTTER_SOUND_TYPE_MOVIE_END,
}

/// Configuration to handle image trimming.
///
/// See [`Trimming::new_centered()`] and the other associated methods for controlled
/// ways of configuring trimming.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trimming {
    /// Trimming configuration relatively to the center of the image.
    #[allow(missing_docs)]
    Centered { width: i16, height: i16 },
    /// Trimming disabled.
    Off,
}

/// Data used by the camera to calibrate image quality for a single camera.
// TODO: Implement Image quality calibration.
#[doc(alias = "CAMU_ImageQualityCalibrationData")]
#[derive(Default, Clone, Copy, Debug)]
pub struct ImageQualityCalibration(pub ctru_sys::CAMU_ImageQualityCalibrationData);

/// Data used by the camera to calibrate image quality when using both outward cameras.
// TODO: Implement Stereo camera calibration.
#[doc(alias = "CAMU_StereoCameraCalibrationData")]
#[derive(Default, Clone, Copy, Debug)]
pub struct StereoCameraCalibration(ctru_sys::CAMU_StereoCameraCalibrationData);

/// Inward camera representation (facing the user of the 3DS).
///
/// Usually used for selfies.
pub struct InwardCam {
    configuration: Configuration,
}

/// Right-side outward camera representation.
pub struct OutwardRightCam {
    configuration: Configuration,
}

/// Left-side outward camera representation.
pub struct OutwardLeftCam {
    configuration: Configuration,
}

/// Both outer cameras combined.
///
/// Usually used for 3D photos.
pub struct BothOutwardCam {
    configuration: Configuration,
}

mod private {
    use super::{BothOutwardCam, InwardCam, OutwardLeftCam, OutwardRightCam, Trimming, ViewSize};

    /// Basic configuration needed to properly use the built-in cameras.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Configuration {
        pub view_size: ViewSize,
        pub trimming: Trimming,
    }

    impl Configuration {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl Default for Configuration {
        fn default() -> Self {
            Self {
                view_size: ViewSize::TopLCD,
                trimming: Trimming::Off,
            }
        }
    }

    pub trait ConfigurableCamera {
        fn configuration(&self) -> &Configuration;

        fn configuration_mut(&mut self) -> &mut Configuration;
    }

    impl ConfigurableCamera for InwardCam {
        fn configuration(&self) -> &Configuration {
            &self.configuration
        }

        fn configuration_mut(&mut self) -> &mut Configuration {
            &mut self.configuration
        }
    }

    impl ConfigurableCamera for OutwardRightCam {
        fn configuration(&self) -> &Configuration {
            &self.configuration
        }

        fn configuration_mut(&mut self) -> &mut Configuration {
            &mut self.configuration
        }
    }

    impl ConfigurableCamera for OutwardLeftCam {
        fn configuration(&self) -> &Configuration {
            &self.configuration
        }

        fn configuration_mut(&mut self) -> &mut Configuration {
            &mut self.configuration
        }
    }

    impl ConfigurableCamera for BothOutwardCam {
        fn configuration(&self) -> &Configuration {
            &self.configuration
        }

        fn configuration_mut(&mut self) -> &mut Configuration {
            &mut self.configuration
        }
    }
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
        }

        Ok(())
    }

    #[doc(alias = "CAMU_GetStereoCameraCalibrationData")]
    /// Returns the currently set [`StereoCameraCalibration`].
    pub fn stereo_calibration(&self) -> crate::Result<StereoCameraCalibration> {
        let mut calibration = StereoCameraCalibration::default();

        unsafe {
            ResultCode(ctru_sys::CAMU_GetStereoCameraCalibrationData(
                &mut calibration.0,
            ))?;
        }

        Ok(calibration)
    }

    #[doc(alias = "CAMU_SetStereoCameraCalibrationData")]
    /// Set the [`StereoCameraCalibration`].
    // TODO: This seems to have no effect.
    pub fn set_stereo_calibration(
        &mut self,
        mut stereo_calibration: StereoCameraCalibration,
    ) -> crate::Result<()> {
        let view_size = self.final_view_size();

        stereo_calibration.0.imageWidth = view_size.0;
        stereo_calibration.0.imageHeight = view_size.1;

        unsafe {
            ResultCode(ctru_sys::CAMU_SetStereoCameraCalibrationData(
                stereo_calibration.0,
            ))?;
        }

        Ok(())
    }
}

impl Camera for InwardCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_IN1.into()
    }
}

impl Camera for OutwardRightCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_OUT1.into()
    }
}

impl Camera for OutwardLeftCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_OUT2.into()
    }
}

impl Camera for BothOutwardCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_OUT1_OUT2.into()
    }

    fn port_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::PORT_BOTH.into()
    }

    fn take_picture(&mut self, buffer: &mut [u8], timeout: Duration) -> crate::Result<()> {
        // Check whether the provided buffer is big enough to store the image.
        let max_size = self.final_byte_length();
        if buffer.len() < max_size {
            return Err(Error::BufferTooShort {
                provided: buffer.len(),
                wanted: max_size,
            });
        }

        let final_view = self.final_view_size();

        // The transfer unit is NOT the "max number of bytes" or whatever the docs make you think it is...
        let transfer_unit = unsafe {
            let mut transfer_unit = 0;

            ResultCode(ctru_sys::CAMU_GetMaxBytes(
                &mut transfer_unit,
                final_view.0,
                final_view.1,
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

        unsafe {
            ResultCode(ctru_sys::CAMU_Activate(self.camera_as_raw()))?;
            ResultCode(ctru_sys::CAMU_ClearBuffer(self.port_as_raw()))?;
        };

        // Synchronize the two cameras.
        unsafe {
            ResultCode(ctru_sys::CAMU_SynchronizeVsyncTiming(
                ctru_sys::SELECT_OUT1.into(),
                ctru_sys::SELECT_OUT2.into(),
            ))?;
        }

        // Start capturing with the camera.
        unsafe {
            ResultCode(ctru_sys::CAMU_StartCapture(self.port_as_raw()))?;
        };

        let receive_event_1 = unsafe {
            let mut completion_handle: Handle = 0;

            ResultCode(ctru_sys::CAMU_SetReceiving(
                &mut completion_handle,
                buffer.as_mut_ptr().cast(),
                ctru_sys::PORT_CAM1.into(),
                (max_size / 2) as u32,
                transfer_unit.try_into().unwrap(),
            ))?;

            completion_handle
        };

        let receive_event_2 = unsafe {
            let mut completion_handle: Handle = 0;

            ResultCode(ctru_sys::CAMU_SetReceiving(
                &mut completion_handle,
                buffer[max_size / 2..].as_mut_ptr().cast(),
                ctru_sys::PORT_CAM2.into(),
                (max_size / 2) as u32,
                transfer_unit.try_into().unwrap(),
            ))?;

            completion_handle
        };

        unsafe {
            // Panicking without closing an SVC handle causes an ARM exception, we have to handle it carefully.
            let wait_result_1 = ResultCode(ctru_sys::svcWaitSynchronization(
                receive_event_1,
                timeout.as_nanos().try_into().unwrap(),
            ));

            let wait_result_2 = ResultCode(ctru_sys::svcWaitSynchronization(
                receive_event_2,
                timeout.as_nanos().try_into().unwrap(),
            ));

            // We close everything first, then we check for possible errors
            let _ = ctru_sys::svcCloseHandle(receive_event_1); // We wouldn't return the error even if there was one, so no use of ResultCode is needed.
            let _ = ctru_sys::svcCloseHandle(receive_event_2);

            // Camera state cleanup
            ResultCode(ctru_sys::CAMU_StopCapture(self.port_as_raw()))?;
            ResultCode(ctru_sys::CAMU_ClearBuffer(self.port_as_raw()))?;
            ResultCode(ctru_sys::CAMU_Activate(ctru_sys::SELECT_NONE.into()))?;

            wait_result_1?;
            wait_result_2?;
        };

        Ok(())
    }
}

/// Generic functionality common to all cameras.
pub trait Camera: private::ConfigurableCamera {
    /// Returns the raw value of the selected camera.
    fn camera_as_raw(&self) -> ctru_sys::u32_;

    /// Returns view size of the selected camera.
    ///
    /// # Notes
    ///
    /// This view is the full resolution at which the camera will take the photo.
    /// If you are interested in the final image's size, calculated while taking into account all processing and modifications,
    /// have a look at [`Camera::final_view_size()`].
    fn view_size(&self) -> ViewSize {
        self.configuration().view_size
    }

    /// Returns the raw port of the selected camera.
    fn port_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::PORT_CAM1.into()
    }

    /// Returns `true` if the camera is busy (receiving data).
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
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

    /// Returns the maximum amount of bytes the final image will occupy in memory based on the view size, trimming, pixel depth and other
    /// modifications set to the camera.
    ///
    /// # Notes
    ///
    /// The value returned will be double the image size if requested by [`BothOutwardCam`].
    /// Remember to query this information again if *any* changes are applied to the [`Camera`] configuration!
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cam::{Cam, Camera};
    /// let cam = Cam::new()?;
    ///
    /// let inward = &cam.inner_cam;
    ///
    /// let transfer_count = inward.final_byte_length();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn final_byte_length(&self) -> usize {
        let size = self.final_view_size();

        let mut res: usize = (size.0 as usize * size.1 as usize) * std::mem::size_of::<i16>();

        // If we are taking a picture using both outwards cameras, we need to expect 2 images, rather than just 1
        if self.port_as_raw() == ctru_sys::PORT_BOTH.into() {
            res *= 2;
        }

        res
    }

    /// Returns the dimensions of the final image based on the view size, trimming and other
    /// modifications set to the camera.
    ///
    /// # Notes
    ///
    /// Remember to query this information again if *any* changes are applied to the [`Camera`] configuration!
    ///
    /// # Example
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cam::{Cam, Camera, Trimming, ViewSize};
    /// let mut cam = Cam::new()?;
    ///
    /// let mut inward = &mut cam.inner_cam;
    ///
    /// // We trim the image down so that it fits on a DS screen!
    /// inward.set_trimming(Trimming::new_centered_with_view(ViewSize::DS));
    ///
    /// // This result will take into account the trimming.
    /// let final_resolution = inward.final_view_size();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn final_view_size(&self) -> (i16, i16) {
        match self.trimming() {
            Trimming::Centered { width, height } => (width, height),
            Trimming::Off => self.view_size().into(),
        }
    }

    /// Returns the [`Trimming`] configuration currently set.
    fn trimming(&self) -> Trimming {
        self.configuration().trimming
    }

    /// Set trimming bounds to trim the camera photo.
    ///
    /// # Notes
    ///
    /// The trimmed image must have a pixel area of (`width * height`) multiple of 128.
    /// If not, a raw `libctru` error may be returned.
    ///
    /// # Panics
    ///
    /// Setting up a [`Trimming`] configurations that exceeds the bounds of the original
    /// image's size will result in a panic.
    #[doc(alias = "CAMU_SetTrimming")]
    fn set_trimming(&mut self, trimming: Trimming) -> crate::Result<()> {
        match trimming {
            Trimming::Centered { width, height } => unsafe {
                let view_size: (i16, i16) = self.view_size().into();
                let trim_size: (i16, i16) = (width, height);

                // Check whether the trim size is within the view.
                assert!(
                    trim_size.0 <= view_size.0 && trim_size.1 <= view_size.1,
                    "trimmed view is bigger than the camera view",
                );

                ResultCode(ctru_sys::CAMU_SetTrimming(self.port_as_raw(), true))?;

                ResultCode(ctru_sys::CAMU_SetTrimmingParamsCenter(
                    self.port_as_raw(),
                    trim_size.0,
                    trim_size.1,
                    view_size.0,
                    view_size.1,
                ))?;
            },
            Trimming::Off => unsafe {
                ResultCode(ctru_sys::CAMU_SetTrimming(self.port_as_raw(), false))?;
            },
        }

        self.configuration_mut().trimming = trimming;

        Ok(())
    }

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
    fn set_view_size(&mut self, size: ViewSize) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetSize(
                self.camera_as_raw(),
                size.into(),
                ctru_sys::CONTEXT_A,
            ))?;
        }

        self.configuration_mut().view_size = size;

        self.set_trimming(Trimming::Off)?;

        Ok(())
    }

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
    /// This operation will override any previously set [`Effect`].
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

    /// Set the [`ImageQualityCalibration`] for the camera.
    #[doc(alias = "CAMU_SetImageQualityCalibrationData")]
    fn set_image_quality_calibration(
        &mut self,
        data: ImageQualityCalibration,
    ) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetImageQualityCalibrationData(data.0))?;
            Ok(())
        }
    }

    /// Returns the current [`ImageQualityCalibration`] for the camera.
    #[doc(alias = "CAMU_GetImageQualityCalibrationData")]
    fn image_quality_calibration(&self) -> crate::Result<ImageQualityCalibration> {
        unsafe {
            let mut data = ImageQualityCalibration::default();
            ResultCode(ctru_sys::CAMU_GetImageQualityCalibrationData(&mut data.0))?;
            Ok(data)
        }
    }

    /// Request the camera to take a picture and write it in a buffer.
    ///
    /// # Errors
    ///
    /// This function will return an error if the camera is already busy or if the timeout duration is reached.
    ///
    /// # Notes
    ///
    /// If the picture is taken using [`BothOutwardCam`], the buffer will have to be able to hold both images
    /// (from each camera), which will be written into it sequentially.
    /// Use [`Camera::final_byte_length()`] to know how big the buffer needs to be to hold your next image.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cam::{Cam, Camera, ViewSize, OutputFormat, WhiteBalance};
    /// let mut cam = Cam::new()?;
    ///
    /// // We borrow the inward facing `Camera`.
    /// let camera = &mut cam.inner_cam;
    ///
    /// camera.set_view_size(ViewSize::TopLCD)?;
    /// camera.set_output_format(OutputFormat::Rgb565)?;
    /// camera.set_noise_filter(true)?;
    /// camera.set_auto_exposure(true)?;
    /// camera.set_white_balance(WhiteBalance::Auto)?;
    ///
    /// // Size of the top screen buffer at 2 bytes per pixel (RGB565).
    /// let mut buffer = vec![0; camera.final_byte_length()];
    ///
    /// // Take picture with 3 seconds of timeout.
    /// camera.take_picture(&mut buffer, Duration::from_secs(3));
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn take_picture(&mut self, buffer: &mut [u8], timeout: Duration) -> crate::Result<()> {
        // Check whether the provided buffer is big enough to store the image.
        let max_size = self.final_byte_length();
        if buffer.len() < max_size {
            return Err(Error::BufferTooShort {
                provided: buffer.len(),
                wanted: max_size,
            });
        }

        let final_view = self.final_view_size();

        // The transfer unit is NOT the "max number of bytes" or whatever the docs make you think it is...
        let transfer_unit = unsafe {
            let mut transfer_unit = 0;

            ResultCode(ctru_sys::CAMU_GetMaxBytes(
                &mut transfer_unit,
                final_view.0,
                final_view.1,
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

        unsafe {
            ResultCode(ctru_sys::CAMU_Activate(self.camera_as_raw()))?;
            ResultCode(ctru_sys::CAMU_ClearBuffer(self.port_as_raw()))?;
        };

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
            // Panicking without closing an SVC handle causes an ARM exception, we have to handle it carefully.
            let wait_result = ResultCode(ctru_sys::svcWaitSynchronization(
                receive_event,
                timeout.as_nanos().try_into().unwrap(),
            ));

            // We close everything first, then we check for possible errors
            let _ = ctru_sys::svcCloseHandle(receive_event); // We wouldn't return the error even if there was one, so no use of ResultCode is needed.

            // Camera state cleanup
            ResultCode(ctru_sys::CAMU_StopCapture(self.port_as_raw()))?;
            ResultCode(ctru_sys::CAMU_ClearBuffer(self.port_as_raw()))?;
            ResultCode(ctru_sys::CAMU_Activate(ctru_sys::SELECT_NONE.into()))?;

            wait_result?;
        };

        Ok(())
    }
}

impl Trimming {
    /// Create a new [`Trimming`] configuration using width and height centered to the original image.
    ///
    /// # Panics
    ///
    /// This function will panic if the pixel area of the new configuration (`width * height`)
    /// is not a multiple of 128.
    pub fn new_centered(width: i16, height: i16) -> Self {
        // Pixel area must be a multiple of 128.
        assert!((width * height) % 128 == 0);

        Self::Centered { width, height }
    }

    /// Create a new [`Trimming`] configuration using a standard view size centered to the original image.
    pub fn new_centered_with_view(size: ViewSize) -> Self {
        let size: (i16, i16) = size.into();

        Self::Centered {
            width: size.0,
            height: size.1,
        }
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
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
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
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
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
