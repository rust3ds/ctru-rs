//! Graphics service.
//!
//! The GFX service controls (in a somewhat high-level way) the console's LCD screens.
//! The screens are subordinate to the GFX service handle and can be used by only one borrower at a time.
#![doc(alias = "graphics")]

use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::Mutex;

use crate::error::Result;
use crate::sealed::Sealed;
use crate::services::ServiceReference;
use crate::services::gspgpu::{self, FramebufferFormat};

/// Trait to handle common functionality for all screens.
///
/// This trait is implemented by the screen structs for working with frame buffers and
/// drawing to the screens. Graphics-related code can be made generic over this
/// trait to work with any of the given screens.
#[doc(alias = "gfxScreen_t")]
pub trait Screen: Sealed {
    /// Returns the `libctru` value for the Screen kind.
    fn as_raw(&self) -> ctru_sys::gfxScreen_t;

    /// Returns the Screen side (left or right).
    fn side(&self) -> Side;

    /// Returns a [`RawFrameBuffer`] for the screen (if the framebuffer was allocated on the HEAP).
    ///
    /// # Notes
    ///
    /// The pointer of the framebuffer returned by this function can change after each call
    /// to this function if double buffering is enabled, so it's suggested to NOT save it for later use.
    ///
    /// # Panics
    ///
    /// If the [`Gfx`] service was initialised via [`Gfx::with_formats_vram()`] this function will crash the program with an ARM exception.
    #[doc(alias = "gfxGetFramebuffer")]
    fn raw_framebuffer(&mut self) -> RawFrameBuffer<'_> {
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

    /// Gets the framebuffer format.
    #[doc(alias = "gfxGetScreenFormat")]
    fn framebuffer_format(&self) -> FramebufferFormat {
        unsafe { ctru_sys::gfxGetScreenFormat(self.as_raw()) }.into()
    }

    /// Change the framebuffer format.
    ///
    /// [`Swap::swap_buffers`] must be called after this method for the configuration
    /// change to take effect.
    #[doc(alias = "gfxSetScreenFormat")]
    fn set_framebuffer_format(&mut self, fmt: FramebufferFormat) {
        unsafe { ctru_sys::gfxSetScreenFormat(self.as_raw(), fmt.into()) }
    }
}

/// The top LCD screen.
///
/// Mutable access to this struct is required to write to the top screen's frame buffer.
///
/// To enable 3D mode, it can be converted into a [`TopScreen3D`].
pub struct TopScreen {
    left: TopScreenLeft,
    right: TopScreenRight,
}

/// The top LCD screen set in stereoscopic 3D mode.
///
/// A helper container for both sides of the top screen. Once the [`TopScreen`] is
/// converted into this, 3D mode will be enabled until this struct is dropped.
pub struct TopScreen3D<'screen> {
    screen: &'screen RefCell<TopScreen>,
}

/// Trait for screens that can have its frame buffers swapped, when double buffering is enabled.
///
/// This trait applies to all [`Screen`]s that have swappable frame buffers.
pub trait Swap: Sealed {
    /// Swaps the video buffers.
    ///
    /// Even if double buffering is disabled, "swapping" the buffers has the side effect
    /// of committing any configuration changes to the buffers (e.g. [`TopScreen::set_wide_mode()`],
    /// [`Screen::set_framebuffer_format()`], [`Swap::set_double_buffering()`]), so it should still be used.
    ///
    /// This should be called once per frame at most.
    #[doc(alias = "gfxScreenSwapBuffers")]
    fn swap_buffers(&mut self);

    /// Set whether to use double buffering.
    ///
    /// # Notes
    ///
    /// Double buffering is enabled by default.
    /// [`Swap::swap_buffers`] must be called after this function for the configuration
    /// change to take effect.
    #[doc(alias = "gfxSetDoubleBuffering")]
    fn set_double_buffering(&mut self, enabled: bool);
}

impl Swap for TopScreen3D<'_> {
    fn swap_buffers(&mut self) {
        unsafe {
            ctru_sys::gfxScreenSwapBuffers(ctru_sys::GFX_TOP, true);
        }
    }

    fn set_double_buffering(&mut self, enabled: bool) {
        unsafe { ctru_sys::gfxSetDoubleBuffering(ctru_sys::GFX_TOP, enabled) }
    }
}

impl Swap for TopScreen {
    fn swap_buffers(&mut self) {
        unsafe {
            ctru_sys::gfxScreenSwapBuffers(ctru_sys::GFX_TOP, false);
        }
    }

    fn set_double_buffering(&mut self, enabled: bool) {
        unsafe { ctru_sys::gfxSetDoubleBuffering(ctru_sys::GFX_TOP, enabled) }
    }
}

