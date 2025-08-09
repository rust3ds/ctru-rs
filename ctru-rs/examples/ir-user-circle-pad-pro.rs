//! ir:USER Circle Pad Pro example.
//!
//! A demo of using the ir:USER service to connect to the Circle Pad Pro.

use ctru::prelude::*;
use ctru::services::gfx::{Flush, Swap};
use ctru::services::ir_user::{CirclePadProInputResponse, ConnectionStatus, IrDeviceId, IrUser};
use ctru::services::svc::HandleExt;
use ctru_sys::Handle;
use std::time::Duration;

// Configuration for this demo of the Circle Pad Pro (not general purpose ir:USER values).
const PACKET_INFO_SIZE: usize = 8;
const MAX_PACKET_SIZE: usize = 32;
const PACKET_COUNT: usize = 1;
const PACKET_BUFFER_SIZE: usize = PACKET_COUNT * (PACKET_INFO_SIZE + MAX_PACKET_SIZE);
const CPP_CONNECTION_POLLING_PERIOD_MS: u8 = 0x08;
const CPP_POLLING_PERIOD_MS: u8 = 0x32;

// This export tells libctru to not initialize ir:rst when initializing HID.
// This is necessary on the New 3DS because ir:rst is mutually exclusive with ir:USER.
#[unsafe(no_mangle)]
unsafe extern "C" fn hidShouldUseIrrst() -> bool {
    false
}

fn main() {
    let apt = Apt::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let top_console = Console::new(gfx.top_screen.borrow_mut());
    let bottom_console = Console::new(gfx.bottom_screen.borrow_mut());
    let mut demo = CirclePadProDemo::new(top_console, bottom_console);
    demo.print_status_info();

    // Initialize HID after ir:USER because libctru also initializes ir:rst,
    // which is mutually exclusive with ir:USER. Initializing HID before ir:USER
    // on New 3DS causes ir:USER to not work.
    let mut hid = Hid::new().unwrap();

    println!("Press A to connect to the CPP, or Start to exit");

    let mut is_connected = false;
    while apt.main_loop() {
        hid.scan_input();

        // Check if we need to exit
        if hid.keys_held().contains(KeyPad::START) {
            break;
        }

        // Check if we've received a packet from the circle pad pro
        let packet_received = demo
            .receive_packet_event
            .wait_for_event(Duration::ZERO)
            .is_ok();
        if packet_received {
            demo.handle_packets();
        }

        // Check if we should start the connection
        if hid.keys_down().contains(KeyPad::A) && !is_connected {
            println!("Attempting to connect to the CPP");

            match demo.connect_to_cpp(&mut hid) {
                ConnectionResult::Connected => is_connected = true,
                ConnectionResult::Canceled => break,
            }
        }

        gfx.wait_for_vblank();
    }
}

struct CirclePadProDemo<'screen> {
    top_console: Console<'screen>,
    bottom_console: Console<'screen>,
    ir_user: IrUser,
    connection_status_event: Handle,
    receive_packet_event: Handle,
}

enum ConnectionResult {
    Connected,
    Canceled,
}

impl<'screen> CirclePadProDemo<'screen> {
    fn new(mut top_console: Console<'screen>, bottom_console: Console<'screen>) -> Self {
        // Set up double buffering on top screen
        top_console.set_double_buffering(true);
        top_console.swap_buffers();

        // Write messages to bottom screen (not double buffered)
        bottom_console.select();
        println!("Welcome to the ir:USER / Circle Pad Pro Demo");

        println!("Starting up ir:USER service");
        let ir_user = IrUser::init(
            PACKET_BUFFER_SIZE,
            PACKET_COUNT,
            PACKET_BUFFER_SIZE,
            PACKET_COUNT,
        )
        .expect("Couldn't initialize ir:USER service");
        println!("ir:USER service initialized");

        // Get event handles
        let connection_status_event = ir_user
            .get_connection_status_event()
            .expect("Couldn't get ir:USER connection status event");
        let receive_packet_event = ir_user
            .get_recv_event()
            .expect("Couldn't get ir:USER recv event");

        Self {
            top_console,
            bottom_console,
            ir_user,
            connection_status_event,
            receive_packet_event,
        }
    }

