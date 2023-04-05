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
    pub inner_cam: InwardCam,
    pub outer_right_cam: OutwardRightCam,
    pub outer_left_cam: OutwardLeftCam,
    pub both_outer_cams: BothOutwardCam,
}

/// Flag to pass to [Camera::flip_image]
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum FlipMode {
    None = ctru_sys::FLIP_NONE,
    Horizontal = ctru_sys::FLIP_HORIZONTAL,
    Vertical = ctru_sys::FLIP_VERTICAL,
    Reverse = ctru_sys::FLIP_REVERSE,
}

/// Flag to pass to [Camera::set_view_size]
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ViewSize {
    TopLCD = ctru_sys::SIZE_CTR_TOP_LCD,
    /// Equivalent to QVga
    BottomLCD = ctru_sys::SIZE_CTR_BOTTOM_LCD,
    Vga = ctru_sys::SIZE_VGA,
    QQVga = ctru_sys::SIZE_QQVGA,
    Cif = ctru_sys::SIZE_CIF,
    QCif = ctru_sys::SIZE_QCIF,
    /// Nintendo DS Screen
    DS = ctru_sys::SIZE_DS_LCD,
    /// Nintendo DS Screen x4
    DSX4 = ctru_sys::SIZE_DS_LCDx4,
}

/// Flag to pass to [Camera::set_frame_rate]
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum FrameRate {
    Fps15 = ctru_sys::FRAME_RATE_15,
    Fps15To5 = ctru_sys::FRAME_RATE_15_TO_5,
    Fps15To2 = ctru_sys::FRAME_RATE_15_TO_2,
    Fps10 = ctru_sys::FRAME_RATE_10,
    Fps8_5 = ctru_sys::FRAME_RATE_8_5,
    Fps5 = ctru_sys::FRAME_RATE_5,
    Fps20 = ctru_sys::FRAME_RATE_20,
    Fps20To5 = ctru_sys::FRAME_RATE_20_TO_5,
    Fps30 = ctru_sys::FRAME_RATE_30,
    Fps30To5 = ctru_sys::FRAME_RATE_30_TO_5,
    Fps15To10 = ctru_sys::FRAME_RATE_15_TO_10,
    Fps20To10 = ctru_sys::FRAME_RATE_20_TO_10,
    Fps30To10 = ctru_sys::FRAME_RATE_30_TO_10,
}

/// Flag to pass to [Camera::set_white_balance] or
/// [Camera::set_white_balance_without_base_up]
#[derive(Copy, Clone, Debug)]
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
    ///Shade
    Temp7000K = ctru_sys::WHITE_BALANCE_7000K,
}

/// Flag to pass to [Camera::set_photo_mode]
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum PhotoMode {
    Normal = ctru_sys::PHOTO_MODE_NORMAL,
    Portrait = ctru_sys::PHOTO_MODE_PORTRAIT,
    Landscape = ctru_sys::PHOTO_MODE_LANDSCAPE,
    NightView = ctru_sys::PHOTO_MODE_NIGHTVIEW,
    Letter = ctru_sys::PHOTO_MODE_LETTER,
}

/// Flag to pass to [Camera::set_effect]
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum Effect {
    None = ctru_sys::EFFECT_NONE,
    Mono = ctru_sys::EFFECT_MONO,
    Sepia = ctru_sys::EFFECT_SEPIA,
    Negative = ctru_sys::EFFECT_NEGATIVE,
    Negafilm = ctru_sys::EFFECT_NEGAFILM,
    Sepia01 = ctru_sys::EFFECT_SEPIA01,
}

/// Flag to pass to [Camera::set_contrast]
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum Contrast {
    /// OFF
    Low = ctru_sys::CONTRAST_LOW,
    /// Brightness ratio: 70
    Normal = ctru_sys::CONTRAST_NORMAL,
    /// Brightness ratio: 90
    High = ctru_sys::CONTRAST_HIGH,
}

/// Flag to pass to [Camera::set_lens_correction]
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum LensCorrection {
    Off = ctru_sys::LENS_CORRECTION_DARK,
    Normal = ctru_sys::LENS_CORRECTION_NORMAL,
    Bright = ctru_sys::LENS_CORRECTION_BRIGHT,
}

/// Flag to pass to [Camera::set_output_format]
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum OutputFormat {
    Yuv422 = ctru_sys::OUTPUT_YUV_422,
    Rgb565 = ctru_sys::OUTPUT_RGB_565,
}

