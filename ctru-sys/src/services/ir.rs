use ::{Result, Handle};

extern "C" {
    pub fn iruInit(sharedmem_addr: *mut u32, sharedmem_size: u32) -> Result;
    pub fn iruExit();
    pub fn iruGetServHandle() -> Handle;
    pub fn iruSendData(buf: *mut u8, size: u32, wait: u8) -> Result;
    pub fn iruRecvData(buf: *mut u8, size: u32, flag: u8,
                       transfercount: *mut u32, wait: u8) -> Result;
    pub fn IRU_Initialize() -> Result;
    pub fn IRU_Shutdown() -> Result;
    pub fn IRU_StartSendTransfer(buf: *mut u8, size: u32) -> Result;
    pub fn IRU_WaitSendTransfer() -> Result;
    pub fn IRU_StartRecvTransfer(size: u32, flag: u8) -> Result;
    pub fn IRU_WaitRecvTransfer(transfercount: *mut u32) -> Result;
    pub fn IRU_SetBitRate(value: u8) -> Result;
    pub fn IRU_GetBitRate(out: *mut u8) -> Result;
    pub fn IRU_SetIRLEDState(value: u32) -> Result;
    pub fn IRU_GetIRLEDRecvState(out: *mut u32) -> Result;
}

