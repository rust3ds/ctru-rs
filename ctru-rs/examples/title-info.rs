//! Title Info example.
//!
//! This example showcases how to retrieve information about the titles installed on the console running the application
//! via the Application Manager (Am) service.

use ctru::prelude::*;
use ctru::services::am::Am;
use ctru::services::fs::MediaType;

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let top_screen = Console::new(gfx.top_screen.borrow_mut());
    let bottom_screen = Console::new(gfx.bottom_screen.borrow_mut());

    // Setup the AM service to retrieve the wanted information.
    let am = Am::new().expect("Couldn't obtain AM controller");

    // Amount of titles installed on the SD card.
    let sd_count = am
        .title_count(MediaType::Sd)
        .expect("Failed to get sd title count");
    // List of titles installed on the SD card.
    let sd_list = am
        .title_list(MediaType::Sd)
        .expect("Failed to get sd title list");

    // Amount of titles installed on the NAND storage.
    let nand_count = am
        .title_count(MediaType::Nand)
        .expect("Failed to get nand title count");
    // List of titles installed on the NAND storage.
    let nand_list = am
        .title_list(MediaType::Nand)
        .expect("Failed to get nand title list");

    let mut offset = 0;
    let mut refresh = true;
    let mut use_nand = false;

    while apt.main_loop() {
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
                offset += 1;
                refresh = true;
            }
        } else if hid.keys_down().intersects(KeyPad::UP) && offset > 0 {
            offset -= 1;
            refresh = true;
        }

        // Render the title list via a scrollable text UI.
        if refresh {
            let mut selected_title = cur_list.get(offset).unwrap();

            // Clear the top screen and write title IDs to it.
            top_screen.select();
            print!("\x1b[2J");

            // Top screen has 30 rows.
            for (i, title) in cur_list.iter().skip(offset).take(29).enumerate() {
                if i == 0 {
                    selected_title = title;
                    println!("=> {:x}", title.id());
                } else {
                    println!("   {:x}", title.id());
                }
            }

            // Clear the bottom screen and write the properties of selected title to it.
            bottom_screen.select();
            bottom_screen.clear();
            println!("Press Start to exit");

            // Move cursor to top left.
            println!("\x1b[1;1");

            println!("Size: {} kB", selected_title.size() / 1024);
            println!("Version: 0x{:x}", selected_title.version());
            println!("Product code: \"{}\"", selected_title.product_code());

            if use_nand {
                println!("Press SELECT to choose SD Card");
                println!("Current medium: NAND");
                println!("Title count: {nand_count}");
            } else {
                println!("Press SELECT to choose NAND");
                println!("Current medium: SD Card");
                println!("Title count: {sd_count}");
            }

            refresh = false;
        }

        gfx.wait_for_vblank();
    }
}
