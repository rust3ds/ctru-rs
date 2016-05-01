use super::shbin::*;

#[repr(C)]
#[derive(Copy)]
pub struct float24Uniform_s {
    id: u32,
    data: [u32; 3usize]
}

#[repr(C)]
#[derive(Copy)]
pub struct shaderInstance_s {
    pub dvle: *mut DVLE_s;
    pub boolUniforms: u16,
    pub intUniforms: [u32; 4usize],
    pub float24Uniforms: *mut float24Uniform_s,
    pub numFloat24Uniforms: u8,
}

#[repr(C)]
#[derive(Copy)]
pub struct shaderProgram_s {
    pub vertexShader: *mut shaderInstance_s,
    pub geometryShader: *mut shaderInstance_s,
    pub geometryShaderInputStride: u8,
}

use ctru::Result;


extern "C" {
    pub fn shaderInstanceInit(si: *mut shaderInstance_s, dvle: *mut DVLE_s) -> Result;
    pub fn shaderInstanceFree(si: *mut shaderInstance_s) -> Result;
    pub fn shaderInstanceSetBool(si: *mut shaderInstance_s, id: ::libc::c_int, value: u8) -> Result;
    pub fn shaderInstanceGetBool(si: *mut shaderInstance_s, id: ::libc::c_int, value: *mut u8) -> Result;
    pub fn shaderInstanceGetUniformLocation(si: *mut shaderInstance_s, name: *const ::libc::c_char) -> Result;
    pub fn shaderProgramInit(sp: *mut shaderProgram_s) -> Result;
    pub fn shaderProgramFree(sp: *mut shaderProgram_s) -> Result;
    pub fn shaderProgramSetVsh(sp: *mut shaderProgram_s, dvle: *mut DVLE_s) -> Result;
    pub fn shaderProgramSetGsh(sp: *mut shaderProgram_s, dvle: *mut DVLE_s, stride: _u8) -> Result;
    pub fn shaderProgramUse(sp: *mut shaderProgram_s) -> Result;
}
