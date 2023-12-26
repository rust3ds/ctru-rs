//! Service API
//!
//! Not all APIs are wrapped in this module, since a lot are fundamentally unsafe.
//! Most APIs should be used directly from `ctru-sys`.

use crate::error::ResultCode;
use ctru_sys::Handle;
use std::time::Duration;

/// Extension trait for [Handle]
pub trait HandleExt {
    /// Wait for an event to fire. If the timeout is reached, an error is returned. You can use
    /// [`Error::is_timeout`] to check if the error is due to a timeout.
    fn wait_for_event(self, timeout: Duration) -> crate::Result<()>;
}

impl HandleExt for Handle {
    fn wait_for_event(self, timeout: Duration) -> crate::Result<()> {
        let timeout = i64::try_from(timeout.as_nanos()).map_err(|e| {
            crate::Error::Other(format!(
                "Failed to convert timeout to 64-bit nanoseconds: {}",
                e
            ))
        })?;
        unsafe {
            ResultCode(ctru_sys::svcWaitSynchronization(self, timeout))?;
        }
        Ok(())
    }
}

/// Creates a command header to be used for IPC. This is a const fn version of [`ctru_sys::IPC_MakeHeader`].
pub const fn make_ipc_header(command_id: u16, normal_params: u8, translate_params: u8) -> u32 {
    ((command_id as u32) << 16)
        | (((normal_params as u32) & 0x3F) << 6)
        | ((translate_params as u32) & 0x3F)
}
