use ctru::gfx::TopScreen3D;
use ctru::prelude::*;

/// See `graphics-bitmap.rs` for details on how the image is generated.
///
/// WARNING: this example uses 3D mode in a rather unnatural way, and should
/// probably not be viewed for too long or at all if you are photosensitive.

const IMAGE: &[u8] = include_bytes!("assets/ferris.rgb");
const ZERO: &[u8] = &[0; IMAGE.len()];

fn main() {
    ctru::init();
    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.bottom_screen.borrow_mut());

    println!("Press Start to exit.\nPress A to switch which side is drawn to.");

    let top_screen = TopScreen3D::from(&gfx.top_screen);

    // TODO set double buffering for top screen

    let mut left = top_screen.left_mut();
    let left_buf = left.get_raw_framebuffer();
    let mut right = top_screen.right_mut();
    let right_buf = right.get_raw_framebuffer();

    // We assume the image is the correct size already, so we ignore width + height.
    let mut buf = left_buf.ptr;

    // Copy the image into the left-side frame buffer
    unsafe {
        buf.copy_from(IMAGE.as_ptr(), IMAGE.len());
    }

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }

        if hid.keys_down().contains(KeyPad::KEY_A) {
            // Clear the side we just drew to by zeroing it out
            unsafe {
                buf.copy_from(ZERO.as_ptr(), ZERO.len());
            }

            // flip which buffer we're writing to, and redraw the image
            buf = if buf == left_buf.ptr {
                right_buf.ptr
            } else {
                left_buf.ptr
            };

            unsafe {
                buf.copy_from(IMAGE.as_ptr(), IMAGE.len());
            }
        }

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
