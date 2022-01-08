pub struct Srv(());

impl Srv {
    pub fn init() -> crate::Result<Srv> {
        unsafe {
            let r = ctru_sys::srvInit();
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
        unsafe { ctru_sys::srvExit() };
    }
}
