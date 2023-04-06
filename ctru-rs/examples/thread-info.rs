//! Prints some interesting system info about the main and (spawned) system threads.

#![feature(horizon_thread_ext)]

use ctru::prelude::*;

use std::os::horizon::thread::BuilderExt;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let _console = Console::new(gfx.top_screen.borrow_mut());

    // Give ourselves up to 30% of the system core's time
    apt.set_app_cpu_time_limit(30)
        .expect("Failed to enable system core");

    print_processor("main thread");
    print_thread_id("main thread");
    print_priority("main thread");
    print_affinity_mask("main thread");

    std::thread::Builder::new()
        .processor_id(1)
        .spawn(|| {
            print_processor("sys thread");
            print_thread_id("sys thread");
            print_priority("sys thread");
            print_affinity_mask("sys thread");
        })
        .unwrap()
        .join()
        .unwrap();

    println!("sys thread exited");
    println!("\nPress Start to exit");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
        
        gfx.wait_for_vblank();
    }
}

fn print_processor(thread_name: &str) {
    println!(
        "{thread_name} processor: {}",
        std::os::horizon::thread::current_processor()
    );
}

fn print_priority(thread_name: &str) {
    println!(
        "{thread_name} priority: {:#x}",
        std::os::horizon::thread::current_priority()
    );
}

fn print_affinity_mask(thread_name: &str) {
    let mut affinity_mask = [0u8; 1];
    let result = unsafe {
        ctru_sys::svcGetThreadAffinityMask(
            affinity_mask.as_mut_ptr(),
            ctru_sys::CUR_THREAD_HANDLE,
            2,
        )
    };

    if ctru_sys::R_FAILED(result) {
        println!("Error getting affinity mask:");
        println!("result level = {}", ctru_sys::R_LEVEL(result));
        println!("result summary = {}", ctru_sys::R_SUMMARY(result));
        println!("result description = {}", ctru_sys::R_DESCRIPTION(result));
        return;
    }

    let affinity_value = affinity_mask[0];
    println!("{thread_name} affinity: {affinity_value:#x?}");
}

fn print_thread_id(thread_name: &str) {
    let mut thread_id = 0;
    let result = unsafe { ctru_sys::svcGetThreadId(&mut thread_id, ctru_sys::CUR_THREAD_HANDLE) };

    if ctru_sys::R_FAILED(result) {
        println!("Error getting thread ID:");
        println!("result level = {}", ctru_sys::R_LEVEL(result));
        println!("result summary = {}", ctru_sys::R_SUMMARY(result));
        println!("result description = {}", ctru_sys::R_DESCRIPTION(result));
        return;
    }

    println!("{thread_name} ID: {thread_id:#x?}")
}
