use ::raw::gfx;

use core::default::Default;
use core::marker::PhantomData;
use core::ops::Drop;

use ::services::gsp::FramebufferFormat;

pub struct Gfx {
    // we do this to prevent people from making a Gfx struct manually
    pd: PhantomData<i32>
}

#[derive(Copy, Clone)]
pub enum Screen {
    Top,
    Bottom
}

#[derive(Copy, Clone)]
pub enum Side {
    Left,
    Right
}

impl From<gfx::gfxScreen_t> for Screen {
    #[inline] fn from(g: gfx::gfxScreen_t) -> Screen {
        use ::raw::gfx::gfxScreen_t::*;
        use self::Screen::*;
        match g {
            GFX_TOP => Top,
            GFX_BOTTOM => Bottom
        }
    }
}

impl From<Screen> for gfx::gfxScreen_t {
    #[inline] fn from(g: Screen) -> gfx::gfxScreen_t {
        use ::raw::gfx::gfxScreen_t::*;
        use self::Screen::*;
        match g {
            Top => GFX_TOP,
            Bottom => GFX_BOTTOM
        }
    }
}

impl From<gfx::gfx3dSide_t> for Side {
    #[inline] fn from(s: gfx::gfx3dSide_t) -> Side {
        use ::raw::gfx::gfx3dSide_t::*;
        use self::Side::*;
        match s {
            GFX_LEFT => Left,
            GFX_RIGHT => Right
        }
    }
}

impl From<Side> for gfx::gfx3dSide_t {
    #[inline] fn from(s: Side) -> gfx::gfx3dSide_t {
        use ::raw::gfx::gfx3dSide_t::*;
        use self::Side::*;
        match s {
            Left => GFX_LEFT,
            Right => GFX_RIGHT
        }
    }
}

impl Gfx {
    pub fn set_3d_enabled(&mut self, enabled: bool) {
        unsafe {
            gfx::gfxSet3D(match enabled { true => 1u8, false => 0u8 });
        }
    }

    pub fn get_framebuffer(& mut self, screen: Screen, side: Side) -> (&'static mut [u8], u16, u16) {
        use core::convert::Into;
        unsafe {
            use core::slice::from_raw_parts_mut;

            let mut w: u16 = 0;
            let mut h: u16 = 0;
            let buf: *mut u8 = gfx::gfxGetFramebuffer(screen.into(), side.into(), &mut w as *mut u16, &mut h as &mut u16);

            let fbfmt = self.get_framebuffer_format(screen);

            (from_raw_parts_mut(buf, (w as usize * h as usize) * fbfmt.pixel_depth_bytes()), w, h)
        }
    }

    pub fn flush_buffers(&mut self) {
        unsafe { gfx::gfxFlushBuffers() };
    }

    pub fn swap_buffers(&mut self) {
        unsafe { gfx::gfxSwapBuffers() };
    }

    pub fn swap_buffers_gpu(&mut self) {
        unsafe { gfx::gfxSwapBuffersGpu() };
    }

    pub fn get_framebuffer_format(&self, screen: Screen) -> FramebufferFormat {
        use core::convert::Into;
        unsafe {
            gfx::gfxGetScreenFormat(screen.into()).into()
        }
    }

    pub fn set_framebuffer_format(&mut self, screen: Screen, fmt: FramebufferFormat) {
        use core::convert::Into;
        unsafe {
            gfx::gfxSetScreenFormat(screen.into(), fmt.into())
        }
    }

    pub fn set_double_buffering(&mut self, screen: Screen, enabled: bool) {
        unsafe {
            gfx::gfxSetDoubleBuffering(screen.into(), match enabled { true => 1u8, false => 0u8 })
        };
    }
}

impl Default for Gfx {
    fn default() -> Self {
        unsafe { gfx::gfxInitDefault() };
        Gfx { pd: PhantomData }
    }
}

impl Drop for Gfx {
    fn drop(&mut self) {
        unsafe { gfx::gfxExit() };
    }
}
