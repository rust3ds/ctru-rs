//! Camera service
//!
//! The CAM service provides access to the cameras. Cameras can return images
//! in the form of byte vectors which can be displayed or used in other ways.

use crate::error::{Error, ResultCode};
use crate::services::gspgpu::FramebufferFormat;
use ctru_sys::Handle;
use std::time::Duration;

/// A reference-counted handle to the CAM service and the usable cameras.
/// The service is closed when all instances of this struct fall out of scope.
///
/// This service requires no special permissions to use.
#[non_exhaustive]
pub struct Cam {
    /// Inside-facing camera.
    pub inner_cam: InwardCam,
    /// Outside-facing right camera.
    pub outer_right_cam: OutwardRightCam,
    /// Outside-facing left camera.
    pub outer_left_cam: OutwardLeftCam,
    /// Both outside-facing cameras (mainly used for 3D photos).
    pub both_outer_cams: BothOutwardCam,
}

/// Flag to pass to [`Camera::flip_image`]
#[doc(alias = "CAMU_Flip")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum FlipMode {
    /// No flip applied.
    None = ctru_sys::FLIP_NONE,
    /// Horizontal flip applied.
    Horizontal = ctru_sys::FLIP_HORIZONTAL,
    /// Vertical flip applied.
    Vertical = ctru_sys::FLIP_VERTICAL,
    /// Both vertical and horizontal flip applied.
    Reverse = ctru_sys::FLIP_REVERSE,
}

/// Flag to pass to [`Camera::set_view_size`]
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

/// Flag to pass to [`Camera::set_frame_rate`]
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

/// Flag to pass to [`Camera::set_white_balance`] or
/// [`Camera::set_white_balance_without_base_up`]
#[doc(alias = "CAMU_WhiteBalance")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WhiteBalance {
    /// Normal
    Auto = ctru_sys::WHITE_BALANCE_AUTO,
    /// Tungsten
    Temp3200K = ctru_sys::WHITE_BALANCE_3200K,
    /// Fluorescent Light
    Temp4150K = ctru_sys::WHITE_BALANCE_4150K,
    /// Daylight
    Temp5200K = ctru_sys::WHITE_BALANCE_5200K,
    /// Cloudy/Horizon
    Temp6000K = ctru_sys::WHITE_BALANCE_6000K,
    /// Shade
    Temp7000K = ctru_sys::WHITE_BALANCE_7000K,
}

/// Flag to pass to [`Camera::set_photo_mode`]
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

/// Flag to pass to [`Camera::set_effect`]
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
    /// Sepia effect. (unknown difference)
    Sepia01 = ctru_sys::EFFECT_SEPIA01,
}

/// Flag to pass to [`Camera::set_contrast`]
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

/// Flag to pass to [`Camera::set_lens_correction`]
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

/// Flag to pass to [`Camera::set_output_format`]
#[doc(alias = "CAMU_OutputFormat")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum OutputFormat {
    /// YUV422 output format. 16 bits per pixel.
    Yuv422 = ctru_sys::OUTPUT_YUV_422,
    /// RGB565 output format. 16 bits per pixel.
    Rgb565 = ctru_sys::OUTPUT_RGB_565,
}

/// Flag to pass to [`Cam::play_shutter_sound`]
#[doc(alias = "CAMU_ShutterSoundType")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ShutterSound {
    /// Normal shutter sound.
    Normal = ctru_sys::SHUTTER_SOUND_TYPE_NORMAL,
    /// Shutter sound to begin a movie recording.
    Movie = ctru_sys::SHUTTER_SOUND_TYPE_MOVIE,
    /// Shutter sound to finish a movie recording.
    MovieEnd = ctru_sys::SHUTTER_SOUND_TYPE_MOVIE_END,
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

/// Struct containing coordinates passed to [`Camera::set_trimming_params`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrimmingParams {
    x_start: i16,
    y_start: i16,
    x_end: i16,
    y_end: i16,
}

impl TrimmingParams {
    /// Creates a new [`TrimmingParams`] and guarantees the start coordinates are less than or
    /// equal to the end coordinates.
    ///
    /// `x_start <= x_end && y_start <= y_end`
    pub fn new(x_start: i16, y_start: i16, x_end: i16, y_end: i16) -> TrimmingParams {
        assert!(x_start <= x_end && y_start <= y_end);
        Self {
            x_start,
            y_start,
            x_end,
            y_end,
        }
    }
}

/// Represents data used by the camera to calibrate image quality
#[doc(alias = "CAMU_ImageQualityCalibrationData")]
#[derive(Default, Clone, Copy, Debug)]
pub struct ImageQualityCalibrationData(pub ctru_sys::CAMU_ImageQualityCalibrationData);

