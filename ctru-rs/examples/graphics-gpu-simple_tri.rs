use citro3d_sys::C3D_Mtx;
use ctru::gfx::{Gfx, Screen, Side};
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};
use ctru_sys::{shaderProgram_s, DVLB_s};

use std::ffi::CString;
use std::mem::MaybeUninit;

const VERTICES: [Vertex; 3] = [
    Vertex::new(200.0, 200.0, 0.5),
    Vertex::new(100.0, 40.0, 0.5),
    Vertex::new(300.0, 40.0, 0.5),
];

fn main() {
    ctru::init();

    let gfx = Gfx::default();
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");

    let top_screen = gfx.top_screen.borrow_mut();

    let target = unsafe {
        citro3d_sys::C3D_Init(citro3d_sys::C3D_DEFAULT_CMDBUF_SIZE);

        let depth_fmt = citro3d_sys::C3D_DEPTHTYPE {
            __e: ctru_sys::GPU_RB_DEPTH24_STENCIL8,
        };

        let target =
            citro3d_sys::C3D_RenderTargetCreate(240, 400, ctru_sys::GPU_RB_RGBA8, depth_fmt);

        // TODO: easier construction of flags
        let transfer_flags =
            ctru_sys::GX_TRANSFER_FMT_RGBA8 << 8 | ctru_sys::GX_TRANSFER_FMT_RGB8 << 12;

        citro3d_sys::C3D_RenderTargetSetOutput(
            target,
            top_screen.as_raw(),
            Side::Left.into(),
            transfer_flags,
        );

        target
    };

    let (program, uloc_projection, projection, vbo_data, vshader_dvlb) = scene_init();

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }

        const CLEAR_COLOR: u32 = 0x68B0D8FF;

        unsafe {
            citro3d_sys::C3D_FrameBegin(citro3d_sys::C3D_FRAME_SYNCDRAW as u8);

            citro3d_sys::C3D_RenderTargetClear(target, citro3d_sys::C3D_CLEAR_ALL, CLEAR_COLOR, 0);
            citro3d_sys::C3D_FrameDrawOn(target);
        }
        scene_render(uloc_projection.into(), &projection);
        unsafe {
            citro3d_sys::C3D_FrameEnd(0);
        }
    }

    scene_exit(vbo_data, program, vshader_dvlb);

    unsafe {
        citro3d_sys::C3D_Fini();
    }
}
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

impl Vertex {
    const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

fn scene_init() -> (shaderProgram_s, i8, C3D_Mtx, *mut libc::c_void, *mut DVLB_s) {
    // Load the vertex shader, create a shader program and bind it
    unsafe {
        // To compile the shader:
        // ```sh
        // picasso assets/vshader.v.pica -o assets/vshader.shbin
        // ```
        // TODO: can we do this in a build script?

        // boo, this way we have to specify the length. Alternative seems to be allocating a
        // slice first, then copying into it with a transmute...
        const SHBIN_BYTES: &[u8; 280] = include_bytes!("assets/vshader.shbin");
        // Assume the data is aligned properly...
        let mut shbin_data: [u32; SHBIN_BYTES.len() / 4] = std::mem::transmute_copy(SHBIN_BYTES);

        let vshader_dvlb = ctru_sys::DVLB_ParseFile(
            shbin_data.as_mut_ptr(),
            shbin_data
                .len()
                .try_into()
                .expect("shader len fits in a u32"),
        );
        let mut program = MaybeUninit::<ctru_sys::shaderProgram_s>::uninit();

        ctru_sys::shaderProgramInit(program.as_mut_ptr());
        ctru_sys::shaderProgramSetVsh(program.as_mut_ptr(), (*vshader_dvlb).DVLE);

        let mut program = program.assume_init();

        citro3d_sys::C3D_BindProgram(&mut program);

        // Get the location of the uniforms
        let projection_name = CString::new("projection").unwrap();
        let uloc_projection = ctru_sys::shaderInstanceGetUniformLocation(
            program.vertexShader,
            projection_name.as_ptr(),
        );

        // Configure attributes for use with the vertex shader
        let attr_info = citro3d_sys::C3D_GetAttrInfo();
        citro3d_sys::AttrInfo_Init(attr_info);
        citro3d_sys::AttrInfo_AddLoader(attr_info, 0, ctru_sys::GPU_FLOAT, 3); // v0=position
        citro3d_sys::AttrInfo_AddFixed(attr_info, 1); // v1=color

        // Set the fixed attribute (color) to solid white
        citro3d_sys::C3D_FixedAttribSet(1, 1.0, 1.0, 1.0, 1.0);

        let mut projection = MaybeUninit::<citro3d_sys::C3D_Mtx>::uninit();
        // Compute the projection matrix
        citro3d_sys::Mtx_OrthoTilt(
            projection.as_mut_ptr(),
            0.0,
            400.0,
            0.0,
            240.0,
            0.0,
            1.0,
            true,
        );
        let projection = projection.assume_init();

        let vertices_len = std::mem::size_of_val(&VERTICES);

        // Create the VBO (vertex buffer object)
        let vbo_data =
            ctru_sys::linearAlloc(vertices_len.try_into().expect("size does not fit in u32"));

        vbo_data.copy_from(VERTICES.as_ptr() as _, vertices_len);

        // Configure buffers
        let buf_info = citro3d_sys::C3D_GetBufInfo();
        citro3d_sys::BufInfo_Init(buf_info);
        citro3d_sys::BufInfo_Add(
            buf_info,
            vbo_data,
            std::mem::size_of::<Vertex>()
                .try_into()
                .expect("size of vertex fits in u32"),
            1,
            0x0,
        );

        // Configure the first fragment shading substage to just pass through the vertex color
        // See https://www.opengl.org/sdk/docs/man2/xhtml/glTexEnv.xml for more insight
        let env = citro3d_sys::C3D_GetTexEnv(0);
        citro3d_sys::C3D_TexEnvInit(env);
        citro3d_sys::C3D_TexEnvSrc(
            env,
            citro3d_sys::C3D_Both as i32,
            ctru_sys::GPU_PRIMARY_COLOR as i32,
            0,
            0,
        );
        citro3d_sys::C3D_TexEnvFunc(
            env,
            citro3d_sys::C3D_Both as i32,
            ctru_sys::GPU_REPLACE as i32,
        );

        (program, uloc_projection, projection, vbo_data, vshader_dvlb)
    }
}

fn scene_render(uloc_projection: i32, projection: &C3D_Mtx) {
    unsafe {
        // Update the uniforms
        citro3d_sys::C3D_FVUnifMtx4x4(ctru_sys::GPU_VERTEX_SHADER, uloc_projection, projection);

        // Draw the VBO
        citro3d_sys::C3D_DrawArrays(ctru_sys::GPU_TRIANGLES, 0, VERTICES.len() as i32);
    }
}

fn scene_exit(
    vbo_data: *mut libc::c_void,
    mut program: shaderProgram_s,
    vshader_dvlb: *mut DVLB_s,
) {
    unsafe {
        ctru_sys::linearFree(vbo_data);

        ctru_sys::shaderProgramFree(&mut program);

        ctru_sys::DVLB_Free(vshader_dvlb);
    }
}
