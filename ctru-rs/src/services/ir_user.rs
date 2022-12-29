use crate::error::ResultCode;
use crate::services::ServiceReference;
use ctru_sys::{Handle, MEMPERM_READ, MEMPERM_READWRITE};
use std::alloc::Layout;
use std::cmp::max;
use std::ffi::CString;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use std::sync::Mutex;

static IR_USER_ACTIVE: Mutex<usize> = Mutex::new(0);
static IR_USER_STATE: Mutex<Option<IrUserState>> = Mutex::new(None);

#[non_exhaustive]
pub struct IrUser {
    _service_reference: ServiceReference,
}

struct IrUserState {
    service_handle: Handle,
    shared_memory_handle: Handle,
    shared_memory: &'static [u8],
    // shared_memory: Box<[u8]>,
}

const INITIALIZE_IRNOP_SHARED_COMMAND_HEADER: u32 = 0x00180182;
const REQUIRE_CONNECTION_COMMAND_HEADER: u32 = 0x00060040;
const GET_CONNECTION_STATUS_EVENT_COMMAND_HEADER: u32 = 0x000C0000;
const GET_RECEIVE_EVENT_COMMAND_HEADER: u32 = 0x000A0000;
const SEND_IR_NOP_COMMAND_HEADER: u32 = 0x000D0042;
const RELEASE_RECEIVED_DATA_COMMAND_HEADER: u32 = 0x00190040;

impl IrUser {
    pub fn init(recv_buffer_size: usize, recv_packet_count: usize, send_buffer_size: usize, send_packet_count: usize) -> crate::Result<Self> {
        let service_reference = ServiceReference::new(
            &IR_USER_ACTIVE,
            true,
            || unsafe {
                println!("Starting IrUser");
                println!("Getting ir:USER service handle");
                let mut service_handle = Handle::default();
                let service_name = CString::new("ir:USER").unwrap();
                ResultCode(ctru_sys::srvGetServiceHandle(
                    &mut service_handle,
                    service_name.as_ptr(),
                ))?;

                println!("Getting shared memory pointer");
                let info_sections_size = 0x30;
                // let packet_count = 3;
                // let max_packet_size = 32;
                // let packet_info_size = 8;
                // let recv_buffer_len =  recv_packet_count * (packet_info_size + max_packet_size);
                // let send_buffer_len =  send_packet_count * (packet_info_size + max_packet_size);

                let minimum_shared_memory_len = info_sections_size + recv_buffer_size + send_buffer_size;
                let shared_memory_len = if minimum_shared_memory_len % 0x1000 != 0 {
                    (minimum_shared_memory_len / 0x1000) * 0x1000 + 0x1000
                } else {
                    (minimum_shared_memory_len / 0x1000) * 0x1000
                };
                assert_eq!(shared_memory_len % 0x1000, 0);
                // let shared_memory_len = info_sections_size + recv_buffer_size + send_buffer_size;
                println!("Shared memory size: {shared_memory_len:#x}");

                // let shared_memory_len = info_sections_size + packet_count * (packet_info_size + max_packet_size);
                // let shared_memory = Box::new([0; 0x1000]);
                // let shared_memory_ptr = ctru_sys::mappableAlloc(shared_memory_len) as *const u8;
                let shared_memory_layout =
                    Layout::from_size_align(shared_memory_len, 0x1000).unwrap();
                let shared_memory_ptr = std::alloc::alloc_zeroed(shared_memory_layout);
                println!(
                    "Using shared memory address: {:#08x}",
                    shared_memory_ptr as usize
                );

                println!("Marking memory block as shared memory");
                let mut shared_memory_handle = Handle::default();
                ResultCode(ctru_sys::svcCreateMemoryBlock(
                    &mut shared_memory_handle,
                    shared_memory_ptr as u32,
                    shared_memory_len as u32,
                    MEMPERM_READ,
                    MEMPERM_READWRITE,
                ))?;
                let shared_memory = &*slice_from_raw_parts(shared_memory_ptr, shared_memory_len);

                println!("Initializing ir:USER service");
                initialize_irnop_shared(InitializeIrnopSharedParams {
                    ir_user_handle: service_handle,
                    shared_memory_len: shared_memory_len as u32,
                    recv_packet_buffer_len: recv_buffer_size as u32,
                    recv_packet_count: recv_packet_count as u32,
                    send_packet_buffer_len: send_buffer_size as u32,
                    send_packet_count: send_packet_count as u32,
                    bit_rate: 4,
                    shared_memory_handle,
                })?;

                println!("Setting IrUserState in static");
                let user_state = IrUserState {
                    service_handle,
                    shared_memory_handle,
                    shared_memory,
                };
                *IR_USER_STATE.lock().unwrap() = Some(user_state);

                println!("Done starting IrUser");
                Ok(())
            },
            || {
                println!("Close called for IrUser");
                let mut shared_mem_guard = IR_USER_STATE.lock().unwrap();
                let shared_mem = shared_mem_guard
                    .take()
                    .expect("ir:USER shared memory mutex shouldn't be empty");
                // (|| unsafe {
                //     // println!("Unmapping the ir:USER shared memory");
                //     // ResultCode(ctru_sys::svcUnmapMemoryBlock(
                //     //     shared_mem.shared_memory_handle,
                //     //     shared_mem.shared_memory.as_ptr() as u32,
                //     // ))?;
                //
                //     println!("Closing memory and service handles");
                //     // ResultCode(ctru_sys::svcCloseHandle(shared_mem.shared_memory_handle))?;
                //     ResultCode(ctru_sys::svcCloseHandle(shared_mem.service_handle))?;
                //
                //     // println!("Freeing shared memory");
                //     // ctru_sys::mappableFree(shared_mem.shared_memory.as_ptr() as *mut libc::c_void);
                //
                //     Ok(())
                // })()
                // .unwrap();
                println!("Done closing IrUser");
            },
        )?;

        Ok(IrUser {
            _service_reference: service_reference,
        })
    }

