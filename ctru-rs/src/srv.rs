use libctru::srv::*;

use std::marker::PhantomData;

pub struct Srv {
    pd: PhantomData<i32>,
}

impl Srv {
    pub fn init() -> ::Result<Srv> {
        unsafe {
            let r = srvInit();
            if r < 0 {
                Err(::Error::from(r))
            } else {
                Ok(Srv { pd: PhantomData })
            }
        }
    }
}

impl Drop for Srv {
    fn drop(&mut self) {
        unsafe { srvExit() };
    }
}
