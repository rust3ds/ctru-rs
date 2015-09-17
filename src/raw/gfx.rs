
use super::services::gsp::*;

#[inline]
pub fn RGB565(r: u32, g: u32, b: u32) {
    (((b)&0x1f)|(((g)&0x3f)<<5)|(((r)&0x1f)<<11));
}

#[inline]
pub fn RGB8_to_565(r: u32, g: u32, b: u32) {
    (((b)>>3)&0x1f)|((((g)>>2)&0x3f)<<5)|((((r)>>3)&0x1f)<<11);
}

#[repr(C)]
pub enum gfxScreen_t {
    GFX_TOP = 0,
    GFX_BOTTOM = 1
}

#[repr(C)]
pub enum gfx3dSide_t {
    GFX_LEFT = 0,
    GFX_RIGHT = 1
}


extern "C" {
    pub static mut gfxTopLeftFramebuffers: [*mut u8; 2usize];
    pub static mut gfxTopRightFramebuffers: [*mut u8; 2usize];
    pub static mut gfxBottomFramebuffers: [*mut u8; 2usize];
    pub static mut gxCmdBuf: *mut u32;

    pub fn gfxInitDefault() -> ();
    pub fn gfxInit(topFormat: GSP_FramebufferFormats, bottomFormat: GSP_FramebufferFormats, vrambuffers: u8) -> ();
    pub fn gfxExit() -> ();
    pub fn gfxSet3D(enable: u8) -> ();
    pub fn gfxSetScreenFormat(screen: gfxScreen_t, format: GSP_FramebufferFormats) -> ();
    pub fn gfxGetScreenFormat(screen: gfxScreen_t) -> GSP_FramebufferFormats;
    pub fn gfxSetDoubleBuffering(screen: gfxScreen_t, doubleBuffering: u8) -> ();
    pub fn gfxFlushBuffers() -> ();
    pub fn gfxSwapBuffers() -> ();
    pub fn gfxSwapBuffersGpu() -> ();
    pub fn gfxGetFramebuffer(screen: gfxScreen_t, side: gfx3dSide_t, width: *mut u16, height: *mut u16) -> *mut u8;
}
