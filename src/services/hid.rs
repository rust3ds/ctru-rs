use core::convert::Into;
use core::marker::PhantomData;

use libctru::services::hid;

pub enum PadKey {
    A,
    B,
    Select,
    Start,
    DPadRight,
    DPadLeft,
    DPadUp,
    DPadDown,
    R,
    L,
    X,
    Y,
    ZL,
    ZR,
    Touch,
    CSRight,
    CSLeft,
    CSUp,
    CSDown,
    CRight,
    CLeft,
    CUp,
    CDown,

    // convenience catch-all for dpad and cpad
    Up,
    Down,
    Left,
    Right,
}

impl From<PadKey> for u32 {
    fn from(p: PadKey) -> u32 {
        use libctru::services::hid::PAD_KEY::*;
        use self::PadKey::*;

        match p {
            Up => KEY_DUP as u32 | KEY_CPAD_UP as u32,
            Down => KEY_DDOWN as u32 | KEY_CPAD_DOWN as u32,
            Left => KEY_DLEFT as u32 | KEY_CPAD_LEFT as u32,
            Right => KEY_DRIGHT as u32 | KEY_CPAD_RIGHT as u32,

            A => KEY_A as u32,
            B => KEY_B as u32,
            X => KEY_X as u32,
            Y => KEY_Y as u32,
            L => KEY_L as u32,
            R => KEY_R as u32,
            ZL => KEY_ZL as u32,
            ZR => KEY_ZR as u32,
            Start => KEY_START as u32,
            Select => KEY_SELECT as u32,
            Touch => KEY_TOUCH as u32,
            CSRight => KEY_CSTICK_RIGHT as u32,
            CSLeft => KEY_CSTICK_LEFT as u32,
            CSUp => KEY_CSTICK_UP as u32,
            CSDown => KEY_CSTICK_DOWN as u32,
            CRight => KEY_CPAD_RIGHT as u32,
            CLeft => KEY_CPAD_LEFT as u32,
            CDown => KEY_CPAD_DOWN as u32,
            CUp => KEY_CPAD_UP as u32,
            DPadLeft => KEY_DLEFT as u32,
            DPadRight => KEY_DRIGHT as u32,
            DPadUp => KEY_DUP as u32,
            DPadDown => KEY_DDOWN as u32,
        }
    }
}

pub struct Hid {
    pd: PhantomData<i32>
}

impl Hid {
    pub fn init() -> Result<Hid, i32> {
        unsafe {
            let r = hid::hidInit();
            if r < 0 {
                Err(r)
            } else {
                Ok(Hid { pd: PhantomData })
            }
        }
    }

    pub fn scan_input(&self) {
        unsafe { hid::hidScanInput() };
    }

    pub fn key_down(&self, key: PadKey) -> bool {
        let k: u32 = key.into();
        unsafe {
            if hid::hidKeysDown() & k != 0 {
                true
            } else {
                false
            }
        }
    }

    pub fn key_held(&self, key: PadKey) -> bool {
        let k: u32 = key.into();
        unsafe {
            if hid::hidKeysHeld() & k != 0 {
                true
            } else {
                false
            }
        }
    }

    pub fn key_up(&self, key: PadKey) -> bool {
        let k: u32 = key.into();
        unsafe {
            if hid::hidKeysUp() & k != 0 {
                return true;
            } else {
                return false;
            }
        }
    }
}

impl Drop for Hid {
    fn drop(&mut self) {
        unsafe { hid::hidExit() };
    }
}
