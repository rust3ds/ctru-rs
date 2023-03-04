use crate::services::cfgu::Language;
use bitflags::bitflags;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Smdh {
    magic: [u8; 4],
    version: u16,
    titles: [SmdhTitle; 16],
    age_ratings: [u8; 16],
    region_lock: RegionLock,
    matchmaker_id: u32,
    matchmaker_bit_id: u64,
    flags: SmdhFlags,
    eula_version_major: u8,
    eula_version_minor: u8,
    optimal_banner_anim_frame: f32,
    streetpass_id: u32,
    _pad: [u8; 8],
    small_icon: [u8; 0x480],
    large_icon: [u8; 0x1200],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SmdhTitle {
    short: [u16; 0x40],
    long: [u16; 0x80],
    publisher: [u16; 0x40],
}

bitflags! {
    #[repr(transparent)]
    pub struct SmdhFlags: u32 {
        const VISIBLE = 0x1;
        const AUTOBOOT_GAMECARD = 0x2;
        const PARENTAL_3D_ALLOW = 0x4;
        const REQUIRE_CTR_EULA = 0x8;
        const AUTOSAVE_ON_EXIT = 0x10;
        const EXTENDED_BANNER = 0x20;
        const REGION_RATING_REQUIRED = 0x40;
        const SAVEDATA_USAGE = 0x80;
        const RECORD_USAGE = 0x100;
        const DISABLE_SD_SAVEDATA_BACKUPS = 0x400;
        const NEW3DS_EXCLUSIVE = 0x1000;
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct RegionLock: u32 {
        const JAPAN = 0x1;
        const NORTH_AMERICA = 0x2;
        const EUROPE = 0x4;
        const AUSTRALIA = 0x8;
        const CHINA = 0x10;
        const KOREA = 0x20;
        const TAIWAN = 0x40;
        const REGION_FREE = 0x7fff_ffff;
    }
}

impl Smdh {
    pub fn short_name(&self, lang: Language) -> String {
        String::from_utf16_lossy(&self.titles[lang as usize].short)
    }

    pub fn long_name(&self, lang: Language) -> String {
        String::from_utf16_lossy(&self.titles[lang as usize].long)
    }

    pub fn publisher(&self, lang: Language) -> String {
        String::from_utf16_lossy(&self.titles[lang as usize].publisher)
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn flags(&self) -> SmdhFlags {
        self.flags
    }

    pub fn region(&self) -> RegionLock {
        self.region_lock
    }

    pub fn eula_version(&self) -> (u8, u8) {
        (self.eula_version_major, self.eula_version_minor)
    }

    pub(crate) fn magic(&self) -> [u8; 4] {
        self.magic
    }

    pub fn streetpass_id(&self) -> u32 {
        self.streetpass_id
    }
}
