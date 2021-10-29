use crate::error::Result;
use crate::raw;

pub struct Sdmc(());

impl Sdmc {
    pub fn init() -> Result<Sdmc> {
        unsafe {
            let r = raw::archiveMountSdmc();
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
        unsafe { raw::archiveUnmountAll() };
    }
}
