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