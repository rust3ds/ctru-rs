//TODO: Verify if anonymous enum is properly represented

use ::Result;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed1 {
    GSPLCD_SCREEN_TOP = 1,
    GSPLCD_SCREEN_BOTTOM = 2,
    GSPLCD_SCREEN_BOTH = 3,
}
extern "C" {
    pub fn gspLcdInit() -> Result;
    pub fn gspLcdExit();
    pub fn GSPLCD_PowerOnBacklight(screen: u32) -> Result;
    pub fn GSPLCD_PowerOffBacklight(screen: u32) -> Result;
}
