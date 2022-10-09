//! Process Services (PS) module. This is used for miscellaneous utility tasks, but
//! is particularly important because it is used to generate random data, which
//! is required for common things like [`HashMap`](std::collections::HashMap).
//! As such, it is initialized by default in `ctru::init` instead of having a safety handler
//! See also <https://www.3dbrew.org/wiki/Process_Services>

use crate::error::LibCtruResult;

#[repr(u32)]
pub enum AESAlgorithm {
    CbcEnc,
    CbcDec,
    CtrEnc,
    CtrDec,
    CcmEnc,
    CcmDec,
}

#[repr(u32)]
pub enum AESKeyType {
    Keyslot0D,
    Keyslot2D,
    Keyslot31,
    Keyslot38,
    Keyslot32,
    Keyslot39Dlp,
    Keyslot2E,
    KeyslotInvalid,
    Keyslot36,
    Keyslot39Nfc,
}

pub fn local_friend_code_seed() -> crate::Result<u64> {
    let mut seed: u64 = 0;

    LibCtruResult(unsafe { ctru_sys::PS_GetLocalFriendCodeSeed(&mut seed) })?;
    Ok(seed)
}

pub fn device_id() -> crate::Result<u32> {
    let mut id: u32 = 0;

    LibCtruResult(unsafe { ctru_sys::PS_GetDeviceId(&mut id) })?;
    Ok(id)
}

pub fn generate_random_bytes(out: &mut [u8]) -> crate::Result<()> {
    LibCtruResult(unsafe {
        ctru_sys::PS_GenerateRandomBytes(out as *mut _ as *mut _, out.len() as u32)
    })?;
    Ok(())
}

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
