//! LCD screens manipulation helper

use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::Mutex;

use crate::error::Result;
use crate::services::gspgpu::{self, FramebufferFormat};
use crate::services::ServiceReference;

mod private {
    use super::{BottomScreen, TopScreen, TopScreenLeft, TopScreenRight};

    pub trait Sealed {}

    impl Sealed for TopScreen {}
    impl Sealed for TopScreenLeft {}
    impl Sealed for TopScreenRight {}
    impl Sealed for BottomScreen {}
}

/// This trait is implemented by the screen structs for working with frame buffers and
/// drawing to the screens. Graphics-related code can be made generic over this
/// trait to work with any of the given screens.
pub trait Screen: private::Sealed {
    /// Returns the `libctru` value for the Screen kind.
    fn as_raw(&self) -> ctru_sys::gfxScreen_t;

    /// Returns the Screen side (left or right).
    fn side(&self) -> Side;

    /// Returns a [`RawFrameBuffer`] for the screen.
    ///
    /// Note that the pointer of the framebuffer returned by this function can
    /// change after each call to this function if double buffering is enabled.
    fn raw_framebuffer(&mut self) -> RawFrameBuffer {
        let mut width = 0;
        let mut height = 0;
        let ptr = unsafe {
            ctru_sys::gfxGetFramebuffer(self.as_raw(), self.side().into(), &mut width, &mut height)
        };
        RawFrameBuffer {
            ptr,
            width,
            height,
            screen: PhantomData,
        }
    }

    /// Sets whether to use double buffering. Enabled by default.
    ///
    /// Note that even when double buffering is disabled, one should still use the `swap_buffers`
    /// method on each frame to keep the gsp configuration up to date
    fn set_double_buffering(&mut self, enabled: bool) {
        unsafe { ctru_sys::gfxSetDoubleBuffering(self.as_raw(), enabled) }
    }

    /// Gets the framebuffer format
    fn framebuffer_format(&self) -> FramebufferFormat {
        unsafe { ctru_sys::gfxGetScreenFormat(self.as_raw()) }.into()
    }

    /// Change the framebuffer format
    fn set_framebuffer_format(&mut self, fmt: FramebufferFormat) {
        unsafe { ctru_sys::gfxSetScreenFormat(self.as_raw(), fmt.into()) }
    }
}

/// The top screen. Mutable access to this struct is required to write to the top
/// screen's frame buffer. To enable 3D mode, it can be converted into a [`TopScreen3D`].
pub struct TopScreen {
    left: TopScreenLeft,
    right: TopScreenRight,
}

/// A helper container for both sides of the top screen. Once the [`TopScreen`] is
/// converted into this, 3D mode will be enabled until this struct is dropped.
pub struct TopScreen3D<'top_screen> {
    screen: &'top_screen RefCell<TopScreen>,
}

struct TopScreenLeft;

struct TopScreenRight;

#[non_exhaustive]
/// The bottom screen. Mutable access to this struct is required to write to the
/// bottom screen's frame buffer.
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
#[repr(u32)]
/// Side of top screen framebuffer
///
/// The top screen of the 3DS can have two separate sets of framebuffers to support its 3D functionality
pub enum Side {
    /// The left framebuffer. This framebuffer is also the one used when 3D is disabled
    Left = ctru_sys::GFX_LEFT,
    /// The right framebuffer
    Right = ctru_sys::GFX_RIGHT,
}

/// A handle to libctru's gfx module. This module is a wrapper around the GSPGPU service that
/// provides helper functions and utilities for software rendering.
///
/// The service exits when this struct is dropped.
pub struct Gfx {
    pub top_screen: RefCell<TopScreen>,
    pub bottom_screen: RefCell<BottomScreen>,
    _service_handler: ServiceReference,
}

static GFX_ACTIVE: Mutex<usize> = Mutex::new(0);

