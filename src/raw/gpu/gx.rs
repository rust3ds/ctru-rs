#[inline]
pub fn GX_BUFFER_DIM(w: u32, h: u32) {
    (((h)<<16)|((w)&0xFFFF));
}

#[repr(C)]
pub enum GX_TRANSFER_FORMAT
{
	GX_TRANSFER_FMT_RGBA8  = 0,
	GX_TRANSFER_FMT_RGB8   = 1,
	GX_TRANSFER_FMT_RGB565 = 2,
	GX_TRANSFER_FMT_RGB5A1 = 3,
	GX_TRANSFER_FMT_RGBA4  = 4
}

#[repr(C)]
pub enum GX_TRANSFER_SCALE
{
	GX_TRANSFER_SCALE_NO = 0,
	GX_TRANSFER_SCALE_X  = 1,
	GX_TRANSFER_SCALE_Y  = 2
}

#[repr(C)]
pub enum GX_FILL_CONTROL {
	GX_FILL_TRIGGER     = 0x001,
	GX_FILL_FINISHED    = 0x002,
	GX_FILL_16BIT_DEPTH = 0x000,
	GX_FILL_24BIT_DEPTH = 0x100,
	GX_FILL_32BIT_DEPTH = 0x200,
}

#[inline]
pub fn GX_TRANSFER_FLIP_VERT(x) {
    ((x)<<0);
}

#[inline]
pub fn GX_TRANSFER_OUT_TILED(x) {
    ((x)<<1);
}

#[inline]
pub fn GX_TRANSFER_RAW_COPY(x) {
    ((x)<<3)
}

#[inline]
pub fn GX_TRANSFER_IN_FORMAT(x)  {
    ((x)<<8);
}

#[inline]
pub fn GX_TRANSFER_OUT_FORMAT(x) {
    ((x)<<12)
}

#[inline]
pub fn GX_TRANSFER_SCALING(x) {
    ((x)<<24);
}

use ctru::Result;


extern "C" {
    pub fn GX_RequestDma(gxbuf: *mut u32, src: *mut u32, dst: *mut u32, length: u32) -> Result;
    pub fn GX_SetCommandList_Last(gxbuf: *mut u32, buf0a: *mut u32, buf0s: u32, flags: u8) -> Result;
    pub fn GX_SetMemoryFill(gxbuf: *mut u32, buf0a: *mut u32, buf0v: u32, buf0e: *mut u32, width0: u16, buf1a: *mut u32, buf1v: u32, buf1e: *mut u32, width1: u16) -> Result;
    pub fn GX_SetDisplayTransfer(gxbuf: *mut u32, inadr: *mut u32, indim: u32, outadr: *mut u32, outdim: u32, flags: u32) -> Result;
    pub fn GX_SetTextureCopy(gxbuf: *mut u32, inadr: *mut u32, indim: u32, outadr: *mut u32, outdim: u32, size: u32, flags: u32) -> Result;
    pub fn GX_SetCommandList_First(gxbuf: *mut u32, buf0a: *mut u32, buf0s: u32, buf1a: *mut u32, buf1s: u32, buf2a: *mut u32, buf2s: u32) -> Result;
}
