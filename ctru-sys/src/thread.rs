use {Handle, Result};
use c_void;
use ThreadFunc;

pub enum Struct_Thread_tag { }
pub type Thread = *mut Struct_Thread_tag;
extern "C" {
    pub fn threadCreate(entrypoint: ThreadFunc,
                        arg: *mut c_void, stack_size: usize,
                        prio: i32,
                        affinity: i32, detached: u8)
     -> Thread;
    pub fn threadGetHandle(thread: Thread) -> Handle;
    pub fn threadGetExitCode(thread: Thread) -> i32;
    pub fn threadFree(thread: Thread);
    pub fn threadJoin(thread: Thread, timeout_ns: u64) -> Result;
    pub fn threadGetCurrent() -> Thread;
    pub fn threadExit(rc: i32);
}
