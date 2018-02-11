use libctru::{socInit, socExit};

use libc::{memalign, free};

pub struct Soc {
    soc_mem: *mut u32,
}

impl Soc {
    pub fn init() -> ::Result<Soc> {
        const SOC_MEM_SIZE: usize = 0x100000;

        unsafe {
            let soc_mem = memalign(0x1000, SOC_MEM_SIZE) as *mut u32;

            let r = socInit(soc_mem, SOC_MEM_SIZE as u32);
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
