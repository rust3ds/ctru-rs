//! LCD screens manipulation helper

use once_cell::sync::Lazy;
use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::Mutex;

use crate::error::Result;
use crate::services::gspgpu::{self, FramebufferFormat};
use crate::services::ServiceReference;

mod private {
    use super::{BottomScreen, TopScreen, TopScreenRight};

    pub trait Sealed {}

    impl Sealed for TopScreen {}
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
    fn get_raw_framebuffer(&mut self) -> RawFrameBuffer {
        let side = self.side();
        RawFrameBuffer::for_screen_side(self, side)
    }

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
/// The top screen. Mutable access to this struct is required to write to the top
/// screen's frame buffer. To enable 3D mode, it can be converted into a [`TopScreen3D`].
pub struct TopScreen;

/// A helper container for both sides of the top screen. Once the [`TopScreen`] is
/// converted into this, 3D mode will be enabled until this struct is dropped.
pub struct TopScreen3D<'top_screen> {
    // morally, this should be &mut or RefMut, but if we do
    // - &mut: it means gfx can no longer be borrowed immutably while this exists
    // - RefMut: we don't have an easy way to obtain Ref<dyn Screen> for the left side.
    //      maybe this one isn't as important since the use case is typically RefMut anyway.
    //      we could just return &dyn Screen instead of Ref<dyn Screen> ?
    left: &'top_screen RefCell<TopScreen>,
    right: RefCell<TopScreenRight>,
}

// TODO: it feels a little weird to have an asymmetric separate type like this,
// but maybe if it's not `pub` it's not as weird...
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
    _service_handler: ServiceReference,
}

static GFX_ACTIVE: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

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
        let _service_handler = ServiceReference::new(
            &GFX_ACTIVE,
            false,
            || unsafe {
                ctru_sys::gfxInit(top_fb_fmt.into(), bottom_fb_fmt.into(), use_vram_buffers);

                Ok(())
            },
            || unsafe { ctru_sys::gfxExit() },
        )?;

        Ok(Self {
            top_screen: RefCell::new(TopScreen),
            bottom_screen: RefCell::new(BottomScreen),
            _service_handler,
        })
    }

    /// Creates a new [Gfx] instance with default init values
    /// It's the same as calling:
    /// `Gfx::with_formats(FramebufferFormat::Bgr8, FramebufferFormat::Bgr8, false)`
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

impl<'screen> RawFrameBuffer<'screen> {
    fn for_screen_side(screen: &'screen mut (impl Screen + ?Sized), side: Side) -> Self {
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

impl TopScreen3D<'_> {
    /// Immutably borrow the left side of the screen.
    pub fn left(&self) -> Ref<dyn Screen> {
        self.left.borrow()
    }

    /// Mutably borrow the left side of the screen.
    pub fn left_mut(&self) -> RefMut<dyn Screen> {
        self.left.borrow_mut()
    }

    /// Immutably borrow the right side of the screen.
    pub fn right(&self) -> Ref<dyn Screen> {
        self.right.borrow()
    }

    /// Mutably borrow the right side of the screen.
    pub fn right_mut(&self) -> RefMut<dyn Screen> {
        self.right.borrow_mut()
    }
}

impl<'a> From<&'a RefCell<TopScreen>> for TopScreen3D<'a> {
    fn from(top_screen: &'a RefCell<TopScreen>) -> Self {
        unsafe {
            ctru_sys::gfxSet3D(true);
        }
        TopScreen3D {
            left: top_screen,
            right: RefCell::new(TopScreenRight),
        }
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
    /// Enable or disable wide mode on the top screen.
    pub fn set_wide_mode(&mut self, enable: bool) {
        unsafe {
            ctru_sys::gfxSetWide(enable);
        }
    }

    /// Returns whether or not wide mode is enabled on the top screen.
    pub fn get_wide_mode(&self) -> bool {
        unsafe { ctru_sys::gfxIsWide() }
    }
}

impl Screen for TopScreen {
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

impl From<Side> for ctru_sys::gfx3dSide_t {
    fn from(s: Side) -> ctru_sys::gfx3dSide_t {
        use self::Side::*;
        match s {
            Left => ctru_sys::GFX_LEFT,
            Right => ctru_sys::GFX_RIGHT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn gfx_duplicate() {
        // We don't need to build a `Gfx` because the test runner has one already
        assert!(matches!(Gfx::init(), Err(Error::ServiceAlreadyActive)))
    }
}
