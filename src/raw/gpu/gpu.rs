use ctru::Handle;

#[inline]
pub fn GPUCMD_HEADER(incremental: i32, mask: i32, reg: i32) {
    (((incremental)<<31)|(((mask)&0xF)<<16)|((reg)&0x3FF));
}

#[inline]
pub fn GPU_TEXTURE_MAG_FILTER(v: i32) {
    (((v)&0x1)<<1); //takes a GPU_TEXTURE_FILTER_PARAM
}

#[inline]
pub fn GPU_TEXTURE_MIN_FILTER(v: i32) {
    (((v)&0x1)<<2); //takes a GPU_TEXTURE_FILTER_PARAM
}

#[inline]
pub fn GPU_TEXTURE_WRAP_S(v: i32) {
    (((v)&0x3)<<8); //takes a GPU_TEXTURE_WRAP_PARAM
}

#[inline]
pub fn GPU_TEXTURE_WRAP_T(v: i32) {
    (((v)&0x3)<<12); //takes a GPU_TEXTURE_WRAP_PARAM
}

#[repr(C)]
pub enum GPU_TEXTURE_FILTER_PARAM {
    GPU_NEAREST = 0x0,
    GPU_LINEAR = 0x1
}

#[repr(C)]
pub enum GPU_TEXTURE_WRAP_PARAM {
    GPU_CLAMP_TO_EDGE = 0x0,
    GPU_REPEAT = 0x1
}

#[repr(C)]
pub enum GPU_TEXUNIT {
	GPU_TEXUNIT0 = 0x1,
	GPU_TEXUNIT1 = 0x2,
	GPU_TEXUNIT2 = 0x4
}

#[repr(C)]
pub enum GPU_TEXCOLOR {
	GPU_RGBA8=0x0,
	GPU_RGB8=0x1,
	GPU_RGBA5551=0x2,
	GPU_RGB565=0x3,
	GPU_RGBA4=0x4,
	GPU_LA8=0x5,
	GPU_HILO8=0x6,
	GPU_L8=0x7,
	GPU_A8=0x8,
	GPU_LA4=0x9,
	GPU_L4=0xA,
	GPU_ETC1=0xB,
	GPU_ETC1A4=0xC
}

#[repr(C)]
pub enum GPU_TESTFUNC {
	GPU_NEVER = 0,
	GPU_ALWAYS = 1,
	GPU_EQUAL = 2,
	GPU_NOTEQUAL = 3,
	GPU_LESS = 4,
	GPU_LEQUAL = 5,
	GPU_GREATER = 6,
	GPU_GEQUAL = 7
}

#[repr(C)]
pub enum GPU_SCISSORMODE {
	GPU_SCISSOR_DISABLE = 0,	// disable scissor test
	GPU_SCISSOR_INVERT = 1,		// exclude pixels inside the scissor box
	// 2 is the same as 0
	GPU_SCISSOR_NORMAL = 3,		// exclude pixels outside of the scissor box
}

#[repr(C)]
pub enum GPU_STENCILOP {
	GPU_KEEP = 0, 		// keep destination value
	GPU_AND_NOT = 1, 	// destination & ~source
	GPU_XOR = 5,		// destination ^ source
	// 2 is the same as 1. Other values are too weird to even be usable.
}

#[repr(C)]
pub enum GPU_WRITEMASK {
	GPU_WRITE_RED = 0x01,
	GPU_WRITE_GREEN = 0x02,
	GPU_WRITE_BLUE = 0x04,
	GPU_WRITE_ALPHA = 0x08,
	GPU_WRITE_DEPTH = 0x10,

	GPU_WRITE_COLOR = 0x0F,
	GPU_WRITE_ALL = 0x1F
}

#[repr(C)]
pub enum GPU_BLENDEQUATION {
	GPU_BLEND_ADD = 0,
	GPU_BLEND_SUBTRACT = 1,
	GPU_BLEND_REVERSE_SUBTRACT = 2,
	GPU_BLEND_MIN = 3,
	GPU_BLEND_MAX = 4
}

#[repr(C)]
pub enum GPU_BLENDFACTOR {
	GPU_ZERO = 0,
	GPU_ONE = 1,
	GPU_SRC_COLOR = 2,
	GPU_ONE_MINUS_SRC_COLOR = 3,
	GPU_DST_COLOR = 4,
	GPU_ONE_MINUS_DST_COLOR = 5,
	GPU_SRC_ALPHA = 6,
	GPU_ONE_MINUS_SRC_ALPHA = 7,
	GPU_DST_ALPHA = 8,
	GPU_ONE_MINUS_DST_ALPHA = 9,
	GPU_CONSTANT_COLOR = 10,
	GPU_ONE_MINUS_CONSTANT_COLOR = 11,
	GPU_CONSTANT_ALPHA = 12,
	GPU_ONE_MINUS_CONSTANT_ALPHA = 13,
	GPU_SRC_ALPHA_SATURATE = 14
}

