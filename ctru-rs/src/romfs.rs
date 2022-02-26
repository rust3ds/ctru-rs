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
use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::Error;

#[non_exhaustive]
pub struct RomFS;

static ROMFS_ACTIVE: AtomicBool = AtomicBool::new(false);

impl RomFS {
    pub fn init() -> crate::Result<Self> {
        match ROMFS_ACTIVE.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => {
                let mount_name = CStr::from_bytes_with_nul(b"romfs\0").unwrap();
                let result = unsafe { ctru_sys::romfsMountSelf(mount_name.as_ptr()) };

                if result < 0 {
                    Err(result.into())
                } else {
                    Ok(Self)
                }
            }
            Err(_) => Err(Error::ServiceAlreadyActive("RomFS")),
        }
    }
}

impl Drop for RomFS {
    fn drop(&mut self) {
        let mount_name = CStr::from_bytes_with_nul(b"romfs\0").unwrap();
        unsafe { ctru_sys::romfsUnmount(mount_name.as_ptr()) };

        ROMFS_ACTIVE.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn romfs_duplicate() {
        let _romfs = RomFS::init().unwrap();

        assert!(RomFS::init().is_err());
    }
}
