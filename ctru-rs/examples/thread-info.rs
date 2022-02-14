//! Prints some interesting system info about the main and (spawned) system threads.

#![feature(horizon_thread_ext)]

use ctru::console::Console;
use ctru::gfx::Gfx;
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};
use std::os::horizon::thread::ThreadBuilderExt;

fn main() {
    ctru::init();
    let gfx = Gfx::default();
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    // Give ourselves up to 30% of the system core's time
    apt.set_app_cpu_time_limit(30)
        .expect("Failed to enable system core");

    print_processor("main thread");
    print_thread_id("main thread");
    print_priority("main thread");
    print_affinity_mask("main thread");

    std::thread::Builder::new()
        .ideal_processor(1)
        .spawn(|| {
            print_processor("sys thread");
            print_thread_id("sys thread");
            print_priority("sys thread");
            print_affinity_mask("sys thread");
            print_thread_list();
        })
        .unwrap()
        .join()
        .unwrap();

    println!("sys thread exited");
    print_thread_list();

    println!("\nPress Start to exit");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }

        gfx.flush_buffers();
        gfx.swap_buffers();
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

fn print_thread_list() {
    let mut thread_ids = [0; 100];
    let mut thread_ids_count = 0;
    let result = unsafe {
        ctru_sys::svcGetThreadList(
            &mut thread_ids_count,
            thread_ids.as_mut_ptr(),
            thread_ids.len() as i32,
            ctru_sys::CUR_PROCESS_HANDLE,
        )
    };

    if ctru_sys::R_FAILED(result) {
        println!("Error getting thread list:");
        println!("result level = {}", ctru_sys::R_LEVEL(result));
        println!("result summary = {}", ctru_sys::R_SUMMARY(result));
        println!("result description = {}", ctru_sys::R_DESCRIPTION(result));
        return;
    }

    println!("Thread list:");
    for thread_id in thread_ids.into_iter().take(thread_ids_count as usize) {
        println!("  {thread_id:#x}");
    }
}
