use std::default::Default;
use std::ops::Drop;

use services::gspgpu::FramebufferFormat;

pub struct Gfx(());

#[derive(Copy, Clone)]
pub enum Screen {
    Top,
    Bottom,
}

#[derive(Copy, Clone)]
pub enum Side {
    Left,
    Right,
}

impl From<::libctru::gfxScreen_t> for Screen {
    fn from(g: ::libctru::gfxScreen_t) -> Screen {
        use self::Screen::*;
        match g {
            ::libctru::GFX_TOP => Top,
            ::libctru::GFX_BOTTOM => Bottom,
            _ => unreachable!(),
        }
    }
}

impl From<Screen> for ::libctru::gfxScreen_t {
    fn from(g: Screen) -> ::libctru::gfxScreen_t {
        use self::Screen::*;
        match g {
            Top => ::libctru::GFX_TOP,
            Bottom => ::libctru::GFX_BOTTOM,
        }
    }
}

impl From<::libctru::gfx3dSide_t> for Side {
    fn from(s: ::libctru::gfx3dSide_t) -> Side {
        use self::Side::*;
        match s {
            ::libctru::GFX_LEFT => Left,
            ::libctru::GFX_RIGHT => Right,
            _ => unreachable!(),
        }
    }
}

impl From<Side> for ::libctru::gfx3dSide_t {
    fn from(s: Side) -> ::libctru::gfx3dSide_t {
        use self::Side::*;
        match s {
            Left => ::libctru::GFX_LEFT,
            Right => ::libctru::GFX_RIGHT,
        }
    }
}

impl Gfx {
    pub fn set_3d_enabled(&mut self, enabled: bool) {
        unsafe {
            ::libctru::gfxSet3D(enabled)
        }
    }

    pub fn get_framebuffer(&mut self, screen: Screen, side: Side) -> (&'static mut [u8], u16, u16) {
        use std::convert::Into;
        unsafe {
            use std::slice::from_raw_parts_mut;

            let mut w: u16 = 0;
            let mut h: u16 = 0;
            let buf: *mut u8 = ::libctru::gfxGetFramebuffer(screen.into(),
                                                      side.into(),
                                                      &mut w as *mut u16,
                                                      &mut h as &mut u16);

            let fbfmt = self.get_framebuffer_format(screen);

            (from_raw_parts_mut(buf, (w as usize * h as usize) * fbfmt.pixel_depth_bytes()), w, h)
        }
    }

    pub fn flush_buffers(&mut self) {
        unsafe { ::libctru::gfxFlushBuffers() };
    }

    pub fn swap_buffers(&mut self) {
        unsafe { ::libctru::gfxSwapBuffers() };
    }

    pub fn swap_buffers_gpu(&mut self) {
        unsafe { ::libctru::gfxSwapBuffersGpu() };
    }

    pub fn get_framebuffer_format(&self, screen: Screen) -> FramebufferFormat {
        use std::convert::Into;
        unsafe { ::libctru::gfxGetScreenFormat(screen.into()).into() }
    }

    pub fn set_framebuffer_format(&mut self, screen: Screen,
                                             fmt: FramebufferFormat) {
        use std::convert::Into;
        unsafe { ::libctru::gfxSetScreenFormat(screen.into(), fmt.into()) }
    }

    pub fn set_double_buffering(&mut self, screen: Screen, enabled: bool) {
        unsafe {
            ::libctru::gfxSetDoubleBuffering(screen.into(), enabled)
        }
    }
}

impl Default for Gfx {
    fn default() -> Self {
        unsafe { ::libctru::gfxInitDefault() };
        Gfx(())
    }
}

impl Drop for Gfx {
    fn drop(&mut self) {
        unsafe { ::libctru::gfxExit() };
    }
}
