use std::borrow::Cow;
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::ops::{ControlFlow, FromResidual, Try};

use ctru_sys::result::{R_DESCRIPTION, R_LEVEL, R_MODULE, R_SUMMARY};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[repr(transparent)]
pub struct ResultCode(pub ctru_sys::Result);

impl Try for ResultCode {
    type Output = ();
    type Residual = Error;

    fn from_output(_: Self::Output) -> Self {
        Self(0)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        // Wait timeouts aren't counted as "failures" in libctru, but an unfinished task means unsafety for us.
        // Luckily all summary cases are for system failures (except RS_SUCCESS).
        // I don't know if there are any cases in libctru where a Result holds a "failing" summary but a "success" code, so we'll just check for both.
        if ctru_sys::R_FAILED(self.0) || ctru_sys::R_SUMMARY(self.0) != ctru_sys::RS_SUCCESS as i32
        {
            ControlFlow::Break(self.into())
        } else {
            ControlFlow::Continue(())
        }
    }
}

impl FromResidual for ResultCode {
    fn from_residual(e: <Self as Try>::Residual) -> Self {
        match e {
            Error::Os(result) => Self(result),
            _ => unreachable!(),
        }
    }
}

impl<T> FromResidual<Error> for Result<T> {
    fn from_residual(e: Error) -> Self {
        Err(e)
    }
}

/// The error type returned by all libctru functions.
#[non_exhaustive]
pub enum Error {
    Os(ctru_sys::Result),
    Libc(String),
    ServiceAlreadyActive,
    OutputAlreadyRedirected,
    BufferTooShort {
        /// Length of the buffer provided by the user.
        provided: usize,
        /// Size of the requested data (in bytes).
        wanted: usize,
    },
}

impl Error {
    /// Create an [`Error`] out of the last set value in `errno`. This can be used
    /// to get a human-readable error string from calls to `libc` functions.
    pub(crate) fn from_errno() -> Self {
        let error_str = unsafe {
            let errno = ctru_sys::errno();
            let str_ptr = libc::strerror(errno);

            // Safety: strerror should always return a valid string,
            // even if the error number is unknown
            CStr::from_ptr(str_ptr)
        };

        // Copy out of the error string, since it may be changed by other libc calls later
        Self::Libc(error_str.to_string_lossy().into())
    }
}

impl From<ctru_sys::Result> for Error {
    fn from(err: ctru_sys::Result) -> Self {
        Error::Os(err)
    }
}

impl From<ResultCode> for Error {
    fn from(err: ResultCode) -> Self {
        Self::Os(err.0)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Self::Os(err) => f
                .debug_struct("Error")
                .field("raw", &format_args!("{err:#08X}"))
                .field("level", &result_code_level_str(err))
                .field("module", &result_code_module_str(err))
                .field("summary", &result_code_summary_str(err))
                .field("description", &result_code_description_str(err))
                .finish(),
            Self::Libc(err) => f.debug_tuple("Libc").field(err).finish(),
            Self::ServiceAlreadyActive => f.debug_tuple("ServiceAlreadyActive").finish(),
            Self::OutputAlreadyRedirected => f.debug_tuple("OutputAlreadyRedirected").finish(),
            Self::BufferTooShort { provided, wanted } => f
                .debug_struct("BufferTooShort")
                .field("provided", provided)
                .field("wanted", wanted)
                .finish(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Self::Os(err) => write!(
                f,
                "libctru result code 0x{err:08X}: [{} {}] {}: {}",
                result_code_level_str(err),
                result_code_module_str(err),
                result_code_summary_str(err),
                result_code_description_str(err)
            ),
            Self::Libc(err) => write!(f, "{err}"),
            Self::ServiceAlreadyActive => write!(f, "service already active"),
            Self::OutputAlreadyRedirected => {
                write!(f, "output streams are already redirected to 3dslink")
            }
            Self::BufferTooShort{provided, wanted} => write!(f, "the provided buffer's length is too short (length = {provided}) to hold the wanted data (size = {wanted})")
        }
    }
}

impl error::Error for Error {}

fn result_code_level_str(result: ctru_sys::Result) -> Cow<'static, str> {
    use ctru_sys::{
        RL_FATAL, RL_INFO, RL_PERMANENT, RL_REINITIALIZE, RL_RESET, RL_STATUS, RL_SUCCESS,
        RL_TEMPORARY, RL_USAGE,
    };

    Cow::Borrowed(match R_LEVEL(result) as u32 {
        RL_SUCCESS => "success",
        RL_INFO => "info",
        RL_FATAL => "fatal",
        RL_RESET => "reset",
        RL_REINITIALIZE => "reinitialize",
        RL_USAGE => "usage",
        RL_PERMANENT => "permanent",
        RL_TEMPORARY => "temporary",
        RL_STATUS => "status",
        code => return Cow::Owned(format!("(unknown level: {code:#x})")),
    })
}

