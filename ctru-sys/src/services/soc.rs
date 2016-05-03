use ::Result;

extern "C" {
    pub fn SOC_Init(context_addr: *mut u32, context_size: u32) -> Result;
    pub fn SOC_Exit() -> Result;
    pub fn gethostid() -> i32;
}