impl Swap for BottomScreen {
    fn swap_buffers(&mut self) {
        unsafe {
            ctru_sys::gfxScreenSwapBuffers(ctru_sys::GFX_BOTTOM, false);
        }
    }

    fn set_double_buffering(&mut self, enabled: bool) {
        unsafe { ctru_sys::gfxSetDoubleBuffering(ctru_sys::GFX_BOTTOM, enabled) }
    }
}

/// A screen with buffers that can be flushed.
///
/// This trait applies to any [`Screen`] that has data written to its frame buffer.
pub trait Flush: Sealed {
    /// Flushes the video buffer(s) for this screen.
    ///
    /// Note that you must still call [`Swap::swap_buffers`] after this method for the buffer contents to be displayed.
    #[doc(alias = "gfxFlushBuffers")]
    fn flush_buffers(&mut self);
}

impl<S: Screen> Flush for S {
    fn flush_buffers(&mut self) {
        let framebuffer = self.raw_framebuffer();

        // Flush the data array. `self.raw_framebuffer` should get the correct parameters for all kinds of screens
        let _ = unsafe {
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

/// The bottom LCD screen.
///
/// Mutable access to this struct is required to write to the bottom screen's frame buffer.
#[derive(Debug)]
#[non_exhaustive]
pub struct BottomScreen;

/// Representation of a framebuffer for one [`Side`] of the top screen, or the entire bottom screen.
///
/// The inner pointer is only valid for one frame if double
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

/// Side of the [`TopScreen`]'s framebuffer.
///
/// The top screen of the 3DS can have two separate sets of framebuffers to support its 3D functionality
#[doc(alias = "gfx3dSide_t")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Side {
    /// The left framebuffer. This framebuffer is also the one used when 3D is disabled
    Left = ctru_sys::GFX_LEFT,
    /// The right framebuffer
    Right = ctru_sys::GFX_RIGHT,
}

/// Handle to the GFX service.
///
/// This service is a wrapper around the lower-level [GSPGPU](crate::services::gspgpu) service that
/// provides helper functions and utilities for software rendering.
pub struct Gfx {
    /// Top screen representation.
    pub top_screen: RefCell<TopScreen>,
    /// Bottom screen representation.
    pub bottom_screen: RefCell<BottomScreen>,
    _service_handler: ServiceReference,
}

pub(crate) static GFX_ACTIVE: Mutex<()> = Mutex::new(());

impl Gfx {
    /// Initialize a new default service handle.
    ///
    /// # Notes
    ///
    /// The new `Gfx` instance will allocate the needed framebuffers in the CPU-GPU shared memory region (to ensure compatibiltiy with all possible uses of the `Gfx` service).
    /// As such, it's the same as calling:
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// # use ctru::services::gfx::Gfx;
    /// # use ctru::services::gspgpu::FramebufferFormat;
    /// #
    /// Gfx::with_formats_shared(FramebufferFormat::Bgr8, FramebufferFormat::Bgr8)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Have a look at [`Gfx::with_formats_vram()`] if you aren't interested in manipulating the framebuffers using the CPU.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::gfx::Gfx;
    ///
    /// let gfx = Gfx::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "gfxInit")]
    pub fn new() -> Result<Self> {
        Gfx::with_formats_shared(FramebufferFormat::Bgr8, FramebufferFormat::Bgr8)
    }

    /// Initialize a new service handle with the chosen framebuffer formats on the HEAP for the top and bottom screens.
    ///
    /// Use [`Gfx::new()`] instead of this function to initialize the module with default parameters
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::gfx::Gfx;
    /// use ctru::services::gspgpu::FramebufferFormat;
    ///
    /// // Top screen uses RGBA8, bottom screen uses RGB565.
    /// // The screen buffers are allocated in the standard HEAP memory, and not in VRAM.
    /// let gfx = Gfx::with_formats_shared(FramebufferFormat::Rgba8, FramebufferFormat::Rgb565)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "gfxInit")]
    pub fn with_formats_shared(
        top_fb_fmt: FramebufferFormat,
        bottom_fb_fmt: FramebufferFormat,
    ) -> Result<Self> {
        Self::with_configuration(top_fb_fmt, bottom_fb_fmt, false)
    }

    /// Initialize a new service handle with the chosen framebuffer formats on the VRAM for the top and bottom screens.
    ///
    /// # Notes
    ///
    /// Though unsafe to do so, it's suggested to use VRAM buffers when working exclusively with the GPU,
    /// since they result in faster performance and less memory waste.
    ///
    /// # Safety
    ///
    /// By initializing the [`Gfx`] service as such, all functionality that relies on CPU manipulation of the framebuffers will
    /// be completely unavailable (usually resulting in an ARM panic if wrongly used).
    ///
    /// Usage of functionality such as [`Console`](crate::console::Console) and [`Screen::raw_framebuffer()`] will result in ARM exceptions.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::{gfx::Gfx, gspgpu::FramebufferFormat};
    ///
    /// // Top screen uses RGBA8, bottom screen uses RGB565.
    /// // The screen buffers are allocated in the in VRAM, so they will NOT be accessible from the CPU.
    /// let gfx = unsafe { Gfx::with_formats_vram(FramebufferFormat::Rgba8, FramebufferFormat::Rgb565)? };
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "gfxInit")]
    pub unsafe fn with_formats_vram(
        top_fb_fmt: FramebufferFormat,
        bottom_fb_fmt: FramebufferFormat,
    ) -> Result<Self> {
        Self::with_configuration(top_fb_fmt, bottom_fb_fmt, true)
    }

