use ::raw::gfx;

pub fn init_default() -> () {
    unsafe {
        gfx::gfxInitDefault();
    }
}

pub fn exit() -> () {
    unsafe {
        gfx::gfxExit();
    }
}

pub fn set_3d_enabled(enabled: bool) -> () {
    unsafe {
        gfx::gfxSet3D(match enabled { true => 1u8, false => 0u8 });
    }
}

pub fn flush_buffers() -> () {
    unsafe {
        gfx::gfxFlushBuffers();
    }
}

pub fn swap_buffers() -> () {
    unsafe {
        gfx::gfxSwapBuffers();
    }
}
