use ::Result;

extern "C" {
    pub fn ptmSysmInit() -> Result;
    pub fn ptmSysmExit();
    pub fn PTMSYSM_ConfigureNew3DSCPU(value: u8) -> Result;
}
