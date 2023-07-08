//! Application Manager service.
//!
//! As the name implies, the AM service manages installed applications. It can:
//! - Read the installed applications on the console and their information (depending on the install location).
//! - Install compatible applications to the console.
//!
//! `ctru` doesn't support installing titles (yet).

use crate::error::ResultCode;
use crate::services::fs::FsMediaType;
use std::cell::OnceCell;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

/// Struct holding general information about a specific title.
#[doc(alias = "AM_TitleEntry")]
pub struct Title<'a> {
    id: u64,
    mediatype: FsMediaType,
    entry: OnceCell<ctru_sys::AM_TitleEntry>,
    _am: PhantomData<&'a Am>,
}

impl<'a> Title<'a> {
    /// Returns this title's ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns this title's unique product code.
    #[doc(alias = "AM_GetTitleProductCode")]
    pub fn product_code(&self) -> crate::Result<String> {
        let mut buf: [u8; 16] = [0; 16];

        unsafe {
            ResultCode(ctru_sys::AM_GetTitleProductCode(
                self.mediatype.into(),
                self.id,
                buf.as_mut_ptr(),
            ))?;
        }
        Ok(String::from_utf8_lossy(&buf).to_string())
    }

    /// Retrieves additional information on the title.
    #[doc(alias = "AM_GetTitleInfo")]
    fn title_info(&self) -> crate::Result<ctru_sys::AM_TitleEntry> {
        let mut info = MaybeUninit::zeroed();

        unsafe {
            ResultCode(ctru_sys::AM_GetTitleInfo(
                self.mediatype.into(),
                1,
                &mut self.id.clone(),
                info.as_mut_ptr() as _,
            ))?;

            Ok(info.assume_init())
        }
    }

    /// Returns the size of this title in bytes.
    pub fn size(&self) -> crate::Result<u64> {
        // Get the internal entry, or fill it if empty.
        let entry = self
            .entry
            .get_or_try_init(|| -> crate::Result<ctru_sys::AM_TitleEntry> { self.title_info() })?;

        Ok(entry.size)
    }

    /// Returns the installed version of this title.
    pub fn version(&self) -> crate::Result<u16> {
        // Get the internal entry, or fill it if empty.
        let entry = self
            .entry
            .get_or_try_init(|| -> crate::Result<ctru_sys::AM_TitleEntry> { self.title_info() })?;

        Ok(entry.version)
    }
}

/// Handle to the Application Manager service.
pub struct Am(());

impl Am {
    /// Initialize a new handle.
    #[doc(alias = "amInit")]
    pub fn new() -> crate::Result<Am> {
        unsafe {
            ResultCode(ctru_sys::amInit())?;
            Ok(Am(()))
        }
    }

    /// Returns the amount of titles currently installed in a specific install location.
    #[doc(alias = "AM_GetTitleCount")]
    pub fn title_count(&self, mediatype: FsMediaType) -> crate::Result<u32> {
        unsafe {
            let mut count = 0;
            ResultCode(ctru_sys::AM_GetTitleCount(mediatype.into(), &mut count))?;
            Ok(count)
        }
    }

    /// Returns the list of titles installed in a specific install location.
    #[doc(alias = "AM_GetTitleList")]
    pub fn title_list(&self, mediatype: FsMediaType) -> crate::Result<Vec<Title>> {
        let count = self.title_count(mediatype)?;
        let mut buf = vec![0; count as usize];
        let mut read_amount = 0;
        unsafe {
            ResultCode(ctru_sys::AM_GetTitleList(
                &mut read_amount,
                mediatype.into(),
                count,
                buf.as_mut_ptr(),
            ))?;
        }
        Ok(buf
            .into_iter()
            .map(|id| Title {
                id,
                mediatype,
                entry: OnceCell::new(),
                _am: PhantomData,
            })
            .collect())
    }
}

impl Drop for Am {
    #[doc(alias = "amExit")]
    fn drop(&mut self) {
        unsafe { ctru_sys::amExit() };
    }
}
