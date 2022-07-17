use bitflags::bitflags;

pub struct Cam(());

bitflags! {
    #[derive(Default)]
    pub struct CamPort: u32 {
        const PORT_NONE = 0;
        const PORT_CAM1 = 1;
        const PORT_CAM2 = 2;
        const PORT_BOTH = Self::PORT_CAM1.bits | Self::PORT_CAM2.bits;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamSelect: u32 {
        const SELECT_NONE      = 0;
        const SELECT_OUT1      = 1;
        const SELECT_IN1       = 2;
        const SELECT_OUT2      = 4;
        const SELECT_IN1_OUT1  = Self::SELECT_OUT1.bits | Self::SELECT_IN1.bits;
        const SELECT_OUT1_OUT2 = Self::SELECT_OUT1.bits | Self::SELECT_OUT2.bits;
        const SELECT_IN1_OUT2  = Self::SELECT_IN1.bits | Self::SELECT_OUT2.bits;
        const SELECT_ALL       = Self::SELECT_IN1.bits | Self::SELECT_OUT1.bits | Self::SELECT_OUT2.bits;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamSize: u32 {
        const SIZE_VGA            = 0;
        const SIZE_QVGA           = 1;
        const SIZE_QQVGA          = 2;
        const SIZE_CIF            = 3;
        const SIZE_QCIF           = 4;
        const SIZE_DS_LCD         = 5;
        const SIZE_DS_LCD_X4      = 6;
        const SIZE_CTR_TOP_LCD    = 7;
        const SIZE_CTR_BOTTOM_LCD = Self::SIZE_QVGA.bits;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamContext: u32 {
        const CONTEXT_NONE = 0;
        const CONTEXT_A    = 1;
        const CONTEXT_B    = 2;
        const CONTEXT_BOTH = Self::CONTEXT_A.bits | Self::CONTEXT_B.bits;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamOutputFormat: u32 {
        const OUTPUT_YUV_422 = 0;
        const OUTPUT_RGB_565 = 1;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CamShutterSoundType: u32 {
        const SHUTTER_SOUND_TYPE_NORMAL    = 0;
        const SHUTTER_SOUND_TYPE_MOVIE     = 1;
        const SHUTTER_SOUND_TYPE_MOVIE_END = 2;
    }
}

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

    pub fn set_size(
        &mut self,
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

    pub fn set_output_format(
        &mut self,
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

    pub fn set_noise_filter(&mut self, camera: CamSelect, enabled: bool) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetNoiseFilter(camera.bits(), enabled);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_auto_exposure(&mut self, camera: CamSelect, enabled: bool) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetAutoExposure(camera.bits(), enabled);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_auto_white_balance(
        &mut self,
        camera: CamSelect,
        enabled: bool,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetAutoWhiteBalance(camera.bits(), enabled);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_trimming(&mut self, port: CamPort, enabled: bool) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetTrimming(port.bits(), enabled);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn get_max_bytes(&self, buf_size: &mut u32, width: i16, height: i16) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_GetMaxBytes(buf_size, width, height);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_transfer_bytes(
        &mut self,
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

    pub fn clear_buffer(&mut self, port: CamPort) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_ClearBuffer(port.bits());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn synchronize_vsync_timing(
        &mut self,
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

    pub fn set_receiving(
        &mut self,
        handle: &mut u32,
        buf: *mut u8,
        port: CamPort,
        size: u32,
        buf_size: i16,
    ) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_SetReceiving(handle, buf as *mut ::libc::c_void, port.bits(), size, buf_size);
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

    pub fn activate(&mut self, camera: CamSelect) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::CAMU_Activate(camera.bits());
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
