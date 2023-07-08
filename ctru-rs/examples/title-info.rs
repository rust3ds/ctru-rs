use ctru::prelude::*;
use ctru::services::am::Am;
use ctru::services::fs::FsMediaType;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let am = Am::new().expect("Couldn't obtain AM controller");
    let top_screen = Console::new(gfx.top_screen.borrow_mut());
    let bottom_screen = Console::new(gfx.bottom_screen.borrow_mut());

    let sd_count = am
        .title_count(FsMediaType::Sd)
        .expect("Failed to get sd title count");
    let sd_list = am
        .title_list(FsMediaType::Sd)
        .expect("Failed to get sd title list");

    let nand_count = am
        .title_count(FsMediaType::Nand)
        .expect("Failed to get nand title count");
    let nand_list = am
        .title_list(FsMediaType::Nand)
        .expect("Failed to get nand title list");

    let mut offset = 0;
    let mut refresh = true;
    let mut use_nand = false;

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
        if hid.keys_down().contains(KeyPad::SELECT) {
            refresh = true;
            offset = 0;
            use_nand = !use_nand;
        }

        let cur_list = if use_nand { &nand_list } else { &sd_list };

        if hid.keys_down().intersects(KeyPad::DOWN) {
            if offset + 1 < cur_list.len() {
                offset = offset + 1;
                refresh = true;
            }
        } else if hid.keys_down().intersects(KeyPad::UP) {
            if offset > 0 {
                offset = offset - 1;
                refresh = true;
            }
        }

        if refresh {
            let mut selected_title = cur_list.iter().skip(offset).next().unwrap();
            // Clear top screen and write title ids to it
            top_screen.select();
            print!("\x1b[2J");

            // Top screen seems to have only 30 rows
            for (i, title) in cur_list.iter().skip(offset).take(29).enumerate() {
                if i == 0 {
                    selected_title = title;
                    println!("=> {:x}", title.id());
                } else {
                    println!("   {:x}", title.id());
                }
            }

            // Clear bottom screen and write properties of selected title to it
            bottom_screen.select();
            println!("\x1b[2J");
            // Move cursor to top left
            println!("\x1b[1;1");

            match selected_title.size() {
                Ok(size) => println!("Size: {} kB", size / 1024),
                Err(e) => println!("Failed to get title size: {}", e),
            }
            match selected_title.version() {
                Ok(version) => println!("Version: 0x{:x}", version),
                Err(e) => println!("Failed to get title version: {}", e),
            }
            match selected_title.product_code() {
                Ok(code) => println!("Product code: \"{code}\""),
                Err(e) => println!("Failed to get product code: {}", e),
            }

            println!("\x1b[26;0HPress START to exit");
            if use_nand {
                println!("Press SELECT to choose SD Card");
                println!("Current medium: NAND");
                println!("Title count: {}", nand_count);
            } else {
                println!("Press SELECT to choose NAND");
                println!("Current medium: SD Card");
                println!("Title count: {}", sd_count);
            }

            refresh = false;
        }

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
