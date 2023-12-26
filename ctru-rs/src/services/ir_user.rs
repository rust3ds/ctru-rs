//! IR (Infrared) User Service.
//!
//! The ir:USER service allows you to communicate with IR devices such as the Circle Pad Pro.
//!
//! The Circle Pad Pro (CPP) is an accessory for the 3DS which adds a second Circle Pad and extra shoulder buttons.
//! On New 3DS systems, the ir:USER service uses the built-in C-stick and new shoulder buttons to emulate the Circle Pad
//! Pro. Many released games which support the second stick and extra shoulder buttons use this service to communicate
//! so they can  support both Old 3DS + CPP and New 3DS.
#![doc(alias = "input")]
#![doc(alias = "controller")]
#![doc(alias = "gamepad")]

use crate::error::ResultCode;
use crate::services::svc::{make_ipc_header, HandleExt};
use crate::services::ServiceReference;
use crate::Error;
use ctru_sys::{Handle, MEMPERM_READ, MEMPERM_READWRITE};
use std::alloc::Layout;
use std::ffi::CString;
use std::ptr::slice_from_raw_parts;
use std::sync::Mutex;

static IR_USER_ACTIVE: Mutex<()> = Mutex::new(());
static IR_USER_STATE: Mutex<Option<IrUserState>> = Mutex::new(None);

/// The "ir:USER" service. This service is used to talk to IR devices such as
/// the Circle Pad Pro.
pub struct IrUser {
    _service_reference: ServiceReference,
}

// We need to hold on to some extra service state, hence this struct.
struct IrUserState {
    service_handle: Handle,
    shared_memory_handle: Handle,
    shared_memory: &'static [u8],
    shared_memory_layout: Layout,
    recv_buffer_size: usize,
    recv_packet_count: usize,
}

// ir:USER syscall command headers
const REQUIRE_CONNECTION_COMMAND_HEADER: u32 = make_ipc_header(6, 1, 0);
const DISCONNECT_COMMAND_HEADER: u32 = make_ipc_header(9, 0, 0);
const GET_RECEIVE_EVENT_COMMAND_HEADER: u32 = make_ipc_header(10, 0, 0);
const GET_CONNECTION_STATUS_EVENT_COMMAND_HEADER: u32 = make_ipc_header(12, 0, 0);
const SEND_IR_NOP_COMMAND_HEADER: u32 = make_ipc_header(13, 1, 2);
const INITIALIZE_IRNOP_SHARED_COMMAND_HEADER: u32 = make_ipc_header(24, 6, 2);
const RELEASE_RECEIVED_DATA_COMMAND_HEADER: u32 = make_ipc_header(25, 1, 0);

// Misc constants
const SHARED_MEM_INFO_SECTIONS_SIZE: usize = 0x30;
const SHARED_MEM_RECV_BUFFER_OFFSET: usize = 0x20;
const PAGE_SIZE: usize = 0x1000;
const IR_BITRATE: u32 = 4;
const PACKET_INFO_SIZE: usize = 8;
const CIRCLE_PAD_PRO_INPUT_RESPONSE_PACKET_ID: u8 = 0x10;