/// Represents data used by the camera to calibrate image quality when using both outward cameras
#[doc(alias = "CAMU_StereoCameraCalibrationData")]
#[derive(Default, Clone, Copy, Debug)]
pub struct StereoCameraCalibrationData(pub ctru_sys::CAMU_StereoCameraCalibrationData);

/// Represents the camera on the inside of the 3DS
#[non_exhaustive]
pub struct InwardCam;

impl Camera for InwardCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_IN1
    }
}

/// Represents the the outer right camera when the 3DS is open and the dual cameras are pointed
/// away from the user
#[non_exhaustive]
pub struct OutwardRightCam;

impl Camera for OutwardRightCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_OUT1
    }
}

/// Represents the the outer left camera when the 3DS is open and the dual cameras are pointed
/// away from the user
#[non_exhaustive]
pub struct OutwardLeftCam;

impl Camera for OutwardLeftCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_OUT2
    }
}

/// Represents the both outer cameras combined
#[non_exhaustive]
pub struct BothOutwardCam;

impl BothOutwardCam {
    /// Sets whether to enable or disable synchronization
    /// of brightness for both left and right cameras
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

impl Camera for BothOutwardCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_OUT1_OUT2
    }

    fn port_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::PORT_BOTH
    }
}

/// Represents a camera and its functionality
pub trait Camera {
    /// Returns the raw value of the selected camera
    fn camera_as_raw(&self) -> ctru_sys::u32_;

