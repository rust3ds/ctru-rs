//! SSLC (TLS) service.

// TODO: Implement remaining functions

use crate::error::ResultCode;

/// Handle to the SSLC service.
pub struct SslC(());

impl SslC {
    /// Initialize a new service handle.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::sslc::SslC;
    ///
    /// let sslc = SslC::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "sslcInit")]
    pub fn new() -> crate::Result<Self> {
        unsafe {
            ResultCode(ctru_sys::sslcInit(0))?;
            Ok(SslC(()))
        }
    }
}

impl Drop for SslC {
    #[doc(alias = "sslcExit")]
    fn drop(&mut self) {
        unsafe { ctru_sys::sslcExit() };
    }
}
