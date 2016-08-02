pub const STDOUT_FILENO: i32 = 1;

#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

extern "C" {
    pub fn abort() -> !;
    pub fn write(fd: i32, buf: *const c_void, count: usize) -> isize;
}
