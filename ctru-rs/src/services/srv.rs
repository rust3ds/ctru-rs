//! Service API
//!
//! Not all APIs are wrapped in this module, since a lot are fundamentally unsafe.
//! Most APIs should be used directly from `ctru-sys`.

use crate::error::ResultCode;
use ctru_sys::Handle;
use std::time::Duration;

/// Wait for an event to fire. If the timeout is reached, an error is returned. You can use
/// [`Error::is_timeout`] to check if the error is due to a timeout.
pub fn wait_for_event(event: Handle, timeout: Duration) -> crate::Result<()> {
    unsafe {
        ResultCode(ctru_sys::svcWaitSynchronization(
            event,
            timeout.as_nanos() as i64,
        ))?;
    }
    Ok(())
}