    // Internal function to handle the initialization of `Gfx`.
    fn with_configuration(
        top_fb_fmt: FramebufferFormat,
        bottom_fb_fmt: FramebufferFormat,
        vram_buffer: bool,
    ) -> Result<Self> {
        let handler = ServiceReference::new(
            &GFX_ACTIVE,
            || unsafe {
                ctru_sys::gfxInit(top_fb_fmt.into(), bottom_fb_fmt.into(), vram_buffer);

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

    /// Waits for the vertical blank event.
    ///
    /// Use this to synchronize your application with the refresh rate of the LCD screens
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::apt::Apt;
    /// use ctru::services::gfx::Gfx;
    /// let apt = Apt::new()?;
    /// let gfx = Gfx::new()?;
    ///
    /// // Simple main loop.
    /// while apt.main_loop() {
    ///     // Main program logic
    ///
    ///     // Wait for the screens to refresh.
    ///     // This blocks the current thread to make it run at 60Hz.
    ///     gfx.wait_for_vblank();
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn wait_for_vblank(&self) {
        gspgpu::wait_for_event(gspgpu::Event::VBlank0, true);
    }
}

impl TopScreen3D<'_> {
    /// Immutably borrow the two sides of the screen as `(left, right)`.
    pub fn split(&self) -> (Ref<'_, TopScreenLeft>, Ref<'_, TopScreenRight>) {
        Ref::map_split(self.screen.borrow(), |screen| (&screen.left, &screen.right))
    }

    /// Mutably borrow the two sides of the screen as `(left, right)`.
    pub fn split_mut(&self) -> (RefMut<'_, TopScreenLeft>, RefMut<'_, TopScreenRight>) {
        RefMut::map_split(self.screen.borrow_mut(), |screen| {
            (&mut screen.left, &mut screen.right)
        })
    }
}

/// Convert the [`TopScreen`] into a [`TopScreen3D`] and activate stereoscopic 3D.
///
/// # Example
///
/// ```
/// # let _runner = test_runner::GdbRunner::default();
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// #
/// use ctru::services::gfx::{Gfx, TopScreen, TopScreen3D};
/// let gfx = Gfx::new()?;
///
/// let mut top_screen = TopScreen3D::from(&gfx.top_screen);
///
/// let (left, right) = top_screen.split_mut();
///
/// // Rendering must be done twice for each side
/// // (with a slight variation in perspective to simulate the eye-to-eye distance).
/// render(left);
/// render(right);
/// #
/// # Ok(())
/// # }
/// #
/// # use ctru::services::gfx::Screen;
/// # use std::cell::RefMut;
/// # fn render(screen: RefMut<'_, dyn Screen>) {}
/// ```
impl<'screen> From<&'screen RefCell<TopScreen>> for TopScreen3D<'screen> {
    #[doc(alias = "gfxSet3D")]
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
    /// # Notes
    ///
    /// [`Swap::swap_buffers`] must be called after this method for the configuration
    /// to take effect.
    ///
    /// Wide mode does NOT work on Old 2DS models (but still does on New 2DS XL models).
    #[doc(alias = "gfxSetWide")]
    pub fn set_wide_mode(&mut self, enable: bool) {
        unsafe {
            ctru_sys::gfxSetWide(enable);
        }
    }

    /// Returns whether or not wide mode is enabled on the top screen.
    #[doc(alias = "gfxIsWide")]
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
        // NOTE: this is expected to fail if using the console test runner, since
        // that necessarily creates a Gfx as part of its test setup:
        let _gfx = Gfx::new().unwrap();

        assert!(matches!(Gfx::new(), Err(Error::ServiceAlreadyActive)));
    }
}
