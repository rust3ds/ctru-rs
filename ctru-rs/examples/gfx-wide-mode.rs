use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let mut console = Console::new(gfx.top_screen.borrow_mut());

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

            console = Console::new(gfx.top_screen.borrow_mut());
            println!("Press A to enable/disable wide screen mode.");
        }
        
        gfx.wait_for_vblank();
    }
}
