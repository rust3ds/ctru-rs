use ctru::console::Console;
use ctru::gfx::{Screen as _, Side};
use ctru::services::hid::KeyPad;
use ctru::services::{Apt, Hid};
use ctru::Gfx;

/// Ferris image taken from <https://rustacean.net> and scaled down to 320x240px.
/// To regenerate the data, you will need to install `imagemagick` and run this
/// command from the `examples` directory:
///
/// ```sh
/// convert assets/ferris.png -channel B -separate \
///     assets/ferris.png -channel G -separate \
///     assets/ferris.png -channel R -separate \
///     -channel RGB -combine -rotate 90 \
///     assets/ferris.rgb
/// ```
static IMAGE: &[u8] = include_bytes!("assets/ferris.rgb");

fn main() {
    ctru::init();
    let gfx = Gfx::default();
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    println!("\x1b[21;16HPress Start to exit.");

    let mut bottom_screen = gfx.bottom_screen.borrow_mut();

    // We don't need double buffering in this example.
    // In this way we can draw our image only once on screen.
    bottom_screen.set_double_buffering(false);

    // The "Left" side framebuffer is the only valid one for bottom screen
    // TODO: make `get_raw_framebuffer` only accept a side for top screen
    // Also, we assume the image is the correct size already...
    let (frame_buffer, _width, _height) = bottom_screen.get_raw_framebuffer(Side::Left);

    // Copy the image into the frame buffer
    unsafe {
        frame_buffer.copy_from(IMAGE.as_ptr(), IMAGE.len());
    }

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