    pub fn require_connection(&self, device_id: IrDeviceId) -> crate::Result<()> {
        println!("RequireConnection called");
        self.send_service_request(
            vec![REQUIRE_CONNECTION_COMMAND_HEADER, device_id.get_id()],
            2,
        )?;

        println!("RequireConnection succeeded");
        Ok(())
    }

    pub fn get_connection_status_event(&self) -> crate::Result<Handle> {
        println!("GetConnectionStatusEvent called");
        let response = self.send_service_request(vec![GET_CONNECTION_STATUS_EVENT_COMMAND_HEADER], 4)?;
        let status_event = response[3] as Handle;

        println!("GetConnectionStatusEvent succeeded");
        Ok(status_event)
    }

    pub fn get_recv_event(&self) -> crate::Result<Handle> {
        println!("GetReceiveEvent called");
        let response = self.send_service_request(vec![GET_RECEIVE_EVENT_COMMAND_HEADER], 4)?;
        let recv_event = response[3] as Handle;

        println!("GetReceiveEvent succeeded");
        Ok(recv_event)
    }

    pub fn start_polling_input(&self, period_ms: u8) -> crate::Result<()> {
        println!("SendIrnop (start_polling_input) called");
        let ir_request: [u8; 3] = [1, period_ms, 0];
        self.send_service_request(
            vec![
                SEND_IR_NOP_COMMAND_HEADER,
                ir_request.len() as u32,
                2 + (ir_request.len() << 14) as u32,
                ir_request.as_ptr() as u32,
            ],
            2,
        )?;

        println!("SendIrnop (start_polling_input) succeeded");
        Ok(())
    }

    pub fn release_received_data(&self, packet_count: u32) -> crate::Result<()> {
        println!("ReleaseReceivedData called");
        self.send_service_request(
            vec![RELEASE_RECEIVED_DATA_COMMAND_HEADER, packet_count],
            2
        )?;

        println!("ReleaseReceivedData succeeded");
        Ok(())
    }

    pub fn process_shared_memory(&self, process_fn: impl FnOnce(&[u8])) {
        println!("Process shared memory started");
        let shared_mem_guard = IR_USER_STATE.lock().unwrap();
        let shared_mem = shared_mem_guard.as_ref().unwrap();

        process_fn(shared_mem.shared_memory);

        println!("Process shared memory succeeded");
    }