fn result_code_summary_str(result: ctru_sys::Result) -> Cow<'static, str> {
    use ctru_sys::{
        RS_CANCELED, RS_INTERNAL, RS_INVALIDARG, RS_INVALIDRESVAL, RS_INVALIDSTATE, RS_NOP,
        RS_NOTFOUND, RS_NOTSUPPORTED, RS_OUTOFRESOURCE, RS_STATUSCHANGED, RS_SUCCESS,
        RS_WOULDBLOCK, RS_WRONGARG,
    };

    Cow::Borrowed(match R_SUMMARY(result) as u32 {
        RS_SUCCESS => "success",
        RS_NOP => "nop",
        RS_WOULDBLOCK => "would_block",
        RS_OUTOFRESOURCE => "out_of_resource",
        RS_NOTFOUND => "not_found",
        RS_INVALIDSTATE => "invalid_state",
        RS_NOTSUPPORTED => "not_supported",
        RS_INVALIDARG => "invalid_arg",
        RS_WRONGARG => "wrong_arg",
        RS_CANCELED => "canceled",
        RS_STATUSCHANGED => "status_changed",
        RS_INTERNAL => "internal",
        RS_INVALIDRESVAL => "invalid_res_val",
        code => return Cow::Owned(format!("(unknown summary: {code:#x})")),
    })
}

fn result_code_description_str(result: ctru_sys::Result) -> Cow<'static, str> {
    use ctru_sys::{
        RD_ALREADY_DONE, RD_ALREADY_EXISTS, RD_ALREADY_INITIALIZED, RD_BUSY, RD_CANCEL_REQUESTED,
        RD_INVALID_ADDRESS, RD_INVALID_COMBINATION, RD_INVALID_ENUM_VALUE, RD_INVALID_HANDLE,
        RD_INVALID_POINTER, RD_INVALID_RESULT_VALUE, RD_INVALID_SELECTION, RD_INVALID_SIZE,
        RD_MISALIGNED_ADDRESS, RD_MISALIGNED_SIZE, RD_NOT_AUTHORIZED, RD_NOT_FOUND,
        RD_NOT_IMPLEMENTED, RD_NOT_INITIALIZED, RD_NO_DATA, RD_OUT_OF_MEMORY, RD_OUT_OF_RANGE,
        RD_SUCCESS, RD_TIMEOUT, RD_TOO_LARGE,
    };

    Cow::Borrowed(match R_DESCRIPTION(result) as u32 {
        RD_SUCCESS => "success",
        RD_INVALID_RESULT_VALUE => "invalid_result_value",
        RD_TIMEOUT => "timeout",
        RD_OUT_OF_RANGE => "out_of_range",
        RD_ALREADY_EXISTS => "already_exists",
        RD_CANCEL_REQUESTED => "cancel_requested",
        RD_NOT_FOUND => "not_found",
        RD_ALREADY_INITIALIZED => "already_initialized",
        RD_NOT_INITIALIZED => "not_initialized",
        RD_INVALID_HANDLE => "invalid_handle",
        RD_INVALID_POINTER => "invalid_pointer",
        RD_INVALID_ADDRESS => "invalid_address",
        RD_NOT_IMPLEMENTED => "not_implemented",
        RD_OUT_OF_MEMORY => "out_of_memory",
        RD_MISALIGNED_SIZE => "misaligned_size",
        RD_MISALIGNED_ADDRESS => "misaligned_address",
        RD_BUSY => "busy",
        RD_NO_DATA => "no_data",
        RD_INVALID_COMBINATION => "invalid_combination",
        RD_INVALID_ENUM_VALUE => "invalid_enum_value",
        RD_INVALID_SIZE => "invalid_size",
        RD_ALREADY_DONE => "already_done",
        RD_NOT_AUTHORIZED => "not_authorized",
        RD_TOO_LARGE => "too_large",
        RD_INVALID_SELECTION => "invalid_selection",
        code => return Cow::Owned(format!("(unknown description: {code:#x})")),
    })
}

