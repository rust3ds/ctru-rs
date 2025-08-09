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
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
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
                ResultCode(unsafe { ctru_sys::romfsMountSelf(c"romfs".as_ptr()) })?;
                Ok(())
            },
            || {
                let _ = unsafe { ctru_sys::romfsUnmount(c"romfs".as_ptr()) };
            },
        )?;

        Ok(Self { _service_handler })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: this test only passes when run with a .3dsx, which for now requires separate build
    // and run steps so the 3dsx is built before the runner looks for the executable
    #[test]
    #[should_panic]
    fn romfs_lock() {
        let romfs = RomFS::new().unwrap();

        drop(ROMFS_ACTIVE.try_lock().unwrap());

        drop(romfs);
    }
}
