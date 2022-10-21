use crate::mii::MiiData;
use bitflags::bitflags;
use std::ffi::CString;

#[derive(Debug, Clone)]
pub enum MiiConfigIndex {
    Index(u32),
    All,
}

#[derive(Debug, Clone)]
pub enum MiiType {
    Guest { index: u32, name: String },
    User,
}

bitflags! {
    pub struct Options: u32 {
        const MII_SELECTOR_CANCEL = ctru_sys::MIISELECTOR_CANCEL;
        const MII_SELECTOR_GUESTS = ctru_sys::MIISELECTOR_GUESTS;
        const MII_SELECTOR_TOP = ctru_sys::MIISELECTOR_TOP;
        const MII_SELECTOR_GUEST_START = ctru_sys::MIISELECTOR_GUESTSTART;
    }
}

pub struct MiiSelector {
    config: Box<ctru_sys::MiiSelectorConf>,
}

pub struct MiiSelectorReturn {
    raw_return: Box<ctru_sys::MiiSelectorReturn>,
    pub mii_data: MiiData,
    pub is_mii_selected: bool,
    pub mii_type: MiiType,
    pub checksum: u16,
}

impl MiiSelector {
    pub fn init() -> Self {
        let mut config = Box::<ctru_sys::MiiSelectorConf>::default();
        unsafe {
            ctru_sys::miiSelectorInit(config.as_mut());
        }
        Self { config }
    }

    pub fn set_title(&mut self, text: &str) {
        // This can only fail if the text contains NUL bytes in the string... which seems
        // unlikely and is documented
        let c_text = CString::new(text).expect("Cstring::new failed");
        unsafe {
            ctru_sys::miiSelectorSetTitle(self.config.as_mut(), c_text.as_ptr());
        }
    }

    pub fn set_options(&mut self, options: Options) {
        unsafe { ctru_sys::miiSelectorSetOptions(self.config.as_mut(), options.bits) }
    }

    pub fn whitelist_guest_mii(&mut self, mii_index: MiiConfigIndex) {
        match mii_index {
            MiiConfigIndex::Index(i) => unsafe {
                ctru_sys::miiSelectorWhitelistGuestMii(self.config.as_mut(), i)
            },
            MiiConfigIndex::All => unsafe {
                ctru_sys::miiSelectorWhitelistGuestMii(
                    self.config.as_mut(),
                    ctru_sys::MIISELECTOR_GUESTMII_SLOTS,
                )
            },
        }
    }

    pub fn blacklist_guest_mii(&mut self, mii_index: MiiConfigIndex) {
        match mii_index {
            MiiConfigIndex::Index(i) => unsafe {
                ctru_sys::miiSelectorBlacklistGuestMii(self.config.as_mut(), i)
            },
            MiiConfigIndex::All => unsafe {
                ctru_sys::miiSelectorBlacklistGuestMii(
                    self.config.as_mut(),
                    ctru_sys::MIISELECTOR_GUESTMII_SLOTS,
                )
            },
        }
    }

    pub fn whitelist_user_mii(&mut self, mii_index: MiiConfigIndex) {
        match mii_index {
            MiiConfigIndex::Index(i) => unsafe {
                ctru_sys::miiSelectorWhitelistUserMii(self.config.as_mut(), i)
            },
            MiiConfigIndex::All => unsafe {
                ctru_sys::miiSelectorWhitelistUserMii(
                    self.config.as_mut(),
                    ctru_sys::MIISELECTOR_USERMII_SLOTS,
                )
            },
        }
    }

    pub fn blacklist_user_mii(&mut self, mii_index: MiiConfigIndex) {
        match mii_index {
            MiiConfigIndex::Index(i) => unsafe {
                ctru_sys::miiSelectorBlacklistUserMii(self.config.as_mut(), i)
            },
            MiiConfigIndex::All => unsafe {
                ctru_sys::miiSelectorBlacklistUserMii(
                    self.config.as_mut(),
                    ctru_sys::MIISELECTOR_USERMII_SLOTS,
                )
            },
        }
    }

    // This function is static inline in libctru
    // https://github.com/devkitPro/libctru/blob/af5321c78ee5c72a55b526fd2ed0d95ca1c05af9/libctru/include/3ds/applets/miiselector.h#L155
    pub fn set_initial_index(&mut self, index: u32) {
        self.config.initial_index = index
    }

    pub fn launch(&mut self) -> MiiSelectorReturn {
        let mut return_val = Box::<ctru_sys::MiiSelectorReturn>::default();
        unsafe { ctru_sys::miiSelectorLaunch(self.config.as_mut(), return_val.as_mut()) }

        return_val.into()
    }
}

impl MiiSelectorReturn {
    pub fn name(&self) -> String {
        let mut tmp = [0u8; 36];
        unsafe {
            ctru_sys::miiSelectorReturnGetName(self.raw_return.as_ref(), tmp.as_mut_ptr(), 36)
        }

        let len = unsafe { libc::strlen(tmp.as_ptr()) };
        let utf8 = unsafe { std::str::from_utf8_unchecked(&tmp[..len]) };

        utf8.to_owned()
    }

    pub fn author(&self) -> String {
        let mut tmp = [0u8; 30];
        unsafe {
            ctru_sys::miiSelectorReturnGetAuthor(self.raw_return.as_ref(), tmp.as_mut_ptr(), 30)
        }

        let len = unsafe { libc::strlen(tmp.as_ptr()) };
        let utf8 = unsafe { std::str::from_utf8_unchecked(&tmp[..len]) };

        utf8.to_owned()
    }

    pub fn valid_checksum(&self) -> bool {
        unsafe { ctru_sys::miiSelectorChecksumIsValid(self.raw_return.as_ref()) }
    }
}

impl From<Box<ctru_sys::MiiSelectorReturn>> for MiiSelectorReturn {
    fn from(ret: Box<ctru_sys::MiiSelectorReturn>) -> Self {
        let checksum = ret.checksum;
        let raw_mii_data = ret.mii._bindgen_opaque_blob;
        let no_mii_selected = ret.no_mii_selected;
        let guest_mii_index_clone = ret.guest_mii_index;
        let mut guest_mii_name = ret.guest_mii_name;

        MiiSelectorReturn {
            raw_return: ret,
            mii_data: raw_mii_data.into(),
            is_mii_selected: no_mii_selected == 0,
            mii_type: if guest_mii_index_clone != 0xFFFFFFFF {
                MiiType::Guest {
                    index: guest_mii_index_clone,
                    name: {
                        let utf16_be = &mut guest_mii_name;
                        utf16_be.reverse();
                        String::from_utf16(utf16_be.as_slice()).unwrap()
                    },
                }
            } else {
                MiiType::User
            },
            checksum,
        }
    }
}

impl From<u32> for MiiConfigIndex {
    fn from(v: u32) -> Self {
        Self::Index(v)
    }
}
