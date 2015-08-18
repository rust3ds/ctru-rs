use ::{Handle, Result};
use ::raw::c_void;

pub const RUNFLAG_APTWORKAROUND: u32 = 1;
pub const RUNFLAG_APTREINIT: u32 = 2;

#[repr(C)]
pub enum NS_APPID {
    APPID_HOMEMENU = 0x101, // Home Menu
	APPID_CAMERA = 0x110, // Camera applet
	APPID_FRIENDS_LIST = 0x112, // Friends List applet
	APPID_GAME_NOTES = 0x113, // Game Notes applet
	APPID_WEB = 0x114, // Internet Browser
	APPID_INSTRUCTION_MANUAL = 0x115, // Instruction Manual applet
	APPID_NOTIFICATIONS = 0x116, // Notifications applet
	APPID_MIIVERSE = 0x117, // Miiverse applet
	APPID_APPLICATION = 0x300, // Application
	APPID_SOFTWARE_KEYBOARD = 0x401, // Software Keyboard
	APPID_APPLETED = 0x402, // appletEd
	APPID_PNOTE_AP = 0x404, // PNOTE_AP
	APPID_SNOTE_AP = 0x405, // SNOTE_AP
	APPID_ERROR = 0x406, // error
	APPID_MINT = 0x407, // mint
	APPID_EXTRAPAD = 0x408, // extrapad
	APPID_MEMOLIB = 0x409, // memolib
} // cf http://3dbrew.org/wiki/NS_and_APT_Services#AppIDs

#[repr(C)]
pub enum APP_STATUS {
	APP_NOTINITIALIZED,
	APP_RUNNING,
	APP_SUSPENDED,
	APP_EXITING,
	APP_SUSPENDING,
	APP_SLEEPMODE,
	APP_PREPARE_SLEEPMODE,
	APP_APPLETSTARTED,
	APP_APPLETCLOSED
}

#[repr(C)]
pub enum APTSIGNAL {
	APTSIGNAL_HOMEBUTTON   = 1,
	// 2: sleep-mode related?
	APTSIGNAL_PREPARESLEEP = 3,
	// 4: triggered when ptm:s GetShellStatus() returns 5.
	APTSIGNAL_ENTERSLEEP   = 5,
	APTSIGNAL_WAKEUP       = 6,
	APTSIGNAL_ENABLE       = 7,
	APTSIGNAL_POWERBUTTON  = 8,
	APTSIGNAL_UTILITY      = 9,
	APTSIGNAL_SLEEPSYSTEM  = 10,
	APTSIGNAL_ERROR        = 11
}

#[repr(C)]
pub enum APTHOOK {
	APTHOOK_ONSUSPEND = 0,
	APTHOOK_ONRESTORE,
	APTHOOK_ONSLEEP,
	APTHOOK_ONWAKEUP,
	APTHOOK_ONEXIT,

	APTHOOK_COUNT,
}

type aptHookFn = Option<extern "C" fn(hook: i32, param: *mut c_void) -> ()>;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct aptHookCookie {
    next: *mut aptHookCookie,
    callback: aptHookFn,
    param: *mut c_void,
}

