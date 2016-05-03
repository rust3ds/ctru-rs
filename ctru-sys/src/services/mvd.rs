use ::Result;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum MVDSTD_Mode {
    MVDMODE_COLORFORMATCONV = 0,
    MVDMODE_VIDEOPROCESSING = 1,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum MVDSTD_InputFormat {
    MVD_INPUT_YUYV422 = 65537,
    MVD_INPUT_H264 = 131073,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum MVDSTD_OutputFormat { MVD_OUTPUT_RGB565 = 262146, }

#[repr(C)]
#[derive(Copy)]
pub struct MVDSTD_Config {
    pub input_type: MVDSTD_InputFormat,
    pub unk_x04: u32,
    pub unk_x08: u32,
    pub inwidth: u32,
    pub inheight: u32,
    pub physaddr_colorconv_indata: u32,
    pub unk_x18: [u32; 10usize],
    pub flag_x40: u32,
    pub unk_x44: u32,
    pub unk_x48: u32,
    pub outheight0: u32,
    pub outwidth0: u32,
    pub unk_x54: u32,
    pub output_type: MVDSTD_OutputFormat,
    pub outwidth1: u32,
    pub outheight1: u32,
    pub physaddr_outdata0: u32,
    pub physaddr_outdata1_colorconv: u32,
    pub unk_x6c: [u32; 44usize],
}
impl ::core::clone::Clone for MVDSTD_Config {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for MVDSTD_Config {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

extern "C" {
    pub fn mvdstdInit(mode: MVDSTD_Mode, input_type: MVDSTD_InputFormat,
                      output_type: MVDSTD_OutputFormat, size: u32) -> Result;
    pub fn mvdstdExit();
    pub fn mvdstdGenerateDefaultConfig(config: *mut MVDSTD_Config,
                                       input_width: u32, input_height: u32,
                                       output_width: u32,
                                       output_height: u32,
                                       vaddr_colorconv_indata: *mut u32,
                                       vaddr_outdata0: *mut u32,
                                       vaddr_outdata1_colorconv: *mut u32);
    pub fn mvdstdProcessFrame(config: *mut MVDSTD_Config,
                              h264_vaddr_inframe: *mut u32,
                              h264_inframesize: u32, h264_frameid: u32)
     -> Result;
    pub fn MVDSTD_SetConfig(config: *mut MVDSTD_Config) -> Result;
}
