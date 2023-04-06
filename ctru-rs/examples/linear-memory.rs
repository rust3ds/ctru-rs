#![feature(allocator_api)]

use ctru::linear::LinearAllocator;
use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let _console = Console::new(gfx.top_screen.borrow_mut());

    let linear_space_before = LinearAllocator::free_space();

    // Normal `Box` on the heap
    let heap_box = Box::new(1492);
    // `Box` living on the linear memory sector
    let linear_box: Box<i32, LinearAllocator> = Box::new_in(2022, LinearAllocator);

    println!("This value is from the heap: {heap_box}");
    println!("This value is from the LINEAR memory: {linear_box}");

    println!("\nLINEAR space free before allocation: {linear_space_before}");
    println!(
        "LINEAR space free after allocation: {}",
        LinearAllocator::free_space()
    );

    println!("\x1b[29;16HPress Start to exit");

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
