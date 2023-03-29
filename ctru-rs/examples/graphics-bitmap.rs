use ctru::prelude::*;
use ctru::services::gfx::Screen;

/// Ferris image taken from <https://rustacean.net> and scaled down to 320x240px.
/// To regenerate the data, you will need to install `imagemagick` and run this
/// command from the `examples` directory:
///
/// ```sh
/// magick assets/ferris.png -channel-fx "red<=>blue" -rotate 90 assets/ferris.rgb
/// ```
///
/// This creates an image appropriate for the default frame buffer format of
/// [`Bgr8`](ctru::services::gspgpu::FramebufferFormat::Bgr8)
/// and rotates the image 90° to account for the portrait mode screen.
static IMAGE: &[u8] = include_bytes!("assets/ferris.rgb");

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    println!("\x1b[21;16HPress Start to exit.");

    let mut bottom_screen = gfx.bottom_screen.borrow_mut();

    // We don't need double buffering in this example.
    // In this way we can draw our image only once on screen.
    bottom_screen.set_double_buffering(false);

    // We assume the image is the correct size already, so we drop width + height.
    let frame_buffer = bottom_screen.get_raw_framebuffer();

    // Copy the image into the frame buffer
    unsafe {
        frame_buffer.ptr.copy_from(IMAGE.as_ptr(), IMAGE.len());
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
