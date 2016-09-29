//! Filesystem service
//!
//! This module contains basic methods to manipulate the contents of the 3DS's filesystem.
//! Only the SD card is currently supported.


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

/// Represents the filesystem service. No file IO can be performed
/// until an instance of this struct is created.
///
/// The service exits when this struct goes out of scope. 
pub struct Fs {
    pd: PhantomData<i32>,
}

/// Handle to an open filesystem archive.
///
/// Archives are automatically closed when they go out of scope.
///
/// # Examples
///
/// ```no_run
/// use ctru::services::fs::Fs
///
/// let fs = Fs::init().unwrap();
/// let sdmc_archive = fs.sdmc().unwrap();
/// ```
pub struct Archive {
    id: ArchiveID,
    handle: u64,
}

/// A reference to an open file on the filesystem.
///
/// An instance of a `File` can be read and/or written to depending
/// on what options It was opened with.
///
/// Files are automatically closed when they go out of scope.
pub struct File {
    handle: u32,
    offset: u64,
}

/// Metadata information about a file.
///
/// This structure is returned from the [`metadata`] function and
/// represents known metadata about a file.
///
/// [`metadata`]: fn.metadata.html
pub struct Metadata {
    attributes: u32,
    size: u64,
}

/// Options and flags which can be used to configure how a [`File`] is opened.
/// This builder exposes the ability to configure how a `File` is opened
/// and what operations are permitted on the open file. The [`File::open`]
/// and [`File::create`] methods are aliases for commonly used options
/// using this builder.
///
/// [`File`]: struct.File.html
/// [`File::open`]: struct.File.html#method.open
/// [`File::create`]: struct.File.html#method.create
///
/// Generally speaking, when using `OpenOptions`, you'll first call [`new()`],
/// then chain calls to methods to set each option, then call [`open()`],
/// passing the path of the file you're trying to open.
///
/// It is required to also pass a reference to the [`Archive`] that the
/// file lives in.
///
/// [`new()`]: struct.OpenOptions.html#method.new
/// [`open()`]: struct.OpenOptions.html#method.open
/// [`Archive`]: struct.Archive.html
///
/// # Examples
///
/// Opening a file to read:
///
/// ```no_run
/// use ctru::services::fs::OpenOptions;
///
/// let fs = Fs::init().unwrap();
/// let sdmc_archive = fs.sdmc().unwrap();
/// let file = OpenOptions::new()
///             .read(true)
///             .archive(&sdmc_archive)
///             .open("foo.txt")
///             .unwrap();
/// ```
///
/// Opening a file for both reading and writing, as well as creating it if it
/// doesn't exist:
///
/// ```no_run
/// use ctru::services::fs::OpenOptions;
///
/// let fs = Fs::init().unwrap();
/// let sdmc_archive = fs.sdmc().unwrap();
/// let file = OpenOptions::new()
///             .read(true)
///             .write(true)
///             .create(true)
///             .archive(&sdmc_archive)
///             .open("foo.txt")
///             .unwrap();
/// ```
#[derive(Clone)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    create: bool,
    arch_handle: u64,
}

/// Iterator over the entries in a directory.
///
/// This iterator is returned from the [`read_dir`] function of this module and
/// will yield instances of `Result<DirEntry, i32>`. Through a [`DirEntry`]
/// information like the entry's path and possibly other metadata can be
/// learned.
///
/// [`read_dir`]: fn.read_dir.html
/// [`DirEntry`]: struct.DirEntry.html
///
/// # Errors
///
/// This Result will return Err if there's some sort of intermittent IO error
/// during iteration.
pub struct ReadDir<'a> {
    handle: Dir,
    root: Arc<PathBuf>,
    arch: &'a Archive,
}

/// Entries returned by the [`ReadDir`] iterator.
///
/// [`ReadDir`]: struct.ReadDir.html
///
/// An instance of `DirEntry` represents an entry inside of a directory on the
/// filesystem. Each entry can be inspected via methods to learn about the full
/// path or possibly other metadata.
pub struct DirEntry<'a> {
    entry: FS_DirectoryEntry,
    root: Arc<PathBuf>,
    arch: &'a Archive,
}

#[doc(hidden)]
struct Dir(u32);

#[doc(hidden)]
unsafe impl Send for Dir {}
#[doc(hidden)]
unsafe impl Sync for Dir {}

impl Fs {
    /// Initializes the FS service.
    ///
    /// # Errors
    ///
    /// This function will return Err if there was an error initializing the
    /// FS service. This typically reflects a problem with the execution
    /// environment and not necessarily your program itself.
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

