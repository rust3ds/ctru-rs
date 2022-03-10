//! LCD screens manipulation helper

use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Drop;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::{Error, Result};
use crate::services::gspgpu::{self, FramebufferFormat};

/// Trait implemented by TopScreen and BottomScreen for common methods
pub trait Screen {
    /// Returns the libctru value for the Screen kind
    fn as_raw(&self) -> ctru_sys::gfxScreen_t;

    /// Sets whether to use double buffering. Enabled by default.
    ///
    /// Note that even when double buffering is disabled, one should still use the `swap_buffers`
    /// method on each frame to keep the gsp configuration up to date
    fn set_double_buffering(&mut self, enabled: bool) {
        unsafe { ctru_sys::gfxSetDoubleBuffering(self.as_raw(), enabled) }
    }

    /// Gets the framebuffer format
    fn get_framebuffer_format(&self) -> FramebufferFormat {
        unsafe { ctru_sys::gfxGetScreenFormat(self.as_raw()).into() }
    }

    /// Change the framebuffer format
    fn set_framebuffer_format(&mut self, fmt: FramebufferFormat) {
        unsafe { ctru_sys::gfxSetScreenFormat(self.as_raw(), fmt.into()) }
    }
}

#[non_exhaustive]
pub struct TopScreen;

#[non_exhaustive]
pub struct BottomScreen;

/// Representation of a framebuffer for one [`Side`] of the top screen, or the
/// entire bottom screen. The inner pointer is only valid for one frame if double
/// buffering is enabled. Data written to `ptr` will be rendered to the screen.
#[derive(Debug)]
pub struct RawFrameBuffer<'screen> {
    /// Pointer to graphics data to be rendered.
    pub ptr: *mut u8,
    /// The width of the framebuffer in pixels.
    pub width: u16,
    /// The height of the framebuffer in pixels.
    pub height: u16,
    /// Keep a mutable reference to the Screen for which this framebuffer is tied.
    screen: PhantomData<&'screen mut dyn Screen>,
}

#[derive(Copy, Clone, Debug)]
/// Side of top screen framebuffer
///
/// The top screen of the 3DS can have two separate sets of framebuffers to support its 3D functionality
pub enum Side {
    /// The left framebuffer. This framebuffer is also the one used when 3D is disabled
    Left,
    /// The right framebuffer
    Right,
}

/// A handle to libctru's gfx module. This module is a wrapper around the GSPGPU service that
/// provides helper functions and utilities for software rendering.
///
/// The service exits when this struct is dropped.
#[non_exhaustive]
pub struct Gfx {
    pub top_screen: RefCell<TopScreen>,
    pub bottom_screen: RefCell<BottomScreen>,
}

static GFX_ACTIVE: AtomicBool = AtomicBool::new(false);

impl Gfx {
    /// Initialize the Gfx module with the chosen framebuffer formats for the top and bottom
    /// screens
    ///
    /// Use `Gfx::init()` instead of this function to initialize the module with default parameters
    pub fn with_formats(
        top_fb_fmt: FramebufferFormat,
        bottom_fb_fmt: FramebufferFormat,
        use_vram_buffers: bool,
    ) -> Result<Self> {
        match GFX_ACTIVE.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => {
                unsafe {
                    ctru_sys::gfxInit(top_fb_fmt.into(), bottom_fb_fmt.into(), use_vram_buffers);
                }

                Ok(Gfx {
                    top_screen: RefCell::new(TopScreen),
                    bottom_screen: RefCell::new(BottomScreen),
                })
            }
            Err(_) => Err(Error::ServiceAlreadyActive("Gfx")),
        }
    }

    /// Creates a new Gfx instance with default init values
    /// It's the same as calling: `Gfx::with_formats(FramebufferFormat::Bgr8, FramebufferFormat::Bgr8, false)
    pub fn init() -> Result<Self> {
        Gfx::with_formats(FramebufferFormat::Bgr8, FramebufferFormat::Bgr8, false)
    }

    /// Flushes the current framebuffers
    pub fn flush_buffers(&self) {
        unsafe { ctru_sys::gfxFlushBuffers() };
    }

    /// Swaps the framebuffers and sets the gsp state
    ///
    /// Use this function when working with software rendering
    pub fn swap_buffers(&self) {
        unsafe { ctru_sys::gfxSwapBuffers() };
    }

    /// Swaps the framebuffers without manipulating the gsp state
    ///
    /// Use this function when working with GPU rendering
    pub fn swap_buffers_gpu(&self) {
        unsafe { ctru_sys::gfxSwapBuffersGpu() };
    }

    /// Waits for the vertical blank interrupt
    ///
    /// Use this to synchronize your application with the refresh rate of the LCD screens
    pub fn wait_for_vblank(&self) {
        gspgpu::wait_for_event(gspgpu::Event::VBlank0, true);
    }
}

