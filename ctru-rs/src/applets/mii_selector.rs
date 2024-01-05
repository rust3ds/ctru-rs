//! Mii Selector applet.
//!
//! This applet opens a window which lets the player/user choose a Mii from the ones present on their console.
//! The selected Mii is readable as a [`Mii`].

use crate::mii::Mii;
use crate::services::{apt::Apt, gfx::Gfx};

use bitflags::bitflags;
use std::{ffi::CString, fmt};

/// Index of a Mii on the [`MiiSelector`] interface.
///
/// See [`MiiSelector::allowlist_user_mii()`] and related functions for more information.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Index {
    /// Specific Mii index.
    ///
    /// # Notes
    ///
    /// Indexes start at 0.
    Index(u32),
    /// All Miis.
    All,
}

/// The type of a Mii.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MiiType {
    /// Guest Mii.
    Guest {
        /// Guest Mii index.
        index: u32,
        /// Guest Mii name.
        name: String,
    },
    /// User-made Mii.
    User,
}

bitflags! {
    /// Options to configure the [`MiiSelector`].
    ///
    /// See [`MiiSelector::set_options()`] to learn how to use them.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct Options: u32 {
        /// Show the cancel button.
        const ENABLE_CANCEL = ctru_sys::MIISELECTOR_CANCEL;
        /// Make guest Miis available to select.
        const ENABLE_GUESTS = ctru_sys::MIISELECTOR_GUESTS;
        /// Show the [`MiiSelector`] window on the top screen.
        const USE_TOP_SCREEN = ctru_sys::MIISELECTOR_TOP;
        /// Start the [`MiiSelector`] on the guests' page. Requires [`Options::ENABLE_GUESTS`].
        const START_WITH_GUESTS = ctru_sys::MIISELECTOR_GUESTSTART;
    }
}

/// Configuration structure to setup the Mii Selector applet.
#[doc(alias = "MiiSelectorConf")]
#[derive(Clone, Debug)]
pub struct MiiSelector {
    config: Box<ctru_sys::MiiSelectorConf>,
}

/// Return value of a successful [`MiiSelector::launch()`].
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Selection {
    /// Data of the selected Mii.
    pub mii_data: Mii,
    /// Type of the selected Mii.
    pub mii_type: MiiType,
}

/// Error returned by an unsuccessful [`MiiSelector::launch()`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The selected Mii's data is corrupt.
    InvalidChecksum,
    /// Either the user cancelled the selection (see [`Options::ENABLE_CANCEL`]) or no valid Miis were available to select.
    NoMiiSelected,
}

impl MiiSelector {
    /// Initialize a new configuration for the Mii Selector applet.
    #[doc(alias = "miiSelectorInit")]
    pub fn new() -> Self {
        let mut config = Box::<ctru_sys::MiiSelectorConf>::default();
        unsafe {
            ctru_sys::miiSelectorInit(config.as_mut());
        }
        Self { config }
    }

    /// Set the title of the Mii Selector window.
    ///
    /// # Panics
    /// This function will panic if the given `&str` contains NUL bytes.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// use ctru::applets::mii_selector::MiiSelector;
    ///
    /// let mut mii_selector = MiiSelector::new();
    /// mii_selector.set_title("Select a Mii!");
    /// # }
    /// ```
    #[doc(alias = "miiSelectorSetTitle")]
    pub fn set_title(&mut self, text: &str) {
        // This can only fail if the text contains NUL bytes in the string... which seems
        // unlikely and is documented
        let c_text = CString::new(text).expect("Failed to convert the title text into a CString");
        unsafe {
            ctru_sys::miiSelectorSetTitle(self.config.as_mut(), c_text.as_ptr());
        }
    }

    /// Set the options of the Mii Selector.
    ///
    /// This will overwrite any previously saved options. Use bitwise operations to set all your wanted options at once.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// use ctru::applets::mii_selector::{MiiSelector, Options};
    /// let mut mii_selector = MiiSelector::new();
    ///
    /// // Setup a `MiiSelector` that can be cancelled and that makes Guest Miis available to select.
    /// let opts = Options::ENABLE_CANCEL & Options::ENABLE_GUESTS;
    /// mii_selector.set_options(opts);
    /// # }
    /// ```
    #[doc(alias = "miiSelectorSetOptions")]
    pub fn set_options(&mut self, options: Options) {
        unsafe { ctru_sys::miiSelectorSetOptions(self.config.as_mut(), options.bits()) }
    }

    /// Allowlist a guest Mii based on its index.
    ///
    /// # Notes
    ///
    /// Guest Mii's won't be available regardless of their allowlist/blocklist state if the [`MiiSelector`] is run without setting [`Options::ENABLE_GUESTS`].
    /// Look into [`MiiSelector::set_options()`] to see how to work with options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// #
    /// use ctru::applets::mii_selector::{Index, MiiSelector};
    /// let mut mii_selector = MiiSelector::new();
    ///
    /// // Allowlist the guest Mii at index 2.
    /// mii_selector.allowlist_guest_mii(Index::Index(2));
    /// # }
    /// ```
    #[doc(alias = "miiSelectorWhitelistGuestMii")]
    pub fn allowlist_guest_mii(&mut self, mii_index: Index) {
        let index = match mii_index {
            Index::Index(i) => i,
            Index::All => ctru_sys::MIISELECTOR_GUESTMII_SLOTS,
        };

        unsafe { ctru_sys::miiSelectorWhitelistGuestMii(self.config.as_mut(), index) }
    }

