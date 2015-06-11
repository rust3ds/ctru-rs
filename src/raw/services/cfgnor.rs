use ::Result;

#[link(name = "ctru")]
extern "C" {
    pub fn CFGNOR_Initialize(value: u8) -> Result;
    pub fn CFGNOR_Shutdown() -> Result;
    pub fn CFGNOR_ReadData(offset: u32, buf: *mut u32, size: u32) -> Result;
    pub fn CFGNOR_WriteData(offset: u32, buf: *mut u32, size: u32) -> Result;
    pub fn CFGNOR_DumpFlash(buf: *mut u32, size: u32) -> Result;
    pub fn CFGNOR_WriteFlash(buf: *mut u32, size: u32) -> Result;
}
