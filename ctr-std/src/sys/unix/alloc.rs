// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use alloc::allocator::{Alloc,Layout,AllocErr,Excess,CannotReallocInPlace};
use alloc::heap;

/// Heap allocator that delegates to the default liballoc heap allocator.
/// Its purpose is to override methods while still using the standard alloc api.
#[derive(Copy, Clone, Default, Debug)]
pub struct Heap;

unsafe impl Alloc for Heap {
    #[inline]
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        heap::Heap.alloc(layout)
    }

    // A nicer handler for out-of-memory situations than the default one. This
    // one prints a message to stderr before aborting. It is critical that this
    // code does not allocate any memory since we are in an OOM situation. Any
    // errors are ignored while printing since there's nothing we can do about
    // them and we are about to exit anyways.
    #[inline]
    fn oom(&mut self, err: AllocErr) -> ! {
        use intrinsics;
        use libc::{self, STDERR_FILENO, c_void};

        let msg = err.description();
        unsafe {
            libc::write(STDERR_FILENO, msg.as_ptr() as *const c_void, msg.len());
            libc::write(STDERR_FILENO, "\n".as_ptr() as *const c_void, 1);
            intrinsics::abort();
        }
    }

    #[inline]
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        heap::Heap.dealloc(ptr, layout)
    }

    #[inline]
    fn usable_size(&self, layout: &Layout) -> (usize, usize) {
        heap::Heap.usable_size(layout)
    }

    #[inline]
    unsafe fn realloc(&mut self,
                      ptr: *mut u8,
                      layout: Layout,
                      new_layout: Layout)
                      -> Result<*mut u8, AllocErr>
    {
        heap::Heap.realloc(ptr, layout, new_layout)
    }

    #[inline]
    unsafe fn alloc_zeroed(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        heap::Heap.alloc_zeroed(layout)
    }

    #[inline]
    unsafe fn alloc_excess(&mut self, layout: Layout) -> Result<Excess, AllocErr> {
        heap::Heap.alloc_excess(layout)
    }

    #[inline]
    unsafe fn realloc_excess(&mut self,
                             ptr: *mut u8,
                             layout: Layout,
                             new_layout: Layout) -> Result<Excess, AllocErr>
    {
        heap::Heap.realloc_excess(ptr, layout, new_layout)
    }

    #[inline]
    unsafe fn grow_in_place(&mut self,
                            ptr: *mut u8,
                            layout: Layout,
                            new_layout: Layout)
                            -> Result<(), CannotReallocInPlace>
    {
        heap::Heap.grow_in_place(ptr, layout, new_layout)
    }

    #[inline]
    unsafe fn shrink_in_place(&mut self,
                              ptr: *mut u8,
                              layout: Layout,
                              new_layout: Layout) -> Result<(), CannotReallocInPlace>
    {
        heap::Heap.shrink_in_place(ptr, layout, new_layout)
    }
}
