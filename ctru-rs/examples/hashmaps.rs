use ctru::console::Console;
use ctru::gfx::Gfx;
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};
use ctru::services::ps::Ps;

fn main() {
    // Initialize services
    ctru::init();
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::default();
    let _console = Console::init(gfx.top_screen.borrow_mut());

    // HashMaps generate hashes thanks to the 3DS' criptografically secure generator.
    // Sadly, this generator is only active when activating the `Ps` service.
    // To do this, we have to make sure the `Ps` service handle is alive for the whole
    // run time (or at least, when `HashMaps` are used).
    // Not having a living `Ps` instance when using `HashMap`s results in a panic
    let _ps = Ps::init().unwrap();

    let mut map = std::collections::HashMap::new();
    map.insert("A Key!", 102);
    map.insert("Another key?", 543);
    map.remove("A Key!");

    println!("{:#?}", map);

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
