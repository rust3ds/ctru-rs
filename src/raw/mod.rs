#![allow(non_camel_case_types)]
#![allow(overflowing_literals)]

pub mod srv;
pub mod svc;

#[repr(C)]
pub enum mediatypes_enum {
    mediatype_NAND,
    mediatype_SDMC,
    mediatype_GAMECARD
}

pub type s8 = i8;
pub type s16 = i16;
pub type s32 = i32;
pub type s64 = i64;

#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2
}

#[repr(C)]
pub enum MemOp {
    MEMOP_FREE = 1,
    MEMOP_ALLOC = 3,
    MEMOP_MAP = 4,
    MEMOP_UNMAP = 5,
    MEMOP_PROT = 6,

    MEMOP_ALLOC_LINEAR = 0x10003,
}

#[repr(C)]
pub enum MemState {
	MEMSTATE_FREE       = 0,
	MEMSTATE_RESERVED   = 1,
	MEMSTATE_IO         = 2,
	MEMSTATE_STATIC     = 3,
	MEMSTATE_CODE       = 4,
	MEMSTATE_PRIVATE    = 5,
	MEMSTATE_SHARED     = 6,
	MEMSTATE_CONTINUOUS = 7,
	MEMSTATE_ALIASED    = 8,
	MEMSTATE_ALIAS      = 9,
	MEMSTATE_ALIASCODE  = 10,
	MEMSTATE_LOCKED     = 11
}

#[repr(C)]
pub enum MemPerm {
	MEMPERM_READ     = 1,
	MEMPERM_WRITE    = 2,
	MEMPERM_EXECUTE  = 4,
	MEMPERM_DONTCARE = 0x10000000,
	MEMPERM_MAX      = 0xFFFFFFFF //force 4-byte
}

#[repr(C)]
pub struct MemInfo {
    pub base_addr: u32,
    pub size: u32,
    pub perm: u32,
    pub state: u32,
}

#[repr(C)]
pub struct PageInfo {
    flags: u32,
}
