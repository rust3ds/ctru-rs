//! LINEAR memory example.
//!
//! This example showcases simple allocation on the LINEAR memory sector.
//! Using LINEAR memory is required when sending data to the GPU or DSP processor.

// You will need to activate this unstable feature to use custom allocators.
#![feature(allocator_api)]

use ctru::linear::LinearAllocator;
use ctru::prelude::*;

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let _console = Console::new(gfx.top_screen.borrow_mut());

    // The `LinearAllocator` is always available for use.
    // Luckily, we can always read how much memory is available to be allocated on it.
    let linear_space_before = LinearAllocator::free_space();

    // Normal `Box` on the heap.
    let heap_box = Box::new(1492);
    // `Box` living on the LINEAR memory.
    let linear_box: Box<i32, LinearAllocator> = Box::new_in(2022, LinearAllocator);

    println!("This value is from the heap: {heap_box}");
    println!("This value is from the LINEAR memory: {linear_box}");

    println!("\nLINEAR space free before allocation: {linear_space_before}");
    println!(
        "LINEAR space free after allocation: {}",
        LinearAllocator::free_space()
    );

    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        gfx.wait_for_vblank();
    }
}
