//! LINEAR memory allocator.
//!
//! LINEAR memory is a sector of the 3DS' RAM that binds virtual addresses exactly to the physical address.
//! As such, it is used for fast and safe memory sharing between different hardware components (such as the GPU and the DSP processor).
//!
//! # Additional Resources
//!
//! - <https://github.com/devkitPro/libctru/blob/master/libctru/source/allocator/linear.cpp>
//! - <https://www.3dbrew.org/wiki/Memory_layout>

use std::alloc::{AllocError, Allocator, Layout};
use std::ptr::NonNull;
use std::rc::{self, Rc};
use std::sync::{self, Arc};

// Implementing an `std::alloc::Allocator` type is the best way to handle this case, since it gives
// us full control over the normal `std` implementations (like `Box`). The only issue is that this is another unstable feature to add.
// Sadly the linear memory allocator included in `libctru` doesn't implement `linearRealloc` at the time of these additions,
// but the default fallback of the `std` will take care of that for us.

/// [`Allocator`] struct for LINEAR memory.
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
        unsafe {
            ctru_sys::linearFree(ptr.as_ptr().cast());
        }
    }
}

/// Trait indicating a type has been allocated using [`LinearAllocator`].
/// This can be used to enforce that a given slice was allocated in LINEAR memory.
///
/// # Safety
///
/// Implementing this trait is a promise that the backing storage was allocated with
/// [`LinearAllocator`]. If this is not the case, attempting to use the
/// data with a `LinearAllocation` bound may result in undefined behavior.
#[diagnostic::on_unimplemented(
    message = "{Self} is not allocated with `ctru::linear::LinearAllocator`"
)]
pub unsafe trait LinearAllocation {}

unsafe impl<T> LinearAllocation for Vec<T, LinearAllocator> {}
unsafe impl<T: ?Sized> LinearAllocation for Rc<T, LinearAllocator> {}
unsafe impl<T: ?Sized> LinearAllocation for rc::Weak<T, LinearAllocator> {}
unsafe impl<T: ?Sized> LinearAllocation for Arc<T, LinearAllocator> {}
unsafe impl<T: ?Sized> LinearAllocation for sync::Weak<T, LinearAllocator> {}
unsafe impl<T: ?Sized> LinearAllocation for Box<T, LinearAllocator> {}

// We could also impl for various std::collections types, but it seems unlikely
// those would ever be used for this purpose in practice, since most of the type
// we're dereferencing to a &[T]. The workaround would just be to convert to a Vec/Box.