impl Gfx {
    /// Initialize the Gfx module with the chosen framebuffer formats for the top and bottom
    /// screens
    ///
    /// Use `Gfx::new()` instead of this function to initialize the module with default parameters
    pub fn with_formats(
        top_fb_fmt: FramebufferFormat,
        bottom_fb_fmt: FramebufferFormat,
        use_vram_buffers: bool,
    ) -> Result<Self> {
        let handler = ServiceReference::new(
            &GFX_ACTIVE,
            false,
            || unsafe {
                ctru_sys::gfxInit(top_fb_fmt.into(), bottom_fb_fmt.into(), use_vram_buffers);

                Ok(())
            },
            || unsafe { ctru_sys::gfxExit() },
        )?;

        Ok(Self {
            top_screen: RefCell::new(TopScreen::new()),
            bottom_screen: RefCell::new(BottomScreen),
            _service_handler: handler,
        })
    }

    /// Creates a new [Gfx] instance with default init values
    /// It's the same as calling:
    /// `Gfx::with_formats(FramebufferFormat::Bgr8, FramebufferFormat::Bgr8, false)`
    pub fn new() -> Result<Self> {
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

impl TopScreen3D<'_> {
    /// Immutably borrow the two sides of the screen as `(left, right)`.
    pub fn split(&self) -> (Ref<dyn Screen>, Ref<dyn Screen>) {
        Ref::map_split(self.screen.borrow(), |screen| {
            (&screen.left as _, &screen.right as _)
        })
    }

    /// Mutably borrow the two sides of the screen as `(left, right)`.
    pub fn split_mut(&self) -> (RefMut<dyn Screen>, RefMut<dyn Screen>) {
        RefMut::map_split(self.screen.borrow_mut(), |screen| {
            (&mut screen.left as _, &mut screen.right as _)
        })
    }
}

impl<'top_screen> From<&'top_screen RefCell<TopScreen>> for TopScreen3D<'top_screen> {
    fn from(top_screen: &'top_screen RefCell<TopScreen>) -> Self {
        unsafe {
            ctru_sys::gfxSet3D(true);
        }

        TopScreen3D { screen: top_screen }
    }
}

impl Drop for TopScreen3D<'_> {
    fn drop(&mut self) {
        unsafe {
            ctru_sys::gfxSet3D(false);
        }
    }
}

impl TopScreen {
    fn new() -> Self {
        Self {
            left: TopScreenLeft,
            right: TopScreenRight,
        }
    }

    /// Enable or disable wide mode on the top screen.
    pub fn set_wide_mode(&mut self, enable: bool) {
        unsafe {
            ctru_sys::gfxSetWide(enable);
        }
    }

    /// Returns whether or not wide mode is enabled on the top screen.
    pub fn is_wide(&self) -> bool {
        unsafe { ctru_sys::gfxIsWide() }
    }
}

impl Screen for TopScreen {
    fn as_raw(&self) -> ctru_sys::gfxScreen_t {
        self.left.as_raw()
    }

    fn side(&self) -> Side {
        self.left.side()
    }
}

impl Screen for TopScreenLeft {
    fn as_raw(&self) -> ctru_sys::gfxScreen_t {
        ctru_sys::GFX_TOP
    }

    fn side(&self) -> Side {
        Side::Left
    }
}

impl Screen for TopScreenRight {
    fn as_raw(&self) -> ctru_sys::gfxScreen_t {
        ctru_sys::GFX_TOP
    }

    fn side(&self) -> Side {
        Side::Right
    }
}

impl Screen for BottomScreen {
    fn as_raw(&self) -> ctru_sys::gfxScreen_t {
        ctru_sys::GFX_BOTTOM
    }

    fn side(&self) -> Side {
        Side::Left
    }
}

from_impl!(Side, ctru_sys::gfx3dSide_t);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn gfx_duplicate() {
        // We don't need to build a `Gfx` because the test runner has one already
        assert!(matches!(Gfx::new(), Err(Error::ServiceAlreadyActive)));
    }
}