    /// Blocklist a guest Mii based on its index.
    ///
    /// # Notes
    ///
    /// Guest Mii's won't be available regardless of their allowlist/blocklist state if the [`MiiSelector`] is run without setting [`Options::ENABLE_GUESTS`].
    /// Look into [`MiiSelector::set_options()`] to see how to work with options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// #
    /// use ctru::applets::mii_selector::{Index, MiiSelector};
    /// let mut mii_selector = MiiSelector::new();
    ///
    /// // Blocklist the guest Mii at index 1 so that it cannot be selected.
    /// mii_selector.blocklist_guest_mii(Index::Index(1));
    /// # }
    /// ```
    #[doc(alias = "miiSelectorBlacklistGuestMii")]
    pub fn blocklist_guest_mii(&mut self, mii_index: Index) {
        let index = match mii_index {
            Index::Index(i) => i,
            Index::All => ctru_sys::MIISELECTOR_GUESTMII_SLOTS,
        };

        unsafe { ctru_sys::miiSelectorBlacklistGuestMii(self.config.as_mut(), index) }
    }

    /// Allowlist a user-created Mii based on its index.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// #
    /// use ctru::applets::mii_selector::{Index, MiiSelector};
    /// let mut mii_selector = MiiSelector::new();
    ///
    /// // Allowlist the user-created Mii at index 0.
    /// mii_selector.allowlist_user_mii(Index::Index(0));
    /// # }
    /// ```
    #[doc(alias = "miiSelectorWhitelistUserMii")]
    pub fn allowlist_user_mii(&mut self, mii_index: Index) {
        let index = match mii_index {
            Index::Index(i) => i,
            Index::All => ctru_sys::MIISELECTOR_USERMII_SLOTS,
        };

        unsafe { ctru_sys::miiSelectorWhitelistUserMii(self.config.as_mut(), index) }
    }

    /// Blocklist a user-created Mii based on its index.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// #
    /// use ctru::applets::mii_selector::{Index, MiiSelector};
    /// let mut mii_selector = MiiSelector::new();
    ///
    /// // Blocklist all user-created Miis so that they cannot be selected.
    /// mii_selector.blocklist_user_mii(Index::All);
    /// # }
    /// ```
    #[doc(alias = "miiSelectorBlacklistUserMii")]
    pub fn blocklist_user_mii(&mut self, mii_index: Index) {
        let index = match mii_index {
            Index::Index(i) => i,
            Index::All => ctru_sys::MIISELECTOR_USERMII_SLOTS,
        };

        unsafe { ctru_sys::miiSelectorBlacklistUserMii(self.config.as_mut(), index) }
    }

    /// Set where the GUI cursor will start at.
    ///
    /// If there's no Mii at that index, the cursor will start at the Mii with the index 0.
    #[doc(alias = "miiSelectorSetInitialIndex")]
    pub fn set_initial_index(&mut self, index: usize) {
        unsafe { ctru_sys::miiSelectorSetInitialIndex(self.config.as_mut(), index as u32) };
    }

    /// Launch the Mii Selector.
    ///
    /// Depending on the configuration, the Mii Selector window will appear either
    /// on the bottom screen (default behaviour) or the top screen (see [`Options::USE_TOP_SCREEN`]).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use ctru::services::{apt::Apt, gfx::Gfx};
    /// #
    /// # let gfx = Gfx::new().unwrap();
    /// # let apt = Apt::new().unwrap();
    /// #
    /// use ctru::applets::mii_selector::{MiiSelector, Options};
    ///
    /// let mut mii_selector = MiiSelector::new();
    /// mii_selector.set_title("Select a Mii!");
    ///
    /// let opts = Options::ENABLE_CANCEL & Options::ENABLE_GUESTS;
    /// mii_selector.set_options(opts);
    ///
    /// let result = mii_selector.launch(&apt, &gfx)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "miiSelectorLaunch")]
    pub fn launch(&mut self, _apt: &Apt, _gfx: &Gfx) -> Result<Selection, Error> {
        let mut return_val = Box::<ctru_sys::MiiSelectorReturn>::default();
        unsafe { ctru_sys::miiSelectorLaunch(self.config.as_mut(), return_val.as_mut()) }

        if return_val.no_mii_selected != 0 {
            return Err(Error::NoMiiSelected);
        }

        if unsafe { ctru_sys::miiSelectorChecksumIsValid(return_val.as_mut()) } {
            Ok((*return_val).into())
        } else {
            Err(Error::InvalidChecksum)
        }
    }
}

impl Default for MiiSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChecksum => write!(f, "selected mii has invalid checksum"),
            Self::NoMiiSelected => write!(f, "no mii was selected"),
        }
    }
}

impl std::error::Error for Error {}

impl From<ctru_sys::MiiSelectorReturn> for Selection {
    fn from(ret: ctru_sys::MiiSelectorReturn) -> Self {
        let raw_mii_data = ret.mii;
        let mut guest_mii_name = ret.guest_mii_name;

        Selection {
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