impl TopScreen {
    /// Enable or disable the 3D stereoscopic effect
    pub fn set_3d_enabled(&mut self, enabled: bool) {
        unsafe {
            ctru_sys::gfxSet3D(enabled);
        }
    }

    /// Enable or disable the wide screen mode (top screen).
    /// This only works when 3D is disabled.
    pub fn set_wide_mode(&mut self, enabled: bool) {
        unsafe {
            ctru_sys::gfxSetWide(enabled);
        }
    }

    /// Get the status of wide screen mode.
    pub fn get_wide_mode(&self) -> bool {
        unsafe { ctru_sys::gfxIsWide() }
    }

    /// Returns a [`RawFrameBuffer`] for the given [`Side`] of the top screen.
    ///
    /// Note that the pointer of the framebuffer returned by this function can
    /// change after each call to this function if double buffering is enabled.
    pub fn get_raw_framebuffer(&mut self, side: Side) -> RawFrameBuffer {
        RawFrameBuffer::for_screen_side(self, side)
    }
}

impl BottomScreen {
    /// Returns a [`RawFrameBuffer`] for the bottom screen.
    ///
    /// Note that the pointer of the framebuffer returned by this function can
    /// change after each call to this function if double buffering is enabled.
    pub fn get_raw_framebuffer(&mut self) -> RawFrameBuffer {
        RawFrameBuffer::for_screen_side(self, Side::Left)
    }
}

impl<'screen> RawFrameBuffer<'screen> {
    fn for_screen_side(screen: &'screen mut dyn Screen, side: Side) -> Self {
        let mut width = 0;
        let mut height = 0;
        let ptr = unsafe {
            ctru_sys::gfxGetFramebuffer(screen.as_raw(), side.into(), &mut width, &mut height)
        };
        Self {
            ptr,
            width,
            height,
            screen: PhantomData,
        }
    }
}

impl Screen for TopScreen {
    fn as_raw(&self) -> ctru_sys::gfxScreen_t {
        ctru_sys::GFX_TOP
    }
}

impl Screen for BottomScreen {
    fn as_raw(&self) -> ctru_sys::gfxScreen_t {
        ctru_sys::GFX_BOTTOM
    }
}

impl From<Side> for ctru_sys::gfx3dSide_t {
    fn from(s: Side) -> ctru_sys::gfx3dSide_t {
        use self::Side::*;
        match s {
            Left => ctru_sys::GFX_LEFT,
            Right => ctru_sys::GFX_RIGHT,
        }
    }
}

impl Drop for Gfx {
    fn drop(&mut self) {
        unsafe { ctru_sys::gfxExit() };

        GFX_ACTIVE.store(false, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gfx_duplicate() {
        // We don't need to build a `Gfx` because the test runner has one already
        match Gfx::init() {
            Err(Error::ServiceAlreadyActive("Gfx")) => return,
            _ => panic!(),
        }
    }
}
