//! Ports of macros in
//! <https://github.com/devkitPro/libctru/blob/master/libctru/include/3ds/result.h>

use crate::Result;

/// Checks whether a result code indicates success.
pub fn R_SUCCEEDED(res: Result) -> bool {
    res >= 0
}

/// Checks whether a result code indicates failure.
pub fn R_FAILED(res: Result) -> bool {
    res < 0
}

/// Returns the level of a result code.
pub fn R_LEVEL(res: Result) -> libc::c_uchar {
    ((res >> 27) & 0x1F) as _
}

/// Returns the summary of a result code.
pub fn R_SUMMARY(res: Result) -> libc::c_uchar {
    ((res >> 21) & 0x3F) as _
}

/// Returns the module ID of a result code.
pub fn R_MODULE(res: Result) -> libc::c_uchar {
    ((res >> 10) & 0xFF) as _
}

/// Returns the description of a result code.
pub fn R_DESCRIPTION(res: Result) -> libc::c_ushort {
    (res & 0x3FF) as _
}

/// Builds a result code from its constituent components.
pub fn MAKERESULT(level: Result, summary: Result, module: Result, description: Result) -> Result {
    ((level & 0x1F) << 27)
        | ((summary & 0x3F) << 21)
        | ((module & 0xFF) << 10)
        | (description & 0x3FF)
}
