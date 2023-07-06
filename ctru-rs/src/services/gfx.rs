//! LCD screens manipulation helper

use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::Mutex;

use crate::error::Result;
use crate::services::gspgpu::{self, FramebufferFormat};
use crate::services::ServiceReference;

mod private {
    use super::{BottomScreen, TopScreen, TopScreen3D, TopScreenLeft, TopScreenRight};

    pub trait Sealed {}

    impl Sealed for TopScreen {}
    impl Sealed for TopScreen3D<'_> {}
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
        let mut width: u16 = 0;
        let mut height: u16 = 0;
        let ptr = unsafe {
            ctru_sys::gfxGetFramebuffer(self.as_raw(), self.side().into(), &mut width, &mut height)
        };
        RawFrameBuffer {
            ptr,
            width: width.into(),
            height: height.into(),
            screen: PhantomData,
        }
    }

    /// Sets whether to use double buffering. Enabled by default.
    ///
    /// [`Swap::swap_buffers`] must be called after this function for the configuration
    /// change to take effect.
    fn set_double_buffering(&mut self, enabled: bool) {
        unsafe { ctru_sys::gfxSetDoubleBuffering(self.as_raw(), enabled) }
    }

    /// Gets the framebuffer format.
    fn framebuffer_format(&self) -> FramebufferFormat {
        unsafe { ctru_sys::gfxGetScreenFormat(self.as_raw()) }.into()
    }

    /// Change the framebuffer format.
    ///
    /// [`Swap::swap_buffers`] must be called after this method for the configuration
    /// change to take effect.
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
pub struct TopScreen3D<'screen> {
    screen: &'screen RefCell<TopScreen>,
}

/// A screen that can have its frame buffers swapped, if double buffering is enabled.
///
/// This trait applies to all [`Screen`]s that have swappable frame buffers.
pub trait Swap: private::Sealed {
    /// Swaps the video buffers.
    ///
    /// If double buffering is disabled, "swapping" the buffers has the side effect
    /// of committing any configuration changes to the buffers (e.g. [`TopScreen::set_wide_mode`],
    /// [`Screen::set_framebuffer_format`], [`Screen::set_double_buffering`]).
    ///
    /// This should be called once per frame at most.
    fn swap_buffers(&mut self);
}

impl Swap for TopScreen3D<'_> {
    fn swap_buffers(&mut self) {
        unsafe {
            ctru_sys::gfxScreenSwapBuffers(ctru_sys::GFX_TOP, true);
        }
    }
}

impl Swap for TopScreen {
    fn swap_buffers(&mut self) {
        unsafe {
            ctru_sys::gfxScreenSwapBuffers(ctru_sys::GFX_TOP, false);
        }
    }
}

impl Swap for BottomScreen {
    fn swap_buffers(&mut self) {
        unsafe {
            ctru_sys::gfxScreenSwapBuffers(ctru_sys::GFX_BOTTOM, false);
        }
    }
}

/// A screen with buffers that can be flushed. This trait applies to any [`Screen`]
/// that has data written to its frame buffer.
pub trait Flush: private::Sealed {
    /// Flushes the video buffer(s) for this screen. Note that you must still call
    /// [`Swap::swap_buffers`] after this method for the buffer contents to be displayed.
    fn flush_buffers(&mut self);
}

impl<S: Screen> Flush for S {
    fn flush_buffers(&mut self) {
        let framebuffer = self.raw_framebuffer();

        // Flush the data array. `self.raw_framebuffer` should get the correct parameters for all kinds of screens
        unsafe {
            ctru_sys::GSPGPU_FlushDataCache(
                framebuffer.ptr.cast(),
                (framebuffer.height * framebuffer.width) as u32,
            )
        };
    }
}

impl Flush for TopScreen3D<'_> {
    /// Unlike most other implementations of [`Flush`], this flushes the buffers for both
    /// the left and right sides of the top screen.
    fn flush_buffers(&mut self) {
        let (mut left, mut right) = self.split_mut();
        left.flush_buffers();
        right.flush_buffers();
    }
}

/// The left side of the top screen, when using 3D mode.
#[derive(Debug)]
#[non_exhaustive]
pub struct TopScreenLeft;

/// The right side of the top screen, when using 3D mode.
#[derive(Debug)]
#[non_exhaustive]
pub struct TopScreenRight;

/// The bottom screen. Mutable access to this struct is required to write to the
/// bottom screen's frame buffer.
#[derive(Debug)]
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
    pub width: usize,
    /// The height of the framebuffer in pixels.
    pub height: usize,
    /// Keep a mutable reference to the Screen for which this framebuffer is tied.
    screen: PhantomData<&'screen mut dyn Screen>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    /// Creates a new [`Gfx`] instance with default init values
    /// It's the same as calling:
    ///
    /// ```
    /// Gfx::with_formats(FramebufferFormat::Bgr8, FramebufferFormat::Bgr8, false)
    /// ```
    pub fn new() -> Result<Self> {
        Gfx::with_formats(FramebufferFormat::Bgr8, FramebufferFormat::Bgr8, false)
    }

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

    /// Waits for the vertical blank interrupt
    ///
    /// Use this to synchronize your application with the refresh rate of the LCD screens
    pub fn wait_for_vblank(&self) {
        gspgpu::wait_for_event(gspgpu::Event::VBlank0, true);
    }
}

impl TopScreen3D<'_> {
    /// Immutably borrow the two sides of the screen as `(left, right)`.
    pub fn split(&self) -> (Ref<TopScreenLeft>, Ref<TopScreenRight>) {
        Ref::map_split(self.screen.borrow(), |screen| (&screen.left, &screen.right))
    }

    /// Mutably borrow the two sides of the screen as `(left, right)`.
    pub fn split_mut(&self) -> (RefMut<TopScreenLeft>, RefMut<TopScreenRight>) {
        RefMut::map_split(self.screen.borrow_mut(), |screen| {
            (&mut screen.left, &mut screen.right)
        })
    }
}

impl<'screen> From<&'screen RefCell<TopScreen>> for TopScreen3D<'screen> {
    fn from(top_screen: &'screen RefCell<TopScreen>) -> Self {
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
    ///
    /// [`Swap::swap_buffers`] must be called after this method for the configuration
    /// to take effect.
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

// When 3D mode is disabled, only the left side is used, so this Screen impl
// just forwards everything to the TopScreenLeft.
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
