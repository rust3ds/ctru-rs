use std::arch::asm;
use libc::c_void;
use ctru_sys::{DebugEventInfo, svcAcceptSession, svcBreak, svcContinueDebugEvent, svcCreateEvent, svcCreateMemoryBlock, svcDebugActiveProcess, svcExitProcess, svcGetProcessDebugEvent, svcGetProcessInfo, svcGetProcessList, svcOpenProcess, svcQueryDebugProcessMemory, svcReadProcessMemory, svcReplyAndReceive, svcSignalEvent, svcSleepThread, svcUnmapMemoryBlock, svcWaitSynchronization, svcWriteProcessMemory};
use crate::Error;

#[derive(PartialEq, Debug)]
pub struct Handle(u32);

/// An abstraction on top of resource handles to enforce type safety.
/// A Handle should only be made from a resource that is guaranteed to be a unique copy.
/// When a Handle is dropped, the underlying resource handle is closed.
/// Handles are intentionally non-copyable to avoid using Handles that have already been closed.
impl Handle {
    /// Returns the raw u32 handle
    /// # Safety
    /// Because a Handle closes itself when it's dropped, a raw handle might have been previously closed.
    /// The user must guarantee the handle will outlive the raw handle (and all copies/clones of the raw handle)
    ///
    /// Admittedly this is less of memory safety and more of logical safety, but since that's the purpose of this abstraction
    /// unsafe will be used in this way.
    pub unsafe fn get_raw(&self) -> u32 {
        self.0
    }

    /// Returns a pseudo handle for the current process
    pub fn get_current_process_handle() -> Self {
        0xFFFF8001.into()
    }
}

impl From<u32> for Handle {
    fn from(raw_handle: u32) -> Self {
        Self(raw_handle)
    }
}

impl Drop for Handle {
    // If this doesn't close, there's not much to recover from
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        close_handle(self.0);
    }
}

/// Reasons for a user to break
#[repr(u32)]
pub enum UserBreakType {
    Panic = ctru_sys::USERBREAK_PANIC,
    Assert = ctru_sys::USERBREAK_ASSERT,
    User = ctru_sys::USERBREAK_USER,
    LoadRo = ctru_sys::USERBREAK_LOAD_RO,
    UnloadRo = ctru_sys::USERBREAK_UNLOAD_RO,
}

impl From<UserBreakType> for u32 {
    fn from(break_type: UserBreakType) -> Self {
        break_type as u32
    }
}

#[repr(u32)]
pub enum EventResetType {
    OneShot = ctru_sys::RESET_ONESHOT,
    Sticky = ctru_sys::RESET_STICKY,
    Pulse = ctru_sys::RESET_PULSE,
}

#[repr(u32)]
pub enum MemoryPermission {
    None = 0,
    Read = 1,
    Write = 2,
    ReadWrite = 3,
    Execute = 4,
    ReadExecute = 5,
    DontCare = 0x10000000,
}

impl From<MemoryPermission> for u32 {
    fn from(perm: MemoryPermission) -> Self {
        perm as u32
    }
}

pub struct MemoryBlock<'a> {
    handle: Handle,
    slice: &'a mut [u8],
}

