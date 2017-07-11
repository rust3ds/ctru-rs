pub struct Apt(());

impl Apt {
    pub fn init() -> ::Result<Apt> {
        unsafe {
            let r = ::libctru::aptInit();
            if r < 0 {
                Err(r.into())
            } else {
                Ok(Apt(()))
            }
        }
    }

    pub fn main_loop(&self) -> bool {
        unsafe {
            ::libctru::aptMainLoop()
        }
    }
}

impl Drop for Apt {
    fn drop(&mut self) {
        unsafe { ::libctru::aptExit() };
    }
}
