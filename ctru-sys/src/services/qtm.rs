use ::Result;

#[repr(C)]
#[derive(Copy)]
pub struct QTM_HeadTrackingInfoCoord {
    pub x: f32,
    pub y: f32,
}
impl ::core::clone::Clone for QTM_HeadTrackingInfoCoord {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for QTM_HeadTrackingInfoCoord {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct QTM_HeadTrackingInfo {
    pub flags: [u8; 5usize],
    pub padding: [u8; 3usize],
    pub floatdata_x08: f32,
    pub coords0: [QTM_HeadTrackingInfoCoord; 4usize],
    pub unk_x2c: [u32; 5usize],
}
impl ::core::clone::Clone for QTM_HeadTrackingInfo {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for QTM_HeadTrackingInfo {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

extern "C" {
    pub fn qtmInit() -> Result;
    pub fn qtmExit();
    pub fn qtmCheckInitialized() -> u8;
    pub fn qtmCheckHeadFullyDetected(info: *mut QTM_HeadTrackingInfo) -> u8;
    pub fn qtmConvertCoordToScreen(coord: *mut QTM_HeadTrackingInfoCoord,
                                   screen_width: *mut f32,
                                   screen_height:
                                       *mut f32,
                                   x: *mut u32, y: *mut u32) -> Result;
    pub fn QTM_GetHeadTrackingInfo(val: u64, out: *mut QTM_HeadTrackingInfo)
     -> Result;
}