#[repr(C)]
pub enum GPU_LOGICOP {
	GPU_LOGICOP_CLEAR = 0,
	GPU_LOGICOP_AND = 1,
	GPU_LOGICOP_AND_REVERSE = 2,
	GPU_LOGICOP_COPY = 3,
	GPU_LOGICOP_SET = 4,
	GPU_LOGICOP_COPY_INVERTED = 5,
	GPU_LOGICOP_NOOP = 6,
	GPU_LOGICOP_INVERT = 7,
	GPU_LOGICOP_NAND = 8,
	GPU_LOGICOP_OR = 9,
	GPU_LOGICOP_NOR = 10,
	GPU_LOGICOP_XOR = 11,
	GPU_LOGICOP_EQUIV = 12,
	GPU_LOGICOP_AND_INVERTED = 13,
	GPU_LOGICOP_OR_REVERSE = 14,
	GPU_LOGICOP_OR_INVERTED = 15
}

#[repr(C)]
pub enum GPU_FORMATS {
	GPU_BYTE = 0,
	GPU_UNSIGNED_BYTE = 1,
	GPU_SHORT = 2,
	GPU_FLOAT = 3
}

//defines for CW ?
#[repr(C)]
pub enum GPU_CULLMODE {
	GPU_CULL_NONE = 0,
	GPU_CULL_FRONT_CCW = 1,
	GPU_CULL_BACK_CCW = 2
}

#[inline]
pub fn GU_ATTRIBFMT(i: i32, n: i32, f: i32) {
    (((((n)-1)<<2)|((f)&3))<<((i)*4));
}

/**
* Texture combiners sources
*/
#[repr(C)]
pub enum GPU_TEVSRC{
	GPU_PRIMARY_COLOR = 0x00,
	GPU_TEXTURE0 = 0x03,
	GPU_TEXTURE1 = 0x04,
	GPU_TEXTURE2 = 0x05,
	GPU_TEXTURE3 = 0x06,
	GPU_CONSTANT = 0x0E,
	GPU_PREVIOUS = 0x0F,
}

/**
* Texture RGB combiners operands
*/
#[repr(C)]
pub enum GPU_TEVOP_RGB{
	GPU_TEVOP_RGB_SRC_COLOR = 0x00,
	GPU_TEVOP_RGB_ONE_MINUS_SRC_COLOR = 0x01,
	GPU_TEVOP_RGB_SRC_ALPHA = 0x02,
	GPU_TEVOP_RGB_ONE_MINUS_SRC_ALPHA = 0x03,
	GPU_TEVOP_RGB_SRC0_RGB = 0x04,
	GPU_TEVOP_RGB_0x05 = 0x05,
	GPU_TEVOP_RGB_0x06 = 0x06,
	GPU_TEVOP_RGB_0x07 = 0x07,
	GPU_TEVOP_RGB_SRC1_RGB = 0x08,
	GPU_TEVOP_RGB_0x09 = 0x09,
	GPU_TEVOP_RGB_0x0A = 0x0A,
	GPU_TEVOP_RGB_0x0B = 0x0B,
	GPU_TEVOP_RGB_SRC2_RGB = 0x0C,
	GPU_TEVOP_RGB_0x0D = 0x0D,
	GPU_TEVOP_RGB_0x0E = 0x0E,
	GPU_TEVOP_RGB_0x0F = 0x0F,
};

/**
* Texture ALPHA combiners operands
*/
#[repr(C)]
pub enum GPU_TEVOP_A {
	GPU_TEVOP_A_SRC_ALPHA = 0x00,
	GPU_TEVOP_A_ONE_MINUS_SRC_ALPHA = 0x01,
	GPU_TEVOP_A_SRC0_RGB = 0x02,
	GPU_TEVOP_A_SRC1_RGB = 0x04,
	GPU_TEVOP_A_SRC2_RGB = 0x06,
}

/**
* Texture combiner functions
*/
pub enum GPU_COMBINEFUNC {
	GPU_REPLACE = 0x00,
	GPU_MODULATE = 0x01,
	GPU_ADD = 0x02,
	GPU_ADD_SIGNED = 0x03,
	GPU_INTERPOLATE = 0x04,
	GPU_SUBTRACT = 0x05,
	GPU_DOT3_RGB = 0x06 //RGB only
}

#[inline]
pub fn GPU_TEVSOURCES(a, b, c) {
    (((a))|((b)<<4)|((c)<<8));
}

#[inline]
pub fn GPU_TEVOPERANDS(a, b, c) {
    (((a))|((b)<<4)|((c)<<8));
}