impl IrUser {
    /// Initialize the ir:USER service. The provided buffer sizes and packet
    /// counts are used to calculate the size of shared memory used by the
    /// service.
    pub fn init(
        recv_buffer_size: usize,
        recv_packet_count: usize,
        send_buffer_size: usize,
        send_packet_count: usize,
    ) -> crate::Result<Self> {
        let service_reference = ServiceReference::new(
            &IR_USER_ACTIVE,
            || unsafe {
                // Get the ir:USER service handle
                let mut service_handle = Handle::default();
                let service_name = CString::new("ir:USER").unwrap();
                ResultCode(ctru_sys::srvGetServiceHandle(
                    &mut service_handle,
                    service_name.as_ptr(),
                ))?;

                // Calculate the shared memory size.
                // Shared memory length must be a multiple of the page size.
                let minimum_shared_memory_len =
                    SHARED_MEM_INFO_SECTIONS_SIZE + recv_buffer_size + send_buffer_size;
                let shared_memory_len = round_up(minimum_shared_memory_len, PAGE_SIZE);

                // Allocate the shared memory
                let shared_memory_layout =
                    Layout::from_size_align(shared_memory_len, PAGE_SIZE).unwrap();
                let shared_memory_ptr = std::alloc::alloc_zeroed(shared_memory_layout);
                let shared_memory = &*slice_from_raw_parts(shared_memory_ptr, shared_memory_len);

                // Mark the memory as shared
                let mut shared_memory_handle = Handle::default();
                ResultCode(ctru_sys::svcCreateMemoryBlock(
                    &mut shared_memory_handle,
                    shared_memory_ptr as u32,
                    shared_memory_len as u32,
                    MEMPERM_READ,
                    MEMPERM_READWRITE,
                ))?;

                // Initialize the ir:USER service with the shared memory
                let request = vec![
                    INITIALIZE_IRNOP_SHARED_COMMAND_HEADER,
                    shared_memory_len as u32,
                    recv_buffer_size as u32,
                    recv_packet_count as u32,
                    send_buffer_size as u32,
                    send_packet_count as u32,
                    IR_BITRATE,
                    0,
                    shared_memory_handle,
                ];
                service_handle.send_service_request(request, 2)?;

                // Set up our service state
                let user_state = IrUserState {
                    service_handle,
                    shared_memory_handle,
                    shared_memory,
                    shared_memory_layout,
                    recv_buffer_size,
                    recv_packet_count,
                };
                let mut ir_user_state = IR_USER_STATE
                    .lock()
                    .map_err(|e| Error::Other(format!("Failed to write to IR_USER_STATE: {e}")))?;
                *ir_user_state = Some(user_state);

                Ok(())
            },
            || {
                // Remove our service state from the global location
                let mut shared_mem_guard = IR_USER_STATE
                    .lock()
                    .expect("Failed to write to IR_USER_STATE");
                let Some(shared_mem) = shared_mem_guard.take() else {
                    // If we don't have any state, then we don't need to clean up.
                    return;
                };

                (move || unsafe {
                    // Close service and memory handles
                    ResultCode(ctru_sys::svcCloseHandle(shared_mem.service_handle))?;
                    ResultCode(ctru_sys::svcCloseHandle(shared_mem.shared_memory_handle))?;

                    // Free shared memory
                    std::alloc::dealloc(
                        shared_mem.shared_memory.as_ptr() as *mut u8,
                        shared_mem.shared_memory_layout,
                    );

                    Ok(())
                })()
                .unwrap();
            },
        )?;

        Ok(IrUser {
            _service_reference: service_reference,
        })
    }

    /// Try to connect to the device with the provided ID.
    pub fn require_connection(&self, device_id: IrDeviceId) -> crate::Result<()> {
        unsafe {
            self.send_service_request(
                vec![REQUIRE_CONNECTION_COMMAND_HEADER, device_id.get_id()],
                2,
            )?;
        }
        Ok(())
    }

    /// Close the current IR connection.
    pub fn disconnect(&self) -> crate::Result<()> {
        unsafe {
            self.send_service_request(vec![DISCONNECT_COMMAND_HEADER], 2)?;
        }
        Ok(())
    }

    /// Get an event handle that activates on connection status changes.
    pub fn get_connection_status_event(&self) -> crate::Result<Handle> {
        let response = unsafe {
            self.send_service_request(vec![GET_CONNECTION_STATUS_EVENT_COMMAND_HEADER], 4)
        }?;
        let status_event = response[3] as Handle;

        Ok(status_event)
    }

    /// Get an event handle that activates when a packet is received.
    pub fn get_recv_event(&self) -> crate::Result<Handle> {
        let response =
            unsafe { self.send_service_request(vec![GET_RECEIVE_EVENT_COMMAND_HEADER], 4) }?;
        let recv_event = response[3] as Handle;

        Ok(recv_event)
    }

    /// Circle Pad Pro specific request.
    ///
    /// This will send a packet to the CPP requesting it to send back packets
    /// with the current device input values.
    pub fn request_input_polling(&self, period_ms: u8) -> crate::Result<()> {
        let ir_request: [u8; 3] = [1, period_ms, (period_ms + 2) << 2];
        unsafe {
            self.send_service_request(
                vec![
                    SEND_IR_NOP_COMMAND_HEADER,
                    ir_request.len() as u32,
                    2 + (ir_request.len() << 14) as u32,
                    ir_request.as_ptr() as u32,
                ],
                2,
            )?;
        }

        Ok(())
    }

    /// Mark the last `packet_count` packets as processed, so their memory in
    /// the receive buffer can be reused.
    pub fn release_received_data(&self, packet_count: u32) -> crate::Result<()> {
        unsafe {
            self.send_service_request(vec![RELEASE_RECEIVED_DATA_COMMAND_HEADER, packet_count], 2)?;
        }
        Ok(())
    }

    /// This will let you directly read the ir:USER shared memory via a callback.
    pub fn process_shared_memory(&self, process_fn: impl FnOnce(&[u8])) {
        let shared_mem_guard = IR_USER_STATE.lock().unwrap();
        let shared_mem = shared_mem_guard.as_ref().unwrap();

        process_fn(shared_mem.shared_memory);
    }

