//! A demo of using the ir:USER service to connect to the Circle Pad Pro.

use ctru::prelude::*;
use ctru::services::ir_user::{CirclePadProInputResponse, IrDeviceId, IrUser};
use std::io::Write;
use std::time::Duration;

const PACKET_INFO_SIZE: usize = 8;
const MAX_PACKET_SIZE: usize = 32;
const PACKET_COUNT: usize = 1;
const PACKET_BUFFER_SIZE: usize = PACKET_COUNT * (PACKET_INFO_SIZE + MAX_PACKET_SIZE);
const CPP_CONNECTION_POLLING_PERIOD_MS: u8 = 0x08;
const CPP_POLLING_PERIOD_MS: u8 = 0x32;

fn main() {
    ctru::init();
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::init().unwrap();
    let top_console = Console::init(gfx.top_screen.borrow_mut());
    let bottom_console = Console::init(gfx.bottom_screen.borrow_mut());

    println!("Welcome to the ir:USER / Circle Pad Pro Demo");

    println!("Starting up ir:USER service");
    let ir_user = IrUser::init(
        PACKET_BUFFER_SIZE,
        PACKET_COUNT,
        PACKET_BUFFER_SIZE,
        PACKET_COUNT,
    )
    .expect("Couldn't initialize ir:USER service");
    println!("ir:USER service initialized\nPress A to connect to the CPP");

    let print_status_info = || {
        top_console.select();
        top_console.clear();
        println!("{:#x?}", ir_user.get_status_info());
        bottom_console.select();
    };

    // Get event handles
    let conn_status_event = ir_user
        .get_connection_status_event()
        .expect("Couldn't get ir:USER connection status event");
    let recv_event = ir_user
        .get_recv_event()
        .expect("Couldn't get ir:USER recv event");
    print_status_info();

    let mut is_connected = false;
    'main_loop: while apt.main_loop() {
        hid.scan_input();

        // Check if we need to exit
        if hid.keys_held().contains(KeyPad::KEY_START) {
            break;
        }

        // Check if we've received a packet from the circle pad pro
        let packet_received = IrUser::wait_for_event(recv_event, Duration::ZERO).is_ok();
        if packet_received {
            handle_packet(&ir_user, &top_console, &bottom_console);
        }

        // Check if we should start the connection
        if hid.keys_down().contains(KeyPad::KEY_A) && !is_connected {
            println!("Attempting to connect to the CPP");

            // Connection loop
            loop {
                hid.scan_input();
                if hid.keys_held().contains(KeyPad::KEY_START) {
                    break 'main_loop;
                }

                // Start the connection process
                ir_user
                    .require_connection(IrDeviceId::CirclePadPro)
                    .expect("Couldn't initialize circle pad pro connection");

                // Wait for the connection to establish
                if let Err(e) =
                    IrUser::wait_for_event(conn_status_event, Duration::from_millis(100))
                {
                    if !e.is_timeout() {
                        panic!("Couldn't initialize circle pad pro connection: {e}");
                    }
                }

                print_status_info();
                if ir_user.get_status_info().connection_status == 2 {
                    println!("Connected!");
                    break;
                }

                // If not connected (ex. timeout), disconnect so we can retry
                ir_user
                    .disconnect()
                    .expect("Failed to disconnect circle pad pro connection");

                // Wait for the disconnect to go through
                if let Err(e) =
                    IrUser::wait_for_event(conn_status_event, Duration::from_millis(100))
                {
                    if !e.is_timeout() {
                        panic!("Couldn't initialize circle pad pro connection: {e}");
                    }
                }
            }

            // Sending first packet retry loop
            loop {
                hid.scan_input();
                if hid.keys_held().contains(KeyPad::KEY_START) {
                    break 'main_loop;
                }

                // Send a request for input to the CPP
                if let Err(e) = ir_user.request_input_polling(CPP_CONNECTION_POLLING_PERIOD_MS) {
                    println!("Error: {e:?}");
                }
                print_status_info();

                // Wait for the response
                let recv_event_result =
                    IrUser::wait_for_event(recv_event, Duration::from_millis(100));
                print_status_info();

                if recv_event_result.is_ok() {
                    println!("Got first packet from CPP");
                    handle_packet(&ir_user, &top_console, &bottom_console);
                    break;
                }

                // We didn't get a response in time, so loop and retry
            }

            is_connected = true;
        }

        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}

fn handle_packet(ir_user: &IrUser, top_console: &Console, bottom_console: &Console) {
    // Use a buffer to avoid flickering the screen (write all output at once)
    let mut output_buffer = Vec::with_capacity(0x1000);

    writeln!(&mut output_buffer, "{:x?}", ir_user.get_status_info()).unwrap();

    ir_user.process_shared_memory(|ir_mem| {
        writeln!(&mut output_buffer, "\nReceiveBufferInfo:").unwrap();
        write_buffer_as_hex(&ir_mem[0x10..0x20], &mut output_buffer);

        writeln!(&mut output_buffer, "\nReceiveBuffer:").unwrap();
        write_buffer_as_hex(&ir_mem[0x20..0x20 + PACKET_BUFFER_SIZE], &mut output_buffer);
        writeln!(&mut output_buffer).unwrap();
    });

    let packets = ir_user.get_packets();
    let packet_count = packets.len();
    writeln!(&mut output_buffer, "\nPacket count: {packet_count}").unwrap();
    let last_packet = packets.last().unwrap();
    writeln!(&mut output_buffer, "{last_packet:02x?}").unwrap();

    let cpp_response = CirclePadProInputResponse::try_from(last_packet)
        .expect("Failed to parse CPP response from IR packet");
    writeln!(&mut output_buffer, "\n{cpp_response:#02x?}").unwrap();

    // Write output to top screen
    top_console.select();
    top_console.clear();
    std::io::stdout().write_all(&output_buffer).unwrap();
    bottom_console.select();

    // Done handling the packet, release it
    ir_user
        .release_received_data(packet_count as u32)
        .expect("Failed to release ir:USER packet");

    // Remind the CPP that we're still listening
    if let Err(e) = ir_user.request_input_polling(CPP_POLLING_PERIOD_MS) {
        println!("Error: {e:?}");
    }
}

fn write_buffer_as_hex(buffer: &[u8], output: &mut Vec<u8>) {
    let mut counter = 0;
    for byte in buffer {
        write!(output, "{byte:02x} ").unwrap();
        counter += 1;
        if counter % 16 == 0 {
            writeln!(output).unwrap();
        }
    }
}
