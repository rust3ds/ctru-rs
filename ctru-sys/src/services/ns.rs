use ::Result;

extern "C" {
    pub fn nsInit() -> Result;
    pub fn nsExit();
    pub fn NS_LaunchTitle(titleid: u64, launch_flags: u32, procid: *mut u32) -> Result;
    pub fn NS_RebootToTitle(mediatype: u8, titleid: u64) -> Result;
}

