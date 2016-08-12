use core::marker::PhantomData;
use core::ptr;
use collections::Vec;

use path::Path;

use libctru::services::fs;

pub struct Fs {
    pd: PhantomData<i32>,
}

impl Fs {
    pub fn init() -> Result<Fs, i32> {
        unsafe {
            let r = fs::fsInit();
            if r < 0 {
                Err(r)
            } else {
                Ok(Fs { pd: PhantomData })
            }
        }
    }

    pub fn sdmc(&self) -> Result<Archive, i32> {
        let mut handle = 0u64;
        unsafe {
            let id = ArchiveID::Sdmc;
            let path = fs::fsMakePath(PathType::Empty.into(), ptr::null() as *const _);
            let ret = fs::FSUSER_OpenArchive(&mut handle, id.into(), path);
            if ret < 0 {
                Err(ret)
            } else {
                Ok(Archive {
                    handle: handle,
                    id: id,
                })
            }
        }
    }
}

impl Drop for Fs {
    fn drop(&mut self) {
        unsafe {
            fs::fsExit();
        }
    }
}


pub struct Archive {
    id: ArchiveID,
    handle: u64,
}

impl Archive {
    pub fn open_file(&self, p: &Path) -> Result<File, i32> {
        unsafe {
            let mut handle: u32 = 0;
            let wide = p.as_os_str().encode_wide().collect::<Vec<_>>();
            let path = fs::fsMakePath(PathType::UTF16.into(), wide.as_slice().as_ptr() as *mut _);
            let ret = fs::FSUSER_OpenFile(&mut handle, self.handle, path, fs::FS_OPEN_READ, 0);
            if ret < 0 {
                Err(ret)
            } else {
                Ok(File { handle: handle })
            }
        }
    }

    pub fn id(&self) -> ArchiveID {
        self.id
    }
}

impl Drop for Archive {
    fn drop(&mut self) {
        unsafe {
            fs::FSUSER_CloseArchive(self.handle);
        }
    }
}

pub struct File {
    handle: u32,
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            fs::FSFILE_Close(self.handle);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PathType {
    Invalid,
    Empty,
    Binary,
    ASCII,
    UTF16,
}

impl From<PathType> for fs::FS_PathType {
    fn from(p: PathType) -> Self {
        use self::PathType::*;
        use libctru::services::fs::FS_PathType::*;
        match p {
            Invalid => PATH_INVALID,
            Empty => PATH_EMPTY,
            Binary => PATH_BINARY,
            ASCII => PATH_ASCII,
            UTF16 => PATH_UTF16,
        }
    }
}

impl From<fs::FS_PathType> for PathType {
    fn from(f: fs::FS_PathType) -> Self {
        use self::PathType::*;
        use libctru::services::fs::FS_PathType::*;
        match f {
            PATH_INVALID => Invalid,
            PATH_EMPTY => Empty,
            PATH_BINARY => Binary,
            PATH_ASCII => ASCII,
            PATH_UTF16 => UTF16,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ArchiveID {
    RomFS,
    Savedata,
    Extdata,
    SharedExtdata,
    SystemSavedata,
    Sdmc,
    SdmcWriteOnly,
    BossExtdata,
    CardSpiFS,
    ExtDataAndBossExtdata,
    SystemSaveData2,
    NandRW,
    NandRO,
    NandROWriteAccess,
    SaveDataAndContent,
    SaveDataAndContent2,
    NandCtrFS,
    TwlPhoto,
    NandTwlFS,
    GameCardSavedata,
    UserSavedata,
    DemoSavedata,
}

impl From<ArchiveID> for fs::FS_ArchiveID {
    fn from(a: ArchiveID) -> Self {
        use self::ArchiveID::*;
        use libctru::services::fs::FS_ArchiveID::*;
        match a {
            RomFS => ARCHIVE_ROMFS,
            Savedata => ARCHIVE_SAVEDATA,
            Extdata => ARCHIVE_EXTDATA,
            SharedExtdata => ARCHIVE_SHARED_EXTDATA,
            SystemSavedata => ARCHIVE_SYSTEM_SAVEDATA,
            Sdmc => ARCHIVE_SDMC,
            SdmcWriteOnly => ARCHIVE_SDMC_WRITE_ONLY,
            BossExtdata => ARCHIVE_BOSS_EXTDATA,
            CardSpiFS => ARCHIVE_CARD_SPIFS,
            ExtDataAndBossExtdata => ARCHIVE_EXTDATA_AND_BOSS_EXTDATA, 
            SystemSaveData2 => ARCHIVE_SYSTEM_SAVEDATA2,
            NandRW => ARCHIVE_NAND_RW,
            NandRO => ARCHIVE_NAND_RO,
            NandROWriteAccess => ARCHIVE_NAND_RO_WRITE_ACCESS,
            SaveDataAndContent => ARCHIVE_SAVEDATA_AND_CONTENT,
            SaveDataAndContent2 => ARCHIVE_SAVEDATA_AND_CONTENT2,
            NandCtrFS => ARCHIVE_NAND_CTR_FS,
            TwlPhoto => ARCHIVE_TWL_PHOTO,
            NandTwlFS => ARCHIVE_NAND_TWL_FS,
            GameCardSavedata => ARCHIVE_GAMECARD_SAVEDATA,
            UserSavedata => ARCHIVE_USER_SAVEDATA,
            DemoSavedata => ARCHIVE_DEMO_SAVEDATA,
        }
    }
}
