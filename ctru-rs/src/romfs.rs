//! This module only gets compiled if the configured RomFS directory is found and the `romfs`
//! feature is enabled.
//!
//! Configure the path in Cargo.toml (the default path is "romfs"). Paths are relative to the
//! `CARGO_MANIFEST_DIR` environment variable, which is the directory containing the manifest of
//! your package.
//!
//! ```toml
//! [package.metadata.cargo-3ds]
//! romfs_dir = "romfs"
//! ```

use std::ffi::CStr;

#[non_exhaustive]
pub struct RomFS;

impl RomFS {
    pub fn new() -> crate::Result<Self> {
        let mount_name = CStr::from_bytes_with_nul(b"romfs\0").unwrap();
        let result = unsafe { ctru_sys::romfsMountSelf(mount_name.as_ptr()) };

        if result < 0 {
            Err(result.into())
        } else {
            Ok(Self)
        }
    }
}

impl Drop for RomFS {
    fn drop(&mut self) {
        let mount_name = CStr::from_bytes_with_nul(b"romfs\0").unwrap();
        unsafe { ctru_sys::romfsUnmount(mount_name.as_ptr()) };
    }
}
