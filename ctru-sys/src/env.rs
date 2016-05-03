//TODO: There are a bunch of static inline functions that bindgen didn't pick up and idk how they work

use ::Handle;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed1 {
    RUNFLAG_APTWORKAROUND = 1,
    RUNFLAG_APTREINIT = 2,
}

extern "C" {
    pub fn envGetHandle(name: *const u8) -> Handle;
}
