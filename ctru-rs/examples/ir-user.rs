use ctru::prelude::*;
use ctru::services::ir_user::{IrDeviceId, IrUser};
use time::Duration;
use ctru::error::ResultCode;

const PACKET_INFO_SIZE: usize = 8;
const MAX_PACKET_SIZE: usize = 32;
const PACKET_COUNT: usize = 1;
const PACKET_BUFFER_SIZE: usize = PACKET_COUNT * (PACKET_INFO_SIZE + MAX_PACKET_SIZE);

fn main() {
    ctru::init();
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::init().unwrap();
    let console = Console::init(gfx.top_screen.borrow_mut());
    let ir_user = IrUser::init(
        PACKET_BUFFER_SIZE,
        PACKET_COUNT,
        PACKET_BUFFER_SIZE,
        PACKET_COUNT,
    )
    .expect("Couldn't initialize ir:USER service");
    let ir_user_connection_status_event = ir_user
        .get_connection_status_event()
        .expect("Couldn't get ir:USER connection status event");
    ir_user
        .require_connection(IrDeviceId::CirclePadPro)
        .expect("Couldn't initialize circle pad pro connection");
    let ir_user_recv_event = ir_user
        .get_recv_event()
        .expect("Couldn't get ir:USER recv event");
    println!("StatusInfo:\n{:#?}", ir_user.get_status_info());

    // Wait for the connection to establish
    (|| unsafe {
        ResultCode(ctru_sys::svcWaitSynchronization(
            ir_user_connection_status_event,
            Duration::seconds(10).whole_nanoseconds() as i64,
        ))?;

        println!("Finished waiting on connection status event");
        println!("StatusInfo:\n{:#?}", ir_user.get_status_info());

        Ok(())
    })().expect("Failed to connect to circle pad pro");

    ir_user
        .start_polling_input(20)
        .expect("Couldn't configure circle pad pro polling interval");

    while apt.main_loop() {
        hid.scan_input();

        // Check if we've received a packet from the circle pad pro
        let check_ir_packet =
            unsafe { ctru_sys::svcWaitSynchronization(ir_user_recv_event, 0) == 0 };

        if check_ir_packet {
            console.clear();

            // Move the cursor back to the top of the screen
            print!("\x1b[0;0H");

            println!("StatusInfo:\n{:?}", ir_user.get_status_info());

            ir_user.process_shared_memory(|ir_mem| {
                println!("\nReceiveBufferInfo:");
                for byte in &ir_mem[0x10..0x20] {
                    print!("{byte:02x} ");
                }

                println!("\nReceiveBuffer:");
                let mut counter = 0;
                for byte in &ir_mem[0x20..0x20 + PACKET_BUFFER_SIZE] {
                    print!("{byte:02x} ");
                    counter += 1;
                    if counter % 16 == 0 {
                        println!()
                    }
                }

                println!("\nSendBufferInfo:");
                for byte in &ir_mem[0x20 + PACKET_BUFFER_SIZE..0x30 + PACKET_BUFFER_SIZE] {
                    print!("{byte:02x} ");
                }

                println!("\n(skipping send packet buffer)");
            });

            // Done handling the packet, release it
            ir_user
                .release_received_data(1)
                .expect("Failed to release ir:USER packet");

            println!("\x1b[29;16HPress Start to exit");
        }

        if hid.keys_held().intersects(KeyPad::KEY_START) {
            break;
        }

        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}
