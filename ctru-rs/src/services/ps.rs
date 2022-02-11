//! Process Services (PS) module. This is used for miscellaneous utility tasks, but
//! is particularly important because it is used to generate random data, which
//! is required for common things like [`HashMap`](std::collections::HashMap).
//! See also <https://www.3dbrew.org/wiki/Process_Services>

/// PS handle. This must not be dropped in order for random generation
/// to work (in most cases, the lifetime of an application).
#[non_exhaustive]
pub struct Ps;

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

impl Ps {
    /// Initialize the PS module.
    pub fn init() -> crate::Result<Self> {
        let r = unsafe { ctru_sys::psInit() };
        if r < 0 {
            Err(r.into())
        } else {
            Ok(Self)
        }
    }

    pub fn local_friend_code_seed(&self) -> crate::Result<u64> {
        let mut seed: u64 = 0;

        let r = unsafe { ctru_sys::PS_GetLocalFriendCodeSeed(&mut seed) };
        if r < 0 {
            Err(r.into())
        } else {
            Ok(seed)
        }
    }

    pub fn device_id(&self) -> crate::Result<u32> {
        let mut id: u32 = 0;

        let r = unsafe { ctru_sys::PS_GetDeviceId(&mut id) };
        if r < 0 {
            Err(r.into())
        } else {
            Ok(id)
        }
    }

    pub fn generate_random_bytes(&self, out: &mut [u8]) -> crate::Result<()> {
        let r =
            unsafe { ctru_sys::PS_GenerateRandomBytes(out as *mut _ as *mut _, out.len() as u32) };
        if r < 0 {
            Err(r.into())
        } else {
            Ok(())
        }
    }
}

impl Drop for Ps {
    fn drop(&mut self) {
        unsafe {
            ctru_sys::psExit();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn construct_hash_map() {
        let _ps = Ps::init().unwrap();

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

    #[test]
    fn construct_hash_map_no_rand() {
        // Without initializing PS, we can't use `libc::getrandom` and constructing
        // a HashMap panics at runtime.
        //
        // If any test case successfully creates a HashMap before this test,
        // the thread-local RandomState in std will be initialized. We spawn
        // a new thread to actually create the hash map, since even in multi-threaded
        // test environment there's a chance this test wouldn't panic because
        // some other test case ran before it.
        //
        // One downside of this approach is that the panic handler for the panicking
        // thread prints to the console, which is not captured by the default test
        // harness and prints even when the test passes.
        std::thread::Builder::new()
            .stack_size(0x20_0000)
            .spawn(|| {
                let map: HashMap<i32, String> = HashMap::from_iter([
                    (1_i32, String::from("123")),
                    (2, String::from("2")),
                    (6, String::from("six")),
                ]);

                dbg!(map);
            })
            .unwrap()
            .join()
            .expect_err("should have panicked");
    }
}
