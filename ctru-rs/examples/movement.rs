//! Movement example.
//!
//! Simple application to showcase the use of the accelerometer and gyroscope.

use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let _console = Console::new(gfx.top_screen.borrow_mut());

    println!("Move the console around!");
    println!("\x1b[29;16HPress Start to exit");

    // Activate the accelerometer and the gyroscope.
    // Because of the complex nature of the movement sensors, they aren't activated by default with the `Hid` service.
    // However, they can simply be turned on and off whenever necessary.
    hid.set_accelerometer(true)
        .expect("Couldn't activate accelerometer");
    hid.set_gyroscope(true)
        .expect("Couldn't activate gyroscope");

    while apt.main_loop() {
        // Scan all the controller inputs.
        // Accelerometer and gyroscope require this step to update the readings.
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        // Be careful: reading without activating the sensors (as done before this loop) will result in a panic.
        println!(
            "\x1b[3;0HAcceleration: {:?}              ",
            Into::<(i16, i16, i16)>::into(
                hid.accelerometer_vector()
                    .expect("could not retrieve acceleration vector")
            )
        );
        println!(
            "\x1b[4;0HGyroscope angular rate: {:?}              ",
            Into::<(i16, i16, i16)>::into(
                hid.gyroscope_rate()
                    .expect("could not retrieve angular rate")
            )
        );

        gfx.wait_for_vblank();
    }
}
