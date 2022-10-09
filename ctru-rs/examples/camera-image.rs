use ctru::console::Console;
use ctru::gfx::{Gfx, Side};
use ctru::services::cam::{Cam, CamOutputFormat, CamShutterSoundType, CamSize, Camera};
use ctru::services::hid::KeyPad;
use ctru::services::{Apt, Hid};
use std::time::Duration;

const WIDTH: usize = 400;
const HEIGHT: usize = 240;

// The screen size is the width and height multiplied by 2 and
// then multiplied by 2 again for 3D images
const BUF_SIZE: usize = WIDTH * HEIGHT * 2 * 2;

const WAIT_TIMEOUT: Duration = Duration::from_micros(300);

fn main() {
    ctru::init();

    let apt = Apt::init().expect("Failed to initialize Apt service.");
    let hid = Hid::init().expect("Failed to initialize Hid service.");
    let gfx = Gfx::init().expect("Failed to initialize GFX service.");

    gfx.top_screen.borrow_mut().set_double_buffering(true);
    gfx.bottom_screen.borrow_mut().set_double_buffering(false);

    let _console = Console::init(gfx.bottom_screen.borrow_mut());

    let mut keys_down;

    println!("Initializing camera");

    let mut cam = Cam::init().expect("Failed to initialize CAM service.");

    {
        let camera = &mut cam.outer_right_cam;

        camera
            .set_view_size(CamSize::CTR_TOP_LCD)
            .expect("Failed to set camera size");
        camera
            .set_output_format(CamOutputFormat::RGB_565)
            .expect("Failed to set camera output format");
        camera
            .set_noise_filter(true)
            .expect("Failed to enable noise filter");
        camera
            .set_auto_exposure(true)
            .expect("Failed to enable auto exposure");
        camera
            .set_auto_white_balance(true)
            .expect("Failed to enable auto white balance");
        camera
            .set_trimming(false)
            .expect("Failed to disable trimming");
    }
    let mut buf = vec![0u8; BUF_SIZE];

    println!("\nPress R to take a new picture");
    println!("Press Start to exit to Homebrew Launcher");

    while apt.main_loop() {
        hid.scan_input();
        keys_down = hid.keys_down();

        if keys_down.contains(KeyPad::KEY_START) {
            break;
        }

        if keys_down.contains(KeyPad::KEY_R) {
            println!("Capturing new image");
            cam.play_shutter_sound(CamShutterSoundType::NORMAL)
                .expect("Failed to play shutter sound");

            let camera = &mut cam.outer_right_cam;

            buf = camera
                .take_picture(
                    WIDTH.try_into().unwrap(),
                    HEIGHT.try_into().unwrap(),
                    WAIT_TIMEOUT,
                )
                .expect("Failed to take picture");
        }

        let img = convert_image_to_rgb8(&buf, 0, 0, WIDTH as usize, HEIGHT as usize);

        unsafe {
            gfx.top_screen
                .borrow_mut()
                .get_raw_framebuffer(Side::Left)
                .ptr
                .copy_from(img.as_ptr(), img.len());
        }

        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}

// The available camera output formats are both using u16 values.
// To write to the frame buffer with the default RGB8 format,
// the values must be converted.
//
// Alternatively, the frame buffer format could be set to RGB565 as well
// but the image would need to be rotated 90 degrees.
fn convert_image_to_rgb8(img: &[u8], x: usize, y: usize, width: usize, height: usize) -> Vec<u8> {
    let mut rgb8 = vec![0u8; img.len()];
    for j in 0..height {
        for i in 0..width {
            // Y-coordinate of where to draw in the frame buffer
            let draw_y = y + height - j;
            // X-coordinate of where to draw in the frame buffer
            let draw_x = x + i;
            // Initial index of where to draw in the frame buffer based on y and x coordinates
            let draw_index = (draw_y + draw_x * height) * 3;

            // Index of the pixel to draw within the image buffer
            let index = (j * width + i) * 2;
            // Pixels in the image are 2 bytes because of the RGB565 format.
            let pixel = u16::from_ne_bytes(img[index..index + 2].try_into().unwrap());
            // b value from the pixel
            let b = (((pixel >> 11) & 0x1F) << 3) as u8;
            // g value from the pixel
            let g = (((pixel >> 5) & 0x3F) << 2) as u8;
            // r value from the pixel
            let r = ((pixel & 0x1F) << 3) as u8;

            // set the r, g, and b values to the calculated index within the frame buffer
            rgb8[draw_index] = r;
            rgb8[draw_index + 1] = g;
            rgb8[draw_index + 2] = b;
        }
    }
    rgb8
}
