use crate::error::Result;
use crate::raw;

pub struct Srv(());

impl Srv {
    pub fn init() -> Result<Srv> {
        unsafe {
            let r = raw::srvInit();
            if r < 0 {
                Err(r.into())
            } else {
                Ok(Srv(()))
            }
        }
    }
}

impl Drop for Srv {
    fn drop(&mut self) {
        unsafe { raw::srvExit() };
    }
}
