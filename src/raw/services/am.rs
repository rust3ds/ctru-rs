use ::raw::c_void;
use ::{Result, Handle};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TitleList {
	titleID: u64,
	size: u64,
	titleVersion: u16,
	unknown2: [u8; 6usize]
}

#[link(name = "ctru")]
extern "C" {
    pub fn amInit() -> Result;
    pub fn amExit() -> Result;
    pub fn amGetSessionHandle() -> *mut Handle;
    pub fn AM_GetTitleCount(mediatype: u8, count: *mut u32) -> Result;
    pub fn AM_GetTitleIdList(mediatype: u8, count: u32, titleIDs: *mut u64) -> Result;
    pub fn AM_GetDeviceId(deviceID: *mut u32) -> Result;
    pub fn AM_ListTitles(mediatype: u8, titleCount: u32, titleIdList: *mut u64, titleList: *mut TitleList) -> Result;
    pub fn AM_StartCiaInstall(mediatype: u8, ciaHandle: *mut Handle) -> Result;
    pub fn AM_StartDlpChildCiaInstall(ciaHandle: *mut Handle) -> Result;
    pub fn AM_CancelCIAInstall(ciaHandle: *mut Handle) -> Result;
    pub fn AM_FinishCiaInstall(mediatype: u8, ciaHandle: *mut Handle) -> Result;
    pub fn AM_DeleteTitle(mediatype: u8, titleID: u64) -> Result;
    pub fn AM_DeleteAppTitle(mediatype: u8, titleID: u64) -> Result;
    pub fn AM_InstallFIRM(titleID: u64) -> Result;
    pub fn AM_GetTitleProductCode(mediatype: u8, titleID: u64, productCode: *mut c_void) -> Result;
}
