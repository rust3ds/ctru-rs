use ::Result;

extern "C" {
    pub fn cfgnorInit(value: u8) -> Result;
    pub fn cfgnorExit();
    pub fn cfgnorDumpFlash(buf: *mut u32, size: u32) -> Result;
    pub fn cfgnorWriteFlash(buf: *mut u32, size: u32) -> Result;
    pub fn CFGNOR_Initialize(value: u8) -> Result;
    pub fn CFGNOR_Shutdown() -> Result;
    pub fn CFGNOR_ReadData(offset: u32, buf: *mut u32, size: u32)
     -> Result;
    pub fn CFGNOR_WriteData(offset: u32, buf: *mut u32, size: u32)
     -> Result;
}
