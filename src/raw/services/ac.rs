use ::{Handle, Result};


extern "C" {
    pub fn acInit() -> Result;
    pub fn acExit() -> Result;
    pub fn ACU_GetWifiStatus(servhandle: *mut Handle, out: *mut u32) -> Result;
    pub fn ACU_WaitInternetConnection() -> Result;
}
