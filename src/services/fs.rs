use core::marker::PhantomData;
use core::ptr;
use collections::Vec;

use path::Path;

use libctru::services::fs::*;

#[derive(Copy, Clone, Debug)]
pub enum PathType {
    Invalid,
    Empty,
    Binary,
    ASCII,
    UTF16,
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


pub struct Fs {
    pd: PhantomData<i32>,
}

pub struct Archive {
    id: ArchiveID,
    handle: u64,
}

pub struct File {
    handle: u32,
    offset: u64,
}

pub struct OpenOptions {
    read: bool,
    write: bool,
    create: bool,
    arch_handle: u64,
}

impl Fs {
    pub fn init() -> Result<Fs, i32> {
        unsafe {
            let r = fsInit();
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
            let path = fsMakePath(PathType::Empty.into(), ptr::null() as *const _);
            let ret = FSUSER_OpenArchive(&mut handle, id.into(), path);
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

impl Archive {
    pub fn file_open(&self, path: &Path) -> Result<File, i32> {
        self.file_open_options().read(true).create(true).open(path)
    }

    pub fn file_open_options(&self) -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            create: false,
            arch_handle: self.handle,
        }
    }

    pub fn get_id(&self) -> ArchiveID {
        self.id
    }
}

impl File {
    pub fn len(&self) -> Result<u64, i32> {
        unsafe {
            let mut len = 0;
            let r = FSFILE_GetSize(self.handle, &mut len);
            if r < 0 {
                Err(r)
            } else {
                Ok(len)
            }
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<u32, i32> {
        unsafe {
            let mut n_read = 0;
            let r = FSFILE_Read(
                self.handle,
                &mut n_read,
                self.offset,
                buf.as_mut_ptr() as _,
                buf.len() as u32
            );

            self.offset += n_read as u64;

            if r < 0 {
                Err(r)
            } else {
                Ok(n_read)
            }
        }
    }
}

impl OpenOptions {
    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.read = read;
        self
    }

    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.write = write;
        self
    }

    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        self.create = create;
        self
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> Result<File, i32> {
        self._open(path.as_ref(), self.get_open_flags())
    }

    fn _open(&self, path: &Path, flags: u32) -> Result<File, i32> {
        unsafe {
            let mut file_handle: u32 = 0;
            let wide = path.as_os_str().encode_wide().collect::<Vec<_>>();
            let ctr_path = fsMakePath(PathType::UTF16.into(), wide.as_ptr() as *mut _);
            let ret = FSUSER_OpenFile(&mut file_handle, self.arch_handle, ctr_path, flags, 0);
            if ret < 0 {
                Err(ret)
            } else {
                Ok(File {
                    handle: file_handle,
                    offset: 0,
                })
            }
        }
    }

    fn get_open_flags(&self) -> u32 {
        match (self.read, self.write, self.create) {
            (true,  false, false) => FS_OPEN_READ,
            (false, true,  false) => FS_OPEN_WRITE,
            (true,  false, true)  => FS_OPEN_READ | FS_OPEN_CREATE,
            (true,  true,  false) => FS_OPEN_READ | FS_OPEN_WRITE,
            (true,  true,  true)  => FS_OPEN_READ | FS_OPEN_WRITE | FS_OPEN_CREATE,
            _ => 0, //failure case
        }
    }
}

impl Drop for Fs {
    fn drop(&mut self) {
        unsafe {
            fsExit();
        }
    }
}


impl Drop for Archive {
    fn drop(&mut self) {
        unsafe {
            FSUSER_CloseArchive(self.handle);
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            FSFILE_Close(self.handle);
        }
    }
}

impl From<PathType> for FS_PathType {
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

impl From<FS_PathType> for PathType {
    fn from(f: FS_PathType) -> Self {
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

impl From<ArchiveID> for FS_ArchiveID {
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
