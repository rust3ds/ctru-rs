use core::marker::PhantomData;

use ::raw::services::apt;

pub enum AppStatus {
    NotInitialized,
    Running,
    Suspended,
    Exiting,
    Suspending,
    SleepMode,
    PrepareSleepMode,
    AppletStarted,
    AppletClosed
}

impl From<AppStatus> for apt::APP_STATUS {
    fn from(a: AppStatus) -> apt::APP_STATUS {
        use self::AppStatus::*;
        match a {
            NotInitialized => apt::APP_STATUS::APP_NOTINITIALIZED,
            Running => apt::APP_STATUS::APP_RUNNING,
            Suspended => apt::APP_STATUS::APP_SUSPENDED,
            Exiting => apt::APP_STATUS::APP_EXITING,
            Suspending => apt::APP_STATUS::APP_SUSPENDING,
            SleepMode => apt::APP_STATUS::APP_SLEEPMODE,
            PrepareSleepMode => apt::APP_STATUS::APP_PREPARE_SLEEPMODE,
            AppletStarted => apt::APP_STATUS::APP_APPLETSTARTED,
            AppletClosed => apt::APP_STATUS::APP_APPLETCLOSED,
        }
    }
}

impl From<apt::APP_STATUS> for AppStatus {
    fn from(a: apt::APP_STATUS) -> AppStatus {
        use self::AppStatus::*;
        match a {
             apt::APP_STATUS::APP_NOTINITIALIZED => NotInitialized,
             apt::APP_STATUS::APP_RUNNING => Running,
             apt::APP_STATUS::APP_SUSPENDED => Suspended,
             apt::APP_STATUS::APP_EXITING => Exiting,
             apt::APP_STATUS::APP_SUSPENDING => Suspending,
             apt::APP_STATUS::APP_SLEEPMODE => SleepMode,
             apt::APP_STATUS::APP_PREPARE_SLEEPMODE => PrepareSleepMode,
             apt::APP_STATUS::APP_APPLETSTARTED => AppletStarted,
             apt::APP_STATUS::APP_APPLETCLOSED => AppletClosed
        }
    }
}

pub struct Apt {
    pd: PhantomData<()>
}

impl Apt {
    pub fn new() -> Result<Apt, i32> {
        unsafe {
            let r = apt::aptInit();
            if r < 0 {
                Err(r)
            } else {
                Ok(Apt { pd: PhantomData })
            }
        }
    }

    pub fn get_status(&self) -> AppStatus {
        unsafe { apt::aptGetStatus().into() }
    }

    pub fn set_status(&mut self, status: AppStatus) {
        unsafe { apt::aptSetStatus(status.into()) };
    }

    /// Return to the home menu.
    ///
    /// When `get_status` returns `AppStatus::Suspending`, you should call this,
    /// otherwise the app will be left stuck in that state.
    ///
    /// The program will not return from this function until the system returns
    /// to the application, or when the status changes to `AppStatus::Exiting`.
    pub fn return_to_menu(&mut self) {
        unsafe { apt::aptReturnToMenu() };
    }

    pub fn main_loop(&mut self, app: &mut Application) {
        unsafe {
            while apt::aptMainLoop() != 0 {
                app.main_loop(self);
                if app.ready_to_quit() {
                    self.set_status(AppStatus::Exiting)
                }
            }
        };
    }
}

impl Drop for Apt {
    fn drop(&mut self) {
        unsafe { apt::aptExit() };
    }
}

pub trait Application {
    /// Program app loop body.
    fn main_loop(&mut self, apt: &mut Apt);

    /// True if the application is ready to quit.
    fn ready_to_quit(&self) -> bool;
}