#[link(name = "ctru")]
extern "C" {
    pub static mut aptEvents: [Handle; 3usize];

    pub fn aptInit() -> Result;
    pub fn aptExit() -> ();
    pub fn aptOpenSession() -> ();
    pub fn aptCloseSession() -> ();
    pub fn aptSetStatus(status: APP_STATUS) -> ();
    pub fn aptGetStatus() -> APP_STATUS;
    pub fn aptGetStatusPower() -> u32;
    pub fn aptSetStatusPower(status: u32) -> ();
    pub fn aptReturnToMenu() -> ();
    pub fn aptWaitStatusEvent() -> ();
    pub fn aptSignalReadyForSleep() -> ();
    pub fn aptGetMenuAppID() -> NS_APPID;
    pub fn aptMainLoop() -> u8;
    pub fn APT_GetLockHandle(handle: *mut Handle, flags: u16, lockHandle: *mut Handle) -> Result;
    pub fn APT_Initialize(handle: *mut Handle, appId: NS_APPID, eventHandle1: *mut Handle, eventHandle2: *mut Handle) -> Result;
    pub fn APT_HardwareResetAsync(handle: *mut Handle) -> Result;
    pub fn APT_Enable(handle: *mut Handle, a: u32) -> Result;
    pub fn APT_GetAppletManInfo(handle: *mut Handle, inval: u8, outval8: *mut u8, outval32: *mut u32, menu_appid: *mut NS_APPID, active_appid: *mut NS_APPID) -> Result;
    pub fn APT_PrepareToJumpToHomeMenu(handle: *mut Handle) -> Result;
    pub fn APT_JumpToHomeMenu(handle: *mut Handle, a: u32, b: u32, c: u32) -> Result;
    pub fn APT_IsRegistered(handle: *mut Handle, appID: NS_APPID, out: *mut u8) -> Result;
    pub fn APT_InquireNotification(handle: *mut Handle, appID: u32, signalType: *mut u8) -> Result;
    pub fn APT_NotifyToWait(handle: *mut Handle, appID: NS_APPID) -> Result;
    pub fn APT_AppletUtility(handle: *mut Handle, out: *mut u32, a: u32, size1: u32, buf1: *mut u8, size2: u32, buf2: *mut u8) -> Result;
    pub fn APT_GlanceParameter(handle: *mut Handle, appID: NS_APPID, bufferSize: u32, buffer: *mut u32, actualSize: *mut u32, signalType: *mut u8) -> Result;
    pub fn APT_ReceiveParameter(handle: *mut Handle, appID: NS_APPID, bufferSize: u32, buffer: *mut u32, actualSize: *mut u32, signalType: *mut u8) -> Result;
    pub fn APT_SendParameter(handle: *mut Handle, src_appID: NS_APPID, dst_appID: NS_APPID, bufferSize: u32, buffer: *mut u32, paramhandle: Handle, signalType: u8) -> Result;
    pub fn APT_SendCaptureBufferInfo(handle: *mut Handle, bufferSize: u32, buffer: *mut u32) -> Result;
    pub fn APT_ReplySleepQuery(handle: *mut Handle, appID: NS_APPID, a: u32) -> Result;
    pub fn APT_ReplySleepNotificationComplete(handle: *mut Handle, appID: NS_APPID) -> Result;
    pub fn APT_PrepareToCloseApplication(handle: *mut Handle, a: u8) -> Result;
    pub fn APT_CloseApplication(handle: *mut Handle, a: u32, b: u32, c: u32) -> Result;
    pub fn APT_SetAppCpuTimeLimit(handle: *mut Handle, percent: u32) -> Result;
    pub fn APT_GetAppCpuTimeLimit(handle: *mut Handle, percent: *mut u32) -> Result;
    pub fn APT_CheckNew3DS_Application(handle: *mut Handle, out: *mut u8) -> Result;
    pub fn APT_CheckNew3DS_System(handle: *mut Handle, out: *mut u8) -> Result;
    pub fn APT_CheckNew3DS(handle: *mut Handle, out: *mut u8) -> Result;
    pub fn APT_PrepareToDoAppJump(handle: *mut Handle, flags: u8, programID: u64, mediatype: u8) -> Result;
    pub fn APT_DoAppJump(handle: *mut Handle, NSbuf0Size: u32, NSbuf1Size: u32, NSbuf0Ptr: *mut u8, NSbuf1Ptr: *mut u8) -> Result;
    pub fn APT_PrepareToStartLibraryApplet(handle: *mut Handle, appID: NS_APPID) -> Result;
    pub fn APT_StartLibraryApplet(handle: *mut Handle, appID: NS_APPID, inhandle: Handle, parambuf: *mut u32, parambufsize: u32) -> Result;
    pub fn APT_LaunchLibraryApplet(appID: NS_APPID, inhandle: Handle, parambuf: *mut u32, parambufsize: u32) -> Result;
    pub fn APT_PrepareToStartSystemApplet(handle: *mut Handle, appID: NS_APPID) -> Result;
    pub fn APT_StartSystemApplet(handle: *mut Handle, appID: NS_APPID, bufSize: u32, applHandle: Handle, buf: *mut u8) -> Result;
}
