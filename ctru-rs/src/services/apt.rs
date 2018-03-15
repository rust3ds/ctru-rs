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

    pub fn set_app_cpu_time_limit(&self, percent: u32) -> ::Result<()> {
        unsafe {
           let r = ::libctru::APT_SetAppCpuTimeLimit(percent);
           if r < 0 {
               Err(r.into())
           } else {
               Ok(())
           }
        }
    }
}

impl Drop for Apt {
    fn drop(&mut self) {
        unsafe { ::libctru::aptExit() };
    }
}
