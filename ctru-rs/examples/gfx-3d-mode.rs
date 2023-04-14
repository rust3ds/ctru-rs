use ctru::prelude::*;
use ctru::services::gfx::{Screen, Side, TopScreen3D};

/// See `graphics-bitmap.rs` for details on how the image is generated.
///
/// WARNING: this example uses 3D mode in a rather unnatural way, and should
/// probably not be viewed for too long or at all if you are photosensitive.

const IMAGE: &[u8] = include_bytes!("assets/ferris.rgb");
static ZERO: &[u8] = &[0; IMAGE.len()];

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.bottom_screen.borrow_mut());

    println!("Press Start to exit.\nPress A to switch sides (be sure to have 3D mode enabled).");

    gfx.top_screen.borrow_mut().set_double_buffering(true);

    let top_screen = TopScreen3D::from(&gfx.top_screen);
    let (mut left, mut right) = top_screen.split_mut();

    let mut current_side = Side::Left;

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        let left_buf = left.raw_framebuffer();
        let right_buf = right.raw_framebuffer();

        // Clear both buffers every time, in case the user switches sides this loop
        unsafe {
            left_buf.ptr.copy_from(ZERO.as_ptr(), ZERO.len());
            right_buf.ptr.copy_from(ZERO.as_ptr(), ZERO.len());
        }

        if hid.keys_down().contains(KeyPad::A) {
            // flip which buffer we're writing to
            current_side = match current_side {
                Side::Left => Side::Right,
                Side::Right => Side::Left,
            };
        }

        let buf = match current_side {
            Side::Left => left_buf.ptr,
            Side::Right => right_buf.ptr,
        };

        unsafe {
            buf.copy_from(IMAGE.as_ptr(), IMAGE.len());
        }

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
