use ::Result;

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
	GX_TRANSFER_FMT_RGBA4  = 4,
}

#[repr(C)]
pub enum GX_TRANSFER_SCALE
{
	GX_TRANSFER_SCALE_NO = 0,
	GX_TRANSFER_SCALE_X  = 1,
	GX_TRANSFER_SCALE_Y  = 2,
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
pub fn GX_TRANSFER_FLIP_VERT(x: i32) {
    ((x)<<0);
}

#[inline]
pub fn GX_TRANSFER_OUT_TILED(x: i32) {
    ((x)<<1);
}

#[inline]
pub fn GX_TRANSFER_RAW_COPY(x: i32) {
    ((x)<<3);
}

#[inline]
pub fn GX_TRANSFER_IN_FORMAT(x: i32)  {
    ((x)<<8);
}

#[inline]
pub fn GX_TRANSFER_OUT_FORMAT(x: i32) {
    ((x)<<12);
}

#[inline]
pub fn GX_TRANSFER_SCALING(x: i32) {
    ((x)<<24);
}

#[inline]
pub fn GX_CMDLIST_BIT0() {
    (1u32<<(0));
}

#[inline]
pub fn GX_CMNDLIST_FLUSH() {
    (1u32<<(1));
}

extern "C" {
    pub static mut gxCmdBuf: *mut u32;

    pub fn GX_RequestDma(src: *mut u32, dst: *mut u32, length: u32)
     -> Result;
    pub fn GX_ProcessCommandList(buf0a: *mut u32, buf0s: u32, flags: u8)
     -> Result;
    pub fn GX_MemoryFill(buf0a: *mut u32, buf0v: u32, buf0e: *mut u32,
                         control0: u16, buf1a: *mut u32, buf1v: u32,
                         buf1e: *mut u32, control1: u16) -> Result;
    pub fn GX_DisplayTransfer(inadr: *mut u32, indim: u32,
                              outadr: *mut u32, outdim: u32, flags: u32)
     -> Result;
    pub fn GX_TextureCopy(inadr: *mut u32, indim: u32, outadr: *mut u32,
                          outdim: u32, size: u32, flags: u32) -> Result;
    pub fn GX_FlushCacheRegions(buf0a: *mut u32, buf0s: u32,
                                buf1a: *mut u32, buf1s: u32,
                                buf2a: *mut u32, buf2s: u32) -> Result;
}

