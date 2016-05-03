//TODO: Fix Bindgen's issues again. 

use ::Result;
use types::*;

#[inline]
pub fn SYSTEM_VERSION(major: i32, minor: i32, revision: i32) {
    (((major)<<24)|((minor)<<16)|((revision)<<8));
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed1 {
    MEMREGION_ALL = 0,
    MEMREGION_APPLICATION = 1,
    MEMREGION_SYSTEM = 2,
    MEMREGION_BASE = 3,
}
pub type MemRegion = Enum_Unnamed1;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed2 {
    pub build: u8,
    pub minor: u8,
    pub mainver: u8,
    pub reserved_x3: u8,
    pub region: u8,
    pub reserved_x5: [u8; 3usize],
}
impl ::core::clone::Clone for Struct_Unnamed2 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type OS_VersionBin = Struct_Unnamed2;
extern "C" {
    pub fn osConvertVirtToPhys(vaddr: *const ::c_void) -> u32;
    pub fn osConvertOldLINEARMemToNew(vaddr: *const ::c_void)
     -> *mut ::c_void;
    pub fn osStrError(error: u32) -> *const u8;
    pub fn osGetMemRegionUsed(region: MemRegion) -> s64;
    pub fn osGetTime() -> u64;
    pub fn osSetSpeedupEnable(enable: u8);
    pub fn osGetSystemVersionData(nver_versionbin: *mut OS_VersionBin,
                                  cver_versionbin: *mut OS_VersionBin)
     -> Result;
    pub fn osGetSystemVersionDataString(nver_versionbin: *mut OS_VersionBin,
                                        cver_versionbin: *mut OS_VersionBin,
                                        sysverstr:
                                            *mut u8,
                                        sysverstr_maxsize: u32) -> Result;
}