    /// Read and parse the ir:USER service status data from shared memory.
    pub fn get_status_info(&self) -> IrUserStatusInfo {
        let shared_mem_guard = IR_USER_STATE.lock().unwrap();
        let shared_mem = shared_mem_guard.as_ref().unwrap().shared_memory;

        IrUserStatusInfo {
            recv_err_result: i32::from_ne_bytes(shared_mem[0..4].try_into().unwrap()),
            send_err_result: i32::from_ne_bytes(shared_mem[4..8].try_into().unwrap()),
            connection_status: match shared_mem[8] {
                0 => ConnectionStatus::Disconnected,
                1 => ConnectionStatus::Connecting,
                2 => ConnectionStatus::Connected,
                n => ConnectionStatus::Unknown(n),
            },
            trying_to_connect_status: shared_mem[9],
            connection_role: shared_mem[10],
            machine_id: shared_mem[11],
            unknown_field_1: shared_mem[12],
            network_id: shared_mem[13],
            unknown_field_2: shared_mem[14],
            unknown_field_3: shared_mem[15],
        }
    }

    /// Read and parse the current packets received from the IR device.
    pub fn get_packets(&self) -> Result<Vec<IrUserPacket>, String> {
        let shared_mem_guard = IR_USER_STATE.lock().unwrap();
        let user_state = shared_mem_guard.as_ref().unwrap();
        let shared_mem = user_state.shared_memory;

        // Find where the packets are, and how many
        let start_index = u32::from_ne_bytes(shared_mem[0x10..0x14].try_into().unwrap());
        let valid_packet_count = u32::from_ne_bytes(shared_mem[0x18..0x1c].try_into().unwrap());

        // Parse the packets
        (0..valid_packet_count as usize)
            .map(|i| {
                // Get the packet info
                let packet_index = (i + start_index as usize) % user_state.recv_packet_count;
                let packet_info_offset =
                    SHARED_MEM_RECV_BUFFER_OFFSET + (packet_index * PACKET_INFO_SIZE);
                let packet_info =
                    &shared_mem[packet_info_offset..packet_info_offset + PACKET_INFO_SIZE];

                let offset_to_data_buffer =
                    u32::from_ne_bytes(packet_info[0..4].try_into().unwrap()) as usize;
                let data_length =
                    u32::from_ne_bytes(packet_info[4..8].try_into().unwrap()) as usize;

                // Find the packet data. The packet data may wrap around the buffer end, so
                // `packet_data` is a function from packet byte offset to value.
                let packet_info_section_size = user_state.recv_packet_count * PACKET_INFO_SIZE;
                let header_size = SHARED_MEM_RECV_BUFFER_OFFSET + packet_info_section_size;
                let data_buffer_size = user_state.recv_buffer_size - packet_info_section_size;
                let packet_data = |idx| -> u8 {
                    let data_buffer_offset = offset_to_data_buffer + idx;
                    shared_mem[header_size + data_buffer_offset % data_buffer_size]
                };

                // Find out how long the payload is (payload length is variable-length encoded)
                let (payload_length, payload_offset) = if packet_data(2) & 0x40 != 0 {
                    // Big payload
                    (
                        ((packet_data(2) as usize & 0x3F) << 8) + packet_data(3) as usize,
                        4,
                    )
                } else {
                    // Small payload
                    ((packet_data(2) & 0x3F) as usize, 3)
                };

                // Check our payload length math against what the packet info contains
                if data_length != payload_offset + payload_length + 1 {
                    return Err(format!(
                        "Invalid payload length (expected {}, got {})",
                        data_length,
                        payload_offset + payload_length + 1
                    ));
                }

                // IR packets start with a magic number, so double check it
                let magic_number = packet_data(0);
                if magic_number != 0xA5 {
                    return Err(format!(
                        "Invalid magic number in packet: {magic_number:#x}, expected 0xA5"
                    ));
                }

                Ok(IrUserPacket {
                    magic_number: packet_data(0),
                    destination_network_id: packet_data(1),
                    payload_length,
                    payload: (payload_offset..payload_offset + payload_length)
                        .map(packet_data)
                        .collect(),
                    checksum: packet_data(payload_offset + payload_length),
                })
            })
            .collect()
    }

    /// Internal helper for calling ir:USER service methods.
    unsafe fn send_service_request(
        &self,
        request: Vec<u32>,
        expected_response_len: usize,
    ) -> crate::Result<Vec<u32>> {
        let mut shared_mem_guard = IR_USER_STATE.lock().unwrap();
        let shared_mem = shared_mem_guard.as_mut().unwrap();

        shared_mem
            .service_handle
            .send_service_request(request, expected_response_len)
    }
}

