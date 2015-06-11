use super::*;
use super::super::{Handle, Result};

extern crate core;
use core::clone::Clone;

#[repr(C)]
pub enum MemOp {
    MEMOP_FREE = 1,
    MEMOP_ALLOC = 3,
    MEMOP_MAP = 4,
    MEMOP_UNMAP = 5,
    MEMOP_PROT = 6,

    MEMOP_ALLOC_LINEAR = 0x10003,
}

#[repr(C)]
pub enum MemState {
	MEMSTATE_FREE       = 0,
	MEMSTATE_RESERVED   = 1,
	MEMSTATE_IO         = 2,
	MEMSTATE_STATIC     = 3,
	MEMSTATE_CODE       = 4,
	MEMSTATE_PRIVATE    = 5,
	MEMSTATE_SHARED     = 6,
	MEMSTATE_CONTINUOUS = 7,
	MEMSTATE_ALIASED    = 8,
	MEMSTATE_ALIAS      = 9,
	MEMSTATE_ALIASCODE  = 10,
	MEMSTATE_LOCKED     = 11
}

#[repr(C)]
pub enum MemPerm {
	MEMPERM_READ     = 1,
	MEMPERM_WRITE    = 2,
	MEMPERM_EXECUTE  = 4,
	MEMPERM_DONTCARE = 0x10000000,
	MEMPERM_MAX      = 0xFFFFFFFF //force 4-byte
}

#[repr(C)]
pub struct MemInfo {
    pub base_addr: u32,
    pub size: u32,
    pub perm: u32,
    pub state: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageInfo {
    pub flags: u32,
}

#[repr(C)]
pub enum ArbitrationType {
	ARBITER_FREE           =0,
	ARBITER_ACQUIRE        =1,
	ARBITER_KERNEL2        =2,
	ARBITER_ACQUIRE_TIMEOUT=3,
	ARBITER_KERNEL4        =4,
}

#[repr(C)]
pub enum DebugEventType {
	DBG_EVENT_PROCESS        = 0,
	DBG_EVENT_CREATE_THREAD  = 1,
	DBG_EVENT_EXIT_THREAD    = 2,
	DBG_EVENT_EXIT_PROCESS   = 3,
	DBG_EVENT_EXCEPTION      = 4,
	DBG_EVENT_DLL_LOAD       = 5,
	DBG_EVENT_DLL_UNLOAD     = 6,
	DBG_EVENT_SCHEDULE_IN    = 7,
	DBG_EVENT_SCHEDULE_OUT   = 8,
	DBG_EVENT_SYSCALL_IN     = 9,
	DBG_EVENT_SYSCALL_OUT    = 10,
	DBG_EVENT_OUTPUT_STRING  = 11,
	DBG_EVENT_MAP            = 12
}

#[repr(C)]
pub enum ProcessEventReason {
	REASON_CREATE = 1,
	REASON_ATTACH = 2
}

#[repr(C)]
#[derive(Copy)]
pub struct ProcessEvent {
    pub program_id: u64,
    pub process_name: [u8; 8usize],
    pub process_id: u32,
    pub reason: u32
}

impl Clone for ProcessEvent {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Copy)]
pub struct CreateThreadEvent {
    pub creator_thread_id: u32,
    pub base_addr: u32,
    pub entry_point: u32
}

impl Clone for CreateThreadEvent {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
pub enum ExitThreadEventReason {
	EXITTHREAD_EVENT_NONE              = 0,
	EXITTHREAD_EVENT_TERMINATE         = 1,
	EXITTHREAD_EVENT_UNHANDLED_EXC     = 2,
	EXITTHREAD_EVENT_TERMINATE_PROCESS = 3
}

#[repr(C)]
pub enum ExitProcessEventReason {
	EXITPROCESS_EVENT_NONE                = 0,
	EXITPROCESS_EVENT_TERMINATE           = 1,
	EXITPROCESS_EVENT_UNHANDLED_EXCEPTION = 2
}

#[repr(C)]
#[derive(Copy)]
pub struct ExitProcessEvent {
    pub reason: u32
}

impl Clone for ExitProcessEvent {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Copy)]
pub struct ExitThreadEvent {
    pub reason: u32
}

