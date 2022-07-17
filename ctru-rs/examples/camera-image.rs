use ctru::console::Console;
use ctru::gfx::{RawFrameBuffer, Screen, Side};
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

    let mut key_down;
    let mut key_held;

    println!("Initializing camera");

    let mut cam = Cam::init().expect("Failed to initialize CAM service.");

    cam.set_size(
        CamSelect::SELECT_OUT1_OUT2,
        CamSize::SIZE_CTR_TOP_LCD,
        CamContext::CONTEXT_A,
    )
    .expect("Failed to set camera size");
    cam.set_output_format(
        CamSelect::SELECT_OUT1_OUT2,
        CamOutputFormat::OUTPUT_RGB_565,
        CamContext::CONTEXT_A,
    )
    .expect("Failed to set camera output format");

    cam.set_noise_filter(CamSelect::SELECT_OUT1_OUT2, true)
        .expect("Failed to enable noise filter");
    cam.set_auto_exposure(CamSelect::SELECT_OUT1_OUT2, true)
        .expect("Failed to enable auto exposure");
    cam.set_auto_white_balance(CamSelect::SELECT_OUT1_OUT2, true)
        .expect("Failed to enable auto white balance");

    cam.set_trimming(CamPort::PORT_CAM1, false)
        .expect("Failed to disable trimming for Cam Port 1");
    cam.set_trimming(CamPort::PORT_CAM2, false)
        .expect("Failed to disable trimming for Cam Port 2");

    let mut buf = vec![0u8; BUF_SIZE];

    gfx.flush_buffers();
    gfx.swap_buffers();
    gfx.wait_for_vblank();

    let mut held_r = false;

    println!("\nPress R to take a new picture");
    println!("Press Start to exit to Homebrew Launcher");

    gfx.top_screen.borrow_mut().set_3d_enabled(false);

    while apt.main_loop() {
        hid.scan_input();
        key_down = hid.keys_down();
        key_held = hid.keys_held();

        if key_down.contains(KeyPad::KEY_START) {
            break;
        }

        if key_held.contains(KeyPad::KEY_R) && !held_r {
            println!("Capturing new image");
            gfx.flush_buffers();
            gfx.swap_buffers();
            gfx.wait_for_vblank();
            held_r = true;
            take_picture(&mut cam, &mut buf);
        } else if !key_held.contains(KeyPad::KEY_R) {
            held_r = false;
        }

        write_picture_to_frame_buffer_rgb_565(
            gfx.top_screen.borrow_mut().get_raw_framebuffer(Side::Left),
            &mut buf,
            0,
            0,
            WIDTH,
            HEIGHT,
        );

        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}

fn take_picture(cam: &mut Cam, buf: &mut [u8]) {
    let buf_size = cam
        .get_max_bytes(WIDTH, HEIGHT)
        .expect("Failed to get max bytes");

    cam.set_transfer_bytes(CamPort::PORT_BOTH, buf_size, WIDTH, HEIGHT)
        .expect("Failed to set transfer bytes");

    cam.activate(CamSelect::SELECT_OUT1_OUT2)
        .expect("Failed to activate camera");

    cam.clear_buffer(CamPort::PORT_BOTH)
        .expect("Failed to clear buffer");
    cam.synchronize_vsync_timing(CamSelect::SELECT_OUT1, CamSelect::SELECT_OUT2)
        .expect("Failed to sync vsync timings");

    cam.start_capture(CamPort::PORT_BOTH)
        .expect("Failed to start capture");

    let receive_event = cam
        .set_receiving(buf, CamPort::PORT_CAM1, SCREEN_SIZE, buf_size as i16)
        .expect("Failed to set receiving");

    let receive_event2 = cam
        .set_receiving(
            &mut buf[SCREEN_SIZE as usize..],
            CamPort::PORT_CAM2,
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

    cam.play_shutter_sound(CamShutterSoundType::SHUTTER_SOUND_TYPE_NORMAL)
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

    cam.activate(CamSelect::SELECT_NONE)
        .expect("Failed to deactivate camera");
}

fn write_picture_to_frame_buffer_rgb_565(
    fb: RawFrameBuffer,
    img: &[u8],
    x: u16,
    y: u16,
    width: i16,
    height: i16,
) {
    let fb_8 = fb.ptr;
    let img_16 = img.as_ptr() as *const u16;
    let mut draw_x;
    let mut draw_y;
    for j in 0..height {
        for i in 0..width {
            draw_y = y as i16 + height - j;
            draw_x = x as i16 + i;
            let v = (draw_y as usize + draw_x as usize * height as usize) * 3;
            let data = unsafe { *img_16.add(j as usize * width as usize + i as usize) };
            let b = (((data >> 11) & 0x1F) << 3) as u8;
            let g = (((data >> 5) & 0x3F) << 2) as u8;
            let r = ((data & 0x1F) << 3) as u8;
            unsafe {
                *fb_8.add(v) = r;
                *fb_8.add(v + 1) = g;
                *fb_8.add(v + 2) = b;
            };
        }
    }
}
