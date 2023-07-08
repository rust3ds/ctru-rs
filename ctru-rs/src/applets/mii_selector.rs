//! Mii Selector applet
//!
//! This module contains the methods to launch the Mii Selector.

use crate::mii::MiiData;
use bitflags::bitflags;
use std::ffi::CString;

/// Index of a Mii used to configure some parameters of the Mii Selector
/// Can be either a single index, or _all_ Miis
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Index {
    Index(u32),
    All,
}

/// The type of a Mii with their respective data
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MiiType {
    Guest { index: u32, name: String },
    User,
}

bitflags! {
    /// Options for the Mii Selector
    pub struct Options: u32 {
        /// Show the cancel button
        const MII_SELECTOR_CANCEL = ctru_sys::MIISELECTOR_CANCEL;
        /// Make guest Miis selectable
        const MII_SELECTOR_GUESTS = ctru_sys::MIISELECTOR_GUESTS;
        /// Show on the top screen
        const MII_SELECTOR_TOP = ctru_sys::MIISELECTOR_TOP;
        /// Start on the guest's page
        const MII_SELECTOR_GUEST_START = ctru_sys::MIISELECTOR_GUESTSTART;
    }
}

/// An instance of the Mii Selector
///
/// # Example
/// ```
/// use ctru::applets::mii_selector::MiiSelector;
///
/// let mut mii_selector = MiiSelector::new();
/// mii_selector.set_title("Example Mii selector");
///
/// let result = mii_selector.launch().unwrap();
/// ```
#[doc(alias = "MiiSelectorConf")]
#[derive(Clone, Debug)]
pub struct MiiSelector {
    config: Box<ctru_sys::MiiSelectorConf>,
}

/// Return value from a MiiSelector's launch
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct SelectionResult {
    pub mii_data: MiiData,
    pub mii_type: MiiType,
}

/// Error type for the Mii selector
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LaunchError {
    /// The selected Mii's data is corrupt in some way
    InvalidChecksum,
    /// Either the user cancelled the selection (see [Options::MII_SELECTOR_CANCEL]), or no valid Miis were available to select
    NoMiiSelected,
}

impl MiiSelector {
    /// Initializes a Mii Selector
    #[doc(alias = "miiSelectorInit")]
    pub fn new() -> Self {
        let mut config = Box::<ctru_sys::MiiSelectorConf>::default();
        unsafe {
            ctru_sys::miiSelectorInit(config.as_mut());
        }
        Self { config }
    }

    /// Set the title of the Mii Selector.
    ///
    /// This function would panic if the given ``&str`` contains NUL bytes.
    #[doc(alias = "miiSelectorSetTitle")]
    pub fn set_title(&mut self, text: &str) {
        // This can only fail if the text contains NUL bytes in the string... which seems
        // unlikely and is documented
        let c_text = CString::new(text).expect("Failed to convert the title text into a CString");
        unsafe {
            ctru_sys::miiSelectorSetTitle(self.config.as_mut(), c_text.as_ptr());
        }
    }

    /// Set the options of the Mii Selector
    #[doc(alias = "miiSelectorSetOptions")]
    pub fn set_options(&mut self, options: Options) {
        unsafe { ctru_sys::miiSelectorSetOptions(self.config.as_mut(), options.bits) }
    }

    /// Whitelist a guest Mii
    #[doc(alias = "miiSelectorWhitelistGuestMii")]
    pub fn whitelist_guest_mii(&mut self, mii_index: Index) {
        let index = match mii_index {
            Index::Index(i) => i,
            Index::All => ctru_sys::MIISELECTOR_GUESTMII_SLOTS,
        };

        unsafe { ctru_sys::miiSelectorWhitelistGuestMii(self.config.as_mut(), index) }
    }

    /// Blacklist a guest Mii
    #[doc(alias = "miiSelectorBlacklistGuestMii")]
    pub fn blacklist_guest_mii(&mut self, mii_index: Index) {
        let index = match mii_index {
            Index::Index(i) => i,
            Index::All => ctru_sys::MIISELECTOR_GUESTMII_SLOTS,
        };

        unsafe { ctru_sys::miiSelectorBlacklistGuestMii(self.config.as_mut(), index) }
    }

    /// Whitelist a user Mii
    #[doc(alias = "miiSelectorWhitelistUserMii")]
    pub fn whitelist_user_mii(&mut self, mii_index: Index) {
        let index = match mii_index {
            Index::Index(i) => i,
            Index::All => ctru_sys::MIISELECTOR_USERMII_SLOTS,
        };

        unsafe { ctru_sys::miiSelectorWhitelistUserMii(self.config.as_mut(), index) }
    }

    /// Blacklist a user Mii
    #[doc(alias = "miiSelectorBlacklistUserMii")]
    pub fn blacklist_user_mii(&mut self, mii_index: Index) {
        let index = match mii_index {
            Index::Index(i) => i,
            Index::All => ctru_sys::MIISELECTOR_USERMII_SLOTS,
        };

        unsafe { ctru_sys::miiSelectorBlacklistUserMii(self.config.as_mut(), index) }
    }

    /// Set where the cursor will be.
    /// If there's no Mii at that index, the cursor will start at the Mii with the index 0
    pub fn set_initial_index(&mut self, index: usize) {
        // This function is static inline in libctru
        // https://github.com/devkitPro/libctru/blob/af5321c78ee5c72a55b526fd2ed0d95ca1c05af9/libctru/include/3ds/applets/miiselector.h#L155
        self.config.initial_index = index as u32;
    }

    /// Launch the Mii Selector.
    /// Returns an error when the checksum of the Mii is invalid.
    #[doc(alias = "miiSelectorLaunch")]
    pub fn launch(&mut self) -> Result<SelectionResult, LaunchError> {
        let mut return_val = Box::<ctru_sys::MiiSelectorReturn>::default();
        unsafe { ctru_sys::miiSelectorLaunch(self.config.as_mut(), return_val.as_mut()) }

        if return_val.no_mii_selected != 0 {
            return Err(LaunchError::NoMiiSelected);
        }

        if unsafe { ctru_sys::miiSelectorChecksumIsValid(return_val.as_mut()) } {
            Ok((*return_val).into())
        } else {
            Err(LaunchError::InvalidChecksum)
        }
    }
}

impl Default for MiiSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ctru_sys::MiiSelectorReturn> for SelectionResult {
    fn from(ret: ctru_sys::MiiSelectorReturn) -> Self {
        let raw_mii_data = ret.mii;
        let mut guest_mii_name = ret.guest_mii_name;

        SelectionResult {
            mii_data: raw_mii_data.into(),
            mii_type: if ret.guest_mii_index != 0xFFFFFFFF {
                MiiType::Guest {
                    index: ret.guest_mii_index,
                    name: {
                        let utf16_be = &mut guest_mii_name;
                        utf16_be.reverse();
                        String::from_utf16(utf16_be.as_slice()).unwrap()
                    },
                }
            } else {
                MiiType::User
            },
        }
    }
}

impl From<u32> for Index {
    fn from(v: u32) -> Self {
        Self::Index(v)
    }
}
