use std::process::Termination;

use ctru::prelude::*;
use ctru::services::gfx::{Flush, Swap};

use super::TestRunner;

/// Run tests using the [`ctru::console::Console`] (print results to the 3DS screen).
/// This is mostly useful for running tests manually, especially on real hardware.
pub struct ConsoleRunner {
    gfx: Gfx,
    hid: Hid,
    apt: Apt,
}

impl TestRunner for ConsoleRunner {
    type Context<'this> = Console<'this>;

    fn new() -> Self {
        let gfx = Gfx::new().unwrap();
        let hid = Hid::new().unwrap();
        let apt = Apt::new().unwrap();

        gfx.top_screen.borrow_mut().set_wide_mode(true);

        Self { gfx, hid, apt }
    }

    fn setup(&mut self) -> Self::Context<'_> {
        Console::new(self.gfx.top_screen.borrow_mut())
    }

    fn cleanup<T: Termination>(mut self, result: T) -> T {
        // We don't actually care about the output of the test result, either
        // way we'll stop and show the results to the user.

        println!("Press START to exit.");

        while self.apt.main_loop() {
            let mut screen = self.gfx.top_screen.borrow_mut();
            screen.flush_buffers();
            screen.swap_buffers();

            self.gfx.wait_for_vblank();

            self.hid.scan_input();
            if self.hid.keys_down().contains(KeyPad::START) {
                break;
            }
        }

        result
    }
}
