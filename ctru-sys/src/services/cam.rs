// TODO: Determine if anonymous enums are properly represented (they probably aren't)

use ::{Handle, Result};
use ::c_void;
use ::types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed1 {
    PORT_NONE = 0,
    PORT_CAM1 = 1,
    PORT_CAM2 = 2,
    PORT_BOTH = 3,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed2 {
    SELECT_NONE = 0,
    SELECT_OUT1 = 1,
    SELECT_IN1 = 2,
    SELECT_OUT2 = 4,
    SELECT_IN1_OUT1 = 3,
    SELECT_OUT1_OUT2 = 5,
    SELECT_IN1_OUT2 = 6,
    SELECT_ALL = 7,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed3 {
    CONTEXT_NONE = 0,
    CONTEXT_A = 1,
    CONTEXT_B = 2,
    CONTEXT_BOTH = 3,
}

pub type CAMU_Context = Enum_Unnamed3;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed4 {
    FLIP_NONE = 0,
    FLIP_HORIZONTAL = 1,
    FLIP_VERTICAL = 2,
    FLIP_REVERSE = 3,
}
pub type CAMU_Flip = Enum_Unnamed4;

pub const SIZE_CTR_BOTTOM_LCD: Enum_Unnamed5 = Enum_Unnamed5::SIZE_QVGA;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed5 {
    SIZE_VGA = 0,
    SIZE_QVGA = 1,
    SIZE_QQVGA = 2,
    SIZE_CIF = 3,
    SIZE_QCIF = 4,
    SIZE_DS_LCD = 5,
    SIZE_DS_LCDx4 = 6,
    SIZE_CTR_TOP_LCD = 7,
}
pub type CAMU_Size = Enum_Unnamed5;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum CAMU_FrameRate {
    FRAME_RATE_15 = 0,
    FRAME_RATE_15_TO_5 = 1,
    FRAME_RATE_15_TO_2 = 2,
    FRAME_RATE_10 = 3,
    FRAME_RATE_8_5 = 4,
    FRAME_RATE_5 = 5,
    FRAME_RATE_20 = 6,
    FRAME_RATE_20_TO_5 = 7,
    FRAME_RATE_30 = 8,
    FRAME_RATE_30_TO_5 = 9,
    FRAME_RATE_15_TO_10 = 10,
    FRAME_RATE_20_TO_10 = 11,
    FRAME_RATE_30_TO_10 = 12,
}

pub const WHITE_BALANCE_NORMAL: Enum_Unnamed7 =
    Enum_Unnamed7::WHITE_BALANCE_AUTO;
pub const WHITE_BALANCE_TUNGSTEN: Enum_Unnamed7 =
    Enum_Unnamed7::WHITE_BALANCE_3200K;
pub const WHITE_BALANCE_WHITE_FLUORESCENT_LIGHT: Enum_Unnamed7 =
    Enum_Unnamed7::WHITE_BALANCE_4150K;
pub const WHITE_BALANCE_DAYLIGHT: Enum_Unnamed7 =
    Enum_Unnamed7::WHITE_BALANCE_5200K;
pub const WHITE_BALANCE_CLOUDY: Enum_Unnamed7 =
    Enum_Unnamed7::WHITE_BALANCE_6000K;
pub const WHITE_BALANCE_HORIZON: Enum_Unnamed7 =
    Enum_Unnamed7::WHITE_BALANCE_6000K;
pub const WHITE_BALANCE_SHADE: Enum_Unnamed7 =
    Enum_Unnamed7::WHITE_BALANCE_7000K;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed7 {
    WHITE_BALANCE_AUTO = 0,
    WHITE_BALANCE_3200K = 1,
    WHITE_BALANCE_4150K = 2,
    WHITE_BALANCE_5200K = 3,
    WHITE_BALANCE_6000K = 4,
    WHITE_BALANCE_7000K = 5,
}
pub type CAMU_WhiteBalance = Enum_Unnamed7;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed8 {
    PHOTO_MODE_NORMAL = 0,
    PHOTO_MODE_PORTRAIT = 1,
    PHOTO_MODE_LANDSCAPE = 2,
    PHOTO_MODE_NIGHTVIEW = 3,
    PHOTO_MODE_LETTER = 4,
}
pub type CAMU_PhotoMode = Enum_Unnamed8;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed9 {
    EFFECT_NONE = 0,
    EFFECT_MONO = 1,
    EFFECT_SEPIA = 2,
    EFFECT_NEGATIVE = 3,
    EFFECT_NEGAFILM = 4,
    EFFECT_SEPIA01 = 5,
}
pub type CAMU_Effect = Enum_Unnamed9;
pub const CONTRAST_LOW: Enum_Unnamed10 = Enum_Unnamed10::CONTRAST_PATTERN_05;
pub const CONTRAST_NORMAL: Enum_Unnamed10 =
    Enum_Unnamed10::CONTRAST_PATTERN_06;
pub const CONTRAST_HIGH: Enum_Unnamed10 = Enum_Unnamed10::CONTRAST_PATTERN_07;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed10 {
    CONTRAST_PATTERN_01 = 0,
    CONTRAST_PATTERN_02 = 1,
    CONTRAST_PATTERN_03 = 2,
    CONTRAST_PATTERN_04 = 3,
    CONTRAST_PATTERN_05 = 4,
    CONTRAST_PATTERN_06 = 5,
    CONTRAST_PATTERN_07 = 6,
    CONTRAST_PATTERN_08 = 7,
    CONTRAST_PATTERN_09 = 8,
    CONTRAST_PATTERN_10 = 9,
    CONTRAST_PATTERN_11 = 10,
}
pub type CAMU_Contrast = Enum_Unnamed10;
pub const LENS_CORRECTION_DARK: Enum_Unnamed11 =
    Enum_Unnamed11::LENS_CORRECTION_OFF;
pub const LENS_CORRECTION_NORMAL: Enum_Unnamed11 =
    Enum_Unnamed11::LENS_CORRECTION_ON_70;
pub const LENS_CORRECTION_BRIGHT: Enum_Unnamed11 =
    Enum_Unnamed11::LENS_CORRECTION_ON_90;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed11 {
    LENS_CORRECTION_OFF = 0,
    LENS_CORRECTION_ON_70 = 1,
    LENS_CORRECTION_ON_90 = 2,
}
pub type CAMU_LensCorrection = Enum_Unnamed11;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed12 { OUTPUT_YUV_422 = 0, OUTPUT_RGB_565 = 1, }
pub type CAMU_OutputFormat = Enum_Unnamed12;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed13 {
    SHUTTER_SOUND_TYPE_NORMAL = 0,
    SHUTTER_SOUND_TYPE_MOVIE = 1,
    SHUTTER_SOUND_TYPE_MOVIE_END = 2,
}
pub type CAMU_ShutterSoundType = Enum_Unnamed13;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed14 {
    pub aeBaseTarget: s16,
    pub kRL: s16,
    pub kGL: s16,
    pub kBL: s16,
    pub ccmPosition: s16,
    pub awbCcmL9Right: u16,
    pub awbCcmL9Left: u16,
    pub awbCcmL10Right: u16,
    pub awbCcmL10Left: u16,
    pub awbX0Right: u16,
    pub awbX0Left: u16,
}
impl ::core::clone::Clone for Struct_Unnamed14 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed14 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type CAMU_ImageQualityCalibrationData = Struct_Unnamed14;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed15 {
    pub isValidRotationXY: u8,
    pub padding: [u8; 3usize],
    pub scale: f32,
    pub rotationZ: f32,
    pub translationX: f32,
    pub translationY: f32,
    pub rotationX: f32,
    pub rotationY: f32,
    pub angleOfViewRight: f32,
    pub angleOfViewLeft: f32,
    pub distanceToChart: f32,
    pub distanceCameras: f32,
    pub imageWidth: s16,
    pub imageHeight: s16,
    pub reserved: [u8; 16usize],
}
impl ::core::clone::Clone for Struct_Unnamed15 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed15 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type CAMU_StereoCameraCalibrationData = Struct_Unnamed15;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed16 {
    pub camera: u8,
    pub exposure: s8,
    pub whiteBalance: u8,
    pub sharpness: s8,
    pub autoExposureOn: u8,
    pub autoWhiteBalanceOn: u8,
    pub frameRate: u8,
    pub photoMode: u8,
    pub contrast: u8,
    pub lensCorrection: u8,
    pub noiseFilterOn: u8,
    pub padding: u8,
    pub autoExposureWindowX: s16,
    pub autoExposureWindowY: s16,
    pub autoExposureWindowWidth: s16,
    pub autoExposureWindowHeight: s16,
    pub autoWhiteBalanceWindowX: s16,
    pub autoWhiteBalanceWindowY: s16,
    pub autoWhiteBalanceWindowWidth: s16,
    pub autoWhiteBalanceWindowHeight: s16,
}
impl ::core::clone::Clone for Struct_Unnamed16 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed16 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type CAMU_PackageParameterCameraSelect = Struct_Unnamed16;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed17 {
    pub camera: u8,
    pub context: u8,
    pub flip: u8,
    pub effect: u8,
    pub size: u8,
}
impl ::core::clone::Clone for Struct_Unnamed17 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed17 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type CAMU_PackageParameterContext = Struct_Unnamed17;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed18 {
    pub camera: u8,
    pub context: u8,
    pub flip: u8,
    pub effect: u8,
    pub width: s16,
    pub height: s16,
    pub cropX0: s16,
    pub cropY0: s16,
    pub cropX1: s16,
    pub cropY1: s16,
}
impl ::core::clone::Clone for Struct_Unnamed18 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed18 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type CAMU_PackageParameterContextDetail = Struct_Unnamed18;

use services::y2r::Y2RU_StandardCoefficient;
extern "C" {
    pub fn camInit() -> Result;
    pub fn camExit();
    pub fn CAMU_StartCapture(port: u32) -> Result;
    pub fn CAMU_StopCapture(port: u32) -> Result;
    pub fn CAMU_IsBusy(busy: *mut u8, port: u32) -> Result;
    pub fn CAMU_ClearBuffer(port: u32) -> Result;
    pub fn CAMU_GetVsyncInterruptEvent(event: *mut Handle, port: u32)
     -> Result;
    pub fn CAMU_GetBufferErrorInterruptEvent(event: *mut Handle, port: u32)
     -> Result;
    pub fn CAMU_SetReceiving(event: *mut Handle,
                             dst: *mut c_void, port: u32,
                             imageSize: u32, transferUnit: s16) -> Result;
    pub fn CAMU_IsFinishedReceiving(finishedReceiving: *mut u8, port: u32)
     -> Result;
    pub fn CAMU_SetTransferLines(port: u32, lines: s16, width: s16,
                                 height: s16) -> Result;
    pub fn CAMU_GetMaxLines(maxLines: *mut s16, width: s16, height: s16)
     -> Result;
    pub fn CAMU_SetTransferBytes(port: u32, bytes: u32, width: s16,
                                 height: s16) -> Result;
    pub fn CAMU_GetTransferBytes(transferBytes: *mut u32, port: u32)
     -> Result;
    pub fn CAMU_GetMaxBytes(maxBytes: *mut u32, width: s16, height: s16)
     -> Result;
    pub fn CAMU_SetTrimming(port: u32, trimming: u8) -> Result;
    pub fn CAMU_IsTrimming(trimming: *mut u8, port: u32) -> Result;
    pub fn CAMU_SetTrimmingParams(port: u32, xStart: s16, yStart: s16,
                                  xEnd: s16, yEnd: s16) -> Result;
    pub fn CAMU_GetTrimmingParams(xStart: *mut s16, yStart: *mut s16,
                                  xEnd: *mut s16, yEnd: *mut s16, port: u32)
     -> Result;
    pub fn CAMU_SetTrimmingParamsCenter(port: u32, trimWidth: s16,
                                        trimHeight: s16, camWidth: s16,
                                        camHeight: s16) -> Result;
    pub fn CAMU_Activate(select: u32) -> Result;
    pub fn CAMU_SwitchContext(select: u32, context: CAMU_Context) -> Result;
    pub fn CAMU_SetExposure(select: u32, exposure: s8) -> Result;
    pub fn CAMU_SetWhiteBalance(select: u32, whiteBalance: CAMU_WhiteBalance)
     -> Result;
    pub fn CAMU_SetWhiteBalanceWithoutBaseUp(select: u32,
                                             whiteBalance: CAMU_WhiteBalance)
     -> Result;
    pub fn CAMU_SetSharpness(select: u32, sharpness: s8) -> Result;
    pub fn CAMU_SetAutoExposure(select: u32, autoExposure: u8) -> Result;
    pub fn CAMU_IsAutoExposure(autoExposure: *mut u8, select: u32) -> Result;
    pub fn CAMU_SetAutoWhiteBalance(select: u32, autoWhiteBalance: u8)
     -> Result;
    pub fn CAMU_IsAutoWhiteBalance(autoWhiteBalance: *mut u8, select: u32)
     -> Result;
    pub fn CAMU_FlipImage(select: u32, flip: CAMU_Flip,
                          context: CAMU_Context) -> Result;
    pub fn CAMU_SetDetailSize(select: u32, width: s16, height: s16,
                              cropX0: s16, cropY0: s16, cropX1: s16,
                              cropY1: s16, context: CAMU_Context) -> Result;
    pub fn CAMU_SetSize(select: u32, size: CAMU_Size, context: CAMU_Context)
     -> Result;
    pub fn CAMU_SetFrameRate(select: u32, frameRate: CAMU_FrameRate)
     -> Result;
    pub fn CAMU_SetPhotoMode(select: u32, photoMode: CAMU_PhotoMode)
     -> Result;
    pub fn CAMU_SetEffect(select: u32, effect: CAMU_Effect,
                          context: CAMU_Context) -> Result;
    pub fn CAMU_SetContrast(select: u32, contrast: CAMU_Contrast) -> Result;
    pub fn CAMU_SetLensCorrection(select: u32,
                                  lensCorrection: CAMU_LensCorrection)
     -> Result;
    pub fn CAMU_SetOutputFormat(select: u32, format: CAMU_OutputFormat,
                                context: CAMU_Context) -> Result;
    pub fn CAMU_SetAutoExposureWindow(select: u32, x: s16, y: s16,
                                      width: s16, height: s16) -> Result;
    pub fn CAMU_SetAutoWhiteBalanceWindow(select: u32, x: s16, y: s16,
                                          width: s16, height: s16) -> Result;
    pub fn CAMU_SetNoiseFilter(select: u32, noiseFilter: u8) -> Result;
    pub fn CAMU_SynchronizeVsyncTiming(select1: u32, select2: u32)
     -> Result;
    pub fn CAMU_GetLatestVsyncTiming(timing: *mut s64, port: u32, past: u32)
     -> Result;
    pub fn CAMU_GetStereoCameraCalibrationData(data:
                                                   *mut CAMU_StereoCameraCalibrationData)
     -> Result;
    pub fn CAMU_SetStereoCameraCalibrationData(data:
                                                   CAMU_StereoCameraCalibrationData)
     -> Result;
    pub fn CAMU_WriteRegisterI2c(select: u32, addr: u16, data: u16)
     -> Result;
    pub fn CAMU_WriteMcuVariableI2c(select: u32, addr: u16, data: u16)
     -> Result;
    pub fn CAMU_ReadRegisterI2cExclusive(data: *mut u16, select: u32,
                                         addr: u16) -> Result;
    pub fn CAMU_ReadMcuVariableI2cExclusive(data: *mut u16, select: u32,
                                            addr: u16) -> Result;
    pub fn CAMU_SetImageQualityCalibrationData(data:
                                                   CAMU_ImageQualityCalibrationData)
     -> Result;
    pub fn CAMU_GetImageQualityCalibrationData(data:
                                                   *mut CAMU_ImageQualityCalibrationData)
     -> Result;
    pub fn CAMU_SetPackageParameterWithoutContext(param:
                                                      CAMU_PackageParameterCameraSelect)
     -> Result;
    pub fn CAMU_SetPackageParameterWithContext(param:
                                                   CAMU_PackageParameterContext)
     -> Result;
    pub fn CAMU_SetPackageParameterWithContextDetail(param:
                                                         CAMU_PackageParameterContextDetail)
     -> Result;
    pub fn CAMU_GetSuitableY2rStandardCoefficient(coefficient:
                                                      *mut Y2RU_StandardCoefficient)
     -> Result;
    pub fn CAMU_PlayShutterSound(sound: CAMU_ShutterSoundType) -> Result;
    pub fn CAMU_DriverInitialize() -> Result;
    pub fn CAMU_DriverFinalize() -> Result;
    pub fn CAMU_GetActivatedCamera(select: *mut u32) -> Result;
    pub fn CAMU_GetSleepCamera(select: *mut u32) -> Result;
    pub fn CAMU_SetSleepCamera(select: u32) -> Result;
    pub fn CAMU_SetBrightnessSynchronization(brightnessSynchronization: u8)
     -> Result;
}
