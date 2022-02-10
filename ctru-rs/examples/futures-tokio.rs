use ctru::console::Console;
use ctru::services::hid::KeyPad;
use ctru::services::ps::Ps;
use ctru::services::{Apt, Hid};
use ctru::Gfx;
use std::time::Duration;

fn main() {
    ctru::init();
    let gfx = Gfx::default();
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _ps = Ps::init().expect("Couldn't initialize PS service");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    // Give ourselves up to 30% of the system core's time
    apt.set_app_cpu_time_limit(30)
        .expect("Failed to enable system core");

    println!("Starting runtime...");

    let (exit_sender, mut exit_receiver) = tokio::sync::oneshot::channel();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("Couldn't build runtime");

    let runtime_thread = std::thread::Builder::new()
        // Run on the system core
        .affinity(1)
        // Use a bigger stack size. Default is 0x1000 but we'd easily overflow that.
        .stack_size(0x200000)
        .spawn(move || {
            runtime.block_on(async move {
                let mut wake_time = tokio::time::Instant::now() + Duration::from_secs(1);
                let mut iteration = 0;
                loop {
                    let sleep_future = tokio::time::sleep_until(wake_time);

                    tokio::select! {
                        // Use the first available future instead of randomizing
                        biased;

                        _ = sleep_future => {
                            println!("Tick {}", iteration);
                            iteration += 1;
                            wake_time += Duration::from_secs(1);
                        }
                        _ = &mut exit_receiver => break,
                    }
                }
            });
        })
        .expect("Failed to create runtime thread");

    println!("Runtime started!");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            println!("Shutting down...");
            let _ = exit_sender.send(());
            let _ = runtime_thread.join();
            break;
        }

        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}
