use crate::error::ResultCode;
use std::ffi::CString;
use std::io::Write;
use std::marker::PhantomData;
pub struct Ac(());

impl Ac {
    /// Initialize a new service handle.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ac::Ac;
    ///
    /// let ac = Ac::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "acInit")]
    pub fn new() -> crate::Result<Ac> {
        unsafe {
            ResultCode(ctru_sys::acInit())?;
            Ok(Ac(()))
        }
    }

    /// Waits for an internet connection
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ac::Ac;
    ///
    /// let ac = Ac::new()?;
    /// 
    /// println!("Waiting for an internet connection...");
    /// ac.wait_for_internet_connection()?;
    /// println!("Connected.");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "acWaitInternetConnection")]
    pub fn wait_internet_connection(&self) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::acWaitInternetConnection())?;

            Ok(())
        }
    }

    /// Returns whether the console is connected to Wi-Fi
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ac::Ac;
    ///
    /// let ac = Ac::new()?;
    /// 
    /// println!("Wi-Fi connected: {}", ac.get_wifi_status()?);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ACU_GetWifiStatus")]
    pub fn get_wifi_status(&self) -> crate::Result<bool> {
        unsafe {
            let mut ret = 0u32;
            ResultCode(ctru_sys::ACU_GetStatus(&mut ret))?;

            Ok(ret == 3)
        }
    }

    /// Returns whether the console is connected to Wi-Fi
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ac::Ac;
    ///
    /// let ac = Ac::new()?;
    /// 
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ACU_GetWifiSecurityMode")]
    pub fn get_wifi_security(&self) -> crate::Result<SecurityMode> {
        unsafe {
            let mut ret = 0u32;
            ResultCode(ctru_sys::ACU_GetSecurityMode(&mut ret))?;
            // fix this, for some reason the bindings have the type as a struct and not enum
            // and so i can't impl TryFrom automatically
            Ok(std::mem::transmute(ret))
        }
    }

    /// Returns the SSID of the Wi-Fi network the console is connected to, or error if the console isn't connected to any network.
    /// 
    /// You can check if the console is connected to a network using [`Self::get_wifi_status()`]
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ac::Ac;
    ///
    /// let ac = Ac::new()?;
    /// 
    /// println!("The console is connected to the network \"{}\"", ac.get_wifi_ssid().unwrap())
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ACU_GetSSID")]
    pub fn get_wifi_ssid(&self) -> crate::Result<String> {
        unsafe {
            let mut len = 0u32;
            ResultCode(ctru_sys::ACU_GetSSIDLength(&mut len))?;
            // we don't really need space for the terminator
            let mut vec: Vec<u8> = vec![0u8; len as usize];
            ResultCode(ctru_sys::ACU_GetSSID(vec.as_mut_ptr()))?;
            // how do i handle this error?
            Ok(String::from_utf8(vec).unwrap())
        }
    }
}

impl Drop for Ac {
    #[doc(alias = "acExit")]
    fn drop(&mut self) {
        unsafe { ctru_sys::acExit() };
    }
}

#[doc(alias = "acSecurityMode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SecurityMode {
    Open = 0,
    WEP40Bit = 1,
    WEP104Bit = 2,
    WEP128Bit = 3,
    WPA_TKIP = 4,
    WPA2_TKIP = 5,
    WPA_AES = 6,
    WPA2_AES = 7
}