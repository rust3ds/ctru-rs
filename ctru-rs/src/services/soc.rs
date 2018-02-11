use libctru::{socInit, socExit};

use libc::{memalign, free};

pub struct Soc {
    soc_mem: *mut u32,
}

impl Soc {
    pub fn init() -> ::Result<Soc> {
        Soc::init_with_buffer_size(0x100000)
    }

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
