//! GSPGPU service

use std::convert::From;
use crate::raw;

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Psc0,
    Psc1,
    VBlank0,
    VBlank1,
    PPF,
    P3D,
    DMA,
}

/// The different framebuffer formats supported by the 3DS
#[derive(Copy, Clone, Debug)]
pub enum FramebufferFormat {
    /// RGBA8. 4 bytes per pixel
    Rgba8,
    /// BGR8. 3 bytes per pixel
    Bgr8,
    /// RGB565. 2 bytes per pixel
    Rgb565,
    /// RGB5A1. 2 bytes per pixel
    Rgb5A1,
    /// RGBA4. 2 bytes per pixel
    Rgba4,
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
pub fn wait_for_event(ev: Event, discard_current: bool) {
    unsafe {
        raw::gspWaitForEvent(ev.into(), discard_current);
    }
}

impl From<raw::GSPGPU_FramebufferFormat> for FramebufferFormat {
    fn from(g: raw::GSPGPU_FramebufferFormat) -> Self {
        use self::FramebufferFormat::*;
        match g {
            raw::GSP_RGBA8_OES => Rgba8,
            raw::GSP_BGR8_OES => Bgr8,
            raw::GSP_RGB565_OES => Rgb565,
            raw::GSP_RGB5_A1_OES => Rgb5A1,
            raw::GSP_RGBA4_OES => Rgba4,
            _ => unreachable!(),
        }
    }
}

impl From<FramebufferFormat> for raw::GSPGPU_FramebufferFormat {
    fn from(g: FramebufferFormat) -> Self {
        use self::FramebufferFormat::*;
        match g {
            Rgba8 => raw::GSP_RGBA8_OES,
            Bgr8 => raw::GSP_BGR8_OES,
            Rgb565 => raw::GSP_RGB565_OES,
            Rgb5A1 => raw::GSP_RGB5_A1_OES,
            Rgba4 => raw::GSP_RGBA4_OES,
        }
    }
}

impl From<Event> for raw::GSPGPU_Event {
    fn from(ev: Event) -> Self {
        use self::Event::*;
        match ev {
            Psc0 => raw::GSPGPU_EVENT_PSC0,
            Psc1 => raw::GSPGPU_EVENT_PSC1,
            VBlank0 => raw::GSPGPU_EVENT_VBlank0,
            VBlank1 => raw::GSPGPU_EVENT_VBlank1,
            PPF => raw::GSPGPU_EVENT_PPF,
            P3D => raw::GSPGPU_EVENT_P3D,
            DMA => raw::GSPGPU_EVENT_DMA,
        }
    }
}
