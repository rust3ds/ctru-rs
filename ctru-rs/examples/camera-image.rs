use ctru::gfx::Screen;
use ctru::prelude::*;
use ctru::services::cam::{Cam, CamOutputFormat, CamShutterSoundType, CamSize, Camera};
use ctru::services::gspgpu::FramebufferFormat;

use std::time::Duration;

const WIDTH: usize = 400;
const HEIGHT: usize = 240;

// The screen size is the width and height multiplied by 2 (RGB565 store pixels in 2 bytes)
// const BUF_SIZE: usize = WIDTH * HEIGHT * 2;

const WAIT_TIMEOUT: Duration = Duration::from_micros(300);

fn main() {
    ctru::use_panic_handler();

    let apt = Apt::init().expect("Failed to initialize Apt service.");
    let hid = Hid::init().expect("Failed to initialize Hid service.");
    let gfx = Gfx::init().expect("Failed to initialize GFX service.");

    gfx.top_screen.borrow_mut().set_double_buffering(true);
    gfx.top_screen
        .borrow_mut()
        .set_framebuffer_format(FramebufferFormat::Rgb565);
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

    let mut buf;

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

            let camera = &mut cam.outer_right_cam;

            buf = camera
                .take_picture(
                    WIDTH.try_into().unwrap(),
                    HEIGHT.try_into().unwrap(),
                    WAIT_TIMEOUT,
                )
                .expect("Failed to take picture");

            cam.play_shutter_sound(CamShutterSoundType::NORMAL)
                .expect("Failed to play shutter sound");

            let img = rotate_image(&buf, WIDTH, HEIGHT);

            unsafe {
                gfx.top_screen
                    .borrow_mut()
                    .get_raw_framebuffer()
                    .ptr
                    .copy_from(img.as_ptr(), img.len());
            }

            gfx.flush_buffers();
            gfx.swap_buffers();
            gfx.wait_for_vblank();
        }
    }
}

// The 3DS' screens are 2 vertical LCD panels rotated by 90 degrees.
// As such, we'll need to write a "vertical" image to the framebuffer to have it displayed properly.
// This functions handles the rotation of an horizontal image to a vertical one.
fn rotate_image(img: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut res = vec![0u8; img.len()];
    for j in 0..height {
        for i in 0..width {
            // Y-coordinate of where to draw in the frame buffer
            // Height must be esclusive of the upper end (otherwise, we'd be writing to the pixel one column to the right when having j=0)
            let draw_y = (height - 1) - j;
            // X-coordinate of where to draw in the frame buffer
            let draw_x = i;

            // Index of the pixel to draw within the image buffer
            let read_index = (j * width + i) * 2;

            // Initial index of where to draw in the frame buffer based on y and x coordinates
            let draw_index = (draw_x * height + draw_y) * 2; // This 2 stands for the number of bytes per pixel (16 bits)

            res[draw_index] = img[read_index];
            res[draw_index + 1] = img[read_index + 1];
        }
    }
    res
}
