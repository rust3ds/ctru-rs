use super::types::*;

#[link(name = "ctru")]
extern "C" {
    pub fn vramAlloc(size: isize) -> *mut c_void;
    pub fn vramMemAlign(size: isize, alignment: isize) -> *mut c_void;
    pub fn vramRealloc(mem: *mut isize, size: isize) -> *mut c_void;
    pub fn vramFree(mem: *mut c_void) -> ();
    pub fn vramSpaceFree() -> u32;
}
