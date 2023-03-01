use crate::error::ResultCode;
use crate::services::fs::FsMediaType;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct TitleInfo {
    id: u64,
    size: u64,
    version: u16,
    pad: u16,
    type_: u32,
}

// Make sure TitleInfo is correct size
const _TITLEINFO_SIZE_CHECK: [u8; 0x18] = [0; std::mem::size_of::<TitleInfo>()];

impl TitleInfo {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn size_bytes(&self) -> u64 {
        self.size
    }
    pub fn version(&self) -> u16 {
        self.version
    }
    pub fn type_(&self) -> u32 {
        self.type_
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
        let mut buf = Vec::with_capacity(count as usize);
        let mut read_amount = 0;
        unsafe {
            ResultCode(ctru_sys::AM_GetTitleList(
                &mut read_amount,
                mediatype as u32,
                count,
                buf.as_mut_ptr(),
            ))?;

            buf.set_len(read_amount as usize);
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
