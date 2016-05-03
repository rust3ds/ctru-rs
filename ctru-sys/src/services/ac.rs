use ::Result;

extern "C" {
    pub fn acInit() -> Result;
    pub fn acExit();
    pub fn acWaitInternetConnection() -> Result;
    pub fn ACU_GetWifiStatus(out: *mut u32) -> Result;
}
