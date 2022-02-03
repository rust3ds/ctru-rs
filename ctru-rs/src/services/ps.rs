//! Process Services (PS) module. This is used for miscellaneous utility tasks, but
//! is particularly important because it is used to generate random data, which
//! is required for common things like [`HashMap`](std::collections::HashMap).
//! See also <https://www.3dbrew.org/wiki/Process_Services>

/// PS handle. This must not be dropped in order for random generation
/// to work (in most cases, the lifetime of an application).
#[non_exhaustive]
pub struct Ps;

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

        let mut m: HashMap<i32, String> = HashMap::from_iter([
            (1_i32, String::from("123")),
            (2, String::from("2")),
            (6, String::from("six")),
        ]);

        println!("{:?}", m);

        m.remove(&2);
        m.insert(5, "ok".into());

        println!("{:#?}", m);
    }

    #[test]
    #[should_panic]
    fn construct_hash_map_no_rand() {
        // Without initializing PS, we can't use `libc::getrandom` and constructing
        // a HashMap panics at runtime.

        let mut m: HashMap<i32, String> = HashMap::from_iter([
            (1_i32, String::from("123")),
            (2, String::from("2")),
            (6, String::from("six")),
        ]);

        println!("{:?}", m);

        m.remove(&2);
        m.insert(5, "ok".into());

        println!("{:#?}", m);
    }
}
