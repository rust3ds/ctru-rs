use ::Result;

extern "C" {
    pub fn initCfgu() -> Result;
    pub fn exitCfgu() -> Result;
    pub fn CFGU_SecureInfoGetRegion(region: *mut u8) -> Result;
    pub fn CFGU_GenHashConsoleUnique(appIDSalt: u32, hash: *mut u64) -> Result;
    pub fn CFGU_GetRegionCanadaUSA(value: *mut u8) -> Result;
    pub fn CFGU_GetSystemModel(model: *mut u8) -> Result;
    pub fn CFGU_GetModelNintendo2DS(value: *mut u8) -> Result;
    pub fn CFGU_GetCountryCodeString(code: u16, string: *mut u16) -> Result;
    pub fn CFGU_GetCountryCodeID(string: u16, code: *mut u16) -> Result;
    pub fn CFGU_GetConfigInfoBlk2(size: u32, blkID: u32, outData: *mut u8) -> Result;
    pub fn CFGU_GetSystemLanguage(language: *mut u8) -> Result;
}
