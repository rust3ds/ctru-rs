use ::types::*;

use super::hid::circlePosition;

extern "C" {
    pub static mut irrstMemHandle: Handle;
    pub static mut irrstSharedMem: *mut vu32;

    pub fn irrstInit() -> Result;
    pub fn irrstExit();
    pub fn irrstScanInput();
    pub fn irrstKeysHeld() -> u32;
    pub fn irrstCstickRead(pos: *mut circlePosition);
    pub fn irrstWaitForEvent(nextEvent: u8);
    pub fn IRRST_GetHandles(outMemHandle: *mut Handle,
                            outEventHandle: *mut Handle) -> Result;
    pub fn IRRST_Initialize(unk1: u32, unk2: u8) -> Result;
    pub fn IRRST_Shutdown() -> Result;

}
