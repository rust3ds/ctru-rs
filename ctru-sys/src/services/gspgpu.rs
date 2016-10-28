use ::libc::c_void;
use ::types::*;

#[inline]
pub fn GSPGPU_REBASE_REG(r: u32) {
    ((r)-0x1EB00000);
}

#[repr(C)]
#[derive(Copy)]
pub struct GSPGPU_FramebufferInfo {
    pub active_framebuf: u32,
    pub framebuf0_vaddr: *mut u32,
    pub framebuf1_vaddr: *mut u32,
    pub framebuf_widthbytesize: u32,
    pub format: u32,
    pub framebuf_dispselect: u32,
    pub unk: u32,
}
impl ::core::clone::Clone for GSPGPU_FramebufferInfo {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for GSPGPU_FramebufferInfo {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum GSPGPU_FramebufferFormats {
    GSP_RGBA8_OES = 0,
    GSP_BGR8_OES = 1,
    GSP_RGB565_OES = 2,
    GSP_RGB5_A1_OES = 3,
    GSP_RGBA4_OES = 4,
}

#[repr(C)]
#[derive(Copy)]
pub struct GSPGPU_CaptureInfoEntry {
    pub framebuf0_vaddr: *mut u32,
    pub framebuf1_vaddr: *mut u32,
    pub format: u32,
    pub framebuf_widthbytesize: u32,
}
impl ::core::clone::Clone for GSPGPU_CaptureInfoEntry {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for GSPGPU_CaptureInfoEntry {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct GSPGPU_CaptureInfo {
    pub screencapture: [GSPGPU_CaptureInfoEntry; 2usize],
}
impl ::core::clone::Clone for GSPGPU_CaptureInfo {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for GSPGPU_CaptureInfo {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

#[repr(C)]
pub enum GSPGPU_Event {
    GSPGPU_EVENT_PSC0 = 0,
    GSPGPU_EVENT_PSC1 = 1,
    GSPGPU_EVENT_VBlank0 = 2,
    GSPGPU_EVENT_VBlank1 = 3,
    GSPGPU_EVENT_PPF = 4,
    GSPGPU_EVENT_P3D = 5,
    GSPGPU_EVENT_DMA = 6,
    GSPGPU_EVENT_MAX = 7,
}

extern "C" {
    pub fn gspInit() -> Result;
    pub fn gspExit();
    pub fn gspSetEventCallback(id: GSPGPU_Event, cb: ThreadFunc,
                               data: *mut c_void,
                               oneShot: u8);
    pub fn gspInitEventHandler(gspEvent: Handle, gspSharedMem: *mut vu8,
                               gspThreadId: u8) -> Result;
    pub fn gspExitEventHandler();
    pub fn gspWaitForEvent(id: GSPGPU_Event, nextEvent: u8);
    pub fn gspWaitForAnyEvent() -> GSPGPU_Event;
    pub fn gspSubmitGxCommand(sharedGspCmdBuf: *mut u32,
                              gxCommand: *mut u32) -> Result;
    pub fn GSPGPU_AcquireRight(flags: u8) -> Result;
    pub fn GSPGPU_ReleaseRight() -> Result;
    pub fn GSPGPU_ImportDisplayCaptureInfo(captureinfo:
                                               *mut GSPGPU_CaptureInfo)
     -> Result;
    pub fn GSPGPU_SaveVramSysArea() -> Result;
    pub fn GSPGPU_RestoreVramSysArea() -> Result;
    pub fn GSPGPU_SetLcdForceBlack(flags: u8) -> Result;
    pub fn GSPGPU_SetBufferSwap(screenid: u32,
                                framebufinfo: *mut GSPGPU_FramebufferInfo)
     -> Result;
    pub fn GSPGPU_FlushDataCache(adr: *const c_void,
                                 size: u32) -> Result;
    pub fn GSPGPU_InvalidateDataCache(adr: *const c_void,
                                      size: u32) -> Result;
    pub fn GSPGPU_WriteHWRegs(regAddr: u32, data: *mut u32, size: u8)
     -> Result;
    pub fn GSPGPU_WriteHWRegsWithMask(regAddr: u32, data: *mut u32,
                                      datasize: u8, maskdata: *mut u32,
                                      masksize: u8) -> Result;
    pub fn GSPGPU_ReadHWRegs(regAddr: u32, data: *mut u32, size: u8)
     -> Result;
    pub fn GSPGPU_RegisterInterruptRelayQueue(eventHandle: Handle,
                                              flags: u32,
                                              outMemHandle: *mut Handle,
                                              threadID: *mut u8) -> Result;
    pub fn GSPGPU_UnregisterInterruptRelayQueue() -> Result;
    pub fn GSPGPU_TriggerCmdReqQueue() -> Result;
}
