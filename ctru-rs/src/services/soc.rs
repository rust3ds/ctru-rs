use libc::{free, memalign};
use std::net::Ipv4Addr;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::Error;
use ctru_sys::{socExit, socInit};

/// Soc service. Initializing this service will enable the use of network sockets and utilities
/// such as those found in `std::net`. The service will be closed when this struct is is dropped.
#[non_exhaustive]
pub struct Soc {
    soc_mem: *mut u32,
}

static SOC_ACTIVE: AtomicBool = AtomicBool::new(false);

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
        match SOC_ACTIVE.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => unsafe {
                let soc_mem = memalign(0x1000, num_bytes) as *mut u32;

                let r = socInit(soc_mem, num_bytes as u32);
                if r < 0 {
                    free(soc_mem as *mut _);
                    Err(r.into())
                } else {
                    Ok(Self { soc_mem })
                }
            },
            Err(_) => Err(Error::ServiceAlreadyActive("Soc")),
        }
    }

    /// IP Address of the Nintendo 3DS system.
    pub fn host_address(&self) -> Ipv4Addr {
        let raw_id = unsafe { libc::gethostid() };
        Ipv4Addr::from(raw_id.to_ne_bytes())
    }
}

impl Drop for Soc {
    fn drop(&mut self) {
        unsafe {
            socExit();
            free(self.soc_mem as *mut _);
        }

        SOC_ACTIVE.store(false, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soc_duplicate() {
        let _soc = Soc::init().unwrap();

        match Soc::init() {
            Err(Error::ServiceAlreadyActive("Soc")) => return,
            _ => panic!(),
        }
    }
}
