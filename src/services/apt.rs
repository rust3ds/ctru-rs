use std::marker::PhantomData;

use libctru::services::apt;

pub enum AppStatus {
    NotInitialized,
    Running,
    Suspended,
    Exiting,
    Suspending,
    SleepMode,
    PrepareSleepMode,
    AppletStarted,
    AppletClosed,
}

impl From<AppStatus> for apt::APT_AppStatus {
    fn from(a: AppStatus) -> apt::APT_AppStatus {
        use self::AppStatus::*;
        use libctru::services::apt::APT_AppStatus::*;
        match a {
            NotInitialized => APP_NOTINITIALIZED,
            Running => APP_RUNNING,
            Suspended => APP_SUSPENDED,
            Exiting => APP_EXITING,
            Suspending => APP_SUSPENDING,
            SleepMode => APP_SLEEPMODE,
            PrepareSleepMode => APP_PREPARE_SLEEPMODE,
            AppletStarted => APP_APPLETSTARTED,
            AppletClosed => APP_APPLETCLOSED,
        }
    }
}

impl From<apt::APT_AppStatus> for AppStatus {
    fn from(a: apt::APT_AppStatus) -> AppStatus {
        use self::AppStatus::*;
        use libctru::services::apt::APT_AppStatus::*;
        match a {
            APP_NOTINITIALIZED => NotInitialized,
            APP_RUNNING => Running,
            APP_SUSPENDED => Suspended,
            APP_EXITING => Exiting,
            APP_SUSPENDING => Suspending,
            APP_SLEEPMODE => SleepMode,
            APP_PREPARE_SLEEPMODE => PrepareSleepMode,
            APP_APPLETSTARTED => AppletStarted,
            APP_APPLETCLOSED => AppletClosed,
        }
    }
}

pub struct Apt {
    pd: PhantomData<i32>
}

impl Apt {
    pub fn init() -> Result<Apt, i32> {
        unsafe {
            let r = apt::aptInit();
            if r < 0 {
                Err(r)
            } else {
                Ok(Apt { pd: PhantomData })
            }
        }
    }

    pub fn main_loop(&self) -> bool {
        unsafe {
            match apt::aptMainLoop() {
                1 => true,
                0 => false,
                _ => unreachable!(),
            }
        }
    }
}

impl Drop for Apt {
    fn drop(&mut self) {
        unsafe { apt::aptExit() };
    }
}
