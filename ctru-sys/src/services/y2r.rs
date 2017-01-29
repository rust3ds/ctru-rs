use ::types::*;
use ::libc::c_void;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Y2RU_InputFormat {
    INPUT_YUV422_INDIV_8 = 0,
    INPUT_YUV420_INDIV_8 = 1,
    INPUT_YUV422_INDIV_16 = 2,
    INPUT_YUV420_INDIV_16 = 3,
    INPUT_YUV422_BATCH = 4,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Y2RU_OutputFormat {
    OUTPUT_RGB_32 = 0,
    OUTPUT_RGB_24 = 1,
    OUTPUT_RGB_16_555 = 2,
    OUTPUT_RGB_16_565 = 3,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Y2RU_Rotation {
    ROTATION_NONE = 0,
    ROTATION_CLOCKWISE_90 = 1,
    ROTATION_CLOCKWISE_180 = 2,
    ROTATION_CLOCKWISE_270 = 3,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Y2RU_BlockAlignment { 
    BLOCK_LINE = 0,
    BLOCK_8_BY_8 = 1, 
}

#[repr(C)]
#[derive(Copy)]
pub struct Y2RU_ColorCoefficients {
    pub rgb_Y: u16,
    pub r_V: u16,
    pub g_V: u16,
    pub g_U: u16,
    pub b_U: u16,
    pub r_offset: u16,
    pub g_offset: u16,
    pub b_offset: u16,
}
impl ::core::clone::Clone for Y2RU_ColorCoefficients {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Y2RU_ColorCoefficients {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Y2RU_StandardCoefficient {
    COEFFICIENT_ITU_R_BT_601 = 0,
    COEFFICIENT_ITU_R_BT_709 = 1,
    COEFFICIENT_ITU_R_BT_601_SCALING = 2,
    COEFFICIENT_ITU_R_BT_709_SCALING = 3,
}

#[repr(C)]
#[derive(Copy)]
pub struct Y2RU_ConversionParams {
    pub _bindgen_bitfield_1_: Y2RU_InputFormat,
    pub _bindgen_bitfield_2_: Y2RU_OutputFormat,
    pub _bindgen_bitfield_3_: Y2RU_Rotation,
    pub _bindgen_bitfield_4_: Y2RU_BlockAlignment,
    pub input_line_width: s16,
    pub input_lines: s16,
    pub _bindgen_bitfield_5_: Y2RU_StandardCoefficient,
    pub unused: u8,
    pub alpha: u16,
}
impl ::core::clone::Clone for Y2RU_ConversionParams {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Y2RU_ConversionParams {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Y2RU_DitheringWeightParams {
    pub w0_xEven_yEven: u16,
    pub w0_xOdd_yEven: u16,
    pub w0_xEven_yOdd: u16,
    pub w0_xOdd_yOdd: u16,
    pub w1_xEven_yEven: u16,
    pub w1_xOdd_yEven: u16,
    pub w1_xEven_yOdd: u16,
    pub w1_xOdd_yOdd: u16,
    pub w2_xEven_yEven: u16,
    pub w2_xOdd_yEven: u16,
    pub w2_xEven_yOdd: u16,
    pub w2_xOdd_yOdd: u16,
    pub w3_xEven_yEven: u16,
    pub w3_xOdd_yEven: u16,
    pub w3_xEven_yOdd: u16,
    pub w3_xOdd_yOdd: u16,
}
impl ::core::clone::Clone for Y2RU_DitheringWeightParams {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Y2RU_DitheringWeightParams {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

extern "C" {
    pub fn y2rInit() -> Result;
    pub fn y2rExit();
    pub fn Y2RU_SetInputFormat(format: Y2RU_InputFormat) -> Result;
    pub fn Y2RU_GetInputFormat(format: *mut Y2RU_InputFormat) -> Result;
    pub fn Y2RU_SetOutputFormat(format: Y2RU_OutputFormat) -> Result;
    pub fn Y2RU_GetOutputFormat(format: *mut Y2RU_OutputFormat) -> Result;
    pub fn Y2RU_SetRotation(rotation: Y2RU_Rotation) -> Result;
    pub fn Y2RU_GetRotation(rotation: *mut Y2RU_Rotation) -> Result;
    pub fn Y2RU_SetBlockAlignment(alignment: Y2RU_BlockAlignment) -> Result;
    pub fn Y2RU_GetBlockAlignment(alignment: *mut Y2RU_BlockAlignment)
     -> Result;
    pub fn Y2RU_SetSpacialDithering(enable: u8) -> Result;
    pub fn Y2RU_GetSpacialDithering(enabled: *mut u8) -> Result;
    pub fn Y2RU_SetTemporalDithering(enable: u8) -> Result;
    pub fn Y2RU_GetTemporalDithering(enabled: *mut u8) -> Result;
    pub fn Y2RU_SetInputLineWidth(line_width: u16) -> Result;
    pub fn Y2RU_GetInputLineWidth(line_width: *mut u16) -> Result;
    pub fn Y2RU_SetInputLines(num_lines: u16) -> Result;
    pub fn Y2RU_GetInputLines(num_lines: *mut u16) -> Result;
    pub fn Y2RU_SetCoefficients(coefficients: *const Y2RU_ColorCoefficients)
     -> Result;
    pub fn Y2RU_GetCoefficients(coefficients: *mut Y2RU_ColorCoefficients)
     -> Result;
    pub fn Y2RU_SetStandardCoefficient(coefficient: Y2RU_StandardCoefficient)
     -> Result;
    pub fn Y2RU_GetStandardCoefficient(coefficients:
                                           *mut Y2RU_ColorCoefficients,
                                       standardCoeff:
                                           Y2RU_StandardCoefficient)
     -> Result;
    pub fn Y2RU_SetAlpha(alpha: u16) -> Result;
    pub fn Y2RU_GetAlpha(alpha: *mut u16) -> Result;
    pub fn Y2RU_SetTransferEndInterrupt(should_interrupt: u8) -> Result;
    pub fn Y2RU_GetTransferEndInterrupt(should_interrupt: *mut u8) -> Result;
    pub fn Y2RU_GetTransferEndEvent(end_event: *mut Handle) -> Result;
    pub fn Y2RU_SetSendingY(src_buf: *const c_void,
                            image_size: u32, transfer_unit: s16,
                            transfer_gap: s16) -> Result;
    pub fn Y2RU_SetSendingU(src_buf: *const c_void,
                            image_size: u32, transfer_unit: s16,
                            transfer_gap: s16) -> Result;
    pub fn Y2RU_SetSendingV(src_buf: *const c_void,
                            image_size: u32, transfer_unit: s16,
                            transfer_gap: s16) -> Result;
    pub fn Y2RU_SetSendingYUYV(src_buf: *const c_void,
                               image_size: u32, transfer_unit: s16,
                               transfer_gap: s16) -> Result;
    pub fn Y2RU_SetReceiving(dst_buf: *mut c_void,
                             image_size: u32, transfer_unit: s16,
                             transfer_gap: s16) -> Result;
    pub fn Y2RU_IsDoneSendingY(is_done: *mut u8) -> Result;
    pub fn Y2RU_IsDoneSendingU(is_done: *mut u8) -> Result;
    pub fn Y2RU_IsDoneSendingV(is_done: *mut u8) -> Result;
    pub fn Y2RU_IsDoneSendingYUYV(is_done: *mut u8) -> Result;
    pub fn Y2RU_IsDoneReceiving(is_done: *mut u8) -> Result;
    pub fn Y2RU_SetDitheringWeightParams(params:
                                             *const Y2RU_DitheringWeightParams)
     -> Result;
    pub fn Y2RU_GetDitheringWeightParams(params:
                                             *mut Y2RU_DitheringWeightParams)
     -> Result;
    pub fn Y2RU_SetConversionParams(params: *const Y2RU_ConversionParams)
     -> Result;
    pub fn Y2RU_StartConversion() -> Result;
    pub fn Y2RU_StopConversion() -> Result;
    pub fn Y2RU_IsBusyConversion(is_busy: *mut u8) -> Result;
    pub fn Y2RU_PingProcess(ping: *mut u8) -> Result;
    pub fn Y2RU_DriverInitialize() -> Result;
    pub fn Y2RU_DriverFinalize() -> Result;
}
