//! File Explorer example.
//!
//! This (rather complex) example creates a working text-based file explorer which shows off using standard library file system APIs to
//! read the SD card and RomFS (if properly read via the `romfs:/` prefix).

use ctru::applets::swkbd::{Button, SoftwareKeyboard};
use ctru::prelude::*;

use std::fs::DirEntry;
use std::os::horizon::fs::MetadataExt;
use std::path::{Path, PathBuf};

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();

    // Mount the RomFS if available.
    #[cfg(all(feature = "romfs", romfs_exists))]
    let _romfs = ctru::services::romfs::RomFS::new().unwrap();

    FileExplorer::new(&apt, &mut hid, &gfx).run();
}

struct FileExplorer<'a> {
    apt: &'a Apt,
    hid: &'a mut Hid,
    gfx: &'a Gfx,
    console: Console<'a>,
    path: PathBuf,
    entries: Vec<DirEntry>,
    running: bool,
}

impl<'a> FileExplorer<'a> {
    fn new(apt: &'a Apt, hid: &'a mut Hid, gfx: &'a Gfx) -> Self {
        let mut top_screen = gfx.top_screen.borrow_mut();
        top_screen.set_wide_mode(true);
        let console = Console::new(top_screen);

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
        // Print the file explorer commands.
        self.print_menu();

        while self.running && self.apt.main_loop() {
            self.hid.scan_input();
            let input = self.hid.keys_down();

            if input.contains(KeyPad::START) {
                break;
            } else if input.contains(KeyPad::B) && self.path.components().count() > 1 {
                self.path.pop();
                self.console.clear();
                self.print_menu();
            // Open a directory/file to read.
            } else if input.contains(KeyPad::A) {
                self.get_input_and_run(Self::set_next_path);
            // Open a specific path using the `SoftwareKeyboard` applet.
            } else if input.contains(KeyPad::X) {
                self.get_input_and_run(Self::set_exact_path);
            }

            self.gfx.wait_for_vblank();
        }
    }

    fn print_menu(&mut self) {
        match std::fs::metadata(&self.path) {
            Ok(metadata) => {
                println!(
                    "Viewing {} (size {} bytes, mode {:#o})",
                    self.path.display(),
                    metadata.len(),
                    metadata.st_mode(),
                );

                if metadata.is_file() {
                    self.print_file_contents();
                    // let the user continue navigating from the parent dir
                    // after dumping the file
                    self.path.pop();
                    self.print_menu();
                    return;
                } else if metadata.is_dir() {
                    self.print_dir_entries();
                } else {
                    println!("unsupported file type: {:?}", metadata.file_type());
                }
            }
            Err(e) => {
                println!("Failed to read {}: {e}", self.path.display())
            }
        };

        println!("Press Start to exit, A to select an entry by number, B to go up a directory, X to set the path.");
    }

    fn print_dir_entries(&mut self) {
        let dir_listing = std::fs::read_dir(&self.path).expect("Failed to open path");
        self.entries = Vec::new();

        for (i, entry) in dir_listing.enumerate() {
            match entry {
                Ok(entry) => {
                    println!("{i:2} - {}", entry.file_name().to_string_lossy());
                    self.entries.push(entry);

                    if (i + 1) % 20 == 0 {
                        self.wait_for_page_down();
                    }
                }
                Err(e) => {
                    println!("{i} - Error: {e}");
                }
            }
        }
    }

    fn print_file_contents(&mut self) {
        match std::fs::read_to_string(&self.path) {
            Ok(contents) => {
                println!("File contents:\n{0:->80}", "");
                println!("{contents}");
                println!("{0:->80}", "");
            }
            Err(err) => {
                println!("Error reading file: {err}");
            }
        }
    }

    // Paginate output.
    fn wait_for_page_down(&mut self) {
        println!("Press A to go to next page, or Start to exit");

        while self.apt.main_loop() {
            self.hid.scan_input();
            let input = self.hid.keys_down();

            if input.contains(KeyPad::A) {
                break;
            }

            if input.contains(KeyPad::START) {
                self.running = false;
                return;
            }

            self.gfx.wait_for_vblank();
        }
    }

    fn get_input_and_run(&mut self, action: impl FnOnce(&mut Self, String)) {
        let mut keyboard = SoftwareKeyboard::default();

        match keyboard.get_string(2048) {
            Ok((path, Button::Right)) => {
                // Clicked "OK".
                action(self, path);
            }
            Ok((_, Button::Left)) => {
                // Clicked "Cancel".
            }
            Ok((_, Button::Middle)) => {
                // This button wasn't shown.
                unreachable!()
            }
            Err(e) => {
                panic!("Error: {e:?}")
            }
        }
    }

    fn set_next_path(&mut self, next_path_index: String) {
        let next_path_index: usize = match next_path_index.parse() {
            Ok(index) => index,
            Err(e) => {
                println!("Number parsing error: {e}");
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

        self.console.clear();
        self.path = next_entry.path();
        self.print_menu();
    }

    fn set_exact_path(&mut self, new_path_str: String) {
        let new_path = Path::new(&new_path_str);
        if !new_path.is_dir() {
            println!("Not a directory: {new_path_str}");
            return;
        }

        self.console.clear();
        self.path = new_path.to_path_buf();
        self.print_menu();
    }
}