impl<'a> Drop for MemoryBlock<'a> {
    // If this doesn't close, there's not much to recover from
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        unmap_memory_block(&self.handle, self.slice);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum DebugFlag {
    InhibitUserCpuExceptionHandlers = 1,
    SignalFaultExceptionEvents = 2,
    InhibitUserCpuExceptionHandlersAndSignalFaultExceptionEvents = 3,
    SignalScheduleEvents = 4,
    SignalSyscallEvents = 8,
    SignalMapEvents = 16,
}

impl From<DebugFlag> for u32 {
    fn from(flag: DebugFlag) -> Self {
        flag as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ProcessInfoType {
    TitleId = 0x10001,
    StartAddress = 0x10005,
}

impl From<ProcessInfoType> for u32 {
    fn from(p_type: ProcessInfoType) -> Self {
        p_type as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemInfo {
    pub base_addr: u32,
    pub size: u32,
    pub perm: u32,
    pub state: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PageInfo {
    pub flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemQueryResponse {
    pub mem_info: MemInfo,
    pub page_info: PageInfo,
}

#[inline(never)]
/// Sends a sync request.
/// This is often used with atomic handles, which are u32s instead of Handles.
/// As a result, this takes a u32 to be more generic, and to avoid converting a u32 to a Handle, then immediately back into a u32.
pub fn control_service(service_op: u32, service_name: &str) -> crate::Result<Handle> {
    let mut result_code: i32;
    let mut handle: u32 = 0;

    unsafe {
        asm!("svc 0xB0", in("r0") service_op, in("r1") &mut handle, in("r2") service_name.as_ptr(), lateout("r0") result_code);
    }

    if result_code < 0 {
        Err(result_code.into())
    } else {
        Ok(handle.into())
    }

}

#[inline(never)]
/// Sends a sync request.
/// This is often used with atomic handles, which are u32s instead of Handles.
/// As a result, this takes a u32 to be more generic, and to avoid converting a u32 to a Handle, then immediately back into a u32.
pub fn send_raw_sync_request(raw_handle: u32) -> crate::Result<()> {
    let mut result_code: i32;

    unsafe {
        asm!("svc 0x32", in("r0") raw_handle, lateout("r0") result_code);
    }

    if result_code < 0 {
        Err(result_code.into())
    } else {
        Ok(())
    }
}

#[inline(never)]
/// Closes a handle.
/// This is pretty much only for implementing Drop on Handle.
/// If you're thinking about using this, consider using a Handle and let it manage closing the underlying handle.
pub fn close_handle(handle: u32) -> crate::Result<()> {
    unsafe {
        let result = ctru_sys::svcCloseHandle(handle);
        if result < 0 {
            Err(result.into())
        } else {
            Ok(())
        }
    }
}

#[inline(never)]
/// Breaks execution.
pub fn break_execution(reason: UserBreakType) -> ! {
    unsafe {
        svcBreak(reason.into())
    }

    // Allow the empty loop to get the 'never' return type
    // We'll never reach this far because the above will break anyways
    #[allow(clippy::empty_loop)]
    loop {}
}

/// Accepts a session to a service.
pub fn accept_session(port: &Handle) -> crate::Result<Handle> {
    let mut raw_handle = 0;
    let result = unsafe { svcAcceptSession(&mut raw_handle, port.get_raw()) };

    if result < 0 {
        Err(result.into())
    } else {
        Ok(raw_handle.into())
    }
}

/// Replies to a request and receives a new request.
pub fn reply_and_receive(raw_handles: &[u32], reply_target: Option<usize>) -> (usize, i32) {
    let raw_reply_target_handle = match reply_target {
        Some(target_index) => raw_handles[target_index],
        None => 0,
    };

    let mut index = -1;

    let result = unsafe {
        svcReplyAndReceive(
            &mut index,
            raw_handles.as_ptr(),
            // If the handle count is wrong, there's not much we can do to recover
            raw_handles.len().try_into().unwrap(),
            raw_reply_target_handle,
        )
            .into()
    };

    (index as usize, result)
}

pub fn create_event(reset_type: EventResetType) -> crate::Result<Handle> {
    let mut raw_handle = 0;
    let result = unsafe { svcCreateEvent(&mut raw_handle, reset_type as u32) };

    if result < 0 {
        Err(result.into())
    } else {
        Ok(raw_handle.into())
    }
}

pub fn sleep_thread(nanoseconds: i64) {
    unsafe { svcSleepThread(nanoseconds) }
}

pub fn signal_event(event: &Handle) -> crate::Result<()> {
    let result = unsafe { svcSignalEvent(event.get_raw()) };
    if result < 0 {
        Err(result.into())
    } else {
        Ok(())
    }
}

pub fn exit_process() -> ! {
    unsafe {
        svcExitProcess();
    }

    // Allow the empty loop to get the 'never' return type
    // We'll never reach this far because the above will break anyways
    #[allow(clippy::empty_loop)]
    loop {}
}

pub fn create_memory_block(
    slice: &mut [u8],
    my_permission: MemoryPermission,
    other_process_permission: MemoryPermission,
) -> crate::Result<Handle> {
    // Check alignment
    // svc::create_memory_block can only take alignments of 0x1000
    if (slice.as_ptr() as u32 & (0x1000 - 1)) != 0 {
        // Invalid alignment error
        return Err(0xc0de008.into());
    }

    let mut handle: u32 = 0;
    let result = unsafe {
        svcCreateMemoryBlock(
            &mut handle,
            slice.as_mut_ptr() as u32,
            slice.len() as u32,
            my_permission as u32,
            other_process_permission as u32,
        )
    };

    if result < 0 {
        Err(result.into())
    } else {
        Ok(handle.into())
    }
}

pub fn unmap_memory_block(memory_block_handle: &Handle, slice: &[u8]) -> crate::Result<()> {
    let result =
        unsafe { svcUnmapMemoryBlock(memory_block_handle.get_raw(), slice.as_ptr() as u32) };

    if result < 0 {
        Err(result.into())
    } else {
        Ok(())
    }
}

pub fn wait_synchronization(handle: &Handle, wait_nanoseconds: i64) -> crate::Result<()> {
    let result = unsafe { svcWaitSynchronization(handle.get_raw(), wait_nanoseconds) };

    if result < 0 {
        Err(result.into())
    } else {
        Ok(())
    }
}

pub fn get_process_list() -> crate::Result<Vec<u32>> {
    let mut process_ids: Vec<u32> = vec![0; 0x40];
    let mut process_count = 0;
    let result = unsafe {
        svcGetProcessList(
            &mut process_count,
            process_ids.as_mut_ptr(),
            process_ids.len() as i32,
        )
    };

    if result < 0 {
        Err(result.into())
    } else {
        process_ids.truncate(process_count as usize);
        Ok(process_ids)
    }
}

pub fn open_process(process_id: u32) -> crate::Result<Handle> {
    let mut raw_handle = 0;
    let result = unsafe { svcOpenProcess(&mut raw_handle, process_id) };

    if result < 0 {
        Err(result.into())
    } else {
        Ok(raw_handle.into())
    }

}

pub fn debug_active_process(process_id: u32) -> crate::Result<Handle> {
    let mut raw_handle = 0u32;
    let result = unsafe { svcDebugActiveProcess(&mut raw_handle, process_id) };

    if result < 0 {
        Err(result.into())
    } else {
        Ok(raw_handle.into())
    }
}

pub fn read_process_memory(debug_process: &Handle, addr: u32, size: u32) -> crate::Result<Vec<u8>> {
    let mut buffer = vec![0; size as usize];
    let result = unsafe {
        svcReadProcessMemory(
            buffer.as_mut_ptr() as *mut c_void,
            debug_process.get_raw(),
            addr,
            size,
        )
    };

    if result < 0 {
        Err(result.into())
    } else {
        Ok(buffer)
    }
}

pub fn write_process_memory(debug_process: &Handle, buffer: &[u8], addr: u32) -> crate::Result<()> {
    let result = unsafe {
        svcWriteProcessMemory(
            debug_process.get_raw(),
            buffer.as_ptr() as *mut c_void,
            addr,
            buffer.len() as u32,
        )
    };

    if result < 0 {
        Err(result.into())
    } else {
        Ok(())
    }
}

pub fn continue_debug_event(debug_process: &Handle, flag: DebugFlag) -> crate::Result<()> {
    let result = unsafe { svcContinueDebugEvent(debug_process.get_raw(), flag.into()) };

    if result < 0 {
        Err(result.into())
    } else {
        Ok(())
    }

}

// TODO: Real implementation.  This is hacked together for now.
pub fn get_process_debug_event(debug_process: &Handle) -> crate::Result<()> {
    let mut info: [u8; 0x28] = [0; 0x28];
    unsafe {
        let result = svcGetProcessDebugEvent(
            std::mem::transmute::<*mut u8, *mut DebugEventInfo>(info.as_mut_ptr()),
            debug_process.get_raw(),
        );

        if result < 0 {
            Err(result.into())
        } else {
            Ok(())
        }
    }
}

// Thanks to Luma3ds
pub fn eat_events(debug_process: &Handle) -> crate::Result<()> {
    loop {
        if let Err(err) = get_process_debug_event(debug_process) {
            if let Error::Os(result) = err {
                if result as u32 == 0xd8402009  {
                    break;
                }
            }
        }
        continue_debug_event(
            debug_process,
            DebugFlag::InhibitUserCpuExceptionHandlersAndSignalFaultExceptionEvents,
        )?;
    }

    Ok(())
}

pub fn get_process_info(process: &Handle, info_type: ProcessInfoType) -> crate::Result<i64> {
    let mut out = 0;
    let result = unsafe { svcGetProcessInfo(&mut out, process.get_raw(), info_type.into()) };

    if result < 0 {
        Err(result.into())
    } else {
        Ok(out)
    }
}

#[inline(never)]
pub fn copy_handle(out_process: &Handle, input: &Handle, in_process: &Handle) -> crate::Result<Handle> {
    let mut result: i32;
    let mut out_handle = 0u32;
    unsafe {
        asm!(
        "
            str r0, [sp, #-4]!
            svc 0xB1
            ldr r2, [sp], #4
            str r1, [r2]
            ",
        in("r0") &mut out_handle,
        in("r1") out_process.get_raw(),
        in("r2") input.get_raw(),
        in("r3") in_process.get_raw(),
        lateout("r0") result
        )
    }

    if result < 0 {
        Err(result.into())
    } else {
        Ok(out_handle.into())
    }
}

pub fn query_debug_process_memory(
    debug_process: &Handle,
    addr: u32,
) -> crate::Result<MemQueryResponse> {
    let mut mem_info = ctru_sys::MemInfo {
        base_addr: 0,
        perm: 0,
        size: 0,
        state: 0,
    };
    let mut page_info = ctru_sys::PageInfo { flags: 0 };
    let result = unsafe {
        svcQueryDebugProcessMemory(&mut mem_info, &mut page_info, debug_process.get_raw(), addr)
    };

    if result < 0 {
        Err(result.into())
    } else {
        Ok(MemQueryResponse {
            mem_info: unsafe { std::mem::transmute::<ctru_sys::MemInfo, MemInfo>(mem_info) },
            page_info: unsafe { std::mem::transmute::<ctru_sys::PageInfo, PageInfo>(page_info) },
        })
    }
}

#[inline(never)]
/// Luma only.
/// Converts a virtual address into a physical address.
///
/// Returns an error if the pointer is invalid or if the caller
/// does not have permissions to the pointer.
pub fn convert_va_to_pa(virtual_addr: *mut u8, write_check: bool) -> crate::Result<*mut u8> {
    let mut physical_addr: *mut u8;

    unsafe {
        asm!("svc 0x90", in("r0") virtual_addr, in("r1") write_check as u32, lateout("r0") physical_addr)
    };

    if physical_addr.is_null() {
        // Invalid pointer error
        return Err(0xc0de00b.into());
    }

    Ok(physical_addr)
}

/// Gets the uncached address of a physical address.
/// Returns an error if the pointer is null.
pub fn convert_pa_to_uncached_pa(physical_addr: *mut u8) -> crate::Result<*mut u8> {
    if physical_addr.is_null() {
        // Invalid pointer error
        return Err(0xc0de00b.into());
    }

    let uncached_physical_addr = ((physical_addr as u32) | (1 << 31)) as *mut u8;

    Ok(uncached_physical_addr)
}