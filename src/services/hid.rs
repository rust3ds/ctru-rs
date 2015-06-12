use ::Result;

use ::raw::services::hid;

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
    Right
}

fn to_raw_padkey(key: PadKey) -> u32 {
    use ::raw::services::hid::PAD_KEY::*;
    use self::PadKey::*;

    match key {
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
        DPadDown => KEY_DDOWN as u32
    }
}

pub fn init() -> Result {
    unsafe {
        // TODO allow sharedMem argument?
        return hid::hidInit(0 as *mut u32);
    }
}

pub fn exit() -> () {
    unsafe {
        hid::hidExit();
    }
}

/// Update ctrulib's button states.
///
/// # Examples
///
/// ```
/// use ctru::service::apt;
///
/// apt::main_loop(|| {
///     scan_input();
///     if key_down(PadKey::A) {
///         apt::set_status(apt::AppStatus::Exiting);
///     }
/// });
/// ```
pub fn scan_input() -> () {
    unsafe {
        hid::hidScanInput();
    }
}

pub fn key_down(key: PadKey) -> bool {
    unsafe {
        if hid::hidKeysDown() & to_raw_padkey(key) != 0 {
            return true;
        } else {
            return false;
        }
    }
}

pub fn key_held(key: PadKey) -> bool {
    unsafe {
        if hid::hidKeysHeld() & to_raw_padkey(key) != 0 {
            return true;
        } else {
            return false;
        }
    }
}

pub fn key_up(key: PadKey) -> bool {
    unsafe {
        if hid::hidKeysUp() & to_raw_padkey(key) != 0 {
            return true;
        } else {
            return false;
        }
    }
}
