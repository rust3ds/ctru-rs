//! 3D Graphics example.
//!
//! This example showcases 3D mode rendering (using the CPU).
//! In a normal application, all rendering should be hanlded via the GPU.

use ctru::prelude::*;
use ctru::services::gfx::{Flush, Screen, Side, Swap, TopScreen3D};

// See `graphics-bitmap.rs` for details on how the image is generated.
//
// WARNING: this example uses 3D mode in a rather unnatural way, and should
// probably not be viewed for too long or at all if you are photosensitive.

const IMAGE: &[u8] = include_bytes!("assets/ferris.rgb");
static ZERO: &[u8] = &[0; IMAGE.len()];

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let _console = Console::new(gfx.bottom_screen.borrow_mut());

    println!("Press A to switch sides (be sure to have set the 3D slider correctly).");
    println!("\x1b[29;16HPress Start to exit");

    gfx.top_screen.borrow_mut().set_double_buffering(true);

    let mut top_screen = TopScreen3D::from(&gfx.top_screen);

    let mut current_side = Side::Left;

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        // Split the TopScreen3D to get references to the two render surfaces.
        let (mut left, mut right) = top_screen.split_mut();

        let left_buf = left.raw_framebuffer();
        let right_buf = right.raw_framebuffer();

        // Clear both buffers every time, in case the user switches sides this loop.
        unsafe {
            left_buf.ptr.copy_from(ZERO.as_ptr(), ZERO.len());
            right_buf.ptr.copy_from(ZERO.as_ptr(), ZERO.len());
        }

        if hid.keys_down().contains(KeyPad::A) {
            // Switch which buffer we're writing to.
            current_side = match current_side {
                Side::Left => Side::Right,
                Side::Right => Side::Left,
            };
        }

        // Obtain the framebuffer of the currently rendered side.
        let buf = match current_side {
            Side::Left => left_buf.ptr,
            Side::Right => right_buf.ptr,
        };

        // Render the image to the surface's buffer.
        unsafe {
            buf.copy_from(IMAGE.as_ptr(), IMAGE.len());
        }

        drop((left, right));

        top_screen.flush_buffers();
        top_screen.swap_buffers();

        gfx.wait_for_vblank();
    }
}
