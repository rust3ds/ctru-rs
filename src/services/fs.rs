use core::marker::PhantomData;
use core::ptr;
use core::slice;
use core::mem;
use alloc::arc::Arc;
use collections::Vec;

use path::{Path, PathBuf};
use ffi::OsString;

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

pub struct Metadata {
    attributes: u32,
    size: u64,
}

#[derive(Clone)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    create: bool,
    arch_handle: u64,
}

pub struct ReadDir<'a> {
    handle: Dir,
    root: Arc<PathBuf>,
    arch: &'a Archive,
}

pub struct DirEntry<'a> {
    entry: FS_DirectoryEntry,
    root: Arc<PathBuf>,
    arch: &'a Archive,
}

struct Dir(u32);

unsafe impl Send for Dir {}
unsafe impl Sync for Dir {}

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
        unsafe {
            let mut handle = 0;
            let id = ArchiveID::Sdmc;
            let path = fsMakePath(PathType::Empty.into(), ptr::null() as _);
            let r = FSUSER_OpenArchive(&mut handle, id.into(), path);
            if r < 0 {
                Err(r)
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
    pub fn get_id(&self) -> ArchiveID {
        self.id
    }
}

impl File {
    pub fn open<P: AsRef<Path>>(arch: &Archive, path: P) -> Result<File, i32> {
        OpenOptions::new().read(true).archive(arch).open(path.as_ref())
    }

    pub fn create<P: AsRef<Path>>(arch: &Archive, path: P) -> Result<File, i32> {
        OpenOptions::new().write(true).create(true).archive(arch).open(path.as_ref())
    }

    pub fn set_len(&mut self, len: u64) -> Result<(), i32> {
        unsafe {
            let r = FSFILE_SetSize(self.handle, len);
            if r < 0 {
                Err(r)
            } else {
                Ok(())
            }
        }
    }

    // Right now the only file metadata we really have is file size
    // This will probably expand later on
    pub fn metadata(&self) -> Result<Metadata, i32> {
        unsafe {
            let mut size = 0;
            let r = FSFILE_GetSize(self.handle, &mut size);
            if r < 0 {
                Err(r)
            } else {
                Ok(Metadata { attributes: 0, size: size })
            }
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, i32> {
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
                Ok(n_read as usize)
            }
        }
    }

    pub fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, i32> {
        unsafe {
            read_to_end_uninitialized(self, buf)
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, i32> {
        unsafe {
            let mut n_written = 0;
            let r = FSFILE_Write(
                self.handle,
                &mut n_written,
                self.offset,
                buf.as_ptr() as _,
                buf.len() as u32,
                FS_WRITE_UPDATE_TIME
            );
            self.offset += n_written as u64;
            if r < 0 {
                Err(r)
            } else {
                Ok(n_written as usize)
            }
        }
    }
}

impl Metadata {
    pub fn is_dir(&self) -> bool {
        self.attributes == self.attributes | FS_ATTRIBUTE_DIRECTORY
    }

    pub fn is_file(&self) -> bool {
        !self.is_dir()
    }

    pub fn len(&self) -> u64 {
        self.size
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            create: false,
            arch_handle: 0,
        }
    }

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

    pub fn archive(&mut self, archive: &Archive) -> &mut OpenOptions {
        self.arch_handle = archive.handle;
        self
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> Result<File, i32> {
        self._open(path.as_ref(), self.get_open_flags())
    }

    fn _open(&self, path: &Path, flags: u32) -> Result<File, i32> {
        unsafe {
            let mut file_handle = 0;
            let path = to_utf16(path);
            let fs_path = fsMakePath(PathType::UTF16.into(), path.as_ptr() as _);
            let r = FSUSER_OpenFile(&mut file_handle, self.arch_handle, fs_path, flags, 0);
            if r < 0 {
                Err(r)
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
            (false, true,  true)  => FS_OPEN_WRITE | FS_OPEN_CREATE,
            (true,  false, true)  => FS_OPEN_READ | FS_OPEN_CREATE,
            (true,  true,  false) => FS_OPEN_READ | FS_OPEN_WRITE,
            (true,  true,  true)  => FS_OPEN_READ | FS_OPEN_WRITE | FS_OPEN_CREATE,
            _ => 0, //failure case
        }
    }
}

impl<'a> Iterator for ReadDir<'a> {
    type Item = Result<DirEntry<'a>, i32>;

    fn next(&mut self) -> Option<Result<DirEntry<'a>, i32>> {
        unsafe {
            let mut ret = DirEntry {
                entry: mem::zeroed(),
                root: self.root.clone(),
                arch: self.arch,
            };
            let mut entries_read = 0;
            let entry_count = 1;
            let r = FSDIR_Read(self.handle.0, &mut entries_read, entry_count, &mut ret.entry);

            if r < 0 {
                return Some(Err(r))
            }
            if entries_read != entry_count {
                return None
            }
            Some(Ok(ret))
        }
    }
}

impl<'a> DirEntry<'a> {
    pub fn path(&self) -> PathBuf {
        self.root.join(&self.file_name())
    }

    pub fn metadata(&self) -> Result<Metadata, i32> {
        metadata(self.arch, self.path())
    }

    pub fn file_name(&self) -> OsString {
        let filename = truncate_utf16_at_nul(&self.entry.name);
        OsString::from_wide(filename)
    }
}

pub fn create_dir<P: AsRef<Path>>(arch: &Archive, path: P) -> Result<(), i32> {
    unsafe {
        let path = to_utf16(path.as_ref());
        let fs_path = fsMakePath(PathType::UTF16.into(), path.as_ptr() as _);
        let r = FSUSER_CreateDirectory(arch.handle, fs_path, FS_ATTRIBUTE_DIRECTORY);
        if r < 0 {
            Err(r)
        } else {
            Ok(())
        }
    }
}

pub fn metadata<P: AsRef<Path>>(arch: &Archive, path: P) -> Result<Metadata, i32> {
    let maybe_file = File::open(&arch, path.as_ref());
    let maybe_dir = read_dir(&arch, path.as_ref());
    match (maybe_file, maybe_dir) {
        (Ok(file), _) => file.metadata(),
        (_, Ok(_dir)) => Ok(Metadata { attributes: FS_ATTRIBUTE_DIRECTORY, size: 0 }),
        (Err(r), _)   => Err(r),
    }
}

pub fn remove_dir<P: AsRef<Path>>(arch: &Archive, path: P) -> Result<(), i32> {
    unsafe {
        let path = to_utf16(path.as_ref());
        let fs_path = fsMakePath(PathType::UTF16.into(), path.as_ptr() as _);
        let r = FSUSER_DeleteDirectory(arch.handle, fs_path);
        if r < 0 {
            Err(r)
        } else {
            Ok(())
        }
    }
}

pub fn remove_dir_all<P: AsRef<Path>>(arch: &Archive, path: P) -> Result<(), i32> {
    unsafe {
        let path = to_utf16(path.as_ref());
        let fs_path = fsMakePath(PathType::UTF16.into(), path.as_ptr() as _);
        let r = FSUSER_DeleteDirectoryRecursively(arch.handle, fs_path);
        if r < 0 {
            Err(r)
        } else {
            Ok(())
        }
    }
}

pub fn read_dir<P: AsRef<Path>>(arch: &Archive, path: P) -> Result<ReadDir, i32> {
    readdir(&arch, path.as_ref())
}

pub fn remove_file<P: AsRef<Path>>(arch: &Archive, path: P) -> Result<(), i32> {
    unsafe {
        let path = to_utf16(path.as_ref());
        let fs_path = fsMakePath(PathType::UTF16.into(), path.as_ptr() as _);
        let r = FSUSER_DeleteFile(arch.handle, fs_path);
        if r < 0 {
            Err(r)
        } else {
            Ok(())
        }
    }
}

pub fn rename<P, Q>(arch: &Archive, from: P, to: Q) -> Result<(), i32>
    where P: AsRef<Path>,
          Q: AsRef<Path> {

    unsafe {
        let from = to_utf16(from.as_ref());
        let to = to_utf16(to.as_ref());

        let fs_from = fsMakePath(PathType::UTF16.into(), from.as_ptr() as _);
        let fs_to = fsMakePath(PathType::UTF16.into(), to.as_ptr() as _);

        let r = FSUSER_RenameFile(arch.handle, fs_from, arch.handle, fs_to);
        if r == 0 {
            return Ok(())
        }
        let r = FSUSER_RenameDirectory(arch.handle, fs_from, arch.handle, fs_to);
        if r == 0 {
            return Ok(())
        }
        Err((r))
    }
}

fn readdir<'a>(arch: &'a Archive, p: &Path) -> Result<ReadDir<'a>, i32> {
    unsafe {
        let mut handle = 0;
        let root = Arc::new(p.to_path_buf());
        let path = to_utf16(p);
        let fs_path = fsMakePath(PathType::UTF16.into(), path.as_ptr() as _);
        let r = FSUSER_OpenDirectory(&mut handle, arch.handle, fs_path);
        if r < 0 {
            Err(r)
        } else {
            Ok(ReadDir { handle: Dir(handle), root: root, arch: arch})
        }
    }
}

// TODO: Determine if we should check UTF-16 paths for interior NULs
fn to_utf16(path: &Path) -> Vec<u16> {
    path.as_os_str().encode_wide().collect::<Vec<_>>()
}

// Adapted from sys/windows/fs.rs in libstd
fn truncate_utf16_at_nul<'a>(v: &'a [u16]) -> &'a [u16] {
    match v.iter().position(|c| *c == 0) {
        // don't include the 0
        Some(i) => &v[..i],
        None => v
    }
}

// Adapted from sys/common/io.rs in libstd
unsafe fn read_to_end_uninitialized(f: &mut File, buf: &mut Vec<u8>) -> Result<usize, i32> {
    let start_len = buf.len();
    buf.reserve(16);

    // Always try to read into the empty space of the vector (from the length to the capacity).
    // If the vector ever fills up then we reserve an extra byte which should trigger the normal
    // reallocation routines for the vector, which will likely double the size.
    //
    // This function is similar to the read_to_end function in std::io, but the logic about
    // reservations and slicing is different enough that this is duplicated here.
    loop {
        if buf.len() == buf.capacity() {
            buf.reserve(1);
        }

        let buf_slice = slice::from_raw_parts_mut(buf.as_mut_ptr().offset(buf.len() as isize),
                                                  buf.capacity() - buf.len());

        match f.read(buf_slice) {
            Ok(0) => { return Ok(buf.len() - start_len); }
            Ok(n) => { let len = buf.len() + n; buf.set_len(len); },
            Err(e) => { return Err(e); }
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

impl Drop for Dir {
    fn drop(&mut self) {
        unsafe {
            FSDIR_Close(self.0);
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
