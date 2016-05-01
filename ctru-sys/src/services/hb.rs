use ::Result;
use ::c_void;


extern "C" {
    pub fn hbInit() -> Result;
    pub fn hbExit() -> ();
    pub fn HB_FlushInvalidateCache() -> Result;
    pub fn HB_GetBootloaderAddresses(load3dsx: *mut *mut c_void, setArgv: *mut *mut c_void) -> Result;
    pub fn HB_ReprotectMemory(addr: *mut u32, pages: u32, mode: u32, reprotectedPages: *mut u32) -> Result;
}
