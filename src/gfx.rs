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

#[inline] fn screen_to_raw(s: Screen) -> gfx::gfxScreen_t {
    match s {
        Screen::Top => gfx::gfxScreen_t::GFX_TOP,
        Screen::Bottom => gfx::gfxScreen_t::GFX_BOTTOM
    }
}

#[inline] fn side3d_to_raw(s: Side) -> gfx::gfx3dSide_t {
    match s {
        Side::Left => gfx::gfx3dSide_t::GFX_LEFT,
        Side::Right => gfx::gfx3dSide_t::GFX_RIGHT
    }
}

impl Gfx {
    pub fn set_3d_enabled(&mut self, enabled: bool) {
        unsafe {
            gfx::gfxSet3D(match enabled { true => 1u8, false => 0u8 });
        }
    }

    pub fn get_framebuffer(& mut self, screen: Screen, side: Side) -> (&'static mut [u8], u16, u16) {
        unsafe {
            use core::slice::from_raw_parts_mut;

            let mut w: u16 = 0;
            let mut h: u16 = 0;
            let buf: *mut u8 = gfx::gfxGetFramebuffer(screen_to_raw(screen), side3d_to_raw(side), &mut w as *mut u16, &mut h as &mut u16);

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
            gfx::gfxGetScreenFormat(screen_to_raw(screen)).into()
        }
    }

    pub fn set_framebuffer_format(&mut self, screen: Screen, fmt: FramebufferFormat) {
        use core::convert::Into;
        unsafe {
            gfx::gfxSetScreenFormat(screen_to_raw(screen), fmt.into())
        }
    }

    pub fn set_double_buffering(&mut self, screen: Screen, enabled: bool) {
        unsafe {
            gfx::gfxSetDoubleBuffering(screen_to_raw(screen), match enabled { true => 1u8, false => 0u8 })
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
