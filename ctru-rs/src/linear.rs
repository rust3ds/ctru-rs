//! Linear memory allocator
//!
//! Linear memory is a sector of the 3DS' RAM that binds virtual addresses exactly to the physical address.
//! As such, it is used for fast and safe memory sharing between services (and is especially needed for GPU and DSP).
//!
//! Resources:<br>
//! <https://github.com/devkitPro/libctru/blob/master/libctru/source/allocator/linear.cpp><br>
//! <https://www.3dbrew.org/wiki/Memory_layout>

use std::alloc::Allocator;

// Implementing an `std::alloc::Allocator` type is the best way to handle this case, since it gives
// us full control over the normal `std` implementations (like `Box`). The only issue is that this is another unstable feature to add.
// Sadly the linear memory allocator included in `libctru` doesn't implement `linearRealloc` at the time of these additions,
// but the default fallback of the `std` will take care of that for us.

/// [`std::alloc::Allocator`] struct for LINEAR memory
pub struct LinearAllocator;

impl LinearAllocator {
    /// Returns the amount of free space left in the LINEAR sector
    pub fn free_space() -> u32 {
        unsafe { ctru_sys::linearSpaceFree() }
    }
}

unsafe impl Allocator for LinearAllocator {
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
        let pointer = unsafe { ctru_sys::linearAlloc(layout.size() as u32) };
        let slice: &mut [u8] =
            unsafe { std::slice::from_raw_parts_mut(pointer as *mut u8, layout.size()) };

        std::ptr::NonNull::new(slice).ok_or(std::alloc::AllocError)
    }

    unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
        ctru_sys::linearFree(ptr.as_ptr() as *mut _);
    }
}
