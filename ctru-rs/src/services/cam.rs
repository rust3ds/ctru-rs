use crate::services::gspgpu::FramebufferFormat;
use bitflags::bitflags;
use ctru_sys::Handle;
use std::cell::RefCell;

#[non_exhaustive]
pub struct Cam {
    pub inner_cam: RefCell<InwardCam>,
    pub outer_right_cam: RefCell<OutwardRightCam>,
    pub outer_left_cam: RefCell<OutwardLeftCam>,
    pub both_outer_cams: RefCell<BothOutwardCam>,
}

bitflags! {
    #[derive(Default)]
    struct CamPort: u32 {
        const NONE = ctru_sys::PORT_NONE;
        const CAM1 = ctru_sys::PORT_CAM1;
        const CAM2 = ctru_sys::PORT_CAM2;
        const BOTH = ctru_sys::PORT_BOTH;
    }
}

bitflags! {
    #[derive(Default)]
    struct CamSelect: u32 {
        const NONE      = ctru_sys::SELECT_NONE;
        const OUT1      = ctru_sys::SELECT_OUT1;
        const IN1       = ctru_sys::SELECT_IN1;
        const OUT2      = ctru_sys::SELECT_OUT2;
        const OUT1_OUT2 = ctru_sys::SELECT_OUT1_OUT2;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamFlip: u32 {
        const NONE       = ctru_sys::FLIP_NONE;
        const HORIZONTAL = ctru_sys::FLIP_HORIZONTAL;
        const VERTICAL   = ctru_sys::FLIP_VERTICAL;
        const REVERSE    = ctru_sys::FLIP_REVERSE;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamSize: u32 {
        const VGA            = ctru_sys::SIZE_VGA;
        const QVGA           = ctru_sys::SIZE_QVGA;
        const QQVGA          = ctru_sys::SIZE_QQVGA;
        const CIF            = ctru_sys::SIZE_CIF;
        const QCIF           = ctru_sys::SIZE_QCIF;
        const DS_LCD         = ctru_sys::SIZE_DS_LCD;
        const DS_LCD_X4      = ctru_sys::SIZE_DS_LCDx4;
        const CTR_TOP_LCD    = ctru_sys::SIZE_CTR_TOP_LCD;
        const CTR_BOTTOM_LCD = ctru_sys::SIZE_CTR_BOTTOM_LCD;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamFrameRate: u32 {
        const RATE_15       = ctru_sys::FRAME_RATE_15;
        const RATE_15_TO_5  = ctru_sys::FRAME_RATE_15_TO_5;
        const RATE_15_TO_2  = ctru_sys::FRAME_RATE_15_TO_2;
        const RATE_10       = ctru_sys::FRAME_RATE_10;
        const RATE_8_5      = ctru_sys::FRAME_RATE_8_5;
        const RATE_5        = ctru_sys::FRAME_RATE_5;
        const RATE_20       = ctru_sys::FRAME_RATE_20;
        const RATE_20_TO_5  = ctru_sys::FRAME_RATE_20_TO_5;
        const RATE_30       = ctru_sys::FRAME_RATE_30;
        const RATE_30_TO_5  = ctru_sys::FRAME_RATE_30_TO_5;
        const RATE_15_TO_10 = ctru_sys::FRAME_RATE_15_TO_10;
        const RATE_20_TO_10 = ctru_sys::FRAME_RATE_20_TO_10;
        const RATE_30_TO_10 = ctru_sys::FRAME_RATE_30_TO_10;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamWhiteBalance: u32 {
        const AUTO  = ctru_sys::WHITE_BALANCE_AUTO;
        const BALANCE_3200K = ctru_sys::WHITE_BALANCE_3200K;
        const BALANCE_4150K = ctru_sys::WHITE_BALANCE_4150K;
        const BALANCE_5200K = ctru_sys::WHITE_BALANCE_5200K;
        const BALANCE_6000K = ctru_sys::WHITE_BALANCE_6000K;
        const BALANCE_7000K = ctru_sys::WHITE_BALANCE_7000K;

        const NORMAL                  = ctru_sys::WHITE_BALANCE_NORMAL;
        const TUNGSTEN                = ctru_sys::WHITE_BALANCE_TUNGSTEN;
        const WHITE_FLUORESCENT_LIGHT = ctru_sys::WHITE_BALANCE_WHITE_FLUORESCENT_LIGHT;
        const DAYLIGHT                = ctru_sys::WHITE_BALANCE_DAYLIGHT;
        const CLOUDY                  = ctru_sys::WHITE_BALANCE_CLOUDY;
        const HORIZON                 = ctru_sys::WHITE_BALANCE_HORIZON;
        const SHADE                   = ctru_sys::WHITE_BALANCE_SHADE;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamPhotoMode: u32 {
        const NORMAL    = ctru_sys::PHOTO_MODE_NORMAL;
        const PORTRAIT  = ctru_sys::PHOTO_MODE_PORTRAIT;
        const LANDSCAPE = ctru_sys::PHOTO_MODE_LANDSCAPE;
        const NIGHTVIEW = ctru_sys::PHOTO_MODE_NIGHTVIEW;
        const LETTER    = ctru_sys::PHOTO_MODE_LETTER;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamEffect: u32 {
        const NONE     = ctru_sys::EFFECT_NONE;
        const MONO     = ctru_sys::EFFECT_MONO;
        const SEPIA    = ctru_sys::EFFECT_SEPIA;
        const NEGATIVE = ctru_sys::EFFECT_NEGATIVE;
        const NEGAFILM = ctru_sys::EFFECT_NEGAFILM;
        const SEPIA01  = ctru_sys::EFFECT_SEPIA01;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamContrast: u32 {
        const PATTERN_01 = ctru_sys::CONTRAST_PATTERN_01;
        const PATTERN_02 = ctru_sys::CONTRAST_PATTERN_02;
        const PATTERN_03 = ctru_sys::CONTRAST_PATTERN_03;
        const PATTERN_04 = ctru_sys::CONTRAST_PATTERN_04;
        const PATTERN_05 = ctru_sys::CONTRAST_PATTERN_05;
        const PATTERN_06 = ctru_sys::CONTRAST_PATTERN_06;
        const PATTERN_07 = ctru_sys::CONTRAST_PATTERN_07;
        const PATTERN_08 = ctru_sys::CONTRAST_PATTERN_08;
        const PATTERN_09 = ctru_sys::CONTRAST_PATTERN_09;
        const PATTERN_10 = ctru_sys::CONTRAST_PATTERN_10;
        const PATTERN_11 = ctru_sys::CONTRAST_PATTERN_11;

        const LOW    = ctru_sys::CONTRAST_LOW;
        const NORMAL = ctru_sys::CONTRAST_NORMAL;
        const HIGH   = ctru_sys::CONTRAST_HIGH;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamLensCorrection: u32 {
        const OFF   = ctru_sys::LENS_CORRECTION_OFF;
        const ON_70 = ctru_sys::LENS_CORRECTION_ON_70;
        const ON_90 = ctru_sys::LENS_CORRECTION_ON_90;

        const DARK = ctru_sys::LENS_CORRECTION_DARK;
        const NORMAL = ctru_sys::LENS_CORRECTION_NORMAL;
        const BRIGHT = ctru_sys::LENS_CORRECTION_BRIGHT;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamOutputFormat: u32 {
        const YUV_422 = ctru_sys::OUTPUT_YUV_422;
        const RGB_565 = ctru_sys::OUTPUT_RGB_565;
    }
}

impl TryFrom<FramebufferFormat> for CamOutputFormat {
    type Error = ();

    fn try_from(value: FramebufferFormat) -> Result<Self, Self::Error> {
        match value {
            FramebufferFormat::Rgb565 => Ok(CamOutputFormat::RGB_565),
            _ => Err(()),
        }
    }
}

impl TryFrom<CamOutputFormat> for FramebufferFormat {
    type Error = ();

    fn try_from(value: CamOutputFormat) -> Result<Self, Self::Error> {
        match value {
            CamOutputFormat::RGB_565 => Ok(FramebufferFormat::Rgb565),
            _ => Err(()),
        }
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamShutterSoundType: u32 {
        const NORMAL    = ctru_sys::SHUTTER_SOUND_TYPE_NORMAL;
        const MOVIE     = ctru_sys::SHUTTER_SOUND_TYPE_MOVIE;
        const MOVIE_END = ctru_sys::SHUTTER_SOUND_TYPE_MOVIE_END;
    }
}

pub struct CamTrimmingParams {
    pub x_start: i16,
    pub y_start: i16,
    pub x_end: i16,
    pub y_end: i16,
}

#[derive(Default)]
pub struct ImageQualityCalibrationData(ctru_sys::CAMU_ImageQualityCalibrationData);

#[derive(Default)]
pub struct StereoCameraCalibrationData(ctru_sys::CAMU_StereoCameraCalibrationData);

#[derive(Default)]
pub struct PackageParameterCameraSelect(ctru_sys::CAMU_PackageParameterCameraSelect);

#[derive(Default)]
pub struct PackageParameterContext(ctru_sys::CAMU_PackageParameterContext);

#[derive(Default)]
pub struct PackageParameterContextDetail(ctru_sys::CAMU_PackageParameterContextDetail);

#[non_exhaustive]
pub struct InwardCam;

impl Camera for InwardCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_IN1
    }
}

#[non_exhaustive]
pub struct OutwardRightCam;

impl Camera for OutwardRightCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_OUT1
    }
}

#[non_exhaustive]
pub struct OutwardLeftCam;

impl Camera for OutwardLeftCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_OUT2
    }
}

#[non_exhaustive]
pub struct BothOutwardCam;

impl Camera for BothOutwardCam {
    fn camera_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::SELECT_OUT1_OUT2
    }

    fn port_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::PORT_BOTH
    }

    fn synchronize_vsync_timing(&self) -> crate::Result<()> {
        unsafe {
            let r =
                ctru_sys::CAMU_SynchronizeVsyncTiming(ctru_sys::SELECT_OUT1, ctru_sys::SELECT_OUT2);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }
}

pub trait Camera {
    fn camera_as_raw(&self) -> ctru_sys::u32_;

    fn port_as_raw(&self) -> ctru_sys::u32_ {
        ctru_sys::PORT_CAM1
    }

    fn start_capture(&mut self) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_StartCapture(self.port_as_raw());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn stop_capture(&mut self) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_StopCapture(self.port_as_raw());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn is_busy(&self) -> crate::Result<bool> {
        unsafe {
            let mut is_busy = false;
            let r = ctru_sys::CAMU_IsBusy(&mut is_busy, self.port_as_raw());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(is_busy)
            }
        }
    }

    fn clear_buffer(&mut self) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_ClearBuffer(self.port_as_raw());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn get_vsync_interrupt_event(&self) -> crate::Result<u32> {
        unsafe {
            let mut event: Handle = 0;
            let r = ctru_sys::CAMU_GetVsyncInterruptEvent(&mut event, self.port_as_raw());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(event)
            }
        }
    }

    fn get_buffer_error_interrupt_event(&self) -> crate::Result<u32> {
        unsafe {
            let mut event: Handle = 0;
            let r = ctru_sys::CAMU_GetBufferErrorInterruptEvent(&mut event, self.port_as_raw());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(event)
            }
        }
    }

    fn set_receiving(
        &mut self,
        buf: &mut [u8],
        size: u32,
        transfer_unit: i16,
    ) -> crate::Result<u32> {
        unsafe {
            let mut completion_handle: Handle = 0;
            let r = ctru_sys::CAMU_SetReceiving(
                &mut completion_handle,
                buf.as_mut_ptr() as *mut ::libc::c_void,
                self.port_as_raw(),
                size,
                transfer_unit,
            );
            if r < 0 {
                Err(r.into())
            } else {
                Ok(completion_handle)
            }
        }
    }

    fn is_finished_receiving(&self) -> crate::Result<bool> {
        unsafe {
            let mut finished_receiving = false;
            let r = ctru_sys::CAMU_IsFinishedReceiving(&mut finished_receiving, self.port_as_raw());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(finished_receiving)
            }
        }
    }

    fn set_transfer_lines(&mut self, lines: i16, width: i16, height: i16) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetTransferLines(self.port_as_raw(), lines, width, height);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_transfer_bytes(&mut self, buf_size: u32, width: i16, height: i16) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetTransferBytes(self.port_as_raw(), buf_size, width, height);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn get_transfer_bytes(&self) -> crate::Result<u32> {
        unsafe {
            let mut transfer_bytes = 0;
            let r = ctru_sys::CAMU_GetTransferBytes(&mut transfer_bytes, self.port_as_raw());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(transfer_bytes)
            }
        }
    }

    fn set_trimming(&mut self, enabled: bool) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetTrimming(self.port_as_raw(), enabled);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn is_trimming(&self) -> crate::Result<bool> {
        unsafe {
            let mut trimming = false;
            let r = ctru_sys::CAMU_IsTrimming(&mut trimming, self.port_as_raw());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(trimming)
            }
        }
    }

    fn set_trimming_params(&mut self, params: CamTrimmingParams) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetTrimmingParams(
                self.port_as_raw(),
                params.x_start,
                params.y_start,
                params.x_end,
                params.y_end,
            );
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn get_trimming_params(&self) -> crate::Result<CamTrimmingParams> {
        unsafe {
            let mut x_start = 0;
            let mut y_start = 0;
            let mut x_end = 0;
            let mut y_end = 0;
            let r = ctru_sys::CAMU_GetTrimmingParams(
                &mut x_start,
                &mut y_start,
                &mut x_end,
                &mut y_end,
                self.port_as_raw(),
            );
            if r < 0 {
                Err(r.into())
            } else {
                Ok(CamTrimmingParams {
                    x_start,
                    y_start,
                    x_end,
                    y_end,
                })
            }
        }
    }

    fn set_trimming_params_center(
        &self,
        trim_width: i16,
        trim_height: i16,
        cam_width: i16,
        cam_height: i16,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetTrimmingParamsCenter(
                self.port_as_raw(),
                trim_width,
                trim_height,
                cam_width,
                cam_height,
            );
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn activate(&mut self) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_Activate(self.camera_as_raw());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn deactivate(&mut self) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_Activate(ctru_sys::SELECT_NONE);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_exposure(&mut self, exposure: i8) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetExposure(self.camera_as_raw(), exposure);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_white_balance(&mut self, white_balance: CamWhiteBalance) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetWhiteBalance(self.camera_as_raw(), white_balance.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_white_balance_without_base_up(
        &mut self,
        white_balance: CamWhiteBalance,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetWhiteBalanceWithoutBaseUp(
                self.camera_as_raw(),
                white_balance.bits(),
            );
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_sharpness(&mut self, sharpness: i8) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetSharpness(self.camera_as_raw(), sharpness);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_auto_exposure(&mut self, enabled: bool) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetAutoExposure(self.camera_as_raw(), enabled);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn is_auto_exposure(&self) -> crate::Result<bool> {
        unsafe {
            let mut is_auto_exposure = false;
            let r = ctru_sys::CAMU_IsAutoExposure(&mut is_auto_exposure, self.camera_as_raw());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(is_auto_exposure)
            }
        }
    }

    fn set_auto_white_balance(&mut self, enabled: bool) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetAutoWhiteBalance(self.camera_as_raw(), enabled);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn is_auto_white_balance(&self) -> crate::Result<bool> {
        unsafe {
            let mut is_auto_white_balance = false;
            let r =
                ctru_sys::CAMU_IsAutoWhiteBalance(&mut is_auto_white_balance, self.camera_as_raw());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(is_auto_white_balance)
            }
        }
    }

    fn flip_image(&mut self, flip: CamFlip) -> crate::Result<()> {
        unsafe {
            let r =
                ctru_sys::CAMU_FlipImage(self.camera_as_raw(), flip.bits(), ctru_sys::CONTEXT_A);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_detail_size(
        &mut self,
        width: i16,
        height: i16,
        crop_0: (i16, i16),
        crop_1: (i16, i16),
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetDetailSize(
                self.camera_as_raw(),
                width,
                height,
                crop_0.0,
                crop_0.1,
                crop_1.0,
                crop_1.1,
                ctru_sys::CONTEXT_A,
            );
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_size(&mut self, size: CamSize) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetSize(self.camera_as_raw(), size.bits(), ctru_sys::CONTEXT_A);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_frame_rate(&mut self, frame_rate: CamFrameRate) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetFrameRate(self.camera_as_raw(), frame_rate.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_photo_mode(&mut self, photo_mode: CamPhotoMode) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetPhotoMode(self.camera_as_raw(), photo_mode.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_effect(&mut self, effect: CamEffect) -> crate::Result<()> {
        unsafe {
            let r =
                ctru_sys::CAMU_SetEffect(self.camera_as_raw(), effect.bits(), ctru_sys::CONTEXT_A);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_contrast(&mut self, contrast: CamContrast) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetContrast(self.camera_as_raw(), contrast.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_lens_correction(&mut self, lens_correction: CamLensCorrection) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetLensCorrection(self.camera_as_raw(), lens_correction.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_output_format(&mut self, format: CamOutputFormat) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetOutputFormat(
                self.camera_as_raw(),
                format.bits(),
                ctru_sys::CONTEXT_A,
            );
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_auto_exposure_window(
        &mut self,
        x: i16,
        y: i16,
        width: i16,
        height: i16,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetAutoExposureWindow(self.camera_as_raw(), x, y, width, height);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_auto_white_balance_window(
        &mut self,
        x: i16,
        y: i16,
        width: i16,
        height: i16,
    ) -> crate::Result<()> {
        unsafe {
            let r =
                ctru_sys::CAMU_SetAutoWhiteBalanceWindow(self.camera_as_raw(), x, y, width, height);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_noise_filter(&mut self, enabled: bool) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetNoiseFilter(self.camera_as_raw(), enabled);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn get_latest_vsync_timing(&self, past: u32) -> crate::Result<i64> {
        let mut timing = 0;
        unsafe {
            let r = ctru_sys::CAMU_GetLatestVsyncTiming(&mut timing, self.port_as_raw(), past);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(timing)
            }
        }
    }

    fn set_image_quality_calibration_data(
        &mut self,
        data: ImageQualityCalibrationData,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetImageQualityCalibrationData(data.0);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn get_image_quality_calibration_data(&self) -> crate::Result<ImageQualityCalibrationData> {
        unsafe {
            let mut data = ImageQualityCalibrationData::default();
            let r = ctru_sys::CAMU_GetImageQualityCalibrationData(&mut data.0);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(data)
            }
        }
    }

    fn set_package_parameter_without_context(
        &mut self,
        param: PackageParameterCameraSelect,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetPackageParameterWithoutContext(param.0);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_package_parameter_with_context(
        &mut self,
        param: PackageParameterContext,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetPackageParameterWithContext(param.0);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_package_parameter_with_context_detail(
        &mut self,
        param: PackageParameterContextDetail,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetPackageParameterWithContextDetail(param.0);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_sleep_camera(&mut self) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetSleepCamera(self.camera_as_raw());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn set_brightness_synchronization(
        &mut self,
        brightness_synchronization: bool,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetBrightnessSynchronization(brightness_synchronization);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    fn synchronize_vsync_timing(&self) -> crate::Result<()> {
        Ok(())
    }

    fn take_picture(
        &mut self,
        width: u16,
        height: u16,
        transfer_unit: u32,
        timeout: i64,
    ) -> crate::Result<Vec<u8>> {
        let screen_size = u32::from(width) * u32::from(width) * 2;

        let mut buf = vec![0u8; usize::try_from(screen_size).unwrap()];

        self.set_transfer_bytes(
            transfer_unit,
            width.try_into().unwrap(),
            height.try_into().unwrap(),
        )?;

        self.activate()?;

        self.clear_buffer()?;

        self.start_capture()?;

        let receive_event =
            self.set_receiving(&mut buf, screen_size, transfer_unit.try_into().unwrap())?;

        unsafe {
            let r = ctru_sys::svcWaitSynchronization(receive_event, timeout);
            if r < 0 {
                return Err(r.into());
            }
        };

        self.stop_capture()?;

        unsafe {
            let r = ctru_sys::svcCloseHandle(receive_event);
            if r < 0 {
                return Err(r.into());
            }
        };

        Ok(buf)
    }
}

impl Cam {
    pub fn init() -> crate::Result<Cam> {
        unsafe {
            let r = ctru_sys::camInit();
            if r < 0 {
                Err(r.into())
            } else {
                Ok(Cam {
                    inner_cam: RefCell::new(InwardCam),
                    outer_right_cam: RefCell::new(OutwardRightCam),
                    outer_left_cam: RefCell::new(OutwardLeftCam),
                    both_outer_cams: RefCell::new(BothOutwardCam),
                })
            }
        }
    }

    pub fn get_max_bytes(&self, width: i16, height: i16) -> crate::Result<u32> {
        unsafe {
            let mut buf_size = 0;
            let r = ctru_sys::CAMU_GetMaxBytes(&mut buf_size, width, height);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(buf_size)
            }
        }
    }

    pub fn get_max_lines(&self, width: i16, height: i16) -> crate::Result<i16> {
        unsafe {
            let mut max_lines = 0;
            let r = ctru_sys::CAMU_GetMaxLines(&mut max_lines, width, height);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(max_lines)
            }
        }
    }

    pub fn play_shutter_sound(&self, sound: CamShutterSoundType) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_PlayShutterSound(sound.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }
}

impl Drop for Cam {
    fn drop(&mut self) {
        unsafe { ctru_sys::camExit() };
    }
}
