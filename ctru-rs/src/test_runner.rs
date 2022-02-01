//! Custom test runner for building/running unit tests on the 3DS.

extern crate test;

use test::TestFn;

use crate::console::Console;
use crate::gfx::Gfx;
use crate::services::hid::{Hid, KeyPad};

/// A custom runner to be used with `#[test_runner]`. This simple implementation
/// runs all tests in series, "failing" on the first one to panic (really, the
/// panic is just treated the same as any normal application panic).
pub(crate) fn test_runner(test_cases: &[&test::TestDescAndFn]) {
    crate::init();

    let gfx = Gfx::default();
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    // TODO: may want to use some more features of standard testing framework,
    // like output capture, filtering, panic handling, etc.
    // For now this is works without too much setup.
    for test_info in test_cases {
        if let TestFn::StaticTestFn(testfn) = test_info.testfn {
            println!("Running test {}", test_info.desc.name);
            testfn();
        } else {
            println!(
                "unsupported test type for {}: {:?}",
                test_info.desc.name, test_info.testfn
            );
        }
    }

    println!("All tests passed! Press START to exit.");

    // TODO: do we need apt.main_loop() here?
    loop {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
    }
}