fn result_code_module_str(result: ctru_sys::Result) -> Cow<'static, str> {
    use ctru_sys::{
        RM_AC, RM_ACC, RM_ACT, RM_AM, RM_AM_LOW, RM_APPLET, RM_APPLICATION, RM_AVD, RM_BOSS,
        RM_CAM, RM_CARD, RM_CARDNOR, RM_CARD_SPI, RM_CEC, RM_CODEC, RM_COMMON, RM_CONFIG, RM_CSND,
        RM_CUP, RM_DBG, RM_DBM, RM_DD, RM_DI, RM_DLP, RM_DMNT, RM_DSP, RM_EC, RM_ENC, RM_FATFS,
        RM_FILE_SERVER, RM_FND, RM_FRIENDS, RM_FS, RM_FSI, RM_GD, RM_GPIO, RM_GSP, RM_GYROSCOPE,
        RM_HID, RM_HIO, RM_HIO_LOW, RM_HTTP, RM_I2C, RM_INVALIDRESVAL, RM_IR, RM_KERNEL, RM_L2B,
        RM_LDR, RM_LOADER_SERVER, RM_MC, RM_MCU, RM_MIC, RM_MIDI, RM_MP, RM_MPWL, RM_MVD, RM_NDM,
        RM_NEIA, RM_NEWS, RM_NEX, RM_NFC, RM_NFP, RM_NGC, RM_NIM, RM_NPNS, RM_NS, RM_NWM, RM_OLV,
        RM_OS, RM_PDN, RM_PI, RM_PIA, RM_PL, RM_PM, RM_PM_LOW, RM_PS, RM_PTM, RM_PXI, RM_QTM,
        RM_RDT, RM_RO, RM_ROMFS, RM_SDMC, RM_SND, RM_SOC, RM_SPI, RM_SPM, RM_SRV, RM_SSL, RM_SWC,
        RM_TCB, RM_TEST, RM_UART, RM_UDS, RM_UPDATER, RM_UTIL, RM_VCTL, RM_WEB_BROWSER,
    };

    Cow::Borrowed(match R_MODULE(result) as u32 {
        RM_COMMON => "common",
        RM_KERNEL => "kernel",
        RM_UTIL => "util",
        RM_FILE_SERVER => "file_server",
        RM_LOADER_SERVER => "loader_server",
        RM_TCB => "tcb",
        RM_OS => "os",
        RM_DBG => "dbg",
        RM_DMNT => "dmnt",
        RM_PDN => "pdn",
        RM_GSP => "gsp",
        RM_I2C => "i2c",
        RM_GPIO => "gpio",
        RM_DD => "dd",
        RM_CODEC => "codec",
        RM_SPI => "spi",
        RM_PXI => "pxi",
        RM_FS => "fs",
        RM_DI => "di",
        RM_HID => "hid",
        RM_CAM => "cam",
        RM_PI => "pi",
        RM_PM => "pm",
        RM_PM_LOW => "pm_low",
        RM_FSI => "fsi",
        RM_SRV => "srv",
        RM_NDM => "ndm",
        RM_NWM => "nwm",
        RM_SOC => "soc",
        RM_LDR => "ldr",
        RM_ACC => "acc",
        RM_ROMFS => "romfs",
        RM_AM => "am",
        RM_HIO => "hio",
        RM_UPDATER => "updater",
        RM_MIC => "mic",
        RM_FND => "fnd",
        RM_MP => "mp",
        RM_MPWL => "mpwl",
        RM_AC => "ac",
        RM_HTTP => "http",
        RM_DSP => "dsp",
        RM_SND => "snd",
        RM_DLP => "dlp",
        RM_HIO_LOW => "hio_low",
        RM_CSND => "csnd",
        RM_SSL => "ssl",
        RM_AM_LOW => "am_low",
        RM_NEX => "nex",
        RM_FRIENDS => "friends",
        RM_RDT => "rdt",
        RM_APPLET => "applet",
        RM_NIM => "nim",
        RM_PTM => "ptm",
        RM_MIDI => "midi",
        RM_MC => "mc",
        RM_SWC => "swc",
        RM_FATFS => "fatfs",
        RM_NGC => "ngc",
        RM_CARD => "card",
        RM_CARDNOR => "cardnor",
        RM_SDMC => "sdmc",
        RM_BOSS => "boss",
        RM_DBM => "dbm",
        RM_CONFIG => "config",
        RM_PS => "ps",
        RM_CEC => "cec",
        RM_IR => "ir",
        RM_UDS => "uds",
        RM_PL => "pl",
        RM_CUP => "cup",
        RM_GYROSCOPE => "gyroscope",
        RM_MCU => "mcu",
        RM_NS => "ns",
        RM_NEWS => "news",
        RM_RO => "ro",
        RM_GD => "gd",
        RM_CARD_SPI => "card_spi",
        RM_EC => "ec",
        RM_WEB_BROWSER => "web_browser",
        RM_TEST => "test",
        RM_ENC => "enc",
        RM_PIA => "pia",
        RM_ACT => "act",
        RM_VCTL => "vctl",
        RM_OLV => "olv",
        RM_NEIA => "neia",
        RM_NPNS => "npns",
        RM_AVD => "avd",
        RM_L2B => "l2b",
        RM_MVD => "mvd",
        RM_NFC => "nfc",
        RM_UART => "uart",
        RM_SPM => "spm",
        RM_QTM => "qtm",
        RM_NFP => "nfp",
        RM_APPLICATION => "application",
        RM_INVALIDRESVAL => "invalid_res_val",
        code => return Cow::Owned(format!("(unknown module: {code:#x})")),
    })
}
