use crate::error::ResultCode;
use crate::services::fs::FsMediaType;

pub struct Am(());

impl Am {
    pub fn init() -> crate::Result<Am> {
        unsafe {
            ResultCode(ctru_sys::amInit())?;
            Ok(Am(()))
        }
    }

    pub fn get_title_count(&self, mediatype: FsMediaType) -> crate::Result<u32> {
        unsafe {
            let mut count = 0;
            ResultCode(ctru_sys::AM_GetTitleCount(mediatype as u32, &mut count))?;
            Ok(count)
        }
    }

    pub fn get_title_list(&self, mediatype: FsMediaType) -> crate::Result<Vec<u64>> {
        unsafe {
            let count = self.get_title_count(mediatype)?;
            let mut buf = Vec::with_capacity(count as usize);
            let mut read_amount = 0;
            ResultCode(ctru_sys::AM_GetTitleList(
                &mut read_amount,
                mediatype as u32,
                count,
                buf.as_mut_ptr(),
            ))?;
            buf.set_len(read_amount as usize);
            Ok(buf)
        }
    }
}

impl Drop for Am {
    fn drop(&mut self) {
        unsafe { ctru_sys::amExit() };
    }
}
