use ctru::console::Console;
use ctru::gfx::{Screen, Side};
use ctru::services::cam::{
    Cam, CamContext, CamOutputFormat, CamPort, CamSelect, CamShutterSoundType, CamSize,
};
use ctru::services::hid::KeyPad;
use ctru::services::{Apt, Hid};
use ctru::Gfx;

const WIDTH: i16 = 400;
const HEIGHT: i16 = 240;
const SCREEN_SIZE: u32 = 192000; // WIDTH * HEIGHT * 2;
const BUF_SIZE: usize = 384000; // SCREEN_SIZE * 2;

const WAIT_TIMEOUT: i64 = 300000000;

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

    cam.set_size(
        CamSelect::OUT1_OUT2,
        CamSize::CTR_TOP_LCD,
        CamContext::A,
    )
    .expect("Failed to set camera size");
    cam.set_output_format(
        CamSelect::OUT1_OUT2,
        CamOutputFormat::RGB_565,
        CamContext::A,
    )
    .expect("Failed to set camera output format");

    cam.set_noise_filter(CamSelect::OUT1_OUT2, true)
        .expect("Failed to enable noise filter");
    cam.set_auto_exposure(CamSelect::OUT1_OUT2, true)
        .expect("Failed to enable auto exposure");
    cam.set_auto_white_balance(CamSelect::OUT1_OUT2, true)
        .expect("Failed to enable auto white balance");

    cam.set_trimming(CamPort::CAM1, false)
        .expect("Failed to disable trimming for Cam Port 1");
    cam.set_trimming(CamPort::CAM2, false)
        .expect("Failed to disable trimming for Cam Port 2");

    let mut buf = [0u8; BUF_SIZE];

    gfx.flush_buffers();
    gfx.swap_buffers();
    gfx.wait_for_vblank();

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
            take_picture(&mut cam, &mut buf);
        }

        let img = convert_image_to_rgb8(buf, 0, 0, WIDTH as usize, HEIGHT as usize);

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

fn take_picture(cam: &mut Cam, buf: &mut [u8]) {
    let buf_size = cam
        .get_max_bytes(WIDTH, HEIGHT)
        .expect("Failed to get max bytes");

    cam.set_transfer_bytes(CamPort::BOTH, buf_size, WIDTH, HEIGHT)
        .expect("Failed to set transfer bytes");

    cam.activate(CamSelect::OUT1_OUT2)
        .expect("Failed to activate camera");

    cam.clear_buffer(CamPort::BOTH)
        .expect("Failed to clear buffer");
    cam.synchronize_vsync_timing(CamSelect::OUT1, CamSelect::OUT2)
        .expect("Failed to sync vsync timings");

    cam.start_capture(CamPort::BOTH)
        .expect("Failed to start capture");

    let receive_event = cam
        .set_receiving(buf, CamPort::CAM1, SCREEN_SIZE, buf_size as i16)
        .expect("Failed to set receiving");

    let receive_event2 = cam
        .set_receiving(
            &mut buf[SCREEN_SIZE as usize..],
            CamPort::CAM2,
            SCREEN_SIZE,
            buf_size as i16,
        )
        .expect("Failed to set receiving");

    unsafe {
        let mut r = ctru_sys::svcWaitSynchronization(receive_event, WAIT_TIMEOUT);
        if r < 0 {
            panic!("Failed to wait for handle synchronization");
        }
        r = ctru_sys::svcWaitSynchronization(receive_event2, WAIT_TIMEOUT);
        if r < 0 {
            panic!("Failed to wait for handle 2 synchronization");
        }
    };

    cam.play_shutter_sound(CamShutterSoundType::NORMAL)
        .expect("Failed to play shutter sound");

    unsafe {
        let mut r = ctru_sys::svcCloseHandle(receive_event);
        if r < 0 {
            panic!("Failed to close handle");
        }
        r = ctru_sys::svcCloseHandle(receive_event2);
        if r < 0 {
            panic!("Failed to close handle 2");
        }
    };

    cam.activate(CamSelect::NONE)
        .expect("Failed to deactivate camera");
}

// The available camera output formats are both using u16 values.
// To write to the frame buffer with the default RGB8 format,
// the values must be converted.
//
// Alternatively, the frame buffer format could be set to RGB565 as well
// but the image would need to be rotated 90 degrees.
fn convert_image_to_rgb8<const SIZE: usize>(
    img: [u8; SIZE],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> [u8; SIZE] {
    let mut rgb8 = [0u8; SIZE];
    let mut draw_x;
    let mut draw_y;
    for j in 0..height {
        for i in 0..width {
            // Y-coordinate of where to draw in the frame buffer
            draw_y = y + height - j;
            // X-coordinate of where to draw in the frame buffer
            draw_x = x + i;
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
