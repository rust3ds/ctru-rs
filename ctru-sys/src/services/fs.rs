// TODO: Determine if anonymous enums are properly represented (they probably aren't)

use ::{Handle, Result};
use ::libc::c_void;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed1 {
    FS_OPEN_READ = 1,
    FS_OPEN_WRITE = 2,
    FS_OPEN_CREATE = 4,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed2 {
    FS_WRITE_FLUSH = 1,
    FS_WRITE_UPDATE_TIME = 256,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed3 {
    FS_ATTRIBUTE_DIRECTORY = 1,
    FS_ATTRIBUTE_HIDDEN = 256,
    FS_ATTRIBUTE_ARCHIVE = 65536,
    FS_ATTRIBUTE_READ_ONLY = 16777216,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed4 {
    MEDIATYPE_NAND = 0,
    MEDIATYPE_SD = 1,
    MEDIATYPE_GAME_CARD = 2,
}

pub type FS_MediaType = Enum_Unnamed4;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed5 {
    ARCHIVE_ROMFS = 3,
    ARCHIVE_SAVEDATA = 4,
    ARCHIVE_EXTDATA = 6,
    ARCHIVE_SHARED_EXTDATA = 7,
    ARCHIVE_SYSTEM_SAVEDATA = 8,
    ARCHIVE_SDMC = 9,
    ARCHIVE_SDMC_WRITE_ONLY = 10,
    ARCHIVE_BOSS_EXTDATA = 305419896,
    ARCHIVE_CARD_SPIFS = 305419897,
    ARCHIVE_EXTDATA_AND_BOSS_EXTDATA = 305419899,
    ARCHIVE_SYSTEM_SAVEDATA2 = 305419900,
    ARCHIVE_NAND_RW = 305419901,
    ARCHIVE_NAND_RO = 305419902,
    ARCHIVE_NAND_RO_WRITE_ACCESS = 305419903,
    ARCHIVE_SAVEDATA_AND_CONTENT = 591751050,
    ARCHIVE_SAVEDATA_AND_CONTENT2 = 591751054,
    ARCHIVE_NAND_CTR_FS = 1450741931,
    ARCHIVE_TWL_PHOTO = 1450741932,
    ARCHIVE_NAND_TWL_FS = 1450741934,
    ARCHIVE_NAND_W_FS = 1450741935,
    ARCHIVE_GAMECARD_SAVEDATA = 1450741937,
    ARCHIVE_USER_SAVEDATA = 1450741938,
    ARCHIVE_DEMO_SAVEDATA = 1450741940,
}
pub type FS_ArchiveID = Enum_Unnamed5;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed6 {
    PATH_INVALID = 0,
    PATH_EMPTY = 1,
    PATH_BINARY = 2,
    PATH_ASCII = 3,
    PATH_UTF16 = 4,
}
pub type FS_PathType = Enum_Unnamed6;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed7 { SECUREVALUE_SLOT_SD = 4096, }
pub type FS_SecureValueSlot = Enum_Unnamed7;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed8 {
    BAUDRATE_512KHZ = 0,
    BAUDRATE_1MHZ = 1,
    BAUDRATE_2MHZ = 2,
    BAUDRATE_4MHZ = 3,
    BAUDRATE_8MHZ = 4,
    BAUDRATE_16MHZ = 5,
}
pub type FS_CardSpiBaudRate = Enum_Unnamed8;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed9 {
    BUSMODE_1BIT = 0,
    BUSMODE_4BIT = 1,
}
pub type FS_CardSpiBusMode = Enum_Unnamed9;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed10 {
    SPECIALCONTENT_UPDATE = 1,
    SPECIALCONTENT_MANUAL = 2,
    SPECIALCONTENT_DLP_CHILD = 3,
}
pub type FS_SpecialContentType = Enum_Unnamed10;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed11 { CARD_CTR = 0, CARD_TWL = 1, }
pub type FS_CardType = Enum_Unnamed11;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed12 { FS_ACTION_UNKNOWN = 0, }
pub type FS_Action = Enum_Unnamed12;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed13 {
    ARCHIVE_ACTION_COMMIT_SAVE_DATA = 0,
    ARCHIVE_ACTION_GET_TIMESTAMP = 1,
}
pub type FS_ArchiveAction = Enum_Unnamed13;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed14 {
    SECURESAVE_ACTION_DELETE = 0,
    SECURESAVE_ACTION_FORMAT = 1,
}
pub type FS_SecureSaveAction = Enum_Unnamed14;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed15 { FILE_ACTION_UNKNOWN = 0, }
pub type FS_FileAction = Enum_Unnamed15;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed16 { DIRECTORY_ACTION_UNKNOWN = 0, }
pub type FS_DirectoryAction = Enum_Unnamed16;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed17 {
    pub name: [u16; 262usize],
    pub shortName: [u8; 10usize],
    pub shortExt: [u8; 4usize],
    pub valid: u8,
    pub reserved: u8,
    pub attributes: u32,
    pub fileSize: u64,
}
impl ::core::clone::Clone for Struct_Unnamed17 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed17 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type FS_DirectoryEntry = Struct_Unnamed17;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed18 {
    pub sectorSize: u32,
    pub clusterSize: u32,
    pub totalClusters: u32,
    pub freeClusters: u32,
}
impl ::core::clone::Clone for Struct_Unnamed18 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed18 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type FS_ArchiveResource = Struct_Unnamed18;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed19 {
    pub programId: u64,
    pub _bindgen_bitfield_1_: FS_MediaType,
    pub padding: [u8; 7usize],
}
impl ::core::clone::Clone for Struct_Unnamed19 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed19 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type FS_ProgramInfo = Struct_Unnamed19;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed20 {
    pub productCode: [u8; 16usize],
    pub companyCode: [u8; 2usize],
    pub remasterVersion: u16,
}
impl ::core::clone::Clone for Struct_Unnamed20 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed20 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type FS_ProductInfo = Struct_Unnamed20;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed21 {
    pub aesCbcMac: [u8; 16usize],
    pub movableSed: [u8; 288usize],
}
impl ::core::clone::Clone for Struct_Unnamed21 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed21 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type FS_IntegrityVerificationSeed = Struct_Unnamed21;

#[repr(C, packed)]
#[derive(Copy)]
pub struct Struct_Unnamed22 {
    pub _bindgen_bitfield_1_: FS_MediaType,
    pub unknown: u8,
    pub reserved1: u16,
    pub saveId: u64,
    pub reserved2: u32,
}
impl ::core::clone::Clone for Struct_Unnamed22 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed22 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type FS_ExtSaveDataInfo = Struct_Unnamed22;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed23 {
    pub _bindgen_bitfield_1_: FS_MediaType,
    pub unknown: u8,
    pub reserved: u16,
    pub saveId: u32,
}
impl ::core::clone::Clone for Struct_Unnamed23 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed23 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type FS_SystemSaveDataInfo = Struct_Unnamed23;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed24 {
    pub ivs: [u8; 16usize],
    pub encryptParameter: [u8; 16usize],
}
impl ::core::clone::Clone for Struct_Unnamed24 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed24 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type FS_DeviceMoveContext = Struct_Unnamed24;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed25 {
    pub _type: FS_PathType,
    pub size: u32,
    pub data: *const c_void,
}
impl ::core::clone::Clone for Struct_Unnamed25 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed25 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type FS_Path = Struct_Unnamed25;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed26 {
    pub id: u32,
    pub lowPath: FS_Path,
    pub handle: u64,
}
impl ::core::clone::Clone for Struct_Unnamed26 {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for Struct_Unnamed26 {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type FS_Archive = Struct_Unnamed26;
extern "C" {
    pub fn fsInit() -> Result;
    pub fn fsExit();
    pub fn fsUseSession(session: Handle, sdmc: u8);
    pub fn fsEndUseSession();
    pub fn fsMakePath(_type: FS_PathType, path: *const c_void)
     -> FS_Path;
    pub fn fsGetSessionHandle() -> *mut Handle;
    pub fn FSUSER_Control(action: FS_Action,
                          input: *mut c_void, inputSize: u32,
                          output: *mut c_void,
                          outputSize: u32) -> Result;
    pub fn FSUSER_Initialize(session: Handle) -> Result;
    pub fn FSUSER_OpenFile(out: *mut Handle, archive: FS_Archive,
                           path: FS_Path, openFlags: u32, attributes: u32)
     -> Result;
    pub fn FSUSER_OpenFileDirectly(out: *mut Handle, archive: FS_Archive,
                                   path: FS_Path, openFlags: u32,
                                   attributes: u32) -> Result;
    pub fn FSUSER_DeleteFile(archive: FS_Archive, path: FS_Path) -> Result;
    pub fn FSUSER_RenameFile(srcArchive: FS_Archive, srcPath: FS_Path,
                             dstArchive: FS_Archive, dstPath: FS_Path)
     -> Result;
    pub fn FSUSER_DeleteDirectory(archive: FS_Archive, path: FS_Path)
     -> Result;
    pub fn FSUSER_DeleteDirectoryRecursively(archive: FS_Archive,
                                             path: FS_Path) -> Result;
    pub fn FSUSER_CreateFile(archive: FS_Archive, path: FS_Path,
                             attributes: u32, fileSize: u64) -> Result;
    pub fn FSUSER_CreateDirectory(archive: FS_Archive, path: FS_Path,
                                  attributes: u32) -> Result;
    pub fn FSUSER_RenameDirectory(srcArchive: FS_Archive, srcPath: FS_Path,
                                  dstArchive: FS_Archive, dstPath: FS_Path)
     -> Result;
    pub fn FSUSER_OpenDirectory(out: *mut Handle, archive: FS_Archive,
                                path: FS_Path) -> Result;
    pub fn FSUSER_OpenArchive(archive: *mut FS_Archive) -> Result;
    pub fn FSUSER_ControlArchive(archive: FS_Archive,
                                 action: FS_ArchiveAction,
                                 input: *mut c_void,
                                 inputSize: u32,
                                 output: *mut c_void,
                                 outputSize: u32) -> Result;
    pub fn FSUSER_CloseArchive(archive: *mut FS_Archive) -> Result;
    pub fn FSUSER_GetFreeBytes(freeBytes: *mut u64, archive: FS_Archive)
     -> Result;
    pub fn FSUSER_GetCardType(_type: *mut FS_CardType) -> Result;
    pub fn FSUSER_GetSdmcArchiveResource(archiveResource:
                                             *mut FS_ArchiveResource)
     -> Result;
    pub fn FSUSER_GetNandArchiveResource(archiveResource:
                                             *mut FS_ArchiveResource)
     -> Result;
    pub fn FSUSER_GetSdmcFatfsError(error: *mut u32) -> Result;
    pub fn FSUSER_IsSdmcDetected(detected: *mut u8) -> Result;
    pub fn FSUSER_IsSdmcWritable(writable: *mut u8) -> Result;
    pub fn FSUSER_GetSdmcCid(out: *mut u8, length: u32) -> Result;
    pub fn FSUSER_GetNandCid(out: *mut u8, length: u32) -> Result;
    pub fn FSUSER_GetSdmcSpeedInfo(speedInfo: *mut u32) -> Result;
    pub fn FSUSER_GetNandSpeedInfo(speedInfo: *mut u32) -> Result;
    pub fn FSUSER_GetSdmcLog(out: *mut u8, length: u32) -> Result;
    pub fn FSUSER_GetNandLog(out: *mut u8, length: u32) -> Result;
    pub fn FSUSER_ClearSdmcLog() -> Result;
    pub fn FSUSER_ClearNandLog() -> Result;
    pub fn FSUSER_CardSlotIsInserted(inserted: *mut u8) -> Result;
    pub fn FSUSER_CardSlotPowerOn(status: *mut u8) -> Result;
    pub fn FSUSER_CardSlotPowerOff(status: *mut u8) -> Result;
    pub fn FSUSER_CardSlotGetCardIFPowerStatus(status: *mut u8) -> Result;
    pub fn FSUSER_CardNorDirectCommand(commandId: u8) -> Result;
    pub fn FSUSER_CardNorDirectCommandWithAddress(commandId: u8,
                                                  address: u32) -> Result;
    pub fn FSUSER_CardNorDirectRead(commandId: u8, size: u32,
                                    output: *mut u8) -> Result;
    pub fn FSUSER_CardNorDirectReadWithAddress(commandId: u8, address: u32,
                                               size: u32, output: *mut u8)
     -> Result;
    pub fn FSUSER_CardNorDirectWrite(commandId: u8, size: u32,
                                     input: *mut u8) -> Result;
    pub fn FSUSER_CardNorDirectWriteWithAddress(commandId: u8, address: u32,
                                                size: u32, input: *mut u8)
     -> Result;
    pub fn FSUSER_CardNorDirectRead_4xIO(commandId: u8, address: u32,
                                         size: u32, output: *mut u8)
     -> Result;
    pub fn FSUSER_CardNorDirectCpuWriteWithoutVerify(address: u32,
                                                     size: u32,
                                                     input: *mut u8)
     -> Result;
    pub fn FSUSER_CardNorDirectSectorEraseWithoutVerify(address: u32)
     -> Result;
    pub fn FSUSER_GetProductInfo(info: *mut FS_ProductInfo, processId: u32)
     -> Result;
    pub fn FSUSER_GetProgramLaunchInfo(info: *mut FS_ProgramInfo,
                                       processId: u32) -> Result;
    pub fn FSUSER_SetCardSpiBaudRate(baudRate: FS_CardSpiBaudRate) -> Result;
    pub fn FSUSER_SetCardSpiBusMode(busMode: FS_CardSpiBusMode) -> Result;
    pub fn FSUSER_SendInitializeInfoTo9() -> Result;
    pub fn FSUSER_GetSpecialContentIndex(index: *mut u16,
                                         mediaType: FS_MediaType,
                                         programId: u64,
                                         _type: FS_SpecialContentType)
     -> Result;
    pub fn FSUSER_GetLegacyRomHeader(mediaType: FS_MediaType, programId: u64,
                                     header: *mut u8) -> Result;
    pub fn FSUSER_GetLegacyBannerData(mediaType: FS_MediaType, programId: u64,
                                      banner: *mut u8) -> Result;
    pub fn FSUSER_CheckAuthorityToAccessExtSaveData(access: *mut u8,
                                                    mediaType: FS_MediaType,
                                                    saveId: u64,
                                                    processId: u32)
     -> Result;
    pub fn FSUSER_QueryTotalQuotaSize(quotaSize: *mut u64, directories: u32,
                                      files: u32, fileSizeCount: u32,
                                      fileSizes: *mut u64) -> Result;
    pub fn FSUSER_AbnegateAccessRight(accessRight: u32) -> Result;
    pub fn FSUSER_DeleteSdmcRoot() -> Result;
    pub fn FSUSER_DeleteAllExtSaveDataOnNand() -> Result;
    pub fn FSUSER_InitializeCtrFileSystem() -> Result;
    pub fn FSUSER_CreateSeed() -> Result;
    pub fn FSUSER_GetFormatInfo(totalSize: *mut u32, directories: *mut u32,
                                files: *mut u32, duplicateData: *mut u8,
                                archiveId: FS_ArchiveID, path: FS_Path)
     -> Result;
    pub fn FSUSER_GetLegacyRomHeader2(headerSize: u32,
                                      mediaType: FS_MediaType, programId: u64,
                                      header: *mut u8) -> Result;
    pub fn FSUSER_GetSdmcCtrRootPath(out: *mut u8, length: u32) -> Result;
    pub fn FSUSER_GetArchiveResource(archiveResource: *mut FS_ArchiveResource,
                                     mediaType: FS_MediaType) -> Result;
    pub fn FSUSER_ExportIntegrityVerificationSeed(seed:
                                                      *mut FS_IntegrityVerificationSeed)
     -> Result;
    pub fn FSUSER_ImportIntegrityVerificationSeed(seed:
                                                      *mut FS_IntegrityVerificationSeed)
     -> Result;
    pub fn FSUSER_FormatSaveData(archiveId: FS_ArchiveID, path: FS_Path,
                                 blocks: u32, directories: u32, files: u32,
                                 directoryBuckets: u32, fileBuckets: u32,
                                 duplicateData: u8) -> Result;
    pub fn FSUSER_GetLegacySubBannerData(bannerSize: u32,
                                         mediaType: FS_MediaType,
                                         programId: u64, banner: *mut u8)
     -> Result;
    pub fn FSUSER_ReadSpecialFile(bytesRead: *mut u32, fileOffset: u64,
                                  size: u32, data: *mut u8) -> Result;
    pub fn FSUSER_GetSpecialFileSize(fileSize: *mut u64) -> Result;
    pub fn FSUSER_CreateExtSaveData(info: FS_ExtSaveDataInfo,
                                    directories: u32, files: u32,
                                    sizeLimit: u64, smdhSize: u32,
                                    smdh: *mut u8) -> Result;
    pub fn FSUSER_DeleteExtSaveData(info: FS_ExtSaveDataInfo) -> Result;
    pub fn FSUSER_ReadExtSaveDataIcon(bytesRead: *mut u32,
                                      info: FS_ExtSaveDataInfo,
                                      smdhSize: u32, smdh: *mut u8)
     -> Result;
    pub fn FSUSER_GetExtDataBlockSize(totalBlocks: *mut u64,
                                      freeBlocks: *mut u64,
                                      blockSize: *mut u32,
                                      info: FS_ExtSaveDataInfo) -> Result;
    pub fn FSUSER_EnumerateExtSaveData(idsWritten: *mut u32, idsSize: u32,
                                       mediaType: FS_MediaType, idSize: u32,
                                       shared: u8, ids: *mut u8) -> Result;
    pub fn FSUSER_CreateSystemSaveData(info: FS_SystemSaveDataInfo,
                                       totalSize: u32, blockSize: u32,
                                       directories: u32, files: u32,
                                       directoryBuckets: u32,
                                       fileBuckets: u32, duplicateData: u8)
     -> Result;
    pub fn FSUSER_DeleteSystemSaveData(info: FS_SystemSaveDataInfo) -> Result;
    pub fn FSUSER_StartDeviceMoveAsSource(context: *mut FS_DeviceMoveContext)
     -> Result;
    pub fn FSUSER_StartDeviceMoveAsDestination(context: FS_DeviceMoveContext,
                                               clear: u8) -> Result;
    pub fn FSUSER_SetArchivePriority(archive: FS_Archive, priority: u32)
     -> Result;
    pub fn FSUSER_GetArchivePriority(priority: *mut u32, archive: FS_Archive)
     -> Result;
    pub fn FSUSER_SetCtrCardLatencyParameter(latency: u64,
                                             emulateEndurance: u8) -> Result;
    pub fn FSUSER_SwitchCleanupInvalidSaveData(enable: u8) -> Result;
    pub fn FSUSER_EnumerateSystemSaveData(idsWritten: *mut u32,
                                          idsSize: u32, ids: *mut u64)
     -> Result;
    pub fn FSUSER_InitializeWithSdkVersion(session: Handle, version: u32)
     -> Result;
    pub fn FSUSER_SetPriority(priority: u32) -> Result;
    pub fn FSUSER_GetPriority(priority: *mut u32) -> Result;
    pub fn FSUSER_SetSaveDataSecureValue(value: u64, slot: FS_SecureValueSlot,
                                         titleUniqueId: u32,
                                         titleVariation: u8) -> Result;
    pub fn FSUSER_GetSaveDataSecureValue(exists: *mut u8, value: *mut u64,
                                         slot: FS_SecureValueSlot,
                                         titleUniqueId: u32,
                                         titleVariation: u8) -> Result;
    pub fn FSUSER_ControlSecureSave(action: FS_SecureSaveAction,
                                    input: *mut c_void,
                                    inputSize: u32,
                                    output: *mut c_void,
                                    outputSize: u32) -> Result;
    pub fn FSUSER_GetMediaType(mediaType: *mut FS_MediaType) -> Result;
    pub fn FSFILE_Control(handle: Handle, action: FS_FileAction,
                          input: *mut c_void, inputSize: u32,
                          output: *mut c_void,
                          outputSize: u32) -> Result;
    pub fn FSFILE_OpenSubFile(handle: Handle, subFile: *mut Handle,
                              offset: u64, size: u64) -> Result;
    pub fn FSFILE_Read(handle: Handle, bytesRead: *mut u32, offset: u64,
                       buffer: *mut c_void, size: u32)
     -> Result;
    pub fn FSFILE_Write(handle: Handle, bytesWritten: *mut u32, offset: u64,
                        buffer: *const c_void, size: u32,
                        flags: u32) -> Result;
    pub fn FSFILE_GetSize(handle: Handle, size: *mut u64) -> Result;
    pub fn FSFILE_SetSize(handle: Handle, size: u64) -> Result;
    pub fn FSFILE_GetAttributes(handle: Handle, attributes: *mut u32)
     -> Result;
    pub fn FSFILE_SetAttributes(handle: Handle, attributes: u32) -> Result;
    pub fn FSFILE_Close(handle: Handle) -> Result;
    pub fn FSFILE_Flush(handle: Handle) -> Result;
    pub fn FSFILE_SetPriority(handle: Handle, priority: u32) -> Result;
    pub fn FSFILE_GetPriority(handle: Handle, priority: *mut u32) -> Result;
    pub fn FSFILE_OpenLinkFile(handle: Handle, linkFile: *mut Handle)
     -> Result;
    pub fn FSDIR_Control(handle: Handle, action: FS_DirectoryAction,
                         input: *mut c_void, inputSize: u32,
                         output: *mut c_void,
                         outputSize: u32) -> Result;
    pub fn FSDIR_Read(handle: Handle, entriesRead: *mut u32,
                      entryCount: u32, entries: *mut FS_DirectoryEntry)
     -> Result;
    pub fn FSDIR_Close(handle: Handle) -> Result;
    pub fn FSDIR_SetPriority(handle: Handle, priority: u32) -> Result;
    pub fn FSDIR_GetPriority(handle: Handle, priority: *mut u32) -> Result;
}
