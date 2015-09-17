
#[inline]
pub fn SYSTEM_VERSION(major: i32, minor: i32, revision: i32) {
    (((major)<<24)|((minor)<<16)|((revision)<<8));
}


extern "C" {
    pub fn osConvertVirtToPhys(vaddr: u32) -> u32;
    pub fn osConvertOldLINEARMemToNew(addr: u32) -> u32;
    pub fn osStrError(error: u32) -> *const u8;
    pub fn osGetFirmVersion() -> u32;
    pub fn osGetKernelVersion() -> u32;
    pub fn osGetTime() -> u64;
}
