//! Process Services.
//!
//! This service handles miscellaneous utility tasks used by the various processes.
//! However, it is particularly important because it is used to generate cryptographically secure random data, which
//! is required for commonly used functionality such as hashing (e.g. [`HashMap`](std::collections::HashMap) will not work without it).
//!
//! See also <https://www.3dbrew.org/wiki/Process_Services>

use crate::error::ResultCode;
use crate::Result;

/// Type of AES algorithm to use.
#[doc(alias = "PS_AESAlgorithm")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AESAlgorithm {
    /// CBC encryption.
    CbcEnc = ctru_sys::PS_ALGORITHM_CBC_ENC,
    /// CBC decryption.
    CbcDec = ctru_sys::PS_ALGORITHM_CBC_DEC,
    /// CTR encryption.
    CtrEnc = ctru_sys::PS_ALGORITHM_CTR_ENC,
    /// CTR decryption.
    CtrDec = ctru_sys::PS_ALGORITHM_CTR_DEC,
    /// CCM encryption.
    CcmEnc = ctru_sys::PS_ALGORITHM_CCM_ENC,
    /// CCM decryption.
    CcmDec = ctru_sys::PS_ALGORITHM_CCM_DEC,
}

/// PS Key slot to use.
#[doc(alias = "PS_AESKeyType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AESKeyType {
    /// Keyslot 0x0D.
    Keyslot0D = ctru_sys::PS_KEYSLOT_0D,
    /// Keyslot 0x2D.
    Keyslot2D = ctru_sys::PS_KEYSLOT_2D,
    /// Keyslot 0x2E.
    Keyslot2E = ctru_sys::PS_KEYSLOT_2E,
    /// Keyslot 0x31.
    Keyslot31 = ctru_sys::PS_KEYSLOT_31,
    /// Keyslot 0x32.
    Keyslot32 = ctru_sys::PS_KEYSLOT_32,
    /// Keyslot 0x36.
    Keyslot36 = ctru_sys::PS_KEYSLOT_36,
    /// Keyslot 0x38.
    Keyslot38 = ctru_sys::PS_KEYSLOT_38,
    /// Keyslot 0x39 (DLP).
    Keyslot39Dlp = ctru_sys::PS_KEYSLOT_39_DLP,
    /// Keyslot 0x39 (NFC).
    Keyslot39Nfc = ctru_sys::PS_KEYSLOT_39_NFC,
    /// Invalid keyslot.
    KeyslotInvalid = ctru_sys::PS_KEYSLOT_INVALID,
}

/// Handle to the PS service.
pub struct Ps(());

impl Ps {
    /// Initialize a new service handle.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ps::Ps;
    ///
    /// let ps = Ps::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "psInit")]
    pub fn new() -> Result<Self> {
        unsafe {
            ResultCode(ctru_sys::psInit())?;
            Ok(Ps(()))
        }
    }

    /// Returns the console's local friend code seed.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ps::Ps;
    /// let ps = Ps::new()?;
    ///
    /// let friend_code_seed = ps.local_friend_code_seed()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "PS_GetLocalFriendCodeSeed")]
    pub fn local_friend_code_seed(&self) -> crate::Result<u64> {
        let mut seed: u64 = 0;

        ResultCode(unsafe { ctru_sys::PS_GetLocalFriendCodeSeed(&mut seed) })?;
        Ok(seed)
    }

    /// Returns the console's devide ID.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ps::Ps;
    /// let ps = Ps::new()?;
    ///
    /// let device_id = ps.device_id()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "PS_GetDeviceId")]
    pub fn device_id(&self) -> crate::Result<u32> {
        let mut id: u32 = 0;

        ResultCode(unsafe { ctru_sys::PS_GetDeviceId(&mut id) })?;
        Ok(id)
    }

    /// Generates cryptografically secure random bytes and writes them into the `out` buffer.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::ps::Ps;
    /// let ps = Ps::new()?;
    ///
    /// let mut buffer = vec![0; 128];
    ///
    /// // The buffer is now randomized!
    /// ps.generate_random_bytes(&mut buffer)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "PS_GenerateRandomBytes")]
    pub fn generate_random_bytes(&self, out: &mut [u8]) -> crate::Result<()> {
        ResultCode(unsafe {
            ctru_sys::PS_GenerateRandomBytes(out.as_mut_ptr().cast(), out.len())
        })?;
        Ok(())
    }
}

impl Drop for Ps {
    #[doc(alias = "psExit")]
    fn drop(&mut self) {
        unsafe {
            ctru_sys::psExit();
        }
    }
}

from_impl!(AESAlgorithm, ctru_sys::PS_AESAlgorithm);
from_impl!(AESKeyType, ctru_sys::PS_AESKeyType);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn construct_hash_map() {
        let mut input = vec![
            (1_i32, String::from("123")),
            (2, String::from("2")),
            (6, String::from("six")),
        ];

        let map: HashMap<i32, String> = HashMap::from_iter(input.clone());

        let mut actual: Vec<_> = map.into_iter().collect();
        input.sort();
        actual.sort();

        assert_eq!(input, actual);
    }
}
