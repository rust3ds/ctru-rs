use bitflags::bitflags;
use ctru_sys::Handle;

pub struct Cam(());

bitflags! {
    #[derive(Default)]
    pub struct CamPort: u32 {
        const PORT_NONE = ctru_sys::PORT_NONE;
        const PORT_CAM1 = ctru_sys::PORT_CAM1;
        const PORT_CAM2 = ctru_sys::PORT_CAM2;
        const PORT_BOTH = ctru_sys::PORT_BOTH;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamSelect: u32 {
        const SELECT_NONE      = ctru_sys::SELECT_NONE;
        const SELECT_OUT1      = ctru_sys::SELECT_OUT1;
        const SELECT_IN1       = ctru_sys::SELECT_IN1;
        const SELECT_OUT2      = ctru_sys::SELECT_OUT2;
        const SELECT_IN1_OUT1  = ctru_sys::SELECT_IN1_OUT1;
        const SELECT_OUT1_OUT2 = ctru_sys::SELECT_OUT1_OUT2;
        const SELECT_IN1_OUT2  = ctru_sys::SELECT_IN1_OUT2;
        const SELECT_ALL       = ctru_sys::SELECT_ALL;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamContext: u32 {
        const CONTEXT_NONE = ctru_sys::CONTEXT_NONE;
        const CONTEXT_A    = ctru_sys::CONTEXT_A;
        const CONTEXT_B    = ctru_sys::CONTEXT_B;
        const CONTEXT_BOTH = ctru_sys::CONTEXT_BOTH;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamFlip: u32 {
        const FLIP_NONE       = ctru_sys::FLIP_NONE;
        const FLIP_HORIZONTAL = ctru_sys::FLIP_HORIZONTAL;
        const FLIP_VERTICAL   = ctru_sys::FLIP_VERTICAL;
        const FLIP_REVERSE    = ctru_sys::FLIP_REVERSE;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamSize: u32 {
        const SIZE_VGA            = ctru_sys::SIZE_VGA;
        const SIZE_QVGA           = ctru_sys::SIZE_QVGA;
        const SIZE_QQVGA          = ctru_sys::SIZE_QQVGA;
        const SIZE_CIF            = ctru_sys::SIZE_CIF;
        const SIZE_QCIF           = ctru_sys::SIZE_QCIF;
        const SIZE_DS_LCD         = ctru_sys::SIZE_DS_LCD;
        const SIZE_DS_LCD_X4      = ctru_sys::SIZE_DS_LCDx4;
        const SIZE_CTR_TOP_LCD    = ctru_sys::SIZE_CTR_TOP_LCD;
        const SIZE_CTR_BOTTOM_LCD = ctru_sys::SIZE_CTR_BOTTOM_LCD;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamFrameRate: u32 {
        const FRAME_RATE_15       = ctru_sys::FRAME_RATE_15;
        const FRAME_RATE_15_TO_5  = ctru_sys::FRAME_RATE_15_TO_5;
        const FRAME_RATE_15_TO_2  = ctru_sys::FRAME_RATE_15_TO_2;
        const FRAME_RATE_10       = ctru_sys::FRAME_RATE_10;
        const FRAME_RATE_8_5      = ctru_sys::FRAME_RATE_8_5;
        const FRAME_RATE_5        = ctru_sys::FRAME_RATE_5;
        const FRAME_RATE_20       = ctru_sys::FRAME_RATE_20;
        const FRAME_RATE_20_TO_5  = ctru_sys::FRAME_RATE_20_TO_5;
        const FRAME_RATE_30       = ctru_sys::FRAME_RATE_30;
        const FRAME_RATE_30_TO_5  = ctru_sys::FRAME_RATE_30_TO_5;
        const FRAME_RATE_15_TO_10 = ctru_sys::FRAME_RATE_15_TO_10;
        const FRAME_RATE_20_TO_10 = ctru_sys::FRAME_RATE_20_TO_10;
        const FRAME_RATE_30_TO_10 = ctru_sys::FRAME_RATE_30_TO_10;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamWhiteBalance: u32 {
        const WHITE_BALANCE_AUTO  = ctru_sys::WHITE_BALANCE_AUTO;
        const WHITE_BALANCE_3200K = ctru_sys::WHITE_BALANCE_3200K;
        const WHITE_BALANCE_4150K = ctru_sys::WHITE_BALANCE_4150K;
        const WHITE_BALANCE_5200K = ctru_sys::WHITE_BALANCE_5200K;
        const WHITE_BALANCE_6000K = ctru_sys::WHITE_BALANCE_6000K;
        const WHITE_BALANCE_7000K = ctru_sys::WHITE_BALANCE_7000K;

        const WHITE_BALANCE_NORMAL                  = ctru_sys::WHITE_BALANCE_NORMAL;
        const WHITE_BALANCE_TUNGSTEN                = ctru_sys::WHITE_BALANCE_TUNGSTEN;
        const WHITE_BALANCE_WHITE_FLUORESCENT_LIGHT = ctru_sys::WHITE_BALANCE_WHITE_FLUORESCENT_LIGHT;
        const WHITE_BALANCE_DAYLIGHT                = ctru_sys::WHITE_BALANCE_DAYLIGHT;
        const WHITE_BALANCE_CLOUDY                  = ctru_sys::WHITE_BALANCE_CLOUDY;
        const WHITE_BALANCE_HORIZON                 = ctru_sys::WHITE_BALANCE_HORIZON;
        const WHITE_BALANCE_SHADE                   = ctru_sys::WHITE_BALANCE_SHADE;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamPhotoMode: u32 {
        const PHOTO_MODE_NORMAL    = ctru_sys::PHOTO_MODE_NORMAL;
        const PHOTO_MODE_PORTRAIT  = ctru_sys::PHOTO_MODE_PORTRAIT;
        const PHOTO_MODE_LANDSCAPE = ctru_sys::PHOTO_MODE_LANDSCAPE;
        const PHOTO_MODE_NIGHTVIEW = ctru_sys::PHOTO_MODE_NIGHTVIEW;
        const PHOTO_MODE_LETTER    = ctru_sys::PHOTO_MODE_LETTER;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamEffect: u32 {
        const EFFECT_NONE     = ctru_sys::EFFECT_NONE;
        const EFFECT_MONO     = ctru_sys::EFFECT_MONO;
        const EFFECT_SEPIA    = ctru_sys::EFFECT_SEPIA;
        const EFFECT_NEGATIVE = ctru_sys::EFFECT_NEGATIVE;
        const EFFECT_NEGAFILM = ctru_sys::EFFECT_NEGAFILM;
        const EFFECT_SEPIA01  = ctru_sys::EFFECT_SEPIA01;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamContrast: u32 {
        const CONTRAST_PATTERN_01 = ctru_sys::CONTRAST_PATTERN_01;
        const CONTRAST_PATTERN_02 = ctru_sys::CONTRAST_PATTERN_02;
        const CONTRAST_PATTERN_03 = ctru_sys::CONTRAST_PATTERN_03;
        const CONTRAST_PATTERN_04 = ctru_sys::CONTRAST_PATTERN_04;
        const CONTRAST_PATTERN_05 = ctru_sys::CONTRAST_PATTERN_05;
        const CONTRAST_PATTERN_06 = ctru_sys::CONTRAST_PATTERN_06;
        const CONTRAST_PATTERN_07 = ctru_sys::CONTRAST_PATTERN_07;
        const CONTRAST_PATTERN_08 = ctru_sys::CONTRAST_PATTERN_08;
        const CONTRAST_PATTERN_09 = ctru_sys::CONTRAST_PATTERN_09;
        const CONTRAST_PATTERN_10 = ctru_sys::CONTRAST_PATTERN_10;
        const CONTRAST_PATTERN_11 = ctru_sys::CONTRAST_PATTERN_11;

        const CONTRAST_LOW    = ctru_sys::CONTRAST_LOW;
        const CONTRAST_NORMAL = ctru_sys::CONTRAST_NORMAL;
        const CONTRAST_HIGH   = ctru_sys::CONTRAST_HIGH;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamLensCorrection: u32 {
        const LENS_CORRECTION_OFF   = ctru_sys::LENS_CORRECTION_OFF;
        const LENS_CORRECTION_ON_70 = ctru_sys::LENS_CORRECTION_ON_70;
        const LENS_CORRECTION_ON_90 = ctru_sys::LENS_CORRECTION_ON_90;

        const LENS_CORRECTION_DARK = ctru_sys::LENS_CORRECTION_DARK;
        const LENS_CORRECTION_NORMAL = ctru_sys::LENS_CORRECTION_NORMAL;
        const LENS_CORRECTION_BRIGHT = ctru_sys::LENS_CORRECTION_BRIGHT;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamOutputFormat: u32 {
        const OUTPUT_YUV_422 = ctru_sys::OUTPUT_YUV_422;
        const OUTPUT_RGB_565 = ctru_sys::OUTPUT_RGB_565;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamShutterSoundType: u32 {
        const SHUTTER_SOUND_TYPE_NORMAL    = ctru_sys::SHUTTER_SOUND_TYPE_NORMAL;
        const SHUTTER_SOUND_TYPE_MOVIE     = ctru_sys::SHUTTER_SOUND_TYPE_MOVIE;
        const SHUTTER_SOUND_TYPE_MOVIE_END = ctru_sys::SHUTTER_SOUND_TYPE_MOVIE_END;
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

impl Cam {
    pub fn init() -> crate::Result<Cam> {
        unsafe {
            let r = ctru_sys::camInit();
            if r < 0 {
                Err(r.into())
            } else {
                Ok(Cam(()))
            }
        }
    }

    pub fn start_capture(&self, port: CamPort) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_StartCapture(port.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn stop_capture(&self, port: CamPort) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_StopCapture(port.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn is_busy(&self, port: CamPort) -> crate::Result<bool> {
        unsafe {
            let mut is_busy = false;
            let r = ctru_sys::CAMU_IsBusy(&mut is_busy, port.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(is_busy)
            }
        }
    }

    pub fn clear_buffer(&self, port: CamPort) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_ClearBuffer(port.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn get_vsync_interrupt_event(&self, port: CamPort) -> crate::Result<u32> {
        unsafe {
            let mut event: Handle = 0;
            let r = ctru_sys::CAMU_GetVsyncInterruptEvent(&mut event, port.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(event)
            }
        }
    }

    pub fn get_buffer_error_interrupt_event(&self, port: CamPort) -> crate::Result<u32> {
        unsafe {
            let mut event: Handle = 0;
            let r = ctru_sys::CAMU_GetBufferErrorInterruptEvent(&mut event, port.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(event)
            }
        }
    }

    pub fn set_receiving(
        &self,
        buf: &mut [u8],
        port: CamPort,
        size: u32,
        buf_size: i16,
    ) -> crate::Result<u32> {
        unsafe {
            let mut completion_handle: Handle = 0;
            let r = ctru_sys::CAMU_SetReceiving(
                &mut completion_handle,
                buf.as_mut_ptr() as *mut ::libc::c_void,
                port.bits(),
                size,
                buf_size,
            );
            if r < 0 {
                Err(r.into())
            } else {
                Ok(completion_handle)
            }
        }
    }

    pub fn is_finished_receiving(&self, port: CamPort) -> crate::Result<bool> {
        unsafe {
            let mut finished_receiving = false;
            let r = ctru_sys::CAMU_IsFinishedReceiving(&mut finished_receiving, port.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(finished_receiving)
            }
        }
    }

    pub fn set_transfer_lines(
        &self,
        port: CamPort,
        lines: i16,
        width: i16,
        height: i16,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetTransferLines(port.bits(), lines, width, height);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
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

    pub fn set_transfer_bytes(
        &self,
        port: CamPort,
        buf_size: u32,
        width: i16,
        height: i16,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetTransferBytes(port.bits(), buf_size, width, height);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn get_transfer_bytes(&self, port: CamPort) -> crate::Result<u32> {
        unsafe {
            let mut transfer_bytes = 0;
            let r = ctru_sys::CAMU_GetTransferBytes(&mut transfer_bytes, port.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(transfer_bytes)
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

    pub fn set_trimming(&self, port: CamPort, enabled: bool) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetTrimming(port.bits(), enabled);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn is_trimming(&self, port: CamPort) -> crate::Result<bool> {
        unsafe {
            let mut trimming = false;
            let r = ctru_sys::CAMU_IsTrimming(&mut trimming, port.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(trimming)
            }
        }
    }

    pub fn set_trimming_params(
        &self,
        port: CamPort,
        params: CamTrimmingParams,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetTrimmingParams(
                port.bits(),
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

    pub fn get_trimming_params(&self, port: CamPort) -> crate::Result<CamTrimmingParams> {
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
                port.bits(),
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

    pub fn set_trimming_params_center(
        &self,
        port: CamPort,
        trim_width: i16,
        trim_height: i16,
        cam_width: i16,
        cam_height: i16,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetTrimmingParamsCenter(
                port.bits(),
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

    pub fn activate(&self, camera: CamSelect) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_Activate(camera.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn switch_context(&self, camera: CamSelect, context: CamContext) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SwitchContext(camera.bits(), context.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_exposure(&self, camera: CamSelect, exposure: i8) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetExposure(camera.bits(), exposure);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_white_balance(
        &self,
        camera: CamSelect,
        white_balance: CamWhiteBalance,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetWhiteBalance(camera.bits(), white_balance.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_white_balance_without_base_up(
        &self,
        camera: CamSelect,
        white_balance: CamWhiteBalance,
    ) -> crate::Result<()> {
        unsafe {
            let r =
                ctru_sys::CAMU_SetWhiteBalanceWithoutBaseUp(camera.bits(), white_balance.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_sharpness(&self, camera: CamSelect, sharpness: i8) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetSharpness(camera.bits(), sharpness);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_auto_exposure(&self, camera: CamSelect, enabled: bool) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetAutoExposure(camera.bits(), enabled);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn is_auto_exposure(&self, camera: CamSelect) -> crate::Result<bool> {
        unsafe {
            let mut is_auto_exposure = false;
            let r = ctru_sys::CAMU_IsAutoExposure(&mut is_auto_exposure, camera.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(is_auto_exposure)
            }
        }
    }

    pub fn set_auto_white_balance(&self, camera: CamSelect, enabled: bool) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetAutoWhiteBalance(camera.bits(), enabled);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn is_auto_white_balance(&self, camera: CamSelect) -> crate::Result<bool> {
        unsafe {
            let mut is_auto_white_balance = false;
            let r = ctru_sys::CAMU_IsAutoWhiteBalance(&mut is_auto_white_balance, camera.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(is_auto_white_balance)
            }
        }
    }

    pub fn flip_image(
        &self,
        camera: CamSelect,
        flip: CamFlip,
        context: CamContext,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_FlipImage(camera.bits(), flip.bits(), context.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_detail_size(
        &self,
        camera: CamSelect,
        width: i16,
        height: i16,
        crop_x0: i16,
        crop_y0: i16,
        crop_x1: i16,
        crop_y1: i16,
        context: CamContext,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetDetailSize(
                camera.bits(),
                width,
                height,
                crop_x0,
                crop_y0,
                crop_x1,
                crop_y1,
                context.bits(),
            );
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_size(
        &self,
        camera: CamSelect,
        size: CamSize,
        context: CamContext,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetSize(camera.bits(), size.bits(), context.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_frame_rate(&self, camera: CamSelect, frame_rate: CamFrameRate) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetFrameRate(camera.bits(), frame_rate.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_photo_mode(&self, camera: CamSelect, photo_mode: CamPhotoMode) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetPhotoMode(camera.bits(), photo_mode.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_effect(
        &self,
        camera: CamSelect,
        effect: CamEffect,
        context: CamContext,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetEffect(camera.bits(), effect.bits(), context.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_contrast(&self, camera: CamSelect, contrast: CamContrast) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetContrast(camera.bits(), contrast.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_lens_correction(
        &self,
        camera: CamSelect,
        lens_correction: CamLensCorrection,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetLensCorrection(camera.bits(), lens_correction.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_output_format(
        &self,
        camera: CamSelect,
        format: CamOutputFormat,
        context: CamContext,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetOutputFormat(camera.bits(), format.bits(), context.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_auto_exposure_window(
        &self,
        camera: CamSelect,
        x: i16,
        y: i16,
        width: i16,
        height: i16,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetAutoExposureWindow(camera.bits(), x, y, width, height);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_auto_white_balance_window(
        &self,
        camera: CamSelect,
        x: i16,
        y: i16,
        width: i16,
        height: i16,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetAutoWhiteBalanceWindow(camera.bits(), x, y, width, height);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_noise_filter(&self, camera: CamSelect, enabled: bool) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetNoiseFilter(camera.bits(), enabled);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn synchronize_vsync_timing(
        &self,
        camera1: CamSelect,
        camera2: CamSelect,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SynchronizeVsyncTiming(camera1.bits(), camera2.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn get_latest_vsync_timing(&self, port: CamPort, past: u32) -> crate::Result<i64> {
        let mut timing = 0;
        unsafe {
            let r = ctru_sys::CAMU_GetLatestVsyncTiming(&mut timing, port.bits(), past);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(timing)
            }
        }
    }

    pub fn write_register_i2c(&self, camera: CamSelect, addr: u16, data: u16) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_WriteRegisterI2c(camera.bits(), addr, data);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn write_mcu_variable_i2c(
        &self,
        camera: CamSelect,
        addr: u16,
        data: u16,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_WriteMcuVariableI2c(camera.bits(), addr, data);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn read_register_i2c_exclusive(&self, camera: CamSelect, addr: u16) -> crate::Result<u16> {
        unsafe {
            let mut data = 0;
            let r = ctru_sys::CAMU_ReadRegisterI2cExclusive(&mut data, camera.bits(), addr);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(data)
            }
        }
    }

    pub fn read_mcu_variable_i2c_exclusive(
        &self,
        camera: CamSelect,
        addr: u16,
    ) -> crate::Result<u16> {
        unsafe {
            let mut data = 0;
            let r = ctru_sys::CAMU_ReadMcuVariableI2cExclusive(&mut data, camera.bits(), addr);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(data)
            }
        }
    }

    pub fn set_image_quality_calibration_data(
        &self,
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

    pub fn get_image_quality_calibration_data(&self) -> crate::Result<ImageQualityCalibrationData> {
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

    pub fn set_package_parameter_without_context(
        &self,
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

    pub fn set_package_parameter_with_context(
        &self,
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

    pub fn set_package_parameter_with_context_detail(
        &self,
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

    pub fn driver_initialize(&self) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_DriverInitialize();
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn driver_finalize(&self) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_DriverFinalize();
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn get_activated_camera(&self, camera: &mut u32) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_GetActivatedCamera(camera);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn get_sleep_camera(&self) -> crate::Result<CamSelect> {
        unsafe {
            let mut camera = 0;
            let r = ctru_sys::CAMU_GetSleepCamera(&mut camera);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(CamSelect::from_bits(camera).unwrap_or_default())
            }
        }
    }

    pub fn set_sleep_camera(&self, camera: CamSelect) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetSleepCamera(camera.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_brightness_synchronization(
        &self,
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
}

impl Drop for Cam {
    fn drop(&mut self) {
        unsafe { ctru_sys::camExit() };
    }
}
