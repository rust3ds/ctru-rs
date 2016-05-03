use ::{Handle, Result};
use ::c_void;

extern "C" {
    pub fn pmInit() -> Result;
    pub fn pmExit();
    pub fn PM_LaunchTitle(mediatype: u8, titleid: u64, launch_flags: u32)
     -> Result;
    pub fn PM_GetTitleExheaderFlags(mediatype: u8, titleid: u64,
                                    out: *mut u8) -> Result;
    pub fn PM_SetFIRMLaunchParams(size: u32, _in: *mut u8) -> Result;
    pub fn PM_GetFIRMLaunchParams(size: u32, out: *mut u8) -> Result;
    pub fn PM_LaunchFIRMSetParams(firm_titleid_low: u32, size: u32,
                                  _in: *mut u8) -> Result;
    pub fn srvPmInit() -> Result;
    pub fn srvPmExit();
    pub fn SRVPM_PublishToProcess(notificationId: u32, process: Handle)
     -> Result;
    pub fn SRVPM_PublishToAll(notificationId: u32) -> Result;
    pub fn SRVPM_RegisterProcess(procid: u32, count: u32,
                                 serviceaccesscontrol: c_void) -> Result;
    pub fn SRVPM_UnregisterProcess(procid: u32) -> Result;
}

