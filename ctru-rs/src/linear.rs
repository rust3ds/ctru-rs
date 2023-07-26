//! LINEAR memory allocator.
//!
//! LINEAR memory is a sector of the 3DS' RAM that binds virtual addresses exactly to the physical address.
//! As such, it is used for fast and safe memory sharing between hardware processors (such as the GPU and the DSP).
//!
//! # Additional Resources
//!
//! - <https://github.com/devkitPro/libctru/blob/master/libctru/source/allocator/linear.cpp><br>
//! - <https://www.3dbrew.org/wiki/Memory_layout>

use std::alloc::{AllocError, Allocator, Layout};
use std::ptr::NonNull;

// Implementing an `std::alloc::Allocator` type is the best way to handle this case, since it gives
// us full control over the normal `std` implementations (like `Box`). The only issue is that this is another unstable feature to add.
// Sadly the linear memory allocator included in `libctru` doesn't implement `linearRealloc` at the time of these additions,
// but the default fallback of the `std` will take care of that for us.

/// [`Allocator`](std::alloc::Allocator) struct for LINEAR memory.
///
/// To use this struct the main crate must activate the `allocator_api` unstable feature.
#[derive(Copy, Clone, Default, Debug)]
pub struct LinearAllocator;

impl LinearAllocator {
    /// Returns the amount of free space left in the LINEAR memory sector.
    #[doc(alias = "linearSpaceFree")]
    pub fn free_space() -> u32 {
        unsafe { ctru_sys::linearSpaceFree() }
    }
}

unsafe impl Allocator for LinearAllocator {
    #[doc(alias = "linearAlloc", alias = "linearMemAlign")]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let pointer = unsafe { ctru_sys::linearMemAlign(layout.size(), layout.align()) };

        NonNull::new(pointer.cast())
            .map(|ptr| NonNull::slice_from_raw_parts(ptr, layout.size()))
            .ok_or(AllocError)
    }

    #[doc(alias = "linearFree")]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        ctru_sys::linearFree(ptr.as_ptr().cast());
    }
}