    fn print_status_info(&mut self) {
        self.top_console.select();
        self.top_console.clear();
        println!("{:#x?}", self.ir_user.get_status_info());
        self.top_console.flush_buffers();
        self.top_console.swap_buffers();
        self.bottom_console.select();
    }

    fn connect_to_cpp(&mut self, hid: &mut Hid) -> ConnectionResult {
        // Connection loop
        loop {
            hid.scan_input();
            if hid.keys_held().contains(KeyPad::START) {
                return ConnectionResult::Canceled;
            }

            // Start the connection process
            self.ir_user
                .require_connection(IrDeviceId::CirclePadPro)
                .expect("Couldn't initialize circle pad pro connection");

            // Wait for the connection to establish
            if let Err(e) = self
                .connection_status_event
                .wait_for_event(Duration::from_millis(100))
                && !e.is_timeout()
            {
                panic!("Couldn't initialize circle pad pro connection: {e}");
            }

            self.print_status_info();
            if self.ir_user.get_status_info().connection_status == ConnectionStatus::Connected {
                println!("Connected!");
                break;
            }

            // If not connected (ex. timeout), disconnect so we can retry
            self.ir_user
                .disconnect()
                .expect("Failed to disconnect circle pad pro connection");

            // Wait for the disconnect to go through
            if let Err(e) = self
                .connection_status_event
                .wait_for_event(Duration::from_millis(100))
                && !e.is_timeout()
            {
                panic!("Couldn't initialize circle pad pro connection: {e}");
            }
        }

        // Sending first packet retry loop
        loop {
            hid.scan_input();
            if hid.keys_held().contains(KeyPad::START) {
                return ConnectionResult::Canceled;
            }

            // Send a request for input to the CPP
            if let Err(e) = self
                .ir_user
                .request_input_polling(CPP_CONNECTION_POLLING_PERIOD_MS)
            {
                println!("Error: {e:?}");
            }
            self.print_status_info();

            // Wait for the response
            let recv_event_result = self
                .receive_packet_event
                .wait_for_event(Duration::from_millis(100));
            self.print_status_info();

            if recv_event_result.is_ok() {
                println!("Got first packet from CPP");
                self.handle_packets();
                break;
            }

            // We didn't get a response in time, so loop and retry
        }

        ConnectionResult::Connected
    }

    fn handle_packets(&mut self) {
        let packets = self
            .ir_user
            .get_packets()
            .expect("Packets should be well formed");
        let packet_count = packets.len();
        let Some(last_packet) = packets.last() else {
            return;
        };
        let status_info = self.ir_user.get_status_info();
        let cpp_response = CirclePadProInputResponse::try_from(last_packet)
            .expect("Failed to parse CPP response from IR packet");

        // Write data to top screen
        self.top_console.select();
        self.top_console.clear();
        println!("{status_info:x?}");

        self.ir_user.process_shared_memory(|ir_mem| {
            println!("\nReceiveBufferInfo:");
            print_buffer_as_hex(&ir_mem[0x10..0x20]);

            println!("\nReceiveBuffer:");
            print_buffer_as_hex(&ir_mem[0x20..0x20 + PACKET_BUFFER_SIZE]);
            println!();
        });

        println!("\nPacket count: {packet_count}");
        println!("{last_packet:02x?}");
        println!("\n{cpp_response:#02x?}");

        // Flush output and switch back to bottom screen
        self.top_console.flush_buffers();
        self.top_console.swap_buffers();
        self.bottom_console.select();

        // Done handling the packets, release them
        self.ir_user
            .release_received_data(packet_count as u32)
            .expect("Failed to release ir:USER packet");

        // Remind the CPP that we're still listening
        if let Err(e) = self.ir_user.request_input_polling(CPP_POLLING_PERIOD_MS) {
            println!("Error: {e:?}");
        }
    }
}

fn print_buffer_as_hex(buffer: &[u8]) {
    let mut counter = 0;
    for byte in buffer {
        print!("{byte:02x} ");
        counter += 1;
        if counter % 16 == 0 {
            println!();
        }
    }
}
