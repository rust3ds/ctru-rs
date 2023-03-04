use crate::error::ResultCode;
use crate::services::fs::FsMediaType;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct TitleInfo(ctru_sys::AM_TitleEntry);
impl TitleInfo {
    pub fn id(&self) -> u64 {
        self.0.titleID
    }
    pub fn size_bytes(&self) -> u64 {
        self.0.size
    }
    pub fn version(&self) -> u16 {
        self.0.version
    }
}

pub struct Title<'a> {
    id: u64,
    mediatype: FsMediaType,
    _am: PhantomData<&'a Am>,
}

impl<'a> Title<'a> {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn get_product_code(&self) -> crate::Result<String> {
        let mut buf: [u8; 16] = [0; 16];

        unsafe {
            ResultCode(ctru_sys::AM_GetTitleProductCode(
                self.mediatype as u32,
                self.id,
                buf.as_mut_ptr(),
            ))?;
        }
        Ok(String::from_utf8_lossy(&buf).to_string())
    }

    pub fn get_title_info(&self) -> crate::Result<TitleInfo> {
        let mut info = MaybeUninit::zeroed();

        unsafe {
            ResultCode(ctru_sys::AM_GetTitleInfo(
                self.mediatype as u32,
                1,
                &mut self.id.clone(),
                info.as_mut_ptr() as _,
            ))?;

            Ok(info.assume_init())
        }
    }
}

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

    pub fn get_title_list(&self, mediatype: FsMediaType) -> crate::Result<Vec<Title>> {
        let count = self.get_title_count(mediatype)?;
        let mut buf = vec![0; count as usize];
        let mut read_amount = 0;
        unsafe {
            ResultCode(ctru_sys::AM_GetTitleList(
                &mut read_amount,
                mediatype as u32,
                count,
                buf.as_mut_ptr(),
            ))?;
        }
        Ok(buf
            .into_iter()
            .map(|id| Title {
                id,
                mediatype,
                _am: PhantomData,
            })
            .collect())
    }
}

impl Drop for Am {
    fn drop(&mut self) {
        unsafe { ctru_sys::amExit() };
    }
}
