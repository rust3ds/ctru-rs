//! Syscall APIs
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

    /// Send a service request to the handle.
    /// The request vector must contain the command header and any parameters.
    /// The request vector is overwritten with the response and returned.
    /// The error in the response is checked and returned as a `Result::Err` if the operation failed.
    ///
    /// # Safety
    /// This function is unsafe because it directly accesses the thread command buffer.
    /// If the request vector or the expected response length is incorrect, or the handle is not a service that accepts
    /// requests, the function may cause undefined behavior.
    unsafe fn send_service_request(
        self,
        request: Vec<u32>,
        expected_response_len: usize,
    ) -> crate::Result<Vec<u32>>;
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

    unsafe fn send_service_request(
        self,
        mut request: Vec<u32>,
        expected_response_len: usize,
    ) -> crate::Result<Vec<u32>> {
        // Copy over the request
        let cmd_buffer_ptr = unsafe { ctru_sys::getThreadCommandBuffer() };

        unsafe {
            std::ptr::copy_nonoverlapping(request.as_ptr(), cmd_buffer_ptr, request.len());

            // Send the request
            ResultCode(ctru_sys::svcSendSyncRequest(self))?;

            // Handle the result returned by the service
            let result = std::ptr::read(cmd_buffer_ptr.add(1));
            ResultCode(result as ctru_sys::Result)?;
        }

        // Copy back the response
        request.clear();
        request.resize(expected_response_len, 0);
        unsafe {
            std::ptr::copy_nonoverlapping(
                cmd_buffer_ptr,
                request.as_mut_ptr(),
                expected_response_len,
            );
        }

        Ok(request)
    }
}

/// Creates a command header to be used for IPC. This is a const fn version of [`ctru_sys::IPC_MakeHeader`].
pub const fn make_ipc_header(command_id: u16, normal_params: u8, translate_params: u8) -> u32 {
    ((command_id as u32) << 16)
        | (((normal_params as u32) & 0x3F) << 6)
        | ((translate_params as u32) & 0x3F)
}
