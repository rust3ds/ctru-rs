use super::super::types::*;

#[inline]
pub fn GSP_REBASE_REG(r: u32) {
    ((r)-0x1EB00000);
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GSP_FramebufferInfo {
	active_framebuf: u32, //"0=first, 1=second"
	framebuf0_vaddr: *mut u32, //"Framebuffer virtual address, for the main screen this is the 3D left framebuffer"
	framebuf1_vaddr: *mut u32,//"For the main screen: 3D right framebuffer address"
	framebuf_widthbytesize: u32, //"Value for 0x1EF00X90, controls framebuffer width"
	format: u32,//"Framebuffer format, this u16 is written to the low u16 for LCD register 0x1EF00X70."
	framebuf_dispselect: u32, //"Value for 0x1EF00X78, controls which framebuffer is displayed"
	unk: u32 //"?"
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum GSP_FramebufferFormats {
	GSP_RGBA8_OES=0, //pixel_size = 4-bytes
	GSP_BGR8_OES=1, //pixel_size = 3-bytes
	GSP_RGB565_OES=2, //pixel_size = 2-bytes
	GSP_RGB5_A1_OES=3, //pixel_size = 2-bytes
	GSP_RGBA4_OES=4 //pixel_size = 2-bytes
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GSP_CaptureInfoEntry { //See this for GSP_CaptureInfoEntry and GSP_CaptureInfo: http://3dbrew.org/wiki/GSPGPU:ImportDisplayCaptureInfo
	framebuf0_vaddr: *mut u32,
	framebuf1_vaddr: *mut u32,
	format: u32,
	framebuf_widthbytesize: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GSP_CaptureInfo {
	screencapture: [GSP_CaptureInfoEntry; 2usize]
}

#[repr(C)]
pub enum GSP_Event {
	GSPEVENT_PSC0 = 0,	// memory fill completed
	GSPEVENT_PSC1,
	GSPEVENT_VBlank0,
	GSPEVENT_VBlank1,
	GSPEVENT_PPF,		// display transfer finished
	GSPEVENT_P3D,		// command list processing finished
	GSPEVENT_DMA,

	GSPEVENT_MAX, // used to know how many events there are
}

use super::super::super::{Result, Handle};


extern "C" {
    pub fn gspInit() -> Result;
    pub fn gspExit() -> ();
    pub fn gspInitEventHandler(gspEvent: Handle, gspSharedMem: *mut vu8, gspThreadId: u8) -> Result;
    pub fn gspExitEventHandler() -> ();
    pub fn gspWaitForEvent(id: GSP_Event, nextEvent: u8) -> ();
    pub fn GSPGPU_AcquireRight(handle: *mut Handle, flags: u8) -> Result;
    pub fn GSPGPU_ReleaseRight(handle: *mut Handle) -> Result;
    pub fn GSPGPU_ImportDisplayCaptureInfo(handle: *mut Handle, captureinfo: *mut GSP_CaptureInfo) -> Result;
    pub fn GSPGPU_SaveVramSysArea(handle: *mut Handle) -> Result;
    pub fn GSPGPU_RestoreVramSysArea(handle: *mut Handle) -> Result;
    pub fn GSPGPU_SetLcdForceBlack(handle: *mut Handle, flags: u8) -> Result;
    pub fn GSPGPU_SetBufferSwap(handle: *mut Handle, screenid: u32, framebufinfo: *mut GSP_FramebufferInfo) -> Result;
    pub fn GSPGPU_FlushDataCache(handle: *mut Handle, adr: *mut u8, size: u32) -> Result;
    pub fn GSPGPU_InvalidateDataCache(handle: *mut Handle, adr: *mut u8, size: u32) -> Result;
    pub fn GSPGPU_WriteHWRegs(handle: *mut Handle, regAddr: u32, data: *mut u32, size: u8) -> Result;
    pub fn GSPGPU_WriteHWRegsWithMask(handle: *mut Handle, regAddr: u32, data: *mut u32, datasize: u8, maskdata: *mut u32, masksize: u8) -> Result;
    pub fn GSPGPU_ReadHWRegs(handle: *mut Handle, regAddr: u32, data: *mut u32, size: u8) -> Result;
    pub fn GSPGPU_RegisterInterruptRelayQueue(handle: *mut Handle, eventHandle: Handle, flags: u32, outMemHandle: *mut Handle, threadID: *mut u8) -> Result;
    pub fn GSPGPU_UnregisterInterruptRelayQueue(handle: *mut Handle) -> Result;
    pub fn GSPGPU_TriggerCmdReqQueue(handle: *mut Handle) -> Result;
    pub fn GSPGPU_SubmitGxCommand(sharedGspCmdBuf: *mut u32, gxCommand: *mut u32, handle: *mut Handle) -> Result;
}
