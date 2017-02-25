use libctru::srv::*;

pub struct Srv(());

impl Srv {
    pub fn init() -> ::Result<Srv> {
        unsafe {
            let r = srvInit();
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
        unsafe { srvExit() };
    }
}
