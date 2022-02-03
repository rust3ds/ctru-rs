#[non_exhaustive]
pub struct Ps;

impl Ps {
    pub fn init() -> crate::Result<Self> {
        let r = unsafe { ctru_sys::psInit() };
        if r < 0 {
            Err(r.into())
        } else {
            Ok(Self)
        }
    }
}

impl Drop for Ps {
    fn drop(&mut self) {
        unsafe {
            ctru_sys::psExit();
        }
    }
}
