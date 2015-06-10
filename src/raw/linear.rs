use super::c_void;

#[link(name = "ctru")]
extern "C" {
    pub fn linearAlloc(size: isize) -> *mut c_void;
    pub fn linearMemAlign(size: isize, alignment: isize) -> *mut c_void;
    pub fn linearRealloc(mem: *mut c_void, size: isize) -> *mut c_void;
    pub fn linearFree(mem: *mut c_void) -> ();
    pub fn linearSpaceFree() -> u32;
}
