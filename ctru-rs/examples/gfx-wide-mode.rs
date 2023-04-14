use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    let apt = Apt::init().unwrap();
    let mut hid = Hid::init().unwrap();
    let gfx = Gfx::init().unwrap();
    let mut console = Console::init(gfx.top_screen.borrow_mut());

    println!("Press A to enable/disable wide screen mode.");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        if hid.keys_down().contains(KeyPad::A) {
            drop(console);

            let wide_mode = gfx.top_screen.borrow().is_wide();
            gfx.top_screen.borrow_mut().set_wide_mode(!wide_mode);

            console = Console::init(gfx.top_screen.borrow_mut());
            println!("Press A to enable/disable wide screen mode.");
        }

        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}
