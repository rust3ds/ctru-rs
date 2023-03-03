use ctru::prelude::*;
use ctru::services::am::Am;
use ctru::services::fs::FsMediaType;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let am = Am::init().expect("Couldn't obtain AM controller");
    let top_screen = Console::init(gfx.top_screen.borrow_mut());
    let bottom_screen = Console::init(gfx.bottom_screen.borrow_mut());

    let sd_count = am
        .get_title_count(FsMediaType::Sd)
        .expect("Failed to get sd title count");
    let sd_list = am
        .get_title_list(FsMediaType::Sd)
        .expect("Failed to get sd title list");

    let nand_count = am
        .get_title_count(FsMediaType::Nand)
        .expect("Failed to get nand title count");
    let nand_list = am
        .get_title_list(FsMediaType::Nand)
        .expect("Failed to get nand title list");

    let mut offset = 0;
    let mut refresh = true;
    let mut use_nand = false;

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
        if hid.keys_down().contains(KeyPad::KEY_SELECT) {
            refresh = true;
            offset = 0;
            use_nand = !use_nand;
        }

        let cur_list = if use_nand { &nand_list } else { &sd_list };

        if hid.keys_down().intersects(KeyPad::KEY_DOWN) {
            if offset + 1 < cur_list.len() {
                offset = offset + 1;
                refresh = true;
            }
        } else if hid.keys_down().intersects(KeyPad::KEY_UP) {
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

            match selected_title.get_title_info() {
                Ok(info) => {
                    println!("Size: {} KB", info.size_bytes() / 1024);
                    println!("Version: 0x{:x}", info.version());
                }
                Err(e) => println!("Failed to get title info: {}", e),
            }
            match selected_title.get_product_code() {
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

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