    /// Returns the raw port of the selected camera
    fn port_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::PORT_CAM1
    }

    /// Returns true if the camera is busy (receiving data)
    #[doc(alias = "CAMU_IsBusy")]
    fn is_busy(&self) -> crate::Result<bool> {
        unsafe {
            let mut is_busy = false;
            ResultCode(ctru_sys::CAMU_IsBusy(&mut is_busy, self.port_as_raw()))?;
            Ok(is_busy)
        }
    }

    /// Returns the maximum amount of transfer bytes based on the view size, trimming, and other
    /// modifications set to the camera
    #[doc(alias = "CAMU_GetTransferBytes")]
    fn transfer_byte_count(&self) -> crate::Result<u32> {
        unsafe {
            let mut transfer_bytes = 0;
            ResultCode(ctru_sys::CAMU_GetTransferBytes(
                &mut transfer_bytes,
                self.port_as_raw(),
            ))?;
            Ok(transfer_bytes)
        }
    }

    /// Sets whether or not the camera should trim the image based on parameters set by
    /// [`Camera::set_trimming_params`]
    #[doc(alias = "CAMU_SetTrimming")]
    fn set_trimming(&mut self, enabled: bool) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetTrimming(self.port_as_raw(), enabled))?;
            Ok(())
        }
    }

    /// Returns whether or not trimming is currently enabled for the camera
    #[doc(alias = "CAMU_IsTrimming")]
    fn is_trimming_enabled(&self) -> crate::Result<bool> {
        unsafe {
            let mut trimming = false;
            ResultCode(ctru_sys::CAMU_IsTrimming(&mut trimming, self.port_as_raw()))?;
            Ok(trimming)
        }
    }

    /// Sets trimming parameters based on coordinates specified inside a [`TrimmingParams`]
    #[doc(alias = "CAMU_SetTrimmingParams")]
    fn set_trimming_params(&mut self, params: TrimmingParams) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetTrimmingParams(
                self.port_as_raw(),
                params.x_start,
                params.y_start,
                params.x_end,
                params.y_end,
            ))?;
            Ok(())
        }
    }

    /// Returns the [`TrimmingParams`] set
    #[doc(alias = "CAMU_GetTrimmingParams")]
    fn trimming_params(&self) -> crate::Result<TrimmingParams> {
        unsafe {
            let mut x_start = 0;
            let mut y_start = 0;
            let mut x_end = 0;
            let mut y_end = 0;
            ResultCode(ctru_sys::CAMU_GetTrimmingParams(
                &mut x_start,
                &mut y_start,
                &mut x_end,
                &mut y_end,
                self.port_as_raw(),
            ))?;

            Ok(TrimmingParams {
                x_start,
                y_start,
                x_end,
                y_end,
            })
        }
    }

    /// Sets the trimming parameters revolving around the center of the image.
    /// The new width will be `trim_width / 2` to the left and right of the center.
    /// The new height will be `trim_height / 2` above and below the center.
    #[doc(alias = "CAMU_SetTrimmingParamsCenter")]
    fn set_trimming_params_center(
        &mut self,
        trim_width: i16,
        trim_height: i16,
        cam_width: i16,
        cam_height: i16,
    ) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetTrimmingParamsCenter(
                self.port_as_raw(),
                trim_width,
                trim_height,
                cam_width,
                cam_height,
            ))?;
            Ok(())
        }
    }

    /// Sets the exposure level of the camera
    #[doc(alias = "CAMU_SetExposure")]
    fn set_exposure(&mut self, exposure: i8) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetExposure(self.camera_as_raw(), exposure))?;
            Ok(())
        }
    }

    /// Sets the white balance mod of the camera based on the passed [`WhiteBalance`] argument
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

    /// Sets the white balance mode of the camera based on the passed [`WhiteBalance`] argument
    // TODO: Explain base up
    #[doc(alias = "CAMU_SetWhiteBalanceWithoutBaseUp")]
    fn set_white_balance_without_base_up(
        &mut self,
        white_balance: WhiteBalance,
    ) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetWhiteBalanceWithoutBaseUp(
                self.camera_as_raw(),
                white_balance.into(),
            ))?;
            Ok(())
        }
    }

    /// Sets the sharpness of the camera
    #[doc(alias = "CAMU_SetSharpness")]
    fn set_sharpness(&mut self, sharpness: i8) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetSharpness(self.camera_as_raw(), sharpness))?;
            Ok(())
        }
    }

    /// Sets whether auto exposure is enabled or disabled for the camera
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

    /// Returns true if auto exposure is enabled for the camera
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

    /// Sets whether auto white balance is enabled or disabled for the camera
    #[doc(alias = "CAMU_SetAutoWhiteBalance")]
    fn set_auto_white_balance(&mut self, enabled: bool) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetAutoWhiteBalance(
                self.camera_as_raw(),
                enabled,
            ))?;
            Ok(())
        }
    }

    /// Returns true if auto white balance is enabled for the camera
    #[doc(alias = "CAMU_IsAutoWhiteBalance")]
    fn is_auto_white_balance_enabled(&self) -> crate::Result<bool> {
        unsafe {
            let mut enabled = false;
            ResultCode(ctru_sys::CAMU_IsAutoWhiteBalance(
                &mut enabled,
                self.camera_as_raw(),
            ))?;
            Ok(enabled)
        }
    }

    /// Sets the flip direction of the camera's image based on the passed [`FlipMode`] argument
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

    /// Sets the image resolution of the camera in detail
    ///
    /// # Errors
    ///
    /// This function will error if the coordinates of the first crop point are greater than the
    /// coordinates of the second crop point.
    ///
    /// # Arguments
    /// * `width` - Width of the image
    /// * `height` - height of the image
    /// * `crop_0` - The first crop point in which the image will be trimmed
    /// * `crop_0` - The second crop point in which the image will be trimmed
    #[doc(alias = "CAMU_SetDetailSize")]
    fn set_detail_size(
        &mut self,
        width: i16,
        height: i16,
        crop_0: (i16, i16),
        crop_1: (i16, i16),
    ) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetDetailSize(
                self.camera_as_raw(),
                width,
                height,
                crop_0.0,
                crop_0.1,
                crop_1.0,
                crop_1.1,
                ctru_sys::CONTEXT_A,
            ))?;
            Ok(())
        }
    }

    /// Sets the view size of the camera based on the passed [`ViewSize`] argument.
    #[doc(alias = "CAMU_SetSize")]
    fn set_view_size(&mut self, size: ViewSize) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetSize(
                self.camera_as_raw(),
                size.into(),
                ctru_sys::CONTEXT_A,
            ))?;
            Ok(())
        }
    }

    /// Sets the frame rate of the camera based on the passed [`FrameRate`] argument.
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

    /// Sets the photo mode of the camera based on the passed [`PhotoMode`] argument.
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

    /// Sets the effect of the camera based on the passed [`Effect`] argument.
    ///
    /// Multiple effects can be set at once by combining the bitflags of [`Effect`]
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

    /// Sets the contrast of the camera based on the passed [`Contrast`] argument.
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

    /// Sets the lens correction of the camera based on the passed [`LensCorrection`] argument.
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

    /// Sets the output format of the camera based on the passed [`OutputFormat`] argument.
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

    /// Sets the region in which auto exposure should be based on.
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

    /// Sets the region in which auto white balance should be based on.
    ///
    /// # Arguments
    ///
    /// * `x` - Starting x coordinate of the window
    /// * `y` - Starting y coordinate of the window
    /// * `width` - Width of the window
    /// * `height` - Height of the window
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

    /// Sets whether the noise filter should be enabled or disabled for the camera
    #[doc(alias = "CAMU_SetNoiseFilter")]
    fn set_noise_filter(&mut self, enabled: bool) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetNoiseFilter(self.camera_as_raw(), enabled))?;
            Ok(())
        }
    }

    /// Sets the image quality calibration data for the camera based on the passed in
    /// [`ImageQualityCalibrationData`] argument
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

    /// Returns the current [`ImageQualityCalibrationData`] for the camera
    #[doc(alias = "CAMU_GetImageQualityCalibrationData")]
    fn image_quality_calibration_data(&self) -> crate::Result<ImageQualityCalibrationData> {
        unsafe {
            let mut data = ImageQualityCalibrationData::default();
            ResultCode(ctru_sys::CAMU_GetImageQualityCalibrationData(&mut data.0))?;
            Ok(data)
        }
    }

    /// Sets the camera as the current sleep camera
    // TODO: Explain sleep camera
    #[doc(alias = "CAMU_SetSleepCamera")]
    fn set_sleep_camera(&mut self) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetSleepCamera(self.camera_as_raw()))?;
            Ok(())
        }
    }

    /// Requests the camera to take a picture and write it in a buffer.
    ///
    /// # Errors
    ///
    /// This will error if the camera is busy or if the timeout duration is reached.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the desired image
    /// * `height` - Height of the desired image
    /// * `timeout` - Duration to wait for the image
    fn take_picture(
        &mut self,
        buffer: &mut [u8],
        width: u16,
        height: u16,
        timeout: Duration,
    ) -> crate::Result<()> {
        let transfer_unit = unsafe {
            let mut buf_size = 0;
            ResultCode(ctru_sys::CAMU_GetMaxBytes(
                &mut buf_size,
                width as i16,
                height as i16,
            ))?;
            Ok::<u32, i32>(buf_size)
        }?;

        unsafe {
            ResultCode(ctru_sys::CAMU_SetTransferBytes(
                self.port_as_raw(),
                transfer_unit,
                width as i16,
                height as i16,
            ))?;
        };

        let screen_size: usize = usize::from(width) * usize::from(height) * 2;
        if buffer.len() < screen_size {
            return Err(Error::BufferTooShort {
                provided: buffer.len(),
                wanted: screen_size,
            });
        }

        unsafe {
            ResultCode(ctru_sys::CAMU_Activate(self.camera_as_raw()))?;
            ResultCode(ctru_sys::CAMU_ClearBuffer(self.port_as_raw()))?;
            ResultCode(ctru_sys::CAMU_StartCapture(self.port_as_raw()))?;
        };

        let receive_event = unsafe {
            let mut completion_handle: Handle = 0;
            ResultCode(ctru_sys::CAMU_SetReceiving(
                &mut completion_handle,
                buffer.as_mut_ptr().cast(),
                self.port_as_raw(),
                screen_size as u32,
                transfer_unit.try_into().unwrap(),
            ))?;
            Ok::<Handle, i32>(completion_handle)
        }?;

        unsafe {
            // Panicking without closing an SVC handle causes an ARM exception, we have to handle it carefully (TODO: SVC module)
            let wait_result = ResultCode(ctru_sys::svcWaitSynchronization(
                receive_event,
                timeout.as_nanos().try_into().unwrap(),
            ));

            // We close everything first, then we check for possible errors
            let _ = ctru_sys::svcCloseHandle(receive_event); // We wouldn't return the error even if there was one, so no use of ResultCode is needed
            ResultCode(ctru_sys::CAMU_StopCapture(self.port_as_raw()))?;
            ResultCode(ctru_sys::CAMU_Activate(ctru_sys::SELECT_NONE))?;

            wait_result?;
        };

        Ok(())
    }
}

impl Cam {
    /// Initializes the CAM service.
    ///
    /// # Errors
    ///
    /// This function will return an error if the service was unable to be initialized.
    /// Since this service requires no special or elevated permissions, errors are
    /// rare in practice.
    #[doc(alias = "camInit")]
    pub fn new() -> crate::Result<Cam> {
        unsafe {
            ResultCode(ctru_sys::camInit())?;
            Ok(Cam {
                inner_cam: InwardCam,
                outer_right_cam: OutwardRightCam,
                outer_left_cam: OutwardLeftCam,
                both_outer_cams: BothOutwardCam,
            })
        }
    }

    /// Plays the specified sound based on the [`ShutterSound`] argument
    #[doc(alias = "CAMU_PlayShutterSound")]
    pub fn play_shutter_sound(&self, sound: ShutterSound) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_PlayShutterSound(sound.into()))?;
            Ok(())
        }
    }
}

impl Drop for Cam {
    #[doc(alias = "camExit")]
    fn drop(&mut self) {
        unsafe { ctru_sys::camExit() };
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
