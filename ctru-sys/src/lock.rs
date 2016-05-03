//<sys/lock.h> from devkitArm, needed for synchronization.rs to compile

//TODO: I don't even know this thing looks really spooky

pub type _LOCK_T = i32;
#[repr(C)]
#[derive(Copy)]
pub struct Struct___lock_t {
    pub lock: _LOCK_T,
    pub thread_tag: u32,
    pub counter: u32,
}
impl ::core::clone::Clone for Struct___lock_t {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct___lock_t {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type _LOCK_RECURSIVE_T = Struct___lock_t;
extern "C" {
    pub fn __libc_lock_init(lock: *mut _LOCK_T);
    pub fn __libc_lock_init_recursive(lock: *mut _LOCK_RECURSIVE_T);
    pub fn __libc_lock_close(lock: *mut _LOCK_T);
    pub fn __libc_lock_close_recursive(lock: *mut _LOCK_RECURSIVE_T);
    pub fn __libc_lock_acquire(lock: *mut _LOCK_T);
    pub fn __libc_lock_acquire_recursive(lock: *mut _LOCK_RECURSIVE_T);
    pub fn __libc_lock_release(lock: *mut _LOCK_T);
    pub fn __libc_lock_release_recursive(lock: *mut _LOCK_RECURSIVE_T);
    pub fn __libc_lock_try_acquire(lock: *mut _LOCK_T)
     -> i32;
    pub fn __libc_lock_try_acquire_recursive(lock: *mut _LOCK_RECURSIVE_T)
     -> i32;
}
