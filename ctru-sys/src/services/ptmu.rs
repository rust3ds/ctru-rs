use ::Result;

extern "C" {
    pub fn ptmuInit() -> Result;
    pub fn ptmuExit();
    pub fn PTMU_GetShellState(out: *mut u8) -> Result;
    pub fn PTMU_GetBatteryLevel(out: *mut u8) -> Result;
    pub fn PTMU_GetBatteryChargeState(out: *mut u8) -> Result;
    pub fn PTMU_GetPedometerState(out: *mut u8) -> Result;
    pub fn PTMU_GetTotalStepCount(steps: *mut u32) -> Result;
}
