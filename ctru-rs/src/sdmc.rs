pub struct Sdmc(());

impl Sdmc {
    pub fn init() -> crate::Result<Sdmc> {
        unsafe {
            let r = ctru_sys::archiveMountSdmc();
            if r < 0 {
                Err(r.into())
            } else {
                Ok(Sdmc(()))
            }
        }
    }
}

impl Drop for Sdmc {
    fn drop(&mut self) {
        unsafe { ctru_sys::archiveUnmountAll() };
    }
}
