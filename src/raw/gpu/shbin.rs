typedef enum{
	VERTEX_SHDR=GPU_VERTEX_SHADER,
	GEOMETRY_SHDR=GPU_GEOMETRY_SHADER
}DVLE_type;

#[repr(C)]
pub enum DVLE_type {
    VERTEX_SHDR=GPU_VERTEX_SHADER,
    GEOMETRY_SHDR=GPU_GEOMETRY_SHADER,
}

typedef enum{
	DVLE_CONST_BOOL=0x0,
	DVLE_CONST_u8=0x1,
	DVLE_CONST_FLOAT24=0x2,
}DVLE_constantType;

#[repr(C)]
pub enum DVLE_constantType {
    DVLE_CONST_BOOL = 0x0,
    DVLE_CONST_u8 = 0x1,
    DVLE_CONST_FLOAT24 = 0x2,
}

#[repr(C)]
pub enum DVLE_outputAttribute_t {
	RESULT_POSITION = 0x0,
	RESULT_NORMALQUAT = 0x1,
	RESULT_COLOR = 0x2,
	RESULT_TEXCOORD0 = 0x3,
	RESULT_TEXCOORD0W = 0x4,
	RESULT_TEXCOORD1 = 0x5,
	RESULT_TEXCOORD2 = 0x6,
	RESULT_VIEW = 0x8
}

#[repr(C)]
#[derive(Copy)]
pub struct DVLP_s {
	codeSize: u32,
	codeData: *mut u32,
	opdescSize: u32,
	opcdescData: *mut u32
}

#[repr(C)]
#[derive(Copy)]
pub struct DVLE_constEntry_s {
	type: u16,
	id: u16,
	data: [u32; 4usize]
}

#[repr(C)]
#[derive(Copy)]
pub struct DVLE_outEntry_s {
	type: u16,
	regID: u16,
	mask: u8,
	unk: [u8; 3usize]
}

#[repr(C)]
#[derive(Copy)]
pub struct DVLE_uniformEntry_s{
	symbolOffset: u32,
	startReg: u16,
	endReg: u16,
}

#[repr(C)]
#[derive(Copy)]
pub struct DVLE_s {
	DVLE_type type: DVLE_type,
	DVLP_s* dvlp: *mut DVLP_s,
    mainOffset: u32,
    endmainOffset: u32,
    constTableSize: u32,
    constTableData: *mut DVLE_constEntry_s,
    outTableSize: u32,
    outTableData: *mut DVLE_outEntry_s,
    uniformTableSize: u32,
    uniformTableData: *mut DVLE_uniformEntry_s,
    symbolTableData: *mut u8,
    outmapMask: u8,
    outmapData: [u32; 8usize]
}

#[repr(C)]
#[derive(Copy)]
pub struct DVLB_s {
    numDVLE: u32,
    DVLP: DVLP_s,
    DVLE: *mut DVLE_s
}

use ctru::raw::types::*;

#[link(name = "ctru")]
extern "C" {
    pub fn DVLB_ParseFile(shbinData: *mut u32, shbinSize: u32) -> *mut DVLB_s;
    pub fn DVLB_Free(dvlb: *mut DVLB_s) -> ();
    pub fn DVLE_GetUniformRegister(dvle: *mut DVLE_s, name: *const u8) -> s8;
    pub fn DVLE_GenerateOutmap(dvle: *mut DVLE_s) -> ();
}
