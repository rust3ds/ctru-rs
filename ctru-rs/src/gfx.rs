//! LCD screens manipulation helper

use std::cell::RefCell;
use std::default::Default;
use std::ops::Drop;

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

    /// Returns a tuple containing a pointer to the specifified framebuffer (as determined by the
    /// calling screen and `Side`), the width of the framebuffer in pixels, and the height of
    /// the framebuffer in pixels
    ///
    /// Note that the pointer returned by this function can change after each call to this function
    /// if double buffering is enabled
    fn get_raw_framebuffer(&self, side: Side) -> (*mut u8, u16, u16) {
        let mut width: u16 = 0;
        let mut height: u16 = 0;
        unsafe {
            let buf: *mut u8 =
                ctru_sys::gfxGetFramebuffer(self.as_raw(), side.into(), &mut width, &mut height);
            (buf, width, height)
        }
    }
}

pub struct TopScreen {
    _private: (),
}
pub struct BottomScreen {
    _private: (),
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
pub struct Gfx {
    pub top_screen: RefCell<TopScreen>,
    pub bottom_screen: RefCell<BottomScreen>,
    _private: ()
}

impl Gfx {
    /// Initialize the Gfx module with the chosen framebuffer formats for the top and bottom
    /// screens
    ///
    /// Use `Gfx::default()` instead of this function to initialize the module with default parameters
    pub fn new(
        top_fb_fmt: FramebufferFormat,
        bottom_fb_fmt: FramebufferFormat,
        use_vram_buffers: bool,
    ) -> Self {
        unsafe {
            ctru_sys::gfxInit(top_fb_fmt.into(), bottom_fb_fmt.into(), use_vram_buffers);
        }
        Gfx {
            top_screen: RefCell::new(TopScreen { _private: () }),
            bottom_screen: RefCell::new(BottomScreen { _private: () }),
        }
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

impl Default for Gfx {
    fn default() -> Self {
        unsafe { ctru_sys::gfxInitDefault() };
        Gfx {
            top_screen: RefCell::new(TopScreen { _private: () }),
            bottom_screen: RefCell::new(BottomScreen { _private: () }),
        }
    }
}

impl Drop for Gfx {
    fn drop(&mut self) {
        unsafe { ctru_sys::gfxExit() };
    }
}
