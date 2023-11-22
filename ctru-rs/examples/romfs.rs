//! RomFS example.
//!
//! This example showcases the RomFS service and how to mount it to include a read-only filesystem within the application bundle.

use ctru::prelude::*;

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let _console = Console::new(gfx.top_screen.borrow_mut());

    cfg_if::cfg_if! {
        // Run this code if RomFS are wanted and available.
        // This never fails as `ctru-rs` examples inherit all of the `ctru-rs` features,
        // but it might if a normal user application wasn't setup correctly.
        if #[cfg(all(feature = "romfs", romfs_exists))] {
            // Mount the romfs volume.
            let _romfs = ctru::services::romfs::RomFS::new().unwrap();

            // Open a simple text file present in the RomFS volume.
            // Remember to use the `romfs:/` prefix when working with `RomFS`.
            let f = std::fs::read_to_string("romfs:/test-file.txt").unwrap();
            println!("Contents of test-file.txt: \n{f}\n");

            let f = std::fs::read_to_string("romfs:/ファイル.txt").unwrap();
            // While `RomFS` supports UTF-16 file paths, `Console` does not, so we'll use a placeholder for the text.
            println!("Contents of [UTF-16 File]: \n{f}\n");
        } else {
            println!("No RomFS was found, are you sure you included it?")
        }
    }

    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        gfx.wait_for_vblank();
    }
}
