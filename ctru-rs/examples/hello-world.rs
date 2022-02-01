use ctru::console::Console;
use ctru::gfx::Gfx;
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};

use std::io::BufWriter;

fn main() {
    ctru::init();
    let gfx = Gfx::default();
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    struct Timespec {
        t: libc::timespec,
    }

    //let inst = std::time::Instant::now();
    let mut t = Timespec { t: libc::timespec { tv_sec: 0, tv_nsec: 0 } };
    let res = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut t.t) };
    println!("{{tv_sec: {}, tv_nsec: {} }} {} clock:{}", t.t.tv_sec, t.t.tv_nsec, res, libc::CLOCK_MONOTONIC);
    ctru::thread::sleep(std::time::Duration::from_secs(2));
    /*let ela = inst.elapsed();
    println!(
        "\x1b[0;0HElapsed: {:#?}",
        ela
    );*/



    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
