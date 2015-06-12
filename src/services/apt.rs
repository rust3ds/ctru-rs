use ::Result;

use core::ops::Fn;

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


fn to_raw_appstatus(status: AppStatus) -> apt::APP_STATUS {
    use self::AppStatus::*;
    match status {
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

fn from_raw_appstatus(status: apt::APP_STATUS) -> AppStatus {
    use self::AppStatus::*;
    match status {
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

pub fn init() -> Result {
    unsafe {
        return apt::aptInit();
    }
}

pub fn exit() -> () {
    unsafe {
        apt::aptExit();
    }
}

pub fn get_status() -> AppStatus {
    unsafe {
        return from_raw_appstatus(apt::aptGetStatus());
    }
}

pub fn set_status(status: AppStatus) -> () {
    unsafe {
        apt::aptSetStatus(to_raw_appstatus(status));
    }
}

/// Return to the home menu.
///
/// When `get_status` returns `AppStatus::Suspending`, you should call this,
/// otherwise the app will be left stuck in that state.
///
/// The program will not return from this function until the system returns
/// to the application, or when the status changes to `AppStatus::Exiting`.
///
/// # Examples
///
/// ```
/// if get_status() == Suspending {
///     return_to_menu();
/// }
/// ```
pub fn return_to_menu() -> () {
    unsafe {
        apt::aptReturnToMenu();
    }
}

/// Execute a function repeatedly until the apt main loop is over.
///
/// # Examples
///
/// ```
/// main_loop(|| {
///     // do things here
/// });
/// ```
pub fn main_loop<F>(f: F) -> () where F : Fn() -> () {
    unsafe {
        while apt::aptMainLoop() != 0 {
            f();
        }
    }
}
