use ::{Result, Handle};


extern "C" {
    pub fn ptmInit() -> Result;
    pub fn ptmExit() -> Result;
    pub fn PTMU_GetShellState(servhandle: *mut Handle, out: *mut u8) -> Result;
    pub fn PTMU_GetBatteryLevel(servhandle: *mut Handle, out: *mut u8) -> Result;
    pub fn PTMU_GetBatteryChargeState(servhandle: *mut Handle, out: *mut u8) -> Result;
    pub fn PTMU_GetPedometerState(servhandle: *mut Handle, out: *mut u8) -> Result;
    pub fn PTMU_GetTotalStepCount(servhandle: *mut Handle, steps: *mut u32) -> Result;
}
