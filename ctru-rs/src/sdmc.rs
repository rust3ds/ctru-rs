use std::marker::PhantomData;

use libctru::sdmc::*;

pub struct Sdmc {
    pd: PhantomData<i32>,
}

impl Sdmc {
    pub fn init() -> ::Result<Sdmc> {
        unsafe {
            let r = sdmcInit();
            if r < 0 {
                Err(::Error::from(r))
            } else {
                Ok(Sdmc { pd: PhantomData })
            }
        }
    }
}

impl Drop for Sdmc {
    fn drop(&mut self) {
        unsafe { sdmcExit() };
    }
}
