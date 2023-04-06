//! This example runs a basic future executor from the `futures` library.
//! Every 60 frames (about 1 second) it prints "Tick" to the console.
//! The executor runs on a separate thread. Internally it yields when it has no more work to do,
//! allowing other threads to run.
//! The example also implements clean shutdown by using a oneshot channel to end the future, thus
//! ending the executor and the thread it runs on.

#![feature(horizon_thread_ext)]

use ctru::prelude::*;

use futures::StreamExt;
use std::os::horizon::thread::BuilderExt;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let _console = Console::new(gfx.top_screen.borrow_mut());

    // Give ourselves up to 30% of the system core's time
    apt.set_app_cpu_time_limit(30)
        .expect("Failed to enable system core");

    println!("Starting executor...");

    let (exit_sender, mut exit_receiver) = futures::channel::oneshot::channel();
    let (mut timer_sender, mut timer_receiver) = futures::channel::mpsc::channel(0);
    let executor_thread = std::thread::Builder::new()
        .processor_id(1)
        .spawn(move || {
            let mut executor = futures::executor::LocalPool::new();

            executor.run_until(async move {
                loop {
                    futures::select! {
                        _ = exit_receiver => break,
                        _ = timer_receiver.next() => {
                            println!("Tick");
                        }
                    }
                }
            });
        })
        .expect("Failed to create executor thread");

    println!("Executor started!");

    let mut frame_count = 0;
    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            println!("Shutting down...");
            let _ = exit_sender.send(());
            let _ = executor_thread.join();
            break;
        }

        frame_count += 1;

        if frame_count == 60 {
            if let Err(e) = timer_sender.try_send(()) {
                println!("Error sending timer message: {e}");
            }
            frame_count = 0;
        }

        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}