    /// Returns a handle to the SDMC (memory card) Archive.
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
    /// Retrieves an Archive's [`ArchiveID`]
    ///
    /// [`ArchiveID`]: enum.ArchiveID.html
    pub fn get_id(&self) -> ArchiveID {
        self.id
    }
}

impl File {
    /// Attempts to open a file in read-only mode.
    ///
    /// See the [`OpenOptions::open`] method for more details.
    ///
    /// # Errors
    ///
    /// This function will return an error if `path` does not already exit.
    /// Other errors may also be returned accoridng to [`OpenOptions::open`]
    /// 
    /// [`OpenOptions::open`]: struct.OpenOptions.html#method.open
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctru::servies::fs::{Fs, File};
    ///
    /// let fs = Fs::init().unwrap()
    /// let sdmc_archive = fs.sdmc().unwrap()
    /// let mut f = File::open("/foo.txt").unwrap();
    /// ```
    pub fn open<P: AsRef<Path>>(arch: &Archive, path: P) -> Result<File, i32> {
        OpenOptions::new().read(true).archive(arch).open(path.as_ref())
    }

    /// Opens a file in write-only mode.
    ///
    /// This function will create a file if it does not exist.
    ///
    /// See the [`OpenOptions::create`] method for more details.
    ///
    /// # Errors
    ///
    /// This function will return an error if `path` does not already exit.
    /// Other errors may also be returned accoridng to [`OpenOptions::create`]
    /// 
    /// [`OpenOptions::create`]: struct.OpenOptions.html#method.create
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctru::servies::fs::{Fs, File};
    ///
    /// let fs = Fs::init().unwrap()
    /// let sdmc_archive = fs.sdmc().unwrap()
    /// let mut f = File::create("/foo.txt").unwrap();
    /// ```
    pub fn create<P: AsRef<Path>>(arch: &Archive, path: P) -> Result<File, i32> {
        OpenOptions::new().write(true).create(true).archive(arch).open(path.as_ref())
    }

    /// Truncates or extends the underlying file, updating the size of this file to become size.
    ///
    /// If the size is less than the current file's size, then the file will be shrunk. If it is
    /// greater than the current file's size, then the file will be extended to size and have all
    /// of the intermediate data filled in with 0s.
    pub fn set_len(&mut self, size: u64) -> Result<(), i32> {
        unsafe {
            let r = FSFILE_SetSize(self.handle, size);
            if r < 0 {
                Err(r)
            } else {
                Ok(())
            }
        }
    }

    /// Queries metadata about the underlying file.
    pub fn metadata(&self) -> Result<Metadata, i32> {
        // The only metadata we have for files right now is file size.
        // This is likely to change in the future.
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

    /// Pull some bytes from the file into the specified buffer, returning
    /// how many bytes were read.
    ///
    /// This function will become private when std::io support is ported
    /// to this library.
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

    /// Read all bytes until EOF in this source, placing them into buf.
    /// 
    /// This function will become private when std::io support is ported
    /// to this library.
    pub fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, i32> {
        unsafe {
            read_to_end_uninitialized(self, buf)
        }
    }

    /// Write a buffer into this object, returning how many bytes were written.
    ///
    /// This function will become private when std::io support is ported
    /// to this library.
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
    /// Returns whether this metadata is for a directory.
    pub fn is_dir(&self) -> bool {
        self.attributes == self.attributes | FS_ATTRIBUTE_DIRECTORY
    }

    /// Returns whether this metadata is for a regular file.
    pub fn is_file(&self) -> bool {
        !self.is_dir()
    }

    /// Returns the size, in bytes, this metadata is for.
    ///
    /// Directories return size = 0.
    pub fn len(&self) -> u64 {
        self.size
    }
}

impl OpenOptions {
    /// Creates a blank set of options ready for configuration.
    ///
    /// All options are initially set to `false`
    pub fn new() -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            create: false,
            arch_handle: 0,
        }
    }

    /// Sets the option for read access.
    ///
    /// This option, when true, will indicate that the file should be
    /// `read`-able if opened.
    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.read = read;
        self
    }

    /// Sets the option for write access.
    ///
    /// This option, when true, will indicate that the file should be
    /// `write`-able if opened.
    ///
    /// If the file already exists, any write calls on it will overwrite
    /// its contents, without truncating it.
    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.write = write;
        self
    }

    /// Sets the option for creating a new file.
    ///
    /// This option indicates whether a new file will be created
    /// if the file does not yet already
    /// exist.
    ///
    /// In order for the file to be created, write access must also be used.
    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        self.create = create;
        self
    }

    /// Sets which archive the file is to be opened in.
    ///
    /// Failing to pass in an archive will result in the file failing to open.
    pub fn archive(&mut self, archive: &Archive) -> &mut OpenOptions {
        self.arch_handle = archive.handle;
        self
    }

    /// Opens a file at `path` with the options specified by `self`
    ///
    /// # Errors
    ///
    /// This function will return an error under a number of different
    /// circumstances, including but not limited to:
    ///
    /// * Opening a file that doesn't exist without setting `create`.
    /// * Attempting to open a file without passing an [`Archive`] reference
    ///   to the `archive` method.
    /// * Filesystem-level errors (full disk, etc).
    /// * Invalid combinations of open options.
    ///
    /// [`Archive`]: struct.Archive.html
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
    /// Returns the full path to the file that this entry represents.
    ///
    /// The full path is created by joining the original path to `read_dir`
    /// with the filename of this entry.
    pub fn path(&self) -> PathBuf {
        self.root.join(&self.file_name())
    }

    /// Return the metadata for the file that this entry points at.
    pub fn metadata(&self) -> Result<Metadata, i32> {
        metadata(self.arch, self.path())
    }

    /// Return the file type for the file that this entry points at.
    pub fn file_name(&self) -> OsString {
        let filename = truncate_utf16_at_nul(&self.entry.name);
        OsString::from_wide(filename)
    }
}