    pub fn get_status_info(&self) -> IrUserStatusInfo {
        let shared_mem_guard = IR_USER_STATE.lock().unwrap();
        let shared_mem = shared_mem_guard.as_ref().unwrap().shared_memory;

        IrUserStatusInfo {
            recv_err_result: i32::from_ne_bytes([shared_mem[0], shared_mem[1], shared_mem[2], shared_mem[3]]),
            send_err_result: i32::from_ne_bytes([shared_mem[4], shared_mem[5], shared_mem[6], shared_mem[7]]),
            connection_status: shared_mem[8],
            trying_to_connect_status: shared_mem[9],
            connection_role: shared_mem[10],
            machine_id: shared_mem[11],
            unknown_field_1: shared_mem[12],
            network_id: shared_mem[13],
            unknown_field_2: shared_mem[14],
            unknown_field_3: shared_mem[15],
        }
    }

    fn send_service_request(
        &self,
        mut request: Vec<u32>,
        expected_response_len: usize,
    ) -> crate::Result<Vec<u32>> {
        let mut shared_mem_guard = IR_USER_STATE.lock().unwrap();
        let shared_mem = shared_mem_guard.as_mut().unwrap();

        let cmd_buffer = unsafe {
            &mut *(slice_from_raw_parts_mut(
                ctru_sys::getThreadCommandBuffer(),
                max(request.len(), expected_response_len),
            ))
        };
        cmd_buffer[0..request.len()].copy_from_slice(&request);

        // Send the request
        unsafe {
            ResultCode(ctru_sys::svcSendSyncRequest(shared_mem.service_handle))?;
        }

        // Handle the result returned by the service
        ResultCode(cmd_buffer[1] as ctru_sys::Result)?;

        // Copy back the response
        request.clear();
        request.extend_from_slice(&cmd_buffer[0..expected_response_len]);

        Ok(request)
    }
}

struct InitializeIrnopSharedParams {
    ir_user_handle: Handle,
    shared_memory_len: u32,
    recv_packet_buffer_len: u32,
    recv_packet_count: u32,
    send_packet_buffer_len: u32,
    send_packet_count: u32,
    bit_rate: u32,
    shared_memory_handle: Handle,
}

unsafe fn initialize_irnop_shared(params: InitializeIrnopSharedParams) -> crate::Result<()> {
    let cmd_buffer = &mut *(slice_from_raw_parts_mut(ctru_sys::getThreadCommandBuffer(), 9));
    cmd_buffer[0] = INITIALIZE_IRNOP_SHARED_COMMAND_HEADER;
    cmd_buffer[1] = params.shared_memory_len;
    cmd_buffer[2] = params.recv_packet_buffer_len;
    cmd_buffer[3] = params.recv_packet_count;
    cmd_buffer[4] = params.send_packet_buffer_len;
    cmd_buffer[5] = params.send_packet_count;
    cmd_buffer[6] = params.bit_rate;
    cmd_buffer[7] = 0;
    cmd_buffer[8] = params.shared_memory_handle;

    // Send the request
    ResultCode(ctru_sys::svcSendSyncRequest(params.ir_user_handle))?;

    // Handle the result returned by the service
    ResultCode(cmd_buffer[1] as ctru_sys::Result)?;

    Ok(())
}

pub enum IrDeviceId {
    CirclePadPro,
    // Pretty sure no other IDs are recognized, but just in case
    Custom(u32),
}

impl IrDeviceId {
    pub fn get_id(&self) -> u32 {
        match *self {
            IrDeviceId::CirclePadPro => 1,
            IrDeviceId::Custom(id) => id,
        }
    }
}

#[derive(Debug)]
pub struct IrUserStatusInfo {
    recv_err_result: ctru_sys::Result,
    send_err_result: ctru_sys::Result,
    connection_status: u8,
    trying_to_connect_status: u8,
    connection_role: u8,
    machine_id: u8,
    unknown_field_1: u8,
    network_id: u8,
    unknown_field_2: u8,
    unknown_field_3: u8,
}