impl Clone for ExitThreadEvent {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Copy)]
pub struct ExceptionEvent {
    pub _type: u32,
    pub address: u32,
    pub argument: u32
}

impl Clone for ExceptionEvent {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
pub enum ExceptionEventType {
	EXC_EVENT_UNDEFINED_INSTRUCTION = 0, // arg: (None)
	EXC_EVENT_UNKNOWN1              = 1, // arg: (None)
	EXC_EVENT_UNKNOWN2              = 2, // arg: address
	EXC_EVENT_UNKNOWN3              = 3, // arg: address
	EXC_EVENT_ATTACH_BREAK          = 4, // arg: (None)
	EXC_EVENT_BREAKPOINT            = 5, // arg: (None)
	EXC_EVENT_USER_BREAK            = 6, // arg: user break type
	EXC_EVENT_DEBUGGER_BREAK        = 7, // arg: (None)
	EXC_EVENT_UNDEFINED_SYSCALL     = 8  // arg: attempted syscall
}

#[repr(C)]
pub enum UserBreakType {
	USERBREAK_PANIC  = 0,
	USERBREAK_ASSERT = 1,
	USERBREAK_USER   = 2
}

/**
* Type of the query for svcGetThreadInfo
*/
#[repr(C)]
pub enum ThreadInfoType {
	THREADINFO_TYPE_UNKNOWN = 0,
    VARIANT2 = 1, // needed because enums must have 2+ variants for C representation
}

#[repr(C)]
#[derive(Copy)]
pub struct SchedulerInOutEvent {
    pub clock_tick: u64
}

impl Clone for SchedulerInOutEvent {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Copy)]
pub struct SyscallInOutEvent {
    pub clock_tick: u64,
    pub syscall: u32,
}

impl Clone for SyscallInOutEvent {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Copy)]
pub struct OutputStringEvent {
    pub string_addr: u32,
    pub string_size: u32
}

impl Clone for OutputStringEvent {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Copy)]
pub struct MapEvent {
    pub mapped_addr: u32,
    pub mapped_size: u32,
    pub memperm: u32,
    pub memstate: u32
}

impl Clone for MapEvent {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Copy)]
pub struct DebugEventInfo {
    pub _type: u32,
    pub thread_id: u32,
    pub unknown: [u32; 2usize],
    pub eventUnion: [u64; 3usize], // must use transmutes to access contents
	// union {
	// 	ProcessEvent process;
	// 	CreateThreadEvent create_thread;
	// 	ExitThreadEvent exit_thread;
	// 	ExitProcessEvent exit_process;
	// 	ExceptionEvent exception;
	// 	/* TODO: DLL_LOAD */
	// 	/* TODO: DLL_UNLOAD */
	// 	SchedulerInOutEvent scheduler;
	// 	SyscallInOutEvent syscall;
	// 	OutputStringEvent output_string;
	// 	MapEvent map;
	// };
}

impl Clone for DebugEventInfo {
    fn clone(&self) -> Self { *self }
}

// getLocalThreadStorage and getThreadCommandBuffer can't be implemented
// due to asm. Custom build step may be necessary.

