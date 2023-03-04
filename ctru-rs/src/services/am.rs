use crate::error::ResultCode;
use crate::services::fs::Fs;
use crate::services::fs::FsMediaType;
use crate::smdh::Smdh;
use std::marker::PhantomData;
use std::mem::size_of;
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

#[derive(Copy, Clone, Debug)]
pub struct Title<'a> {
    id: u64,
    mediatype: FsMediaType,
    _am: PhantomData<&'a Am>,
}

impl<'a> Title<'a> {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn low_u32(&self) -> u32 {
        (self.id & 0x0000_0000_ffff_ffff) as u32
    }

    pub fn high_u32(&self) -> u32 {
        ((self.id & 0xffff_ffff_0000_0000) >> 32) as u32
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

    pub fn get_smdh(&self) -> crate::Result<Smdh> {
        // i have no idea how to make this look better
        let archive_path_data: [u32; 4] =
            [self.low_u32(), self.high_u32(), self.mediatype as u32, 0x0];
        let smdh_path_data: [u32; 5] = [0x0, 0x0, 0x2, u32::from_le_bytes(*b"icon"), 0x0];

        let archive_path = ctru_sys::FS_Path {
            type_: ctru_sys::PATH_BINARY,
            size: size_of::<[u32; 4]>() as u32,
            data: archive_path_data.as_ptr() as *const libc::c_void,
        };
        let smdh_path = ctru_sys::FS_Path {
            type_: ctru_sys::PATH_BINARY,
            size: size_of::<[u32; 5]>() as u32,
            data: smdh_path_data.as_ptr() as *const libc::c_void,
        };

        let _fs = Fs::init();
        let mut smdh_handle = 0;

        let smdh: Smdh = unsafe {
            let mut ret = MaybeUninit::zeroed();
            ResultCode(ctru_sys::FSUSER_OpenFileDirectly(
                &mut smdh_handle,
                ctru_sys::ARCHIVE_SAVEDATA_AND_CONTENT,
                archive_path,
                smdh_path,
                ctru_sys::FS_OPEN_READ,
                0x0,
            ))?;

            ResultCode(ctru_sys::FSFILE_Read(
                smdh_handle,
                std::ptr::null_mut(),
                0x0,
                ret.as_mut_ptr() as *mut libc::c_void,
                size_of::<Smdh>() as u32,
            ))?;

            ResultCode(ctru_sys::FSFILE_Close(smdh_handle))?;

            ret.assume_init()
        };

        assert_eq!(&smdh.magic(), b"SMDH");

        Ok(smdh)
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
