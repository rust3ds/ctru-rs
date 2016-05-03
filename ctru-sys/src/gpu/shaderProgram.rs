use ::Result;
use ::types::*;
use gpu::shbin::*;


#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub id: u32,
    pub data: [u32; 3usize],
}
impl ::core::clone::Clone for Struct_Unnamed1 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed1 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type float24Uniform_s = Struct_Unnamed1;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed2 {
    pub dvle: *mut DVLE_s,
    pub boolUniforms: u16,
    pub boolUniformMask: u16,
    pub intUniforms: [u32; 4usize],
    pub float24Uniforms: *mut float24Uniform_s,
    pub intUniformMask: u8,
    pub numFloat24Uniforms: u8,
}
impl ::core::clone::Clone for Struct_Unnamed2 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type shaderInstance_s = Struct_Unnamed2;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed3 {
    pub vertexShader: *mut shaderInstance_s,
    pub geometryShader: *mut shaderInstance_s,
    pub geoShaderInputPermutation: [u32; 2usize],
    pub geoShaderInputStride: u8,
    pub geoShaderMode: u8,
}
impl ::core::clone::Clone for Struct_Unnamed3 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed3 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type shaderProgram_s = Struct_Unnamed3;
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed4 {
    GSH_NORMAL = 0,
    GSH_PARTICLE = 1,
    GSH_SUBDIVISION_LOOP = 2,
    GSH_SUBDIVISION_CATMULL_CLARK = 3,
}
pub type geoShaderMode = Enum_Unnamed4;
extern "C" {
    pub fn shaderInstanceInit(si: *mut shaderInstance_s, dvle: *mut DVLE_s)
     -> Result;
    pub fn shaderInstanceFree(si: *mut shaderInstance_s) -> Result;
    pub fn shaderInstanceSetBool(si: *mut shaderInstance_s,
                                 id: i32, value: u8)
     -> Result;
    pub fn shaderInstanceGetBool(si: *mut shaderInstance_s,
                                 id: i32, value: *mut u8)
     -> Result;
    pub fn shaderInstanceGetUniformLocation(si: *mut shaderInstance_s,
                                            name:
                                                *const u8)
     -> s8;
    pub fn shaderProgramInit(sp: *mut shaderProgram_s) -> Result;
    pub fn shaderProgramFree(sp: *mut shaderProgram_s) -> Result;
    pub fn shaderProgramSetVsh(sp: *mut shaderProgram_s, dvle: *mut DVLE_s)
     -> Result;
    pub fn shaderProgramSetGsh(sp: *mut shaderProgram_s, dvle: *mut DVLE_s,
                               stride: u8) -> Result;
    pub fn shaderProgramSetGshInputPermutation(sp: *mut shaderProgram_s,
                                               permutation: u64) -> Result;
    pub fn shaderProgramSetGshMode(sp: *mut shaderProgram_s,
                                   mode: geoShaderMode) -> Result;
    pub fn shaderProgramConfigure(sp: *mut shaderProgram_s, sendVshCode: u8,
                                  sendGshCode: u8) -> Result;
    pub fn shaderProgramUse(sp: *mut shaderProgram_s) -> Result;
}
