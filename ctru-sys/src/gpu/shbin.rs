use ::types::*;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed1 {
    VERTEX_SHDR = 0,
    GEOMETRY_SHDR = 1,
}
pub type DVLE_type = Enum_Unnamed1;
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed2 {
    DVLE_CONST_BOOL = 0,
    DVLE_CONST_u8 = 1,
    DVLE_CONST_FLOAT24 = 2,
}
pub type DVLE_constantType = Enum_Unnamed2;
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed3 {
    RESULT_POSITION = 0,
    RESULT_NORMALQUAT = 1,
    RESULT_COLOR = 2,
    RESULT_TEXCOORD0 = 3,
    RESULT_TEXCOORD0W = 4,
    RESULT_TEXCOORD1 = 5,
    RESULT_TEXCOORD2 = 6,
    RESULT_VIEW = 8,
}
pub type DVLE_outputAttribute_t = Enum_Unnamed3;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed4 {
    pub codeSize: u32,
    pub codeData: *mut u32,
    pub opdescSize: u32,
    pub opcdescData: *mut u32,
}
impl ::core::clone::Clone for Struct_Unnamed4 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed4 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type DVLP_s = Struct_Unnamed4;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed5 {
    pub _type: u16,
    pub id: u16,
    pub data: [u32; 4usize],
}
impl ::core::clone::Clone for Struct_Unnamed5 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed5 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type DVLE_constEntry_s = Struct_Unnamed5;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed6 {
    pub _type: u16,
    pub regID: u16,
    pub mask: u8,
    pub unk: [u8; 3usize],
}
impl ::core::clone::Clone for Struct_Unnamed6 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed6 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type DVLE_outEntry_s = Struct_Unnamed6;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed7 {
    pub symbolOffset: u32,
    pub startReg: u16,
    pub endReg: u16,
}
impl ::core::clone::Clone for Struct_Unnamed7 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed7 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type DVLE_uniformEntry_s = Struct_Unnamed7;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed8 {
    pub _type: DVLE_type,
    pub dvlp: *mut DVLP_s,
    pub mainOffset: u32,
    pub endmainOffset: u32,
    pub constTableSize: u32,
    pub constTableData: *mut DVLE_constEntry_s,
    pub outTableSize: u32,
    pub outTableData: *mut DVLE_outEntry_s,
    pub uniformTableSize: u32,
    pub uniformTableData: *mut DVLE_uniformEntry_s,
    pub symbolTableData: *mut u8,
    pub outmapMask: u8,
    pub outmapData: [u32; 8usize],
    pub outmapMode: u32,
    pub outmapClock: u32,
}
impl ::core::clone::Clone for Struct_Unnamed8 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed8 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type DVLE_s = Struct_Unnamed8;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed9 {
    pub numDVLE: u32,
    pub DVLP: DVLP_s,
    pub DVLE: *mut DVLE_s,
}
impl ::core::clone::Clone for Struct_Unnamed9 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed9 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type DVLB_s = Struct_Unnamed9;
extern "C" {
    pub fn DVLB_ParseFile(shbinData: *mut u32, shbinSize: u32)
     -> *mut DVLB_s;
    pub fn DVLB_Free(dvlb: *mut DVLB_s);
    pub fn DVLE_GetUniformRegister(dvle: *mut DVLE_s,
                                   name: *const u8) -> s8;
    pub fn DVLE_GenerateOutmap(dvle: *mut DVLE_s);
}
