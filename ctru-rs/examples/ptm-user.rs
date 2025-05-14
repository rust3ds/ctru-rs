//! Power-Time Services example.
//!
//! This example shows off common functionality found in the PTM family of system services, like pedometer steps count, battery state reading
//! and some light shows with the notification LED.

use ctru::prelude::*;
use ctru::services::ptm::user::{BatteryLevel, PTMUser};

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let _top_screen = Console::new(gfx.top_screen.borrow_mut());

    let ptm_user = PTMUser::new().unwrap();

    // Let's gather some simple data with PTM:User
    let battery_level = ptm_user.battery_level().unwrap();
    let charging = ptm_user.is_charging().unwrap();
    let steps = ptm_user.step_count().unwrap();

    if battery_level >= BatteryLevel::Low {
        println!("The battery level is sufficient to play a while.")
    } else {
        println!("The battery level is low.")
    }

    if charging {
        println!("The battery is currently charging.")
    } else {
        println!("The battery is discharging.")
    }

    println!("You accumulated a total of {steps} steps.");

    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
    }
}
