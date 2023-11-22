#![feature(horizon_thread_ext)]

use ctru::prelude::*;

use std::os::horizon::thread::BuilderExt;
use std::time::Duration;

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let _console = Console::new(gfx.top_screen.borrow_mut());

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

    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
    }
}
