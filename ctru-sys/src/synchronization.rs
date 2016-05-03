//TODO: Implement stuff that bindgen doesn't catch

use Handle;

use super::lock::*;

pub type LightLock = _LOCK_T;
pub type RecursiveLock = _LOCK_RECURSIVE_T;
extern "C" {
    pub fn __sync_get_arbiter() -> Handle;
    pub fn LightLock_Init(lock: *mut LightLock);
    pub fn LightLock_Lock(lock: *mut LightLock);
    pub fn LightLock_TryLock(lock: *mut LightLock) -> i32;
    pub fn LightLock_Unlock(lock: *mut LightLock);
    pub fn RecursiveLock_Init(lock: *mut RecursiveLock);
    pub fn RecursiveLock_Lock(lock: *mut RecursiveLock);
    pub fn RecursiveLock_TryLock(lock: *mut RecursiveLock) -> i32;
    pub fn RecursiveLock_Unlock(lock: *mut RecursiveLock);
}
