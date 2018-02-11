use libctru::{socInit, socExit};

use libc::{memalign, free};

/// Soc service. Initializing this service will enable the use of network sockets and utilities
/// such as those found in `std::net`. The service will be closed when this struct is is dropped.
pub struct Soc {
    soc_mem: *mut u32,
}

impl Soc {
    /// Initialize the Soc service with a default buffer size of 0x100000 bytes
    ///
    /// # Errors
    ///
    /// This function will return an error if the `Soc` service is already initialized
    pub fn init() -> ::Result<Soc> {
        Soc::init_with_buffer_size(0x100000)
    }

    /// Initialize the Soc service with a custom buffer size in bytes. The size should be
    /// 0x100000 bytes or greater.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `Soc` service is already initialized
    pub fn init_with_buffer_size(num_bytes: usize) -> ::Result<Soc> {
        unsafe {
            let soc_mem = memalign(0x1000, num_bytes) as *mut u32;

            let r = socInit(soc_mem, num_bytes as u32);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(Soc { soc_mem, })
            }
        }
    }
}

impl Drop for Soc {
    fn drop(&mut self) {
        unsafe {
            socExit();
            free(self.soc_mem as *mut _);
        }
    }
}