// Internal helper for rounding up a value to a multiple of another value.
fn round_up(value: usize, multiple: usize) -> usize {
    if value % multiple != 0 {
        (value / multiple) * multiple + multiple
    } else {
        (value / multiple) * multiple
    }
}

/// An enum which represents the different IR devices the 3DS can connect to via
/// the ir:USER service.
pub enum IrDeviceId {
    /// Circle Pad Pro
    CirclePadPro,
    /// Other devices
    // Pretty sure no other IDs are recognized, but just in case
    Custom(u32),
}

impl IrDeviceId {
    /// Get the ID of the device.
    pub fn get_id(&self) -> u32 {
        match *self {
            IrDeviceId::CirclePadPro => 1,
            IrDeviceId::Custom(id) => id,
        }
    }
}

/// This struct holds a parsed copy of the ir:USER service status (from shared memory).
#[derive(Debug)]
pub struct IrUserStatusInfo {
    /// The result of the last receive operation.
    pub recv_err_result: ctru_sys::Result,
    /// The result of the last send operation.
    pub send_err_result: ctru_sys::Result,
    /// The current connection status.
    pub connection_status: ConnectionStatus,
    /// The status of the connection attempt.
    pub trying_to_connect_status: u8,
    /// The role of the device in the connection (value meaning is unknown).
    pub connection_role: u8,
    /// The machine ID of the device.
    pub machine_id: u8,
    /// Unknown field.
    pub unknown_field_1: u8,
    /// The network ID of the connection.
    pub network_id: u8,
    /// Unknown field.
    pub unknown_field_2: u8,
    /// Unknown field.
    pub unknown_field_3: u8,
}

/// Connection status values for [`IrUserStatusInfo`].
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionStatus {
    /// Device is not connected
    Disconnected = 0,
    /// Waiting for device to connect
    Connecting = 1,
    /// Device is connected
    Connected = 2,
    /// Unknown connection status
    Unknown(u8),
}

/// A packet of data sent/received to/from the IR device.
#[derive(Debug)]
pub struct IrUserPacket {
    /// The magic number of the packet. Should always be 0xA5.
    pub magic_number: u8,
    /// The destination network ID.
    pub destination_network_id: u8,
    /// The length of the payload.
    pub payload_length: usize,
    /// The payload data.
    pub payload: Vec<u8>,
    /// The checksum of the packet.
    pub checksum: u8,
}

/// Circle Pad Pro response packet holding the current device input signals and status.
#[derive(Debug, Default)]
pub struct CirclePadProInputResponse {
    /// The X value of the C-stick.
    pub c_stick_x: u16,
    /// The Y value of the C-stick.
    pub c_stick_y: u16,
    /// The battery level of the Circle Pad Pro.
    pub battery_level: u8,
    /// Whether the ZL button is pressed.
    pub zl_pressed: bool,
    /// Whether the ZR button is pressed.
    pub zr_pressed: bool,
    /// Whether the R button is pressed.
    pub r_pressed: bool,
    /// Unknown field.
    pub unknown_field: u8,
}

impl TryFrom<&IrUserPacket> for CirclePadProInputResponse {
    type Error = String;

    fn try_from(packet: &IrUserPacket) -> Result<Self, Self::Error> {
        if packet.payload.len() != 6 {
            return Err(format!(
                "Invalid payload length (expected 6 bytes, got {})",
                packet.payload.len()
            ));
        }

        let response_id = packet.payload[0];
        if response_id != CIRCLE_PAD_PRO_INPUT_RESPONSE_PACKET_ID {
            return Err(format!(
                "Invalid response ID (expected {CIRCLE_PAD_PRO_INPUT_RESPONSE_PACKET_ID}, got {:#x}",
                packet.payload[0]
            ));
        }

        let c_stick_x = packet.payload[1] as u16 + (((packet.payload[2] & 0x0F) as u16) << 8);
        let c_stick_y =
            (((packet.payload[2] & 0xF0) as u16) >> 4) + ((packet.payload[3] as u16) << 4);
        let battery_level = packet.payload[4] & 0x1F;
        let zl_pressed = packet.payload[4] & 0x20 == 0;
        let zr_pressed = packet.payload[4] & 0x40 == 0;
        let r_pressed = packet.payload[4] & 0x80 == 0;
        let unknown_field = packet.payload[5];

        Ok(CirclePadProInputResponse {
            c_stick_x,
            c_stick_y,
            battery_level,
            zl_pressed,
            zr_pressed,
            r_pressed,
            unknown_field,
        })
    }
}
