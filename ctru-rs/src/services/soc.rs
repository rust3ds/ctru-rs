use libc::memalign;
use once_cell::sync::Lazy;
use std::net::Ipv4Addr;
use std::sync::Mutex;

use crate::error::LibCtruResult;
use crate::services::ServiceReference;
use crate::Error;

/// Soc service. Initializing this service will enable the use of network sockets and utilities
/// such as those found in `std::net`. The service will be closed when this struct is is dropped.
#[non_exhaustive]
pub struct Soc {
    _service_handler: ServiceReference,
    sock_3dslink: libc::c_int,
}

static SOC_ACTIVE: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

impl Soc {
    /// Initialize the Soc service with a default buffer size of 0x100000 bytes
    ///
    /// # Errors
    ///
    /// This function will return an error if the `Soc` service is already initialized
    pub fn init() -> crate::Result<Self> {
        Self::init_with_buffer_size(0x100000)
    }

    /// Initialize the Soc service with a custom buffer size in bytes. The size should be
    /// 0x100000 bytes or greater.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `Soc` service is already initialized
    pub fn init_with_buffer_size(num_bytes: usize) -> crate::Result<Self> {
        let _service_handler = ServiceReference::new(
            &SOC_ACTIVE,
            false,
            || {
                let soc_mem = unsafe { memalign(0x1000, num_bytes) } as *mut u32;
                LibCtruResult(unsafe { ctru_sys::socInit(soc_mem, num_bytes as u32) })?;

                Ok(())
            },
            // `socExit` returns an error code. There is no documentantion of when errors could happen,
            // but we wouldn't be able to handle them in the `Drop` implementation anyways.
            // Surely nothing bad will happens :D
            || unsafe {
                // The socket buffer is freed automatically by `socExit`
                let _ = ctru_sys::socExit();
            },
        )?;

        Ok(Self {
            _service_handler,
            sock_3dslink: -1,
        })
    }

    /// IP Address of the Nintendo 3DS system.
    pub fn host_address(&self) -> Ipv4Addr {
        let raw_id = unsafe { libc::gethostid() };
        Ipv4Addr::from(raw_id.to_ne_bytes())
    }

    /// Redirect output streams (i.e. [`println`] and [`eprintln`]) to the `3dslink` server.
    /// Requires `3dslink` >= 0.6.1 and `new-hbmenu` >= 2.3.0.
    ///
    /// # Errors
    ///
    /// Returns an error if a connection cannot be established to the server, or
    /// output was already previously redirected.
    pub fn redirect_to_3dslink(&mut self, stdout: bool, stderr: bool) -> crate::Result<()> {
        if self.sock_3dslink >= 0 {
            return Err(Error::OutputAlreadyRedirected);
        }

        if !stdout && !stderr {
            return Ok(());
        }

        self.sock_3dslink = unsafe { ctru_sys::link3dsConnectToHost(stdout, stderr) };
        if self.sock_3dslink < 0 {
            Err(Error::from_errno())
        } else {
            Ok(())
        }
    }
}

impl Drop for Soc {
    fn drop(&mut self) {
        if self.sock_3dslink >= 0 {
            unsafe {
                libc::close(self.sock_3dslink);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soc_duplicate() {
        let _soc = Soc::init().unwrap();

        assert!(matches!(Soc::init(), Err(Error::ServiceAlreadyActive)))
    }
}
