use super::c_void;

#[link(name = "ctru")]
extern "C" {
    pub fn linearAlloc(size: i32) -> *mut c_void;
    pub fn linearMemAlign(size: i32, alignment: i32) -> *mut c_void;
    pub fn linearRealloc(mem: *mut c_void, size: i32) -> *mut c_void;
    pub fn linearFree(mem: *mut c_void) -> ();
    pub fn linearSpaceFree() -> u32;
}
