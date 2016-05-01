use ::{Result, Handle};


extern "C" {
    pub fn IRU_Initialize(sharedmem_addr: *mut u32, sharedmem_size: u32) -> Result;
    pub fn IRU_Shutdown() -> Result;
    pub fn IRU_GetServHandle() -> Handle;
    pub fn IRU_SendData(buf: *mut u8, size: u32, wait: u32) -> Result;
    pub fn IRU_RecvData(buf: *mut u8, size: u32, flag: u8, transfercount: *mut u32, wait: u32) -> Result;
    pub fn IRU_SetBitRate(value: u8) -> Result;
    pub fn IRU_GetBitRate(out: *mut u8) -> Result;
    pub fn IRU_SetIRLEDState(value: u32) -> Result;
    pub fn IRU_GetIRLEDRecvState(out: *mut u32) -> Result;
}
