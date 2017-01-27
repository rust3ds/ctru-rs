use std::marker::PhantomData;

use libctru::services::apt;

pub struct Apt {
    pd: PhantomData<i32>
}

impl Apt {
    pub fn init() -> ::Result<Apt> {
        unsafe {
            let r = apt::aptInit();
            if r < 0 {
                Err(::Error::from(r))
            } else {
                Ok(Apt { pd: PhantomData })
            }
        }
    }

    pub fn main_loop(&self) -> bool {
        unsafe {
            match apt::aptMainLoop() {
                1 => true,
                0 => false,
                _ => unreachable!(),
            }
        }
    }
}

impl Drop for Apt {
    fn drop(&mut self) {
        unsafe { apt::aptExit() };
    }
}
