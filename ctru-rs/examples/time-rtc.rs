use ctru::console::Console;
use ctru::gfx::Gfx;
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};

fn main() {
    ctru::init();

    let gfx = Gfx::init_default().expect("Couldn't obtaint GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");

    let _console = Console::init(gfx.top_screen.borrow_mut());

    print!("\x1b[30;16HPress Start to exit.");

    // Main loop
    while apt.main_loop() {
        // Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }

        // Technically, this actually just gets local time and assumes it's UTC,
        // since the 3DS doesn't seem to support timezones...
        let cur_time = time::OffsetDateTime::now_utc();

        let hours = cur_time.hour();
        let minutes = cur_time.minute();
        let seconds = cur_time.second();

        let weekday = cur_time.weekday().to_string();
        let month = cur_time.month().to_string();
        let day = cur_time.day();
        let year = cur_time.year();

        println!("\x1b[1;1H{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds);
        println!("{} {} {} {}", weekday, month, day, year);

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
