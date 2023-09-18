//! Read-Only Memory FileSystem service.
//!
//! This service lets the application access a virtual mounted device created using a folder included within the application bundle.
//! After mounting the RomFS file system, the included files and folders will be accessible exactly like any other file, just by using the drive prefix `romfs:/<file-path>`.
//!
//! # Usage
//!
//! This module only gets compiled if the configured RomFS directory is found and the `romfs`
//! feature is enabled.
//!
//! Configure the path in your project's `Cargo.toml` manifest (the default path is "romfs"). Paths are relative to the
//! `CARGO_MANIFEST_DIR` environment variable, which is the directory containing the manifest of
//! your package.
//!
//! ```toml
//! [package.metadata.cargo-3ds]
//! romfs_dir = "romfs"
//! ```
//!
//! Alternatively, you can include the RomFS archive manually when building with `3dsxtool`.
//!
//! # Notes
//!
//! `std::path` has problems when parsing file paths that include the `romfs:` prefix.
//! As such, it's suggested to use the paths directly or to do simple append operations to avoid unexpected behaviour.
//! Related [issue](https://github.com/rust-lang/rust/issues/52331).
#![doc(alias = "embed")]
#![doc(alias = "filesystem")]

use crate::error::ResultCode;
use std::ffi::CStr;
use std::sync::Mutex;

use crate::services::ServiceReference;

/// Handle to the RomFS service.
pub struct RomFS {
    _service_handler: ServiceReference,
}

static ROMFS_ACTIVE: Mutex<()> = Mutex::new(());

impl RomFS {
    /// Mount the bundled RomFS archive as a virtual drive.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::romfs::RomFS;
    ///
    /// let romfs = RomFS::new()?;
    ///
    /// // Remember to include the RomFS archive and to use your actual files!
    /// let contents = std::fs::read_to_string("romfs:/test-file.txt");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "romfsMountSelf")]
    pub fn new() -> crate::Result<Self> {
        let _service_handler = ServiceReference::new(
            &ROMFS_ACTIVE,
            || {
                let mount_name = CStr::from_bytes_with_nul(b"romfs\0").unwrap();
                ResultCode(unsafe { ctru_sys::romfsMountSelf(mount_name.as_ptr()) })?;
                Ok(())
            },
            || {
                let mount_name = CStr::from_bytes_with_nul(b"romfs\0").unwrap();
                let _ = unsafe { ctru_sys::romfsUnmount(mount_name.as_ptr()) };
            },
        )?;

        Ok(Self { _service_handler })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn romfs_counter() {
        let _romfs = RomFS::new().unwrap();
        let value = *ROMFS_ACTIVE.lock().unwrap();

        assert_eq!(value, 1);

        drop(_romfs);

        let value = *ROMFS_ACTIVE.lock().unwrap();

        assert_eq!(value, 0);
    }
}
