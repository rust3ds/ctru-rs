use ::{Handle, Result};

#[repr(C)]
#[derive(Copy)]
pub struct AM_TitleEntry {
    pub titleID: u64,
    pub size: u64,
    pub version: u16,
    pub unk: [u8; 6usize],
}

impl ::core::clone::Clone for AM_TitleEntry {
    fn clone(&self) -> Self { *self }
}

impl ::core::default::Default for AM_TitleEntry {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

extern "C" {
    pub fn amInit() -> Result;
    pub fn amExit();
    pub fn amGetSessionHandle() -> *mut Handle;
    pub fn AM_GetTitleCount(mediatype: u8, count: *mut u32) -> Result;
    pub fn AM_GetTitleIdList(mediatype: u8, count: u32, titleIDs: *mut u64)
     -> Result;
    pub fn AM_GetDeviceId(deviceID: *mut u32) -> Result;
    pub fn AM_ListTitles(mediatype: u8, titleCount: u32,
                         titleIdList: *mut u64, titleList: *mut AM_TitleEntry)
     -> Result;
    pub fn AM_StartCiaInstall(mediatype: u8, ciaHandle: *mut Handle)
     -> Result;
    pub fn AM_StartDlpChildCiaInstall(ciaHandle: *mut Handle) -> Result;
    pub fn AM_CancelCIAInstall(ciaHandle: *mut Handle) -> Result;
    pub fn AM_FinishCiaInstall(mediatype: u8, ciaHandle: *mut Handle)
     -> Result;
    pub fn AM_DeleteTitle(mediatype: u8, titleID: u64) -> Result;
    pub fn AM_DeleteAppTitle(mediatype: u8, titleID: u64) -> Result;
    pub fn AM_InstallNativeFirm() -> Result;
    pub fn AM_InstallFirm(titleID: u64) -> Result;
    pub fn AM_GetTitleProductCode(mediatype: u8, titleID: u64, 
                                  productCode: *mut u8)
     -> Result;
    pub fn AM_GetCiaFileInfo(mediatype: u8, titleEntry: *mut AM_TitleEntry,
                             fileHandle: Handle) -> Result;
    pub fn AM_InitializeExternalTitleDatabase(overwrite: u8) -> Result;
    pub fn AM_QueryAvailableExternalTitleDatabase(available: *mut u8)
     -> Result;
}
