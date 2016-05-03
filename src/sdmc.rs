use core::marker::PhantomData;

use libctru::sdmc::*;

pub struct Sdmc {
    pd: PhantomData<i32>
}

impl Sdmc {
    pub fn new() -> Result<Self, i32> {
        unsafe {
            let r = sdmcInit();
            if r < 0 {
                Err(r)
            } else {
                Ok(Sdmc { pd: PhantomData })
            }
        }
    }
}

impl Drop for Sdmc {
    fn drop(&mut self) {
        unsafe {
            sdmcExit()
        };
    }
}
