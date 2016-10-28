use ::types::*;

pub const HID_SHAREDMEM_DEFAULT: u32 = 0x10000000;

#[repr(C)]
pub enum PAD_KEY {
    KEY_A = 1,
    KEY_B = 2,
    KEY_SELECT = 4,
    KEY_START = 8,
    KEY_DRIGHT = 16,
    KEY_DLEFT = 32,
    KEY_DUP = 64,
    KEY_DDOWN = 128,
    KEY_R = 256,
    KEY_L = 512,
    KEY_X = 1024,
    KEY_Y = 2048,
    KEY_ZL = 16384,
    KEY_ZR = 32768,
    KEY_TOUCH = 1048576,
    KEY_CSTICK_RIGHT = 16777216,
    KEY_CSTICK_LEFT = 33554432,
    KEY_CSTICK_UP = 67108864,
    KEY_CSTICK_DOWN = 134217728,
    KEY_CPAD_RIGHT = 268435456,
    KEY_CPAD_LEFT = 536870912,
    KEY_CPAD_UP = 1073741824,
    KEY_CPAD_DOWN = -2147483648,
    KEY_UP = 1073741888,
    KEY_DOWN = -2147483520,
    KEY_LEFT = 536870944,
    KEY_RIGHT = 268435472,

    // Generic catch-all directions
    /*KEY_UP    = KEY_DUP    | KEY_CPAD_UP,
    KEY_DOWN  = KEY_DDOWN  | KEY_CPAD_DOWN,
    KEY_LEFT  = KEY_DLEFT  | KEY_CPAD_LEFT,
    KEY_RIGHT = KEY_DRIGHT | KEY_CPAD_RIGHT,*/
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct touchPosition {
    px: u16,
    py: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct circlePosition {
    dx: s16,
    dy: s16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct accelVector {
    x: s16,
    y: s16,
    z: s16
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct angularRate {
    x: s16, //roll
    z: s16, //yaw
    y: s16, //pitch
}

#[repr(C)]
pub enum HID_Event {
    HIDEVENT_PAD0 = 0,
    HIDEVENT_PAD1 = 1,
    HIDEVENT_Accel = 2,
    HIDEVENT_Gyro = 3,
    HIDEVENT_DebugPad = 4,
    HIDEVENT_MAX = 5,

}


extern "C" {
    pub fn hidInit() -> Result;
    pub fn hidExit();
    pub fn hidScanInput();
    pub fn hidKeysHeld() -> u32;
    pub fn hidKeysDown() -> u32;
    pub fn hidKeysUp() -> u32;
    pub fn hidTouchRead(pos: *mut touchPosition);
    pub fn hidCircleRead(pos: *mut circlePosition);
    pub fn hidAccelRead(vector: *mut accelVector);
    pub fn hidGyroRead(rate: *mut angularRate);
    pub fn hidWaitForEvent(id: HID_Event, nextEvent: u8);
    pub fn HIDUSER_GetHandles(outMemHandle: *mut Handle,
                              eventpad0: *mut Handle, eventpad1: *mut Handle,
                              eventaccel: *mut Handle, eventgyro: *mut Handle,
                              eventdebugpad: *mut Handle) -> Result;
    pub fn HIDUSER_EnableAccelerometer() -> Result;
    pub fn HIDUSER_DisableAccelerometer() -> Result;
    pub fn HIDUSER_EnableGyroscope() -> Result;
    pub fn HIDUSER_DisableGyroscope() -> Result;
    pub fn HIDUSER_GetGyroscopeRawToDpsCoefficient(coeff:
                                                       *mut f32)
     -> Result;
    pub fn HIDUSER_GetSoundVolume(volume: *mut u8) -> Result;
}

