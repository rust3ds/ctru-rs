//! Network Socket service.
//!
//! By using this service the program enables the use of network sockets and utilities such as those found in `std::net`, which are completely inaccessible by default.
//! As such, remember to hold a handle to this service handle while using any network functionality, or else the `std::net` methods will return generic OS errors.
#![doc(alias = "socket")]
#![doc(alias = "network")]

use libc::memalign;
use std::net::Ipv4Addr;
use std::sync::Mutex;

use crate::error::ResultCode;
use crate::services::ServiceReference;
use crate::Error;

/// Handle to the Network Socket service.
pub struct Soc {
    _service_handler: ServiceReference,
    sock_3dslink: libc::c_int,
}

static SOC_ACTIVE: Mutex<usize> = Mutex::new(0);

impl Soc {
    /// Initialize a new service handle using a socket buffer size of `0x100000` bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if the [`Soc`] service is already being used.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::soc::Soc;
    ///
    /// let soc = Soc::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "socInit")]
    pub fn new() -> crate::Result<Self> {
        Self::init_with_buffer_size(0x100000)
    }

    /// Initialize a new service handle using a custom socket buffer size.
    ///
    /// The size should be `0x100000` bytes or greater.
    ///
    /// # Errors
    ///
    /// This function will return an error if the [`Soc`] service is already being used.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::soc::Soc;
    ///
    /// let soc = Soc::init_with_buffer_size(0x100000)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "socInit")]
    pub fn init_with_buffer_size(num_bytes: usize) -> crate::Result<Self> {
        let _service_handler = ServiceReference::new(
            &SOC_ACTIVE,
            false,
            || {
                let soc_mem = unsafe { memalign(0x1000, num_bytes) } as *mut u32;
                ResultCode(unsafe { ctru_sys::socInit(soc_mem, num_bytes as u32) })?;

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

    /// Returns the local IP Address of the Nintendo 3DS system.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::soc::Soc;
    /// let soc = Soc::new()?;
    ///
    /// let address = soc.host_address();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "gethostid")]
    pub fn host_address(&self) -> Ipv4Addr {
        let raw_id = unsafe { libc::gethostid() };
        Ipv4Addr::from(raw_id.to_ne_bytes())
    }

    /// Redirect output streams (i.e. `stdout` and `stderr`) to the `3dslink` server.
    ///
    /// With this redirection it is possible to send (and view in real time) the output of `stdout` operations,
    /// such as `println!` or `dbg!`.
    ///
    /// Requires `3dslink` >= 0.6.1 and `new-hbmenu` >= 2.3.0 and the use of the `--server` flag.
    /// The `--server` flag is also availble to use via `cargo-3ds` if the requirements are met.
    ///
    /// # Errors
    ///
    /// Returns an error if a connection cannot be established to the server,
    /// or if the output was already being redirected.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::soc::Soc;
    /// let mut soc = Soc::new()?;
    ///
    /// // Redirect to the `3dslink` server that sent this program.
    /// let address = soc.redirect_to_3dslink(true, true)?;
    ///
    /// println!("I'm visible from a PC!");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "link3dsConnectToHost")]
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
    #[doc(alias = "socExit")]
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
        let _soc = Soc::new().unwrap();

        assert!(matches!(Soc::new(), Err(Error::ServiceAlreadyActive)))
    }
}
