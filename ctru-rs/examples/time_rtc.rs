use std::ptr;

use ctru::console::Console;
use ctru::gfx::{Gfx, Screen};
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const WEEKDAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

const MONTH_TO_WEEKDAY_TABLE: [i32; 12] = [
    0 % 7,   //january    31
    31 % 7,  //february   28+1(leap year)
    59 % 7,  //march      31
    90 % 7,  //april      30
    120 % 7, //may        31
    151 % 7, //june       30
    181 % 7, //july       31
    212 % 7, //august     31
    243 % 7, //september  30
    273 % 7, //october    31
    304 % 7, //november   30
    334 % 7, //december   3
];

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}

/// <http://en.wikipedia.org/wiki/Calculating_the_day_of_the_week>
fn get_day_of_week(mut day: i32, month: usize, mut year: i32) -> usize {
    day += 2 * (3 - ((year / 100) % 4));
    year %= 100;
    day += year + (year / 4);
    day += MONTH_TO_WEEKDAY_TABLE[month]
        - if is_leap_year(year) && month <= 1 {
            1
        } else {
            0
        };
    (day % 7).try_into().expect("day is not valid usize")
}

fn main() {
    ctru::init();

    let gfx = Gfx::default();
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");

    let _console = Console::init(&gfx, Screen::Top);

    print!("\x1b[30;16HPress Start to exit.");

    // Main loop
    while apt.main_loop() {
        // Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }

        // Get the current time. ctru_sys bindings should be used rather than
        // plain libc ones, for reasons I don't understand yet...
        let unix_time: ctru_sys::time_t = unsafe { ctru_sys::time(std::ptr::null_mut()) };
        let time = unsafe { ptr::read(ctru_sys::gmtime(&unix_time as *const _)) };

        let hours = time.tm_hour;
        let minutes = time.tm_min;
        let seconds = time.tm_sec;
        let day = time.tm_mday;
        let month = time.tm_mon;
        let year = time.tm_year + 1900;

        let month: usize = month.try_into().expect("month is not valid usize");

        println!("\x1b[1;1H{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds);
        print!(
            "{} {} {} {}",
            WEEKDAYS
                .get(get_day_of_week(day, month, year))
                .expect("invalid weekday"),
            MONTHS[month],
            day,
            year
        );

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
