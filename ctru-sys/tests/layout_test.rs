//! This is a stub for the generated layout test. We use bindgen callbacks along
//! with the [`cpp`] crate to compile actual `sizeof` and `alignof` calls in C,
//! as opposed to bindgen's generated layout tests which use hardcoded size literals.
//!
//! This should help ensure that the generated bindings are correct for the actual
//! ABI used by libctru and the devkitARM toolchain, instead of just what libclang
//! thinks they should be at bindgen time.

#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run_gdb)]

extern crate shim_3ds;

use std::mem::offset_of;

fn size_of_ret<T, U>(_f: impl Fn(U) -> T) -> usize {
    ::std::mem::size_of::<T>()
}

macro_rules! size_of {
    ($ty:ident::$field:ident) => {{
        $crate::size_of_ret(|x: $ty| x.$field)
    }};
    ($ty:ty) => {
        ::std::mem::size_of::<$ty>()
    };
    ($expr:expr) => {
        ::std::mem::size_of_val(&$expr)
    };
}

fn align_of_ret<T, U>(_f: impl Fn(U) -> T) -> usize {
    ::std::mem::align_of::<T>()
}

macro_rules! align_of {
    ($ty:ident::$field:ident) => {{
        // This matches the semantics of C++ alignof when it is applied to a struct
        // member. Packed structs may under-align fields, so we take the minimum
        // of the align of the struct and the type of the field itself.
        $crate::align_of_ret(|x: $ty| x.$field).min(align_of!($ty))
    }};
    ($ty:ty) => {
        ::std::mem::align_of::<$ty>()
    };
    ($expr:expr) => {
        ::std::mem::align_of_val(&$expr)
    };
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod generated_tests {
    use super::*;

    use cpp::cpp;
    use ctru_sys::*;

    include!(concat!(env!("OUT_DIR"), "/generated_layout_test.rs"));
}

mod helper_tests {
    macro_rules! packed_struct {
        ($name:ident, $size:literal) => {
            #[repr(C, packed($size))]
            struct $name {
                a: u8,
                b: u16,
                c: u32,
                d: u64,
            }
        };
    }

    packed_struct!(PackedStruct1, 1);
    packed_struct!(PackedStruct2, 2);
    packed_struct!(PackedStruct4, 4);
    packed_struct!(PackedStruct8, 8);

    #[test]
    fn align_of_matches_cpp() {
        // Expected values are based on C++: https://godbolt.org/z/dPnP7nEse
        assert_eq!(align_of!(PackedStruct1), 1);
        assert_eq!(align_of!(PackedStruct1::a), 1);
        assert_eq!(align_of!(PackedStruct1::b), 1);
        assert_eq!(align_of!(PackedStruct1::c), 1);
        assert_eq!(align_of!(PackedStruct1::d), 1);

        assert_eq!(align_of!(PackedStruct2), 2);
        assert_eq!(align_of!(PackedStruct2::a), 1);
        assert_eq!(align_of!(PackedStruct2::b), 2);
        assert_eq!(align_of!(PackedStruct2::c), 2);
        assert_eq!(align_of!(PackedStruct2::d), 2);

        assert_eq!(align_of!(PackedStruct4), 4);
        assert_eq!(align_of!(PackedStruct4::a), 1);
        assert_eq!(align_of!(PackedStruct4::b), 2);
        assert_eq!(align_of!(PackedStruct4::c), 4);
        assert_eq!(align_of!(PackedStruct4::d), 4);

        assert_eq!(align_of!(PackedStruct8), 8);
        assert_eq!(align_of!(PackedStruct8::a), 1);
        assert_eq!(align_of!(PackedStruct8::b), 2);
        assert_eq!(align_of!(PackedStruct8::c), 4);
        assert_eq!(align_of!(PackedStruct8::d), 8);
    }
}