#[link(name="ctru")]
extern "C" {
    pub fn svcControlMemory(addr_out: *mut u32, addr0: u32, addr1: u32, size: u32, op: MemOp, perm: MemPerm) -> s32;
    pub fn svcQueryMemory(info: *mut MemInfo, out: *mut PageInfo, addr: u32) -> s32;
    pub fn svcExitProcess() -> ();
    pub fn svcCreateThread(thread: *mut Handle, entrypoint: ThreadFunc, arg: u32, stack_top: *mut u32, thread_priority: s32, processor_id: s32) -> s32;
    pub fn svcExitThread() -> ();
    pub fn svcSleepThread(ns: s64) -> ();
    pub fn svcSetThreadPriority(thread: Handle, prio: s32) -> s32;
    pub fn svcCreateMutex(mutex: *mut Handle, initially_locked: u8) -> s32;
    pub fn svcReleaseMutex(handle: Handle) -> s32;
    pub fn svcCreateSemaphore(semaphore: *mut Handle, initial_count: s32, max_count: s32) -> s32;
    pub fn svcReleaseSemaphore(count: *mut s32, semaphore: Handle, release_count: s32) -> s32;
    pub fn svcCreateEvent(event: *mut Handle, reset_type: u8) -> s32;
    pub fn svcSignalEvent(handle: Handle) -> s32;
    pub fn svcClearEvent(handle: Handle) -> s32;
    pub fn svcCreateTimer(timer: *mut Handle, reset_type: u8) -> s32;
    pub fn svcSetTimer(timer: Handle, initial: s64, interval: s64) -> s32;
    pub fn svcCancelTimer(timer: Handle) -> s32;
    pub fn svcClearTimer(timer: Handle) -> s32;
    pub fn svcCreateMemoryBlock(memblock: *mut Handle, addr: u32, size: u32, my_perm: MemPerm, other_perm: MemPerm) -> s32;
    pub fn svcMapMemoryBlock(memblock: Handle, addr: u32, my_perm: MemPerm, other_perm: MemPerm) -> s32;
    pub fn svcUnmapMemoryBlock(memblock: Handle, addr: u32) -> s32;
    pub fn svcCreateAddressArbiter(arbiter: *mut Handle) -> s32;
    pub fn svcArbitrateAddress(arbiter: Handle, addr: u32, _type: ArbitrationType, value: s32, nanoseconds: s64) -> s32;
    pub fn svcWaitSynchronization(handle: Handle, nanoseconds: s64) -> s32;
    pub fn svcWaitSynchronizationN(out: *mut s32, handles: *mut Handle, handles_num: s32, wait_all: u8, nanoseconds: s64) -> s32;
    pub fn svcCloseHandle(handle: Handle) -> s32;
    pub fn svcDuplicateHandle(out: *mut Handle, original: Handle) -> s32;
    pub fn svcGetSystemTick() -> u64;
    pub fn svcGetSystemInfo(out: *mut s64, _type: u32, param: s32) -> s32;
    pub fn svcGetProcessInfo(out: *mut s64, process: Handle, _type: u32) -> s32;
    pub fn svcConnectToPort(out: *mut Handle, portName: *const u8) -> s32;
    pub fn svcSendSyncRequest(session: Handle) -> s32;
    pub fn svcOpenProcess(process: *mut Handle, processId: u32) -> Result;
    pub fn svcGetProcessId(out: *mut u32, handle: Handle) -> s32;
    pub fn svcGetThreadId(out: *mut u32, handle: Handle) -> s32;
    pub fn svcOutputDebugString(string: *const u8, length: i32) -> s32;
    pub fn svcCreatePort(portServer: *mut Handle, portClient: *mut Handle, name: *const u8, maxSessions: s32) -> Result;
    pub fn svcDebugActiveProcess(debug: *mut Handle, processId: u32) -> Result;
    pub fn svcBreakDebugProcess(debug: Handle) -> Result;
    pub fn svcTerminateDebugProcess(debug: Handle) -> Result;
    pub fn svcGetProcessDebugEvent(info: *mut DebugEventInfo, debug: Handle) -> Result;
    pub fn svcContinueDebugEvent(debug: Handle, flags: u32) -> Result;
    pub fn svcGetProcessList(processCount: *mut s32, processIds: *mut u32, processIdMaxCount: s32) -> Result;
    pub fn svcReadProcessMemory(buffer: *mut u8, debug: Handle, addr: u32, size: u32) -> Result;
    pub fn svcMapProcessMemory(process: Handle, startAddr: u32, endAddr: u32) -> Result;
    pub fn svcUnmapProcessMemory(process: Handle, startAddr: u32, endAddr: u32) -> Result;
    pub fn svcQueryProcessMemory(info: *mut MemInfo, out: *mut PageInfo, process: Handle, addr: u32) -> Result;
    pub fn svcGetProcessorID() -> s32;
}
