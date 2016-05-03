use ::Result;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum CFG_Region {
    CFG_REGION_JPN = 0,
    CFG_REGION_USA = 1,
    CFG_REGION_EUR = 2,
    CFG_REGION_AUS = 3,
    CFG_REGION_CHN = 4,
    CFG_REGION_KOR = 5,
    CFG_REGION_TWN = 6,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum CFG_Language {
    CFG_LANGUAGE_JP = 0,
    CFG_LANGUAGE_EN = 1,
    CFG_LANGUAGE_FR = 2,
    CFG_LANGUAGE_DE = 3,
    CFG_LANGUAGE_IT = 4,
    CFG_LANGUAGE_ES = 5,
    CFG_LANGUAGE_ZH = 6,
    CFG_LANGUAGE_KO = 7,
    CFG_LANGUAGE_NL = 8,
    CFG_LANGUAGE_PT = 9,
    CFG_LANGUAGE_RU = 10,
    CFG_LANGUAGE_TW = 11,
}

extern "C" {
    pub fn cfguInit() -> Result;
    pub fn cfguExit();
    pub fn CFGU_SecureInfoGetRegion(region: *mut u8) -> Result;
    pub fn CFGU_GenHashConsoleUnique(appIDSalt: u32, hash: *mut u64)
     -> Result;
    pub fn CFGU_GetRegionCanadaUSA(value: *mut u8) -> Result;
    pub fn CFGU_GetSystemModel(model: *mut u8) -> Result;
    pub fn CFGU_GetModelNintendo2DS(value: *mut u8) -> Result;
    pub fn CFGU_GetCountryCodeString(code: u16, string: *mut u16) -> Result;
    pub fn CFGU_GetCountryCodeID(string: u16, code: *mut u16) -> Result;
    pub fn CFGU_GetConfigInfoBlk2(size: u32, blkID: u32, outData: *mut u8)
     -> Result;
    pub fn CFGU_GetSystemLanguage(language: *mut u8) -> Result;
}
