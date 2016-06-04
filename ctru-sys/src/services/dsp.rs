use ::{Handle, Result};
use ::libc::c_void;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum DSP_InterruptType {
    DSP_INTERRUPT_PIPE = 2,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum DSP_PipeDirection {
    DSP_PIPE_INPUT = 0,
    DSP_PIPE_OUTPUT = 1,
}

extern "C" {
    pub fn dspInit() -> Result;
    pub fn dspExit();
    pub fn DSP_GetHeadphoneStatus(is_inserted: *mut u8) -> Result;
    pub fn DSP_FlushDataCache(address: *const c_void,
                              size: u32) -> Result;
    pub fn DSP_InvalidateDataCache(address: *const c_void,
                                   size: u32) -> Result;
    pub fn DSP_GetSemaphoreHandle(semaphore: *mut Handle) -> Result;
    pub fn DSP_SetSemaphore(value: u16) -> Result;
    pub fn DSP_SetSemaphoreMask(mask: u16) -> Result;
    pub fn DSP_LoadComponent(component: *const c_void,
                             size: u32, prog_mask: u16, data_mask: u16,
                             is_loaded: *mut u8) -> Result;
    pub fn DSP_UnloadComponent() -> Result;
    pub fn DSP_RegisterInterruptEvents(handle: Handle, interrupt: u32,
                                       channel: u32) -> Result;
    pub fn DSP_ReadPipeIfPossible(channel: u32, peer: u32,
                                  buffer: *mut c_void,
                                  length: u16, length_read: *mut u16)
     -> Result;
    pub fn DSP_WriteProcessPipe(channel: u32,
                                buffer: *const c_void,
                                length: u32) -> Result;
    pub fn DSP_ConvertProcessAddressFromDspDram(dsp_address: u32,
                                                arm_address: *mut u32)
     -> Result;
    pub fn DSP_RecvData(regNo: u16, value: *mut u16) -> Result;
    pub fn DSP_RecvDataIsReady(regNo: u16, is_ready: *mut u8) -> Result;
    pub fn DSP_SendData(regNo: u16, value: u16) -> Result;
    pub fn DSP_SendDataIsEmpty(regNo: u16, is_empty: *mut u8) -> Result;
}