/// Creates a new, empty directory at the provided path
///
/// # Errors
///
/// This function will return an error in the following situations,
/// but is not limited to just these cases:
///
/// * User lacks permissions to create directory at `path` 
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

/// Recursively create a directory and all of its parent components if they are missing.
///
/// # Errors
///
/// This function will return an error in the following situations,
/// but is not limited to just these cases:
///
/// * If any directory in the path specified by `path` does not already exist
///   and it could not be created otherwise.
pub fn create_dir_all<P: AsRef<Path>>(arch: &Archive, path: P) -> Result<(), i32> {
    let path = path.as_ref();
    let mut dir = PathBuf::new();
    let mut result = Ok(());

    for component in path.components() {
        let component = component.as_os_str();
        dir.push(component);
        result = create_dir(arch, &dir);
    }
    result
}

/// Given a path, query the file system to get information about a file, directory, etc
pub fn metadata<P: AsRef<Path>>(arch: &Archive, path: P) -> Result<Metadata, i32> {
    let maybe_file = File::open(&arch, path.as_ref());
    let maybe_dir = read_dir(&arch, path.as_ref());
    match (maybe_file, maybe_dir) {
        (Ok(file), _) => file.metadata(),
        (_, Ok(_dir)) => Ok(Metadata { attributes: FS_ATTRIBUTE_DIRECTORY, size: 0 }),
        (Err(r), _)   => Err(r),
    }
}

/// Removes an existing, empty directory.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * The user lacks permissions to remove the directory at the provided path.
/// * The directory isn't empty.
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

/// Removes a directory at this path, after removing all its contents. Use carefully!
/// 
/// # Errors
///
/// see `file::remove_file` and `fs::remove_dir`
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

/// Returns an iterator over the entries within a directory.
///
/// The iterator will yield instances of Result<DirEntry, i32>. New errors
/// may be encountered after an iterator is initially constructed.
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * The provided path doesn't exist.
/// * The process lacks permissions to view the contents.
/// * The path points at a non-directory file.
pub fn read_dir<P: AsRef<Path>>(arch: &Archive, path: P) -> Result<ReadDir, i32> {
    unsafe {
        let mut handle = 0;
        let root = Arc::new(path.as_ref().to_path_buf());
        let path = to_utf16(path.as_ref());
        let fs_path = fsMakePath(PathType::UTF16.into(), path.as_ptr() as _);
        let r = FSUSER_OpenDirectory(&mut handle, arch.handle, fs_path);
        if r < 0 {
            Err(r)
        } else {
            Ok(ReadDir { handle: Dir(handle), root: root, arch: arch})
        }
    }
}

/// Removes a file from the filesystem.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * path points to a directory.
/// * The user lacks permissions to remove the file.
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

/// Rename a file or directory to a new name, replacing the original file
/// if to already exists.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * from does not exist.
/// * The user lacks permissions to view contents.
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