/// Flag to pass to [Cam::play_shutter_sound]
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ShutterSound {
    Normal = ctru_sys::SHUTTER_SOUND_TYPE_NORMAL,
    Movie = ctru_sys::SHUTTER_SOUND_TYPE_MOVIE,
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

/// Struct containing coordinates passed to [Camera::set_trimming_params].
pub struct TrimmingParams {
    x_start: i16,
    y_start: i16,
    x_end: i16,
    y_end: i16,
}

impl TrimmingParams {
    /// Creates a new [CamTrimmingParams] and guarantees the start coordinates are less than or
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
#[derive(Default)]
pub struct ImageQualityCalibrationData(pub ctru_sys::CAMU_ImageQualityCalibrationData);

/// Represents data used by the camera to calibrate image quality when using both outward cameras
#[derive(Default)]
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
    fn is_busy(&self) -> crate::Result<bool> {
        unsafe {
            let mut is_busy = false;
            ResultCode(ctru_sys::CAMU_IsBusy(&mut is_busy, self.port_as_raw()))?;
            Ok(is_busy)
        }
    }

    /// Returns the maximum amount of transfer bytes based on the view size, trimming, and other
    /// modifications set to the camera
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
    /// [Camera::set_trimming_params]
    fn set_trimming(&mut self, enabled: bool) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetTrimming(self.port_as_raw(), enabled))?;
            Ok(())
        }
    }

    /// Returns whether or not trimming is currently enabled for the camera
    fn is_trimming_enabled(&self) -> crate::Result<bool> {
        unsafe {
            let mut trimming = false;
            ResultCode(ctru_sys::CAMU_IsTrimming(&mut trimming, self.port_as_raw()))?;
            Ok(trimming)
        }
    }

    /// Sets trimming parameters based on coordinates specified inside a [TrimmingParams]
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

    /// Returns the [TrimmingParams] set
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
    fn set_exposure(&mut self, exposure: i8) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetExposure(self.camera_as_raw(), exposure))?;
            Ok(())
        }
    }

    /// Sets the white balance mod of the camera based on the passed [WhiteBalance] argument
    fn set_white_balance(&mut self, white_balance: WhiteBalance) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetWhiteBalance(
                self.camera_as_raw(),
                white_balance.into(),
            ))?;
            Ok(())
        }
    }

    /// Sets the white balance mode of the camera based on the passed [WhiteBalance] argument
    // TODO: Explain base up
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
    fn set_sharpness(&mut self, sharpness: i8) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetSharpness(self.camera_as_raw(), sharpness))?;
            Ok(())
        }
    }

    /// Sets whether auto exposure is enabled or disabled for the camera
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

    /// Sets the flip direction of the camera's image based on the passed [FlipMode] argument
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

    /// Sets the view size of the camera based on the passed [ViewSize] argument.
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

    /// Sets the frame rate of the camera based on the passed [FrameRate] argument.
    fn set_frame_rate(&mut self, frame_rate: FrameRate) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetFrameRate(
                self.camera_as_raw(),
                frame_rate.into(),
            ))?;
            Ok(())
        }
    }

    /// Sets the photo mode of the camera based on the passed [PhotoMode] argument.
    fn set_photo_mode(&mut self, photo_mode: PhotoMode) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetPhotoMode(
                self.camera_as_raw(),
                photo_mode.into(),
            ))?;
            Ok(())
        }
    }

    /// Sets the effect of the camera based on the passed [Effect] argument.
    ///
    /// Multiple effects can be set at once by combining the bitflags of [CamEffect]
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

    /// Sets the contrast of the camera based on the passed [Contrast] argument.
    fn set_contrast(&mut self, contrast: Contrast) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetContrast(
                self.camera_as_raw(),
                contrast.into(),
            ))?;
            Ok(())
        }
    }

    /// Sets the lens correction of the camera based on the passed [LensCorrection] argument.
    fn set_lens_correction(&mut self, lens_correction: LensCorrection) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetLensCorrection(
                self.camera_as_raw(),
                lens_correction.into(),
            ))?;
            Ok(())
        }
    }

    /// Sets the output format of the camera based on the passed [OutputFormat] argument.
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
    fn set_noise_filter(&mut self, enabled: bool) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetNoiseFilter(self.camera_as_raw(), enabled))?;
            Ok(())
        }
    }

    /// Sets the image quality calibration data for the camera based on the passed in
    /// [ImageQualityCalibrationData] argument
    fn set_image_quality_calibration_data(
        &mut self,
        data: ImageQualityCalibrationData,
    ) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_SetImageQualityCalibrationData(data.0))?;
            Ok(())
        }
    }

    /// Returns the current [ImageQualityCalibrationData] for the camera
    fn image_quality_calibration_data(&self) -> crate::Result<ImageQualityCalibrationData> {
        unsafe {
            let mut data = ImageQualityCalibrationData::default();
            ResultCode(ctru_sys::CAMU_GetImageQualityCalibrationData(&mut data.0))?;
            Ok(data)
        }
    }

    /// Sets the camera as the current sleep camera
    // TODO: Explain sleep camera
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
    pub fn init() -> crate::Result<Cam> {
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

    /// Plays the specified sound based on the [ShutterSound] argument
    pub fn play_shutter_sound(&self, sound: ShutterSound) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::CAMU_PlayShutterSound(sound.into()))?;
            Ok(())
        }
    }
}

impl Drop for Cam {
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
