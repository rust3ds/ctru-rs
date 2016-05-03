#[repr(C)]
pub enum mediatypes_enum {
    mediatype_NAND = 0,
    mediatype_SDMC = 1,
    mediatype_GAMECARD = 2,
}

pub type s8 = i8;
pub type s16 = i16;
pub type s32 = i32;
pub type s64 = i64;

// UHH, DUNNO HOW TO USE VOLATILES ????
pub type vu8 = u8;
pub type vu32 = u32;

// typedef uint8_t u8;
// typedef uint16_t u16;
// typedef uint32_t u32;
// typedef uint64_t u64;
//
// typedef int8_t s8;
// typedef int16_t s16;
// typedef int32_t s32;
// typedef int64_t s64;
//
// typedef volatile u8 vu8;
// typedef volatile u16 vu16;
// typedef volatile u32 vu32;
// typedef volatile u64 vu64;
//
// typedef volatile s8 vs8;
// typedef volatile s16 vs16;
// typedef volatile s32 vs32;
// typedef volatile s64 vs64;
