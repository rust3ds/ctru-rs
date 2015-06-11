use ::Result;

#[link(name = "ctru")]
extern "C" {
    pub fn pmInit() -> Result;
    pub fn pmExit() -> Result;
    pub fn PM_LaunchTitle(mediatype: u8, titleid: u64, launch_flags: u32) -> Result;
    pub fn PM_GetTitleExheaderFlags(mediatype: u8, titleid: u64, out: *mut u8) -> Result;
    pub fn PM_SetFIRMLaunchParams(size: u32, _in: *mut u8) -> Result;
    pub fn PM_GetFIRMLaunchParams(size: u32, out: *mut u8) -> Result;
    pub fn PM_LaunchFIRMSetParams(firm_titleid_low: u32, size: u32, _in: *mut u8) -> Result;
}
