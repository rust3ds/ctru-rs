use {Result, Handle};
use libc::c_void;

extern "C" {
    pub fn srvInit() -> Result;
    pub fn srvExit() -> Result;
    pub fn srvGetSessionHandle() -> * const Handle;
    pub fn srvRegisterClient() -> Result;
    pub fn srvGetServiceHandle(out: *mut Handle, name: * const u8) -> Result;
    pub fn srvRegisterService(out: *mut Handle, name: * const u8) -> Result;
    pub fn srvUnregisterService(name: * const u8) -> Result;
    pub fn srvPmInit() -> Result;
    pub fn srvRegisterProcess(procid: u32, count: u32, serviceaccesscontrol: *mut c_void) -> Result;
    pub fn srvUnregisterProcess(procid: u32) -> Result;
}
