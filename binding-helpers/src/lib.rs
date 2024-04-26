pub mod gen;

#[macro_export]
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

#[macro_export]
macro_rules! align_of {
    ($ty:ident::$field:ident) => {{
        $create::align_of_ret(|x: $ty| x.$field)
    }};
    ($ty:ty) => {
        ::std::mem::align_of::<$ty>()
    };
    ($expr:expr) => {
        ::std::mem::align_of_val(&$expr)
    };
}

#[doc(hidden)]
pub fn size_of_ret<T, U>(_f: impl Fn(U) -> T) -> usize {
    ::std::mem::size_of::<T>()
}

#[doc(hidden)]
pub fn align_of_ret<T, U>(_f: impl Fn(U) -> T) -> usize {
    ::std::mem::align_of::<T>()
}
