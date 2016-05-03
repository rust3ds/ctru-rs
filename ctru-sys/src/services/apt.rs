use ::{Result, Handle};
use ::c_void;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum NS_APPID {
        APPID_HOMEMENU = 0x101, // Home Menu
        APPID_CAMERA = 0x110, // Camera applet
        APPID_FRIENDS_LIST = 0x112, // Friends List applet
        APPID_GAME_NOTES = 0x113, // Game Notes applet
        APPID_WEB = 0x114, // Internet Browser
        APPID_INSTRUCTION_MANUAL = 0x115, // Instruction Manual applet
        APPID_NOTIFICATIONS = 0x116, // Notifications applet
        APPID_MIIVERSE = 0x117, // Miiverse applet
        APPID_MIIVERSE_POSTING = 0x118,
        APPID_AMIIBO_SETTINGS = 0x119,
        APPID_APPLICATION = 0x300, // Application
        APPID_ESHOP = 0x301,
        APPID_SOFTWARE_KEYBOARD = 0x401, // Software Keyboard
        APPID_APPLETED = 0x402, // appletEd
        APPID_PNOTE_AP = 0x404, // PNOTE_AP
        APPID_SNOTE_AP = 0x405, // SNOTE_AP
        APPID_ERROR = 0x406, // error
        APPID_MINT = 0x407, // mint
        APPID_EXTRAPAD = 0x408, // extrapad
        APPID_MEMOLIB = 0x409, // memolib
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum APT_AppStatus {
    APP_NOTINITIALIZED = 0,
    APP_RUNNING = 1,
    APP_SUSPENDED = 2,
    APP_EXITING = 3,
    APP_SUSPENDING = 4,
    APP_SLEEPMODE = 5,
    APP_PREPARE_SLEEPMODE = 6,
    APP_APPLETSTARTED = 7,
    APP_APPLETCLOSED = 8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum APT_Signal {
    APTSIGNAL_HOMEBUTTON = 1,
    APTSIGNAL_PREPARESLEEP = 3,
    APTSIGNAL_ENTERSLEEP = 5,
    APTSIGNAL_WAKEUP = 6,
    APTSIGNAL_ENABLE = 7,
    APTSIGNAL_POWERBUTTON = 8,
    APTSIGNAL_UTILITY = 9,
    APTSIGNAL_SLEEPSYSTEM = 10,
    APTSIGNAL_ERROR = 11,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum APT_HookType {
    APTHOOK_ONSUSPEND = 0,
    APTHOOK_ONRESTORE = 1,
    APTHOOK_ONSLEEP = 2,
    APTHOOK_ONWAKEUP = 3,
    APTHOOK_ONEXIT = 4,
    APTHOOK_COUNT = 5,
}

pub type aptHookFn = Option<unsafe extern "C" fn(hook: APT_HookType, param: *mut c_void)>;

#[repr(C)]
#[derive(Copy)]
pub struct aptHookCookie {
    pub next: *mut aptHookCookie,
    pub callback: aptHookFn,
    pub param: *mut c_void,
}
impl ::core::clone::Clone for aptHookCookie {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for aptHookCookie {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

extern "C" {
    pub static mut aptEvents: [Handle; 3usize];
}
extern "C" {
    pub fn aptInit() -> Result;
    pub fn aptExit();
    pub fn aptOpenSession();
    pub fn aptCloseSession();
    pub fn aptSetStatus(status: APT_AppStatus);
    pub fn aptGetStatus() -> APT_AppStatus;
    pub fn aptGetStatusPower() -> u32;
    pub fn aptSetStatusPower(status: u32);
    pub fn aptReturnToMenu();
    pub fn aptWaitStatusEvent();
    pub fn aptSignalReadyForSleep();
    pub fn aptGetMenuAppID() -> NS_APPID;
    pub fn aptMainLoop() -> u8;
    pub fn aptHook(cookie: *mut aptHookCookie, callback: aptHookFn,
                   param: *mut c_void);
    pub fn aptUnhook(cookie: *mut aptHookCookie);
    pub fn APT_GetLockHandle(flags: u16, lockHandle: *mut Handle) -> Result;
    pub fn APT_Initialize(appId: NS_APPID, eventHandle1: *mut Handle,
                          eventHandle2: *mut Handle) -> Result;
    pub fn APT_Finalize(appId: NS_APPID) -> Result;
    pub fn APT_HardwareResetAsync() -> Result;
    pub fn APT_Enable(a: u32) -> Result;
    pub fn APT_GetAppletManInfo(inval: u8, outval8: *mut u8,
                                outval32: *mut u32,
                                menu_appid: *mut NS_APPID,
                                active_appid: *mut NS_APPID) -> Result;
    pub fn APT_GetAppletInfo(appID: NS_APPID, pProgramID: *mut u64,
                             pMediaType: *mut u8, pRegistered: *mut u8,
                             pLoadState: *mut u8, pAttributes: *mut u32)
     -> Result;
    pub fn APT_GetAppletProgramInfo(id: u32, flags: u32,
                                    titleversion: *mut u16) -> Result;
    pub fn APT_GetProgramID(pProgramID: *mut u64) -> Result;
    pub fn APT_PrepareToJumpToHomeMenu() -> Result;
    pub fn APT_JumpToHomeMenu(param: *const u8, paramSize: usize,
                              handle: Handle) -> Result;
    pub fn APT_PrepareToJumpToApplication(a: u32) -> Result;
    pub fn APT_JumpToApplication(param: *const u8, paramSize: usize,
                                 handle: Handle) -> Result;
    pub fn APT_IsRegistered(appID: NS_APPID, out: *mut u8) -> Result;
    pub fn APT_InquireNotification(appID: u32, signalType: *mut APT_Signal)
     -> Result;
    pub fn APT_NotifyToWait(appID: NS_APPID) -> Result;
    pub fn APT_AppletUtility(out: *mut u32, a: u32, size1: u32,
                             buf1: *mut u8, size2: u32, buf2: *mut u8)
     -> Result;
    pub fn APT_GlanceParameter(appID: NS_APPID, bufferSize: u32,
                               buffer: *mut u32, actualSize: *mut u32,
                               signalType: *mut u8) -> Result;
    pub fn APT_ReceiveParameter(appID: NS_APPID, bufferSize: u32,
                                buffer: *mut u32, actualSize: *mut u32,
                                signalType: *mut u8) -> Result;
    pub fn APT_SendParameter(src_appID: NS_APPID, dst_appID: NS_APPID,
                             bufferSize: u32, buffer: *mut u32,
                             paramhandle: Handle, signalType: u8) -> Result;
    pub fn APT_SendCaptureBufferInfo(bufferSize: u32, buffer: *mut u32)
     -> Result;
    pub fn APT_ReplySleepQuery(appID: NS_APPID, a: u32) -> Result;
    pub fn APT_ReplySleepNotificationComplete(appID: NS_APPID) -> Result;
    pub fn APT_PrepareToCloseApplication(a: u8) -> Result;
    pub fn APT_CloseApplication(param: *const u8, paramSize: usize,
                                handle: Handle) -> Result;
    pub fn APT_SetAppCpuTimeLimit(percent: u32) -> Result;
    pub fn APT_GetAppCpuTimeLimit(percent: *mut u32) -> Result;
    pub fn APT_CheckNew3DS_Application(out: *mut u8) -> Result;
    pub fn APT_CheckNew3DS_System(out: *mut u8) -> Result;
    pub fn APT_CheckNew3DS(out: *mut u8) -> Result;
    pub fn APT_PrepareToDoAppJump(flags: u8, programID: u64, mediatype: u8)
     -> Result;
    pub fn APT_DoAppJump(NSbuf0Size: u32, NSbuf1Size: u32,
                         NSbuf0Ptr: *mut u8, NSbuf1Ptr: *mut u8) -> Result;
    pub fn APT_PrepareToStartLibraryApplet(appID: NS_APPID) -> Result;
    pub fn APT_StartLibraryApplet(appID: NS_APPID, inhandle: Handle,
                                  parambuf: *mut u32, parambufsize: u32)
     -> Result;
    pub fn APT_LaunchLibraryApplet(appID: NS_APPID, inhandle: Handle,
                                   parambuf: *mut u32, parambufsize: u32)
     -> Result;
    pub fn APT_PrepareToStartSystemApplet(appID: NS_APPID) -> Result;
    pub fn APT_StartSystemApplet(appID: NS_APPID, bufSize: u32,
                                 applHandle: Handle, buf: *mut u8) -> Result;
}
