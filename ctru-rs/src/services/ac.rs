//! The Automatic Connection (AC) service handles Wi-Fi and network settings.
//! It can:
//! - Connect to a network or slot
//! - Get information about a network, such as its SSID or security settings
use crate::error::ResultCode;
use crate::services::ServiceReference;
use std::sync::Mutex;
/// Handle to the Automatic Connection (AC) service, that handles Wi-Fi and network settings.
pub struct Ac {
    _service_handler: ServiceReference,
}

static AC_ACTIVE: Mutex<()> = Mutex::new(());

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
        Ok(Ac {
            _service_handler: ServiceReference::new(
                &AC_ACTIVE,
                || {
                    ResultCode(unsafe { ctru_sys::acInit() })?;

                    Ok(())
                },
                || unsafe {
                    ctru_sys::acExit();
                },
            )?,
        })
    }

    /// Waits for an internet connection.
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
    /// ac.wait_internet_connection()?;
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

    /// Returns the current Wi-Fi connection status.
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
    /// println!("Wi-Fi status: {:?}", ac.wifi_status()?);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ACU_GetWifiStatus")]
    pub fn wifi_status(&self) -> crate::Result<NetworkStatus> {
        unsafe {
            let mut ret = 0u32;
            ResultCode(ctru_sys::ACU_GetStatus(&mut ret))?;

            Ok(match ret {
                0 => NetworkStatus::None,
                1 => NetworkStatus::Idle,
                2 => NetworkStatus::LANConnected,
                3 => NetworkStatus::WANConnected,
                _ => return Err(crate::Error::Other(format!("Unknown value {}", ret))),
            })
        }
    }

    /// Returns the [`SecurityMode`] of the currently connected network, or error if the console isn't connected to any network.
    ///
    /// You can check if the console is connected to a network using [`Ac::wifi_status()`].
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
    /// if ac.wifi_status()? == NetworkStatus::WANConnected {
    ///     println!("Network security: {:?}", ac.wifi_security()?);
    /// }
    ///
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ACU_GetWifiSecurityMode")]
    pub fn wifi_security(&self) -> crate::Result<SecurityMode> {
        unsafe {
            let mut ret = 0u32;
            ResultCode(ctru_sys::ACU_GetSecurityMode(&mut ret))?;
            // fix this, for some reason the bindings have the type as a struct and not enum
            // and so i can't impl TryFrom automatically
            Ok(match ret {
                0 => SecurityMode::Open,

                1 => SecurityMode::WEP40Bit,
                2 => SecurityMode::WEP104Bit,
                3 => SecurityMode::WEP128Bit,

                4 => SecurityMode::WPA_TKIP,
                5 => SecurityMode::WPA2_TKIP,

                6 => SecurityMode::WPA_AES,
                7 => SecurityMode::WPA2_AES,

                _ => return Err(crate::Error::Other(format!("Unknown value {}", ret))),
            })
        }
    }

    /// Returns the SSID of the Wi-Fi network the console is connected to, or error if the console isn't connected to any network.
    ///
    /// You can check if the console is connected to a network using [`Ac::wifi_status()`].
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
    /// println!("The console is connected to the network \"{}\"", ac.wifi_ssid().unwrap());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ACU_GetSSID")]
    pub fn wifi_ssid(&self) -> crate::Result<String> {
        unsafe {
            let mut len = 0u32;
            ResultCode(ctru_sys::ACU_GetSSIDLength(&mut len))?;
            // we don't really need space for the terminator
            let mut vec = vec![0u8; len as usize];
            ResultCode(ctru_sys::ACU_GetSSID(vec.as_mut_ptr()))?;
            Ok(String::from_utf8(vec)?)
        }
    }

    /// Returns whether the console is connected to a proxy.
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
    /// println!("Proxy enabled: {}", ac.proxy_enabled()?);
    ///
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ACU_GetProxyEnable")]
    pub fn proxy_enabled(&self) -> crate::Result<bool> {
        unsafe {
            let mut ret = false;
            ResultCode(ctru_sys::ACU_GetProxyEnable(&mut ret))?;

            Ok(ret)
        }
    }

    /// Returns the connected network's proxy port, if present.
    ///
    /// You can check if the console is using a proxy with [`Ac::proxy_enabled()`]
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
    /// println!("Proxy port: {}", ac.proxy_port()?);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ACU_GetProxyPort")]
    pub fn proxy_port(&self) -> crate::Result<u32> {
        unsafe {
            let mut ret = 0u32;
            ResultCode(ctru_sys::ACU_GetProxyPort(&mut ret))?;

            Ok(ret)
        }
    }

    /// Returns the connected network's proxy username, if present.
    ///
    /// You can check if the console is using a proxy with [`Ac::proxy_enabled()`]
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
    /// println!("Proxy username: {}", ac.proxy_username()?);
    ///
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ACU_GetProxyUserName")]
    pub fn proxy_username(&self) -> crate::Result<String> {
        unsafe {
            let mut vec = vec![0u8; 0x20];
            ResultCode(ctru_sys::ACU_GetProxyUserName(vec.as_mut_ptr()))?;

            Ok(String::from_utf8(vec)?)
        }
    }

    /// Returns the connected network's proxy password, if present.
    ///
    /// You can check if the console is using a proxy with [`Ac::proxy_enabled()`]
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
    /// println!("Proxy password: {}", ac.proxy_password()?);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ACU_GetProxyPassword")]
    pub fn proxy_password(&self) -> crate::Result<String> {
        unsafe {
            let mut vec = vec![0u8; 0x20];
            ResultCode(ctru_sys::ACU_GetProxyPassword(vec.as_mut_ptr()))?;

            Ok(String::from_utf8(vec))
        }
    }

    /// Load the selected network slot, if present.
    ///
    /// Note: this method requires `ac:i` access
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ac::{Ac, NetworkSlot};
    ///
    /// let ac = Ac::new()?;
    ///
    /// ac.load_network_slot(NetworkSlot::Second)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "ACI_LoadNetworkSetting")]
    pub fn load_network_slot(&mut self, slot: NetworkSlot) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::ACI_LoadNetworkSetting(slot as u32))?;
            Ok(())
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
#[non_exhaustive]
#[allow(non_camel_case_types)]
/// Represents all the supported Wi-Fi security modes.
pub enum SecurityMode {
    /// No authentication
    Open = ctru_sys::AC_OPEN,
    /// WEP 40bit authentication
    WEP40Bit = ctru_sys::AC_WEP_40BIT,
    /// WEP 104bit authentication
    WEP104Bit = ctru_sys::AC_WEP_104BIT,
    /// WEP 128bit authentication
    WEP128Bit = ctru_sys::AC_WEP_128BIT,
    /// WPA-TKIP authentication
    WPA_TKIP = ctru_sys::AC_WPA_TKIP,
    /// WPA2-TKIP authentication
    WPA2_TKIP = ctru_sys::AC_WPA2_TKIP,
    /// WPA-AES authentication
    WPA_AES = ctru_sys::AC_WPA_AES,
    /// WPA2-AES authentication
    WPA2_AES = ctru_sys::AC_WPA2_AES,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
/// Represents a network slot, like in the System Settings
pub enum NetworkSlot {
    /// The first network slot
    First = 0,
    /// The second network slot
    Second = 1,
    /// The third network slot
    Third = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[non_exhaustive]
/// Represents the current Wi-Fi status
pub enum NetworkStatus {
    /// Wi-Fi turned off
    None = 0,
    /// Not connected
    Idle = 1,
    /// Connected, only LAN.
    LANConnected = 2,
    /// Connected to the Internet.
    WANConnected = 3,
}

from_impl!(SecurityMode, ctru_sys::acSecurityMode);