#[repr(C)]
pub enum GPU_Primitive_t {
	GPU_TRIANGLES = 0x0000,
	GPU_TRIANGLE_STRIP = 0x0100,
	GPU_TRIANGLE_FAN = 0x0200,
	GPU_UNKPRIM = 0x0300 // ?
}

#[repr(C)]
pub enum GPU_SHADER_TYPE {
	GPU_VERTEX_SHADER=0x0,
	GPU_GEOMETRY_SHADER=0x1
}

#[link(name = "ctru")]
extern "C" {
    pub fn GPU_Init(gsphandle: *mut Handle) -> ();
    pub fn GPU_Reset(gxbuf: *mut u32, gpuBuf: *mut u32, gpuBufSize: u32) -> ();

    pub fn GPUCMD_SetBuffer(adr: *mut u32, size: u32, offset: u32) -> ();
    pub fn GPUCMD_SetBufferOffset(offset: u32) -> ();
    pub fn GPUCMD_GetBuffer(adr: *mut *mut u32, size: *mut u32, offset: *mut u32) -> ();
    pub fn GPUCMD_AddRawCommands(cmd: *mut u32, size: u32) -> ();
    pub fn GPUCMD_Run(gxbuf: *mut u32) -> ();
    pub fn GPUCMD_FlushAndRun(gxbuf: *mut u32) -> ();
    pub fn GPUCMD_Add(header: u32, param: *mut u32, paramlength: u32) -> ();
    pub fn GPUCMD_Finalize() -> ();
    pub fn GPU_SetFloatUniform(_type: GPU_SHADER_TYPE, startreg: u32, data: *mut u32, numreg: u32) -> ();
    pub fn GPU_SetViewport(depthBuffer: *mut u32, colorBuffer: *mut u32, x: u32, y: u32, w: u32, h: u32) -> ();
    pub fn GPU_SetScissorTest(mode: GPU_SCISSORMODE, x: u32, y: u32, w: u32, h: u32) -> ();
    pub fn GPU_DepthMap(zScale: f32, zOffset: f32) -> ();
    pub fn GPU_SetAlphaTest(enable: u8, function: GPU_TESTFUNC, _ref: u8) -> ();
    pub fn GPU_SetDepthTestAndWriteMask(enable: u8, function: GPU_TESTFUNC, writemask: GPU_WRITEMASK) -> ();
    pub fn GPU_SetStencilTest(enable: u8, function: GPU_TESTFUNC, _ref: _u8, mask: _u8, replace: _u8) -> ();
    pub fn GPU_SetStencilOp(sfail: GPU_STENCILOP, dfail: GPU_STENCILOP, pass: GPU_STENCILOP) -> ();
    pub fn GPU_SetFaceCulling(mode: GPU_CULLMODE) -> ();
    pub fn GPU_SetAlphaBlending(colorEquation: GPU_BLENDEQUATION, alphaEquation: GPU_BLENDEQUATION, colorSrc: GPU_BLENDFACTOR, colorDst: GPU_BLENDFACTOR, alphaSrc: GPU_BLENDFACTOR, alphaDst: GPU_BLENDFACTOR) -> ();
    pub fn GPU_SetColorLogicOp(op: GPU_LOGICOP) -> ();
    pub fn GPU_SetBlendingColor(r: u8, g: u8, b: u8, a: u8) -> ();
    pub fn GPU_SetAttributeBuffers(totalAttributes: u8, baseAddress: *mut u32, attributeFormats: u64, attributeMask: u16, attributePermutation: u64, numBuffers: u8, bufferOffsets: *mut u32, bufferPermutations: *mut u64, bufferNumAttributes: *mut u8) -> ();
    pub fn GPU_SetTextureEnable(units: GPU_TEXUNIT) -> ();
    pub fn GPU_SetTexture(unit: GPU_TEXUNIT, data: *mut u32, width: u16, height: u16, param: u32, colorType: GPU_TEXCOLOR) -> ();
    pub fn GPU_SetTexEnv(id: u8, rgbSources: u16, alphaSources: u16, rgbOperands: u16, alphaOperands: u16, rgbCombine: GPU_COMBINEFUNC, alphaCombine: GPU_COMBINEFUNC, constantColor: u32) -> ();
    pub fn GPU_DrawArray(primitive: GPU_Primitive_t, n: u32) -> ();
    pub fn GPU_DrawElements(primitive: GPU_Primitive_t, indexArray: *mut u32, n: u32) -> ();
    pub fn GPU_FinishDrawing() -> ();
    pub fn GPU_SetShaderOutmap(outmapData: *mut u32) -> ();
    pub fn GPU_SendShaderCode(_type: GPU_SHADER_TYPE, data: *mut u32, offset: u16, length: u16) -> ();
    pub fn GPU_SendOperandDescriptors(_type: GPU_SHADER_TYPE, data: *mut u32, offset: u16, length: u16) -> ();
}
