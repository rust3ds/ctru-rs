//! GSPGPU service

/// GSPGPU events that can be awaited.
#[doc(alias = "GSPGPU_Event")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Event {
    /// Memory fill completed.
    Psc0 = ctru_sys::GSPGPU_EVENT_PSC0,
    /// TODO: Unknown.
    Psc1 = ctru_sys::GSPGPU_EVENT_PSC1,
    /// TODO: Unknown.
    VBlank0 = ctru_sys::GSPGPU_EVENT_VBlank0,
    /// TODO: Unknown.
    VBlank1 = ctru_sys::GSPGPU_EVENT_VBlank1,
    /// Display transfer finished.
    PPF = ctru_sys::GSPGPU_EVENT_PPF,
    /// Command list processing finished.
    P3D = ctru_sys::GSPGPU_EVENT_P3D,
    /// TODO: Unknown.
    DMA = ctru_sys::GSPGPU_EVENT_DMA,
}

#[doc(alias = "GSPGPU_FramebufferFormat")]
/// Framebuffer formats supported by the 3DS.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum FramebufferFormat {
    /// RGBA8. 4 bytes per pixel
    Rgba8 = ctru_sys::GSP_RGBA8_OES,
    /// BGR8. 3 bytes per pixel
    Bgr8 = ctru_sys::GSP_BGR8_OES,
    /// RGB565. 2 bytes per pixel
    Rgb565 = ctru_sys::GSP_RGB565_OES,
    /// RGB5A1. 2 bytes per pixel
    Rgb5A1 = ctru_sys::GSP_RGB5_A1_OES,
    /// RGBA4. 2 bytes per pixel
    Rgba4 = ctru_sys::GSP_RGBA4_OES,
}

impl FramebufferFormat {
    /// Returns the number of bytes per pixel used by this FramebufferFormat
    pub fn pixel_depth_bytes(&self) -> usize {
        use self::FramebufferFormat::*;
        match *self {
            Rgba8 => 4,
            Bgr8 => 3,
            Rgb565 => 2,
            Rgb5A1 => 2,
            Rgba4 => 2,
        }
    }
}

/// Waits for a GSPGPU event to occur.
///
/// `discard_current` determines whether to discard the current event and wait for the next event
#[doc(alias = "gspWaitForEvent")]
pub fn wait_for_event(ev: Event, discard_current: bool) {
    unsafe {
        ctru_sys::gspWaitForEvent(ev.into(), discard_current);
    }
}

impl From<ctru_sys::GSPGPU_FramebufferFormat> for FramebufferFormat {
    fn from(g: ctru_sys::GSPGPU_FramebufferFormat) -> Self {
        use self::FramebufferFormat::*;
        match g {
            ctru_sys::GSP_RGBA8_OES => Rgba8,
            ctru_sys::GSP_BGR8_OES => Bgr8,
            ctru_sys::GSP_RGB565_OES => Rgb565,
            ctru_sys::GSP_RGB5_A1_OES => Rgb5A1,
            ctru_sys::GSP_RGBA4_OES => Rgba4,
            _ => unreachable!(),
        }
    }
}

from_impl!(FramebufferFormat, ctru_sys::GSPGPU_FramebufferFormat);
from_impl!(Event, ctru_sys::GSPGPU_Event);
