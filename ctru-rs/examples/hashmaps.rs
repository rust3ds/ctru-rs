use ctru::prelude::*;

fn main() {
    // Initialize services
    //
    // HashMaps generate hashes thanks to the 3DS' cryptografically secure generator.
    // This generator is only active when activating the `PS` service.
    // This service is automatically initialized in `ctru::init`
    ctru::init();
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::init().unwrap();
    let _console = Console::init(gfx.top_screen.borrow_mut());

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
