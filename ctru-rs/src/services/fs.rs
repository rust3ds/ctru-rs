//! FileSystem service.
//!
//! Currently, this module contains only datatypes to easily operate with unsafe [`ctru_sys`] code regarding the file-system functionality.
#![doc(alias = "filesystem")]

use bitflags::bitflags;

bitflags! {
    #[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    struct Open: u32 {
        const FS_OPEN_READ   = ctru_sys::FS_OPEN_READ;
        const FS_OPEN_WRITE  = ctru_sys::FS_OPEN_WRITE;
        const FS_OPEN_CREATE = ctru_sys::FS_OPEN_CREATE;
    }

    #[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    struct Write: u32 {
        const FS_WRITE_FLUSH       = ctru_sys::FS_WRITE_FLUSH;
        const FS_WRITE_UPDATE_TIME = ctru_sys::FS_WRITE_UPDATE_TIME;
    }

    #[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    struct Attribute: u32 {
        const FS_ATTRIBUTE_DIRECTORY = ctru_sys::FS_ATTRIBUTE_DIRECTORY;
        const FS_ATTRIBUTE_HIDDEN    = ctru_sys::FS_ATTRIBUTE_HIDDEN;
        const FS_ATTRIBUTE_ARCHIVE   = ctru_sys::FS_ATTRIBUTE_ARCHIVE;
        const FS_ATTRIBUTE_READ_ONLY = ctru_sys::FS_ATTRIBUTE_READ_ONLY;
    }
}

/// Media type used for storage.
#[doc(alias = "FS_MediaType")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum MediaType {
    /// Internal NAND memory.
    Nand = ctru_sys::MEDIATYPE_NAND,
    /// External SD card.
    Sd = ctru_sys::MEDIATYPE_SD,
    /// Game Cartridge.
    GameCard = ctru_sys::MEDIATYPE_GAME_CARD,
}

/// Kind of file path.
#[doc(alias = "FS_PathType")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PathType {
    /// Invalid path.
    Invalid = ctru_sys::PATH_INVALID,
    /// Empty path.
    Empty = ctru_sys::PATH_EMPTY,
    /// Binary path.
    ///
    /// Its meaning differs depending on the Archive it is used on.
    Binary = ctru_sys::PATH_BINARY,
    /// ASCII path.
    ASCII = ctru_sys::PATH_ASCII,
    /// UTF-16 path.
    UTF16 = ctru_sys::PATH_UTF16,
}

/// Index of the various usable data archives.
#[doc(alias = "FS_ArchiveID")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ArchiveID {
    /// Read-Only Memory File System.
    RomFS = ctru_sys::ARCHIVE_ROMFS,
    /// Game save data.
    Savedata = ctru_sys::ARCHIVE_SAVEDATA,
    /// Game ext data.
    Extdata = ctru_sys::ARCHIVE_EXTDATA,
    /// Shared ext data.
    SharedExtdata = ctru_sys::ARCHIVE_SHARED_EXTDATA,
    /// System save data.
    SystemSavedata = ctru_sys::ARCHIVE_SYSTEM_SAVEDATA,
    /// SD card.
    Sdmc = ctru_sys::ARCHIVE_SDMC,
    /// SD card (write-only).
    SdmcWriteOnly = ctru_sys::ARCHIVE_SDMC_WRITE_ONLY,
    /// BOSS ext data.
    BossExtdata = ctru_sys::ARCHIVE_BOSS_EXTDATA,
    /// Card SPI File System.
    CardSpiFS = ctru_sys::ARCHIVE_CARD_SPIFS,
    /// Game ext data and BOSS data.
    ExtDataAndBossExtdata = ctru_sys::ARCHIVE_EXTDATA_AND_BOSS_EXTDATA,
    /// System save data.
    SystemSaveData2 = ctru_sys::ARCHIVE_SYSTEM_SAVEDATA2,
    /// Internal NAND (read-write).
    NandRW = ctru_sys::ARCHIVE_NAND_RW,
    /// Internal NAND (read-only).
    NandRO = ctru_sys::ARCHIVE_NAND_RO,
    /// Internal NAND (read-only write access).
    NandROWriteAccess = ctru_sys::ARCHIVE_NAND_RO_WRITE_ACCESS,
    /// User save data and ExeFS/RomFS.
    SaveDataAndContent = ctru_sys::ARCHIVE_SAVEDATA_AND_CONTENT,
    /// User save data and ExeFS/RomFS (only ExeFS for fs:LDR).
    SaveDataAndContent2 = ctru_sys::ARCHIVE_SAVEDATA_AND_CONTENT2,
    /// NAND CTR File System.
    NandCtrFS = ctru_sys::ARCHIVE_NAND_CTR_FS,
    /// TWL photo.
    TwlPhoto = ctru_sys::ARCHIVE_TWL_PHOTO,
    /// NAND TWL File System.
    NandTwlFS = ctru_sys::ARCHIVE_NAND_TWL_FS,
    /// Game card save data.
    GameCardSavedata = ctru_sys::ARCHIVE_GAMECARD_SAVEDATA,
    /// User save data.
    UserSavedata = ctru_sys::ARCHIVE_USER_SAVEDATA,
    /// Demo save data.
    DemoSavedata = ctru_sys::ARCHIVE_DEMO_SAVEDATA,
}

from_impl!(MediaType, ctru_sys::FS_MediaType);
from_impl!(PathType, ctru_sys::FS_PathType);
from_impl!(ArchiveID, ctru_sys::FS_ArchiveID);
