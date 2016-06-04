//TODO: Implement static functions

use {Handle, Result};
use libc::c_void;
use ThreadFunc;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed1 {
    MEMOP_FREE = 1,
    MEMOP_RESERVE = 2,
    MEMOP_ALLOC = 3,
    MEMOP_MAP = 4,
    MEMOP_UNMAP = 5,
    MEMOP_PROT = 6,
    MEMOP_REGION_APP = 256,
    MEMOP_REGION_SYSTEM = 512,
    MEMOP_REGION_BASE = 768,
    MEMOP_OP_MASK = 255,
    MEMOP_REGION_MASK = 3840,
    MEMOP_LINEAR_FLAG = 65536,
    MEMOP_ALLOC_LINEAR = 65539,
}
pub type MemOp = Enum_Unnamed1;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed2 {
    MEMSTATE_FREE = 0,
    MEMSTATE_RESERVED = 1,
    MEMSTATE_IO = 2,
    MEMSTATE_STATIC = 3,
    MEMSTATE_CODE = 4,
    MEMSTATE_PRIVATE = 5,
    MEMSTATE_SHARED = 6,
    MEMSTATE_CONTINUOUS = 7,
    MEMSTATE_ALIASED = 8,
    MEMSTATE_ALIAS = 9,
    MEMSTATE_ALIASCODE = 10,
    MEMSTATE_LOCKED = 11,
}
pub type MemState = Enum_Unnamed2;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed3 {
    MEMPERM_READ = 1,
    MEMPERM_WRITE = 2,
    MEMPERM_EXECUTE = 4,
    MEMPERM_DONTCARE = 268435456,
}
pub type MemPerm = Enum_Unnamed3;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed4 {
    pub base_addr: u32,
    pub size: u32,
    pub perm: u32,
    pub state: u32,
}
impl ::core::clone::Clone for Struct_Unnamed4 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed4 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type MemInfo = Struct_Unnamed4;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed5 {
    pub flags: u32,
}
impl ::core::clone::Clone for Struct_Unnamed5 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed5 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type PageInfo = Struct_Unnamed5;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed6 {
    ARBITRATION_SIGNAL = 0,
    ARBITRATION_WAIT_IF_LESS_THAN = 1,
    ARBITRATION_DECREMENT_AND_WAIT_IF_LESS_THAN = 2,
    ARBITRATION_WAIT_IF_LESS_THAN_TIMEOUT = 3,
    ARBITRATION_DECREMENT_AND_WAIT_IF_LESS_THAN_TIMEOUT = 4,
}
pub type ArbitrationType = Enum_Unnamed6;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed7 { THREADINFO_TYPE_UNKNOWN = 0, }
pub type ThreadInfoType = Enum_Unnamed7;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed8 { REASON_CREATE = 1, REASON_ATTACH = 2, }
pub type ProcessEventReason = Enum_Unnamed8;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed9 {
    pub program_id: u64,
    pub process_name: [u8; 8usize],
    pub process_id: u32,
    pub reason: u32,
}
impl ::core::clone::Clone for Struct_Unnamed9 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed9 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type ProcessEvent = Struct_Unnamed9;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed10 {
    EXITPROCESS_EVENT_NONE = 0,
    EXITPROCESS_EVENT_TERMINATE = 1,
    EXITPROCESS_EVENT_UNHANDLED_EXCEPTION = 2,
}
pub type ExitProcessEventReason = Enum_Unnamed10;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed11 {
    pub reason: u32,
}
impl ::core::clone::Clone for Struct_Unnamed11 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed11 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type ExitProcessEvent = Struct_Unnamed11;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed12 {
    pub creator_thread_id: u32,
    pub base_addr: u32,
    pub entry_point: u32,
}
impl ::core::clone::Clone for Struct_Unnamed12 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed12 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type CreateThreadEvent = Struct_Unnamed12;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed13 {
    EXITTHREAD_EVENT_NONE = 0,
    EXITTHREAD_EVENT_TERMINATE = 1,
    EXITTHREAD_EVENT_UNHANDLED_EXC = 2,
    EXITTHREAD_EVENT_TERMINATE_PROCESS = 3,
}
pub type ExitThreadEventReason = Enum_Unnamed13;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed14 {
    pub reason: u32,
}
impl ::core::clone::Clone for Struct_Unnamed14 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed14 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type ExitThreadEvent = Struct_Unnamed14;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed15 {
    USERBREAK_PANIC = 0,
    USERBREAK_ASSERT = 1,
    USERBREAK_USER = 2,
}
pub type UserBreakType = Enum_Unnamed15;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed16 {
    EXC_EVENT_UNDEFINED_INSTRUCTION = 0,
    EXC_EVENT_UNKNOWN1 = 1,
    EXC_EVENT_UNKNOWN2 = 2,
    EXC_EVENT_UNKNOWN3 = 3,
    EXC_EVENT_ATTACH_BREAK = 4,
    EXC_EVENT_BREAKPOINT = 5,
    EXC_EVENT_USER_BREAK = 6,
    EXC_EVENT_DEBUGGER_BREAK = 7,
    EXC_EVENT_UNDEFINED_SYSCALL = 8,
}
pub type ExceptionEventType = Enum_Unnamed16;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed17 {
    pub _type: u32,
    pub address: u32,
    pub argument: u32,
}
impl ::core::clone::Clone for Struct_Unnamed17 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed17 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type ExceptionEvent = Struct_Unnamed17;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed18 {
    pub clock_tick: u64,
}
impl ::core::clone::Clone for Struct_Unnamed18 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed18 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type SchedulerInOutEvent = Struct_Unnamed18;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed19 {
    pub clock_tick: u64,
    pub syscall: u32,
}
impl ::core::clone::Clone for Struct_Unnamed19 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed19 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type SyscallInOutEvent = Struct_Unnamed19;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed20 {
    pub string_addr: u32,
    pub string_size: u32,
}
impl ::core::clone::Clone for Struct_Unnamed20 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed20 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type OutputStringEvent = Struct_Unnamed20;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed21 {
    pub mapped_addr: u32,
    pub mapped_size: u32,
    pub memperm: u32,
    pub memstate: u32,
}
impl ::core::clone::Clone for Struct_Unnamed21 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed21 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type MapEvent = Struct_Unnamed21;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed22 {
    DBG_EVENT_PROCESS = 0,
    DBG_EVENT_CREATE_THREAD = 1,
    DBG_EVENT_EXIT_THREAD = 2,
    DBG_EVENT_EXIT_PROCESS = 3,
    DBG_EVENT_EXCEPTION = 4,
    DBG_EVENT_DLL_LOAD = 5,
    DBG_EVENT_DLL_UNLOAD = 6,
    DBG_EVENT_SCHEDULE_IN = 7,
    DBG_EVENT_SCHEDULE_OUT = 8,
    DBG_EVENT_SYSCALL_IN = 9,
    DBG_EVENT_SYSCALL_OUT = 10,
    DBG_EVENT_OUTPUT_STRING = 11,
    DBG_EVENT_MAP = 12,
}
pub type DebugEventType = Enum_Unnamed22;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed23 {
    pub _type: u32,
    pub thread_id: u32,
    pub unknown: [u32; 2usize],
    pub _bindgen_data_1_: [u64; 3usize],
}
impl Struct_Unnamed23 {
    pub unsafe fn process(&mut self) -> *mut ProcessEvent {
        let raw: *mut u8 = ::core::mem::transmute(&self._bindgen_data_1_);
        ::core::mem::transmute(raw.offset(0))
    }
    pub unsafe fn create_thread(&mut self) -> *mut CreateThreadEvent {
        let raw: *mut u8 = ::core::mem::transmute(&self._bindgen_data_1_);
        ::core::mem::transmute(raw.offset(0))
    }
    pub unsafe fn exit_thread(&mut self) -> *mut ExitThreadEvent {
        let raw: *mut u8 = ::core::mem::transmute(&self._bindgen_data_1_);
        ::core::mem::transmute(raw.offset(0))
    }
    pub unsafe fn exit_process(&mut self) -> *mut ExitProcessEvent {
        let raw: *mut u8 = ::core::mem::transmute(&self._bindgen_data_1_);
        ::core::mem::transmute(raw.offset(0))
    }
    pub unsafe fn exception(&mut self) -> *mut ExceptionEvent {
        let raw: *mut u8 = ::core::mem::transmute(&self._bindgen_data_1_);
        ::core::mem::transmute(raw.offset(0))
    }
    pub unsafe fn scheduler(&mut self) -> *mut SchedulerInOutEvent {
        let raw: *mut u8 = ::core::mem::transmute(&self._bindgen_data_1_);
        ::core::mem::transmute(raw.offset(0))
    }
    pub unsafe fn syscall(&mut self) -> *mut SyscallInOutEvent {
        let raw: *mut u8 = ::core::mem::transmute(&self._bindgen_data_1_);
        ::core::mem::transmute(raw.offset(0))
    }
    pub unsafe fn output_string(&mut self) -> *mut OutputStringEvent {
        let raw: *mut u8 = ::core::mem::transmute(&self._bindgen_data_1_);
        ::core::mem::transmute(raw.offset(0))
    }
    pub unsafe fn map(&mut self) -> *mut MapEvent {
        let raw: *mut u8 = ::core::mem::transmute(&self._bindgen_data_1_);
        ::core::mem::transmute(raw.offset(0))
    }
}
impl ::core::clone::Clone for Struct_Unnamed23 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed23 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type DebugEventInfo = Struct_Unnamed23;
extern "C" {
    pub fn svcControlMemory(addr_out: *mut u32, addr0: u32, addr1: u32,
                            size: u32, op: MemOp, perm: MemPerm) -> Result;
    pub fn svcControlProcessMemory(process: Handle, addr0: u32, addr1: u32,
                                   size: u32, _type: u32, perm: u32)
     -> Result;
    pub fn svcCreateMemoryBlock(memblock: *mut Handle, addr: u32, size: u32,
                                my_perm: MemPerm, other_perm: MemPerm)
     -> Result;
    pub fn svcMapMemoryBlock(memblock: Handle, addr: u32, my_perm: MemPerm,
                             other_perm: MemPerm) -> Result;
    pub fn svcMapProcessMemory(process: Handle, startAddr: u32,
                               endAddr: u32) -> Result;
    pub fn svcUnmapProcessMemory(process: Handle, startAddr: u32,
                                 endAddr: u32) -> Result;
    pub fn svcUnmapMemoryBlock(memblock: Handle, addr: u32) -> Result;
    pub fn svcStartInterProcessDma(dma: *mut Handle, dstProcess: Handle,
                                   dst: *mut c_void,
                                   srcProcess: Handle,
                                   src: *const c_void,
                                   size: u32,
                                   dmaConfig: *mut c_void)
     -> Result;
    pub fn svcStopDma(dma: Handle) -> Result;
    pub fn svcGetDmaState(dmaState: *mut c_void, dma: Handle)
     -> Result;
    pub fn svcQueryMemory(info: *mut MemInfo, out: *mut PageInfo, addr: u32)
     -> Result;
    pub fn svcQueryProcessMemory(info: *mut MemInfo, out: *mut PageInfo,
                                 process: Handle, addr: u32) -> Result;
    pub fn svcInvalidateProcessDataCache(process: Handle,
                                         addr: *mut c_void,
                                         size: u32) -> Result;
    pub fn svcFlushProcessDataCache(process: Handle,
                                    addr: *const c_void,
                                    size: u32) -> Result;
    pub fn svcReadProcessMemory(buffer: *mut c_void,
                                debug: Handle, addr: u32, size: u32)
     -> Result;
    pub fn svcWriteProcessMemory(debug: Handle,
                                 buffer: *const c_void,
                                 addr: u32, size: u32) -> Result;
    pub fn svcOpenProcess(process: *mut Handle, processId: u32) -> Result;
    pub fn svcExitProcess();
    pub fn svcTerminateProcess(process: Handle) -> Result;
    pub fn svcGetProcessInfo(out: *mut s64, process: Handle, _type: u32)
     -> Result;
    pub fn svcGetProcessId(out: *mut u32, handle: Handle) -> Result;
    pub fn svcGetProcessList(processCount: *mut s32, processIds: *mut u32,
                             processIdMaxCount: s32) -> Result;
    pub fn svcCreatePort(portServer: *mut Handle, portClient: *mut Handle,
                         name: *const u8,
                         maxSessions: s32) -> Result;
    pub fn svcConnectToPort(out: *mut Handle,
                            portName: *const u8)
     -> Result;
    pub fn svcCreateThread(thread: *mut Handle, entrypoint: ThreadFunc,
                           arg: u32, stack_top: *mut u32,
                           thread_priority: s32, processor_id: s32) -> Result;
    pub fn svcOpenThread(thread: *mut Handle, process: Handle, threadId: u32)
     -> Result;
    pub fn svcExitThread();
    pub fn svcSleepThread(ns: s64);
    pub fn svcGetThreadPriority(out: *mut s32, handle: Handle) -> Result;
    pub fn svcSetThreadPriority(thread: Handle, prio: s32) -> Result;
    pub fn svcGetThreadAffinityMask(affinitymask: *mut u8, thread: Handle,
                                    processorcount: s32) -> Result;
    pub fn svcSetThreadAffinityMask(thread: Handle, affinitymask: *mut u8,
                                    processorcount: s32) -> Result;
    pub fn svcGetThreadIdealProcessor(processorid: *mut s32, thread: Handle)
     -> Result;
    pub fn svcSetThreadIdealProcessor(thread: Handle, processorid: s32)
     -> Result;
    pub fn svcGetProcessorID() -> s32;
    pub fn svcGetThreadId(out: *mut u32, handle: Handle) -> Result;
    pub fn svcGetProcessIdOfThread(out: *mut u32, handle: Handle) -> Result;
    pub fn svcGetThreadInfo(out: *mut s64, thread: Handle,
                            _type: ThreadInfoType) -> Result;
    pub fn svcCreateMutex(mutex: *mut Handle, initially_locked: u8) -> Result;
    pub fn svcReleaseMutex(handle: Handle) -> Result;
    pub fn svcCreateSemaphore(semaphore: *mut Handle, initial_count: s32,
                              max_count: s32) -> Result;
    pub fn svcReleaseSemaphore(count: *mut s32, semaphore: Handle,
                               release_count: s32) -> Result;
    pub fn svcCreateEvent(event: *mut Handle, reset_type: u8) -> Result;
    pub fn svcSignalEvent(handle: Handle) -> Result;
    pub fn svcClearEvent(handle: Handle) -> Result;
    pub fn svcWaitSynchronization(handle: Handle, nanoseconds: s64) -> Result;
    pub fn svcWaitSynchronizationN(out: *mut s32, handles: *mut Handle,
                                   handles_num: s32, wait_all: u8,
                                   nanoseconds: s64) -> Result;
    pub fn svcCreateAddressArbiter(arbiter: *mut Handle) -> Result;
    pub fn svcArbitrateAddress(arbiter: Handle, addr: u32,
                               _type: ArbitrationType, value: s32,
                               nanoseconds: s64) -> Result;
    pub fn svcSendSyncRequest(session: Handle) -> Result;
    pub fn svcAcceptSession(session: *mut Handle, port: Handle) -> Result;
    pub fn svcReplyAndReceive(index: *mut s32, handles: *mut Handle,
                              handleCount: s32, replyTarget: Handle)
     -> Result;
    pub fn svcCreateTimer(timer: *mut Handle, reset_type: u8) -> Result;
    pub fn svcSetTimer(timer: Handle, initial: s64, interval: s64) -> Result;
    pub fn svcCancelTimer(timer: Handle) -> Result;
    pub fn svcClearTimer(timer: Handle) -> Result;
    pub fn svcGetSystemTick() -> u64;
    pub fn svcCloseHandle(handle: Handle) -> Result;
    pub fn svcDuplicateHandle(out: *mut Handle, original: Handle) -> Result;
    pub fn svcGetSystemInfo(out: *mut s64, _type: u32, param: s32) -> Result;
    pub fn svcKernelSetState(_type: u32, param0: u32, param1: u32,
                             param2: u32) -> Result;
    pub fn svcBreak(breakReason: UserBreakType);
    pub fn svcOutputDebugString(str: *const u8,
                                length: i32) -> Result;
    pub fn svcDebugActiveProcess(debug: *mut Handle, processId: u32)
     -> Result;
    pub fn svcBreakDebugProcess(debug: Handle) -> Result;
    pub fn svcTerminateDebugProcess(debug: Handle) -> Result;
    pub fn svcGetProcessDebugEvent(info: *mut DebugEventInfo, debug: Handle)
     -> Result;
    pub fn svcContinueDebugEvent(debug: Handle, flags: u32) -> Result;
    pub fn svcBackdoor(callback:
                           ::core::option::Option<extern "C" fn() -> s32>)
     -> Result;
}
