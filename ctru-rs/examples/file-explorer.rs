//! A file explorer which shows off using standard library file system APIs to
//! read the SD card.

use ctru::applets::swkbd::{Button, Swkbd};
use ctru::console::Console;
use ctru::services::hid::KeyPad;
use ctru::services::{Apt, Hid};
use ctru::Gfx;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

fn main() {
    ctru::init();
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::default();

    #[cfg(all(feature = "romfs", romfs_exists))]
    let _romfs = ctru::romfs::RomFS::new().unwrap();

    FileExplorer::init(&apt, &hid, &gfx).run();
}

struct FileExplorer<'a> {
    apt: &'a Apt,
    hid: &'a Hid,
    gfx: &'a Gfx,
    console: Console<'a>,
    path: PathBuf,
    entries: Vec<DirEntry>,
    running: bool,
}

impl<'a> FileExplorer<'a> {
    fn init(apt: &'a Apt, hid: &'a Hid, gfx: &'a Gfx) -> Self {
        gfx.top_screen.borrow_mut().set_wide_mode(true);
        let console = Console::init(gfx.top_screen.borrow_mut());

        FileExplorer {
            apt,
            hid,
            gfx,
            console,
            path: PathBuf::from("/"),
            entries: Vec::new(),
            running: false,
        }
    }

    fn run(&mut self) {
        self.running = true;
        self.print_menu();

        while self.running && self.apt.main_loop() {
            self.hid.scan_input();
            let input = self.hid.keys_down();

            if input.contains(KeyPad::KEY_START) {
                break;
            } else if input.contains(KeyPad::KEY_B) {
                self.path.pop();
                self.console.clear();
                self.print_menu();
            } else if input.contains(KeyPad::KEY_A) {
                self.get_input_and_run(Self::set_next_path);
            } else if input.contains(KeyPad::KEY_X) {
                self.get_input_and_run(Self::set_exact_path);
            }

            self.gfx.flush_buffers();
            self.gfx.swap_buffers();
            self.gfx.wait_for_vblank();
        }
    }

    fn print_menu(&mut self) {
        println!("Viewing {}", self.path.display());

        let dir_listing = std::fs::read_dir(&self.path).expect("Failed to open path");
        self.entries = Vec::new();

        for (i, entry) in dir_listing.enumerate() {
            match entry {
                Ok(entry) => {
                    println!("{:2} - {}", i, entry.file_name().to_string_lossy());
                    self.entries.push(entry);

                    // Paginate the output
                    if (i + 1) % 20 == 0 {
                        println!("Press A to go to next page, or Start to exit");

                        while self.apt.main_loop() {
                            self.hid.scan_input();
                            let input = self.hid.keys_down();

                            if input.contains(KeyPad::KEY_A) {
                                break;
                            }

                            if input.contains(KeyPad::KEY_START) {
                                self.running = false;
                                return;
                            }

                            self.gfx.wait_for_vblank();
                        }
                    }
                }
                Err(e) => {
                    println!("{} - Error: {}", i, e);
                }
            }
        }

        println!("Start to exit, A to select an entry by number, B to go up a directory, X to set the path.");
    }

    fn get_input_and_run(&mut self, action: impl FnOnce(&mut Self, String)) {
        let mut keyboard = Swkbd::default();
        let mut new_path_str = String::new();

        match keyboard.get_utf8(&mut new_path_str) {
            Ok(Button::Right) => {
                // Clicked "OK"
                action(self, new_path_str);
            }
            Ok(Button::Left) => {
                // Clicked "Cancel"
            }
            Ok(Button::Middle) => {
                // This button wasn't shown
                unreachable!()
            }
            Err(e) => {
                panic!("Error: {:?}", e)
            }
        }
    }

    fn set_next_path(&mut self, next_path_index: String) {
        let next_path_index: usize = match next_path_index.parse() {
            Ok(index) => index,
            Err(e) => {
                println!("Number parsing error: {}", e);
                return;
            }
        };

        let next_entry = match self.entries.get(next_path_index) {
            Some(entry) => entry,
            None => {
                println!("Input number of bounds");
                return;
            }
        };

        if !next_entry.file_type().unwrap().is_dir() {
            println!("Not a directory: {}", next_path_index);
            return;
        }

        self.console.clear();
        self.path = next_entry.path();
        self.print_menu();
    }

    fn set_exact_path(&mut self, new_path_str: String) {
        let new_path = Path::new(&new_path_str);
        if !new_path.is_dir() {
            println!("Not a directory: {}", new_path_str);
            return;
        }

        self.console.clear();
        self.path = new_path.to_path_buf();
        self.print_menu();
    }
}
