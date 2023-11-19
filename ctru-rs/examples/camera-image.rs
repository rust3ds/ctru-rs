//! Camera image example.
//!
//! This example demonstrates how to use the built-in cameras to take a picture and display it to the screen.

use ctru::prelude::*;
use ctru::services::cam::{
    Cam, Camera, OutputFormat, ShutterSound, Trimming, ViewSize, WhiteBalance,
};
use ctru::services::gfx::{Flush, Screen, Swap, TopScreen3D};
use ctru::services::gspgpu::FramebufferFormat;

use std::time::Duration;

const WAIT_TIMEOUT: Duration = Duration::from_millis(300);

fn main() {
    ctru::use_panic_handler();

    let apt = Apt::new().expect("Failed to initialize Apt service.");
    let mut hid = Hid::new().expect("Failed to initialize Hid service.");
    let gfx = Gfx::new().expect("Failed to initialize GFX service.");

    gfx.top_screen.borrow_mut().set_double_buffering(true);
    gfx.top_screen
        .borrow_mut()
        .set_framebuffer_format(FramebufferFormat::Rgb565);

    let mut top_screen_3d = TopScreen3D::from(&gfx.top_screen);

    let _console = Console::new(gfx.bottom_screen.borrow_mut());

    println!("Initializing camera");

    let mut cam = Cam::new().expect("Failed to initialize CAM service.");

    // Camera setup.
    let camera = &mut cam.both_outer_cams;
    {
        camera
            .set_view_size(ViewSize::TopLCD)
            .expect("Failed to set camera size");
        camera
            .set_output_format(OutputFormat::Rgb565)
            .expect("Failed to set camera output format");
        camera
            .set_noise_filter(true)
            .expect("Failed to enable noise filter");
        camera
            .set_auto_exposure(true)
            .expect("Failed to enable auto exposure");
        camera
            .set_white_balance(WhiteBalance::Auto)
            .expect("Failed to enable auto white balance");
        // This line has no effect on the camera since the photos are already shot with `TopLCD` size.
        camera
            .set_trimming(Trimming::new_centered_with_view(ViewSize::TopLCD))
            .expect("Failed to enable trimming");
    }

    // We don't intend on making any other modifications to the camera, so this size should be enough.
    let len = camera.final_byte_length();
    let mut buf = vec![0u8; len];

    println!("\nPress R to take a new picture");
    println!("Press Start to exit");

    while apt.main_loop() {
        hid.scan_input();
        let keys_down = hid.keys_down();

        if keys_down.contains(KeyPad::START) {
            break;
        }

        // If the user presses the R button.
        if keys_down.contains(KeyPad::R) {
            println!("Capturing new image");

            let camera = &mut cam.both_outer_cams;

            // Take a picture and write it to the buffer.
            camera
                .take_picture(&mut buf, WAIT_TIMEOUT)
                .expect("Failed to take picture");

            let (width, height) = camera.final_view_size();

            // Play the normal shutter sound.
            cam.play_shutter_sound(ShutterSound::Normal)
                .expect("Failed to play shutter sound");

            {
                let (mut left_side, mut right_side) = top_screen_3d.split_mut();

                // Rotate the left image and correctly display it on the screen.
                rotate_image_to_screen(
                    &buf,
                    left_side.raw_framebuffer().ptr,
                    width as usize,
                    height as usize,
                );

                // Rotate the right image and correctly display it on the screen.
                rotate_image_to_screen(
                    &buf[len / 2..],
                    right_side.raw_framebuffer().ptr,
                    width as usize,
                    height as usize,
                );
            }

            // We will only flush and swap the "camera" screen, since the other screen is handled by the `Console`.
            top_screen_3d.flush_buffers();
            top_screen_3d.swap_buffers();

            gfx.wait_for_vblank();
        }
    }
}

// The 3DS' screens are 2 vertical LCD panels rotated by 90 degrees.
// As such, we'll need to write a "vertical" image to the framebuffer to have it displayed properly.
// This functions rotates an horizontal image by 90 degrees to the right.
// This function is only supposed to be used in this example. In a real world application, the program should use the GPU to draw to the screen.
fn rotate_image_to_screen(src: &[u8], framebuf: *mut u8, width: usize, height: usize) {
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

            unsafe {
                // We'll work with pointers since the framebuffer is a raw pointer regardless.
                // The offsets are completely safe as long as the width and height are correct.
                let pixel_pointer = framebuf.add(draw_index);
                pixel_pointer.copy_from(src.as_ptr().add(read_index), 2);
            }
        }
    }
}
