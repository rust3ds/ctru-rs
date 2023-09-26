//! Application Manager service.
//!
//! As the name implies, the AM service manages installed applications. It can:
//! - Read the installed applications on the console and their information (depending on the install location).
//! - Install compatible applications to the console.
//!
//! TODO: [`ctru-rs`](crate) doesn't support installing or uninstalling titles yet.
#![doc(alias = "app")]
#![doc(alias = "manager")]

use std::marker::PhantomData;

use crate::error::ResultCode;
use crate::services::fs::FsMediaType;

/// General information about a specific title entry.
#[doc(alias = "AM_TitleEntry")]
pub struct Title<'a> {
    id: u64,
    mediatype: FsMediaType,
    size: u64,
    version: u16,
    _am: PhantomData<&'a Am>,
}

impl<'a> Title<'a> {
    /// Returns this title's ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns this title's unique product code.
    #[doc(alias = "AM_GetTitleProductCode")]
    pub fn product_code(&self) -> String {
        let mut buf: [u8; 16] = [0; 16];

        // This operation is safe as long as the title was correctly obtained via [`Am::title_list()`].
        unsafe {
            let _ =
                ctru_sys::AM_GetTitleProductCode(self.mediatype.into(), self.id, buf.as_mut_ptr());
        }

        String::from_utf8_lossy(&buf).to_string()
    }

    /// Returns the size of this title in bytes.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Returns the installed version of this title.
    pub fn version(&self) -> u16 {
        self.version
    }
}

/// Handle to the Application Manager service.
pub struct Am(());

impl Am {
    /// Initialize a new service handle.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::am::Am;
    ///
    /// let app_manager = Am::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "amInit")]
    pub fn new() -> crate::Result<Am> {
        unsafe {
            ResultCode(ctru_sys::amInit())?;
            Ok(Am(()))
        }
    }

    /// Returns the amount of titles currently installed in a specific install location.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::am::Am;
    /// use ctru::services::fs::FsMediaType;
    /// let app_manager = Am::new()?;
    ///
    /// // Number of titles installed on the Nand storage.
    /// let nand_count = app_manager.title_count(FsMediaType::Nand);
    ///
    /// // Number of apps installed on the SD card storage
    /// let sd_count = app_manager.title_count(FsMediaType::Sd);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "AM_GetTitleCount")]
    pub fn title_count(&self, mediatype: FsMediaType) -> crate::Result<u32> {
        unsafe {
            let mut count = 0;
            ResultCode(ctru_sys::AM_GetTitleCount(mediatype.into(), &mut count))?;
            Ok(count)
        }
    }

    /// Returns the list of titles installed in a specific install location.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::am::Am;
    /// use ctru::services::fs::FsMediaType;
    /// let app_manager = Am::new()?;
    ///
    /// // Number of apps installed on the SD card storage
    /// let sd_titles = app_manager.title_list(FsMediaType::Sd)?;
    ///
    /// // Unique product code identifier of the 5th installed title.
    /// let product_code = sd_titles[4].product_code();
    /// #
    /// # Ok(())
    /// # }
    /// ```
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

        let mut info: Vec<ctru_sys::AM_TitleEntry> = Vec::with_capacity(count as _);

        unsafe {
            ResultCode(ctru_sys::AM_GetTitleInfo(
                mediatype.into(),
                count,
                buf.as_mut_ptr(),
                info.as_mut_ptr() as _,
            ))?;

            info.set_len(count as _);
        };

        Ok(info
            .into_iter()
            .map(|title| Title {
                id: title.titleID,
                mediatype,
                size: title.size,
                version: title.version,
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
