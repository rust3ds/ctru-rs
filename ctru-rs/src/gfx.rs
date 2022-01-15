//! LCD screens manipulation helper

use std::default::Default;
use std::ops::Drop;

use crate::services::gspgpu::{self, FramebufferFormat};

/// A handle to libctru's gfx module. This module is a wrapper around the GSPGPU service that
/// provides helper functions and utilities for software rendering.
///
/// The service exits when this struct is dropped.
pub struct Gfx(());

/// Available screens on the 3DS
#[derive(Copy, Clone, Debug)]
pub enum Screen {
    /// The top screen
    Top,
    /// The bottom screen
    Bottom,
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
        Gfx(())
    }

    /// Enable or disable the 3D stereoscopic effect
    pub fn set_3d_enabled(&mut self, enabled: bool) {
        unsafe { ctru_sys::gfxSet3D(enabled) }
    }

    /// Enable or disable the wide screen mode (top screen).
    ///
    /// This only works when 3D is disabled.
    pub fn set_wide_mode(&mut self, enabled: bool) {
        unsafe { ctru_sys::gfxSetWide(enabled) };
    }

    /// Get the status of wide screen mode.
    pub fn get_wide_mode(&self) -> bool {
        unsafe { ctru_sys::gfxIsWide() }
    }

    /// Sets whether to use double buffering. Enabled by default.
    ///
    /// Note that even when double buffering is disabled, one should still use the `swap_buffers`
    /// method on each frame to keep the gsp configuration up to date
    pub fn set_double_buffering(&mut self, screen: Screen, enabled: bool) {
        unsafe { ctru_sys::gfxSetDoubleBuffering(screen.into(), enabled) }
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

    /// Gets the framebuffer format for a screen
    pub fn get_framebuffer_format(&self, screen: Screen) -> FramebufferFormat {
        unsafe { ctru_sys::gfxGetScreenFormat(screen.into()).into() }
    }

    /// Change the framebuffer format for a screen
    pub fn set_framebuffer_format(&mut self, screen: Screen, fmt: FramebufferFormat) {
        unsafe { ctru_sys::gfxSetScreenFormat(screen.into(), fmt.into()) }
    }

    /// Returns a tuple containing a pointer to the specifified framebuffer (as determined by the
    /// provided `Screen` and `Side`), the width of the framebuffer in pixels, and the height of
    /// the framebuffer in pixels
    ///
    /// Note that the pointer returned by this function can change after each call to this function
    /// if double buffering is enabled
    pub unsafe fn get_raw_framebuffer(&self, screen: Screen, side: Side) -> (*mut u8, u16, u16) {
        let mut width: u16 = 0;
        let mut height: u16 = 0;
        let buf: *mut u8 =
            ctru_sys::gfxGetFramebuffer(screen.into(), side.into(), &mut width, &mut height);
        (buf, width, height)
    }
}

impl From<Screen> for ctru_sys::gfxScreen_t {
    fn from(g: Screen) -> ctru_sys::gfxScreen_t {
        use self::Screen::*;
        match g {
            Top => ctru_sys::GFX_TOP,
            Bottom => ctru_sys::GFX_BOTTOM,
        }
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
        Gfx(())
    }
}

impl Drop for Gfx {
    fn drop(&mut self) {
        unsafe { ctru_sys::gfxExit() };
    }
}
