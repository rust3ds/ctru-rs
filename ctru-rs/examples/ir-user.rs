use ctru::error::ResultCode;
use ctru::prelude::*;
use ctru::services::ir_user::{CirclePadProInputResponse, IrDeviceId, IrUser, IrUserStatusInfo};
use std::io::Write;
use time::Duration;

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
    let bottom_console = Console::init(gfx.bottom_screen.borrow_mut());
    let top_console = Console::init(gfx.top_screen.borrow_mut());

    let ir_user = IrUser::init(
        PACKET_BUFFER_SIZE,
        PACKET_COUNT,
        PACKET_BUFFER_SIZE,
        PACKET_COUNT,
    )
    .expect("Couldn't initialize ir:USER service");

    let print_status_info = || {
        bottom_console.select();
        bottom_console.clear();
        println!("{:#x?}", ir_user.get_status_info());
        top_console.select();
    };

    let conn_status_event = ir_user
        .get_connection_status_event()
        .expect("Couldn't get ir:USER connection status event");
    let recv_event = ir_user
        .get_recv_event()
        .expect("Couldn't get ir:USER recv event");
    print_status_info();

    // // Wait for the connection to establish
    // (|| unsafe {
    //     ResultCode(ctru_sys::svcWaitSynchronization(
    //         ir_user_connection_status_event,
    //         Duration::seconds(10).whole_nanoseconds() as i64,
    //     ))?;
    //
    //     println!("Finished waiting on connection status event");
    //     println!("StatusInfo:\n{:#?}", ir_user.get_status_info());
    //
    //     Ok(())
    // })().expect("Failed to connect to circle pad pro");

    let mut step = 0;

    'main_loop: while apt.main_loop() {
        hid.scan_input();

        // Check if we've received a packet from the circle pad pro
        let check_ir_packet = unsafe { ctru_sys::svcWaitSynchronization(recv_event, 0) == 0 };

        if check_ir_packet {
            handle_packet(&ir_user, &top_console, &bottom_console);
        }

        if hid.keys_held().contains(KeyPad::KEY_START) {
            break;
        }

        if hid.keys_down().contains(KeyPad::KEY_A) {
            match step {
                0 => {
                    loop {
                        hid.scan_input();
                        if hid.keys_held().contains(KeyPad::KEY_START) {
                            break 'main_loop;
                        }

                        ir_user
                            .require_connection(IrDeviceId::CirclePadPro)
                            .expect("Couldn't initialize circle pad pro connection");

                        // Wait for the connection to establish
                        (|| unsafe {
                            ResultCode(ctru_sys::svcWaitSynchronization(
                                conn_status_event,
                                Duration::milliseconds(10).whole_nanoseconds() as i64,
                            ))?;

                            Ok(())
                        })()
                        .expect("Failed to synchronize on connection status event");

                        print_status_info();
                        let status_info = ir_user.get_status_info();
                        if status_info.connection_status == 2 {
                            println!("Connected!");
                            break;
                        }

                        ir_user
                            .disconnect()
                            .expect("Failed to disconnect circle pad pro connection");

                        // Wait for the disconnect to go through
                        (|| unsafe {
                            ResultCode(ctru_sys::svcWaitSynchronization(
                                conn_status_event,
                                Duration::milliseconds(10).whole_nanoseconds() as i64,
                            ))?;

                            Ok(())
                        })()
                        .expect("Failed to synchronize on connection status event");
                    }
                    // }
                    // _ => {
                    loop {
                        hid.scan_input();
                        if hid.keys_held().contains(KeyPad::KEY_START) {
                            break 'main_loop;
                        }

                        if let Err(e) =
                            ir_user.start_polling_input(CPP_CONNECTION_POLLING_PERIOD_MS)
                        {
                            println!("Error: {e:?}");
                        }
                        print_status_info();

                        let check_ir_packet = unsafe {
                            ctru_sys::svcWaitSynchronization(
                                recv_event,
                                Duration::milliseconds(10).whole_nanoseconds() as i64,
                            ) == 0
                        };
                        print_status_info();

                        if check_ir_packet {
                            println!("Got packet from CPP");
                            handle_packet(&ir_user, &top_console, &bottom_console);
                            break;
                        }
                    }
                }
                _ => {}
            }

            step += 1;
        }

        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}

fn handle_packet(ir_user: &IrUser, top_console: &Console, bottom_console: &Console) {
    // Use a buffer to avoid flickering the screen (write all output at once)
    let mut output_buffer = Vec::with_capacity(0x1000);

    ir_user.process_shared_memory(|ir_mem| {
        writeln!(&mut output_buffer, "ReceiveBufferInfo:").unwrap();
        let mut counter = 0;
        for byte in &ir_mem[0x10..0x20] {
            write!(&mut output_buffer, "{byte:02x} ").unwrap();
            counter += 1;
            if counter % 12 == 0 {
                writeln!(&mut output_buffer, "").unwrap();
            }
        }

        writeln!(&mut output_buffer, "\nReceiveBuffer:").unwrap();
        counter = 0;
        for byte in &ir_mem[0x20..0x20 + PACKET_BUFFER_SIZE] {
            write!(&mut output_buffer, "{byte:02x} ").unwrap();
            counter += 1;
            if counter % 12 == 0 {
                writeln!(&mut output_buffer, "").unwrap();
            }
        }

        writeln!(&mut output_buffer, "").unwrap();
    });

    let mut packets = ir_user.get_packets();
    let packet_count = packets.len();
    writeln!(&mut output_buffer, "Packet count: {packet_count}").unwrap();
    let last_packet = packets.pop().unwrap();
    writeln!(&mut output_buffer, "Last packet:\n{last_packet:02x?}").unwrap();

    bottom_console.select();
    bottom_console.clear();
    std::io::stdout().write_all(&output_buffer).unwrap();

    // Use println in case this fails
    let cpp_response = CirclePadProInputResponse::try_from(last_packet)
        .expect("Failed to parse CPP response from IR packet");
    println!("CPP Response:\n{cpp_response:#02x?}");

    top_console.select();

    // Done handling the packet, release it
    ir_user
        .release_received_data(packet_count as u32)
        .expect("Failed to release ir:USER packet");

    if let Err(e) = ir_user.start_polling_input(CPP_POLLING_PERIOD_MS) {
        println!("Error: {e:?}");
    }
}
