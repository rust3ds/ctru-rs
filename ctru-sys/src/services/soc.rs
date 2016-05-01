use ::Result;


extern "C" {
    pub fn SOC_Initialize(context_addr: *mut u32, context_size: u32) -> Result;
    pub fn SOC_Shutdown() -> Result;
}
