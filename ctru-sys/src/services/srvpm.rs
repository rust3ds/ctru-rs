use ::{Handle, Result};
use ::libc::c_void;

extern "C" {
    pub fn srvPmInit() -> Result;
    pub fn srvPmExit();
    pub fn SRVPM_PublishToProcess(notificationId: u32, process: Handle) -> Result;
    pub fn SRVPM_PublishToAll(notificationId: u32) -> Result;
    pub fn SRVPM_RegisterProcess(procid: u32, count: u32,
                                 serviceaccesscontrol: c_void) -> Result;
    pub fn SRVPM_UnregisterProcess(procid: u32) -> Result;
}
