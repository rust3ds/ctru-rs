#![feature(horizon_thread_ext)]

use ctru::console::Console;
use ctru::gfx::Gfx;
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};
use std::os::horizon::thread::ThreadBuilderExt;
use std::time::Duration;

fn main() {
    // Initialize services
    ctru::init();
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::default();
    let _console = Console::init(gfx.top_screen.borrow_mut());

    let prio = std::os::horizon::thread::current_priority();
    println!("Main thread prio: {}\n", prio);

    for ix in 0..3 {
        std::thread::Builder::new()
            .priority(prio - 1)
            .spawn(move || {
                let sleep_duration: u64 = 1000 + ix * 250;
                let mut i = 0;
                loop {
                    println!("Thread{ix} says {i}");
                    i += 1;
                    std::thread::sleep(Duration::from_millis(sleep_duration));
                }
            })
            .unwrap();

        println!("Created thread {ix}");
    }

    while apt.main_loop() {
        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
    }
}
