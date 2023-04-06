use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    // Initialize services
    //
    // HashMaps generate hashes thanks to the 3DS' cryptografically secure generator.
    // This generator is only active when activating the `PS` service.
    // This service is automatically initialized.
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let _console = Console::new(gfx.top_screen.borrow_mut());

    let mut map = std::collections::HashMap::new();
    map.insert("A Key!", 102);
    map.insert("Another key?", 543);
    map.remove("A Key!");

    println!("{map:#?}");

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
    }
}
