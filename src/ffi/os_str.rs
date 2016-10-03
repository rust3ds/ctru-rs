// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use collections::borrow::{Borrow, Cow, ToOwned};
use core::fmt::{self, Debug};
use core::mem;
use collections::String;
use core::ops;
use core::cmp;
use core::hash::{Hash, Hasher};
use collections::Vec;

use sys::wtf8::{Wtf8, Wtf8Buf};
use sys::{AsInner, IntoInner, FromInner};

pub use sys::wtf8::EncodeWide;

/// A type that can represent owned, mutable platform-native strings, but is
/// cheaply inter-convertible with Rust strings.
///
/// The need for this type arises from the fact that:
///
/// * On Unix systems, strings are often arbitrary sequences of non-zero
///   bytes, in many cases interpreted as UTF-8.
///
/// * On Windows, strings are often arbitrary sequences of non-zero 16-bit
///   values, interpreted as UTF-16 when it is valid to do so.
///
/// * In Rust, strings are always valid UTF-8, but may contain zeros.
///
/// `OsString` and `OsStr` bridge this gap by simultaneously representing Rust
/// and platform-native string values, and in particular allowing a Rust string
/// to be converted into an "OS" string with no cost.
#[derive(Clone)]
pub struct OsString {
    inner: Wtf8Buf 
}

/// Slices into OS strings (see `OsString`).
pub struct OsStr {
    inner: Wtf8
}

impl OsString {
    /// Constructs a new empty `OsString`.
    pub fn new() -> OsString {
        OsString { inner: Wtf8Buf::from_string(String::new()) }
    }

    fn _from_bytes(vec: Vec<u8>) -> Option<OsString> {
        String::from_utf8(vec).ok().map(OsString::from)
    }

    /// Converts to an `OsStr` slice.
    pub fn as_os_str(&self) -> &OsStr {
        self
    }

    /// Converts the `OsString` into a `String` if it contains valid Unicode data.
    ///
    /// On failure, ownership of the original `OsString` is returned.
    pub fn into_string(self) -> Result<String, OsString> {
        self.inner.into_string().map_err(|buf| OsString { inner: buf} )
    }

    /// Extends the string with the given `&OsStr` slice.
    pub fn push<T: AsRef<OsStr>>(&mut self, s: T) {
        self.inner.push_wtf8(&s.as_ref().inner)
    }

    /// Creates a new `OsString` with the given capacity.
    ///
    /// The string will be able to hold exactly `capacity` lenth units of other
    /// OS strings without reallocating. If `capacity` is 0, the string will not
    /// allocate.
    ///
    /// See main `OsString` documentation information about encoding.
    pub fn with_capacity(capacity: usize) -> OsString {
        OsString {
            inner: Wtf8Buf::with_capacity(capacity)
        }
    }

    /// Truncates the `OsString` to zero length.
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Returns the capacity this `OsString` can hold without reallocating.
    ///
    /// See `OsString` introduction for information about encoding.
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Reserves capacity for at least `additional` more capacity to be inserted
    /// in the given `OsString`.
    ///
    /// The collection may reserve more space to avoid frequent reallocations.
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    /// Reserves the minimum capacity for exactly `additional` more capacity to
    /// be inserted in the given `OsString`. Does nothing if the capacity is
    /// already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it
    /// requests. Therefore capacity can not be relied upon to be precisely
    /// minimal. Prefer reserve if future insertions are expected.
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional)
    }

    /// Creates an `OsString` from a potentially ill-formed UTF-16 slice of
    /// 16-bit code units.
    ///
    /// This is lossless: calling `.encode_wide()` on the resulting string
    /// will always return the original code units.
    ///
    /// NOTE: This function was copied from the windows implementation of OsStringExt
    pub fn from_wide(wide: &[u16]) -> OsString {
        OsString { inner: Wtf8Buf::from_wide(wide) }
    }
}

impl From<String> for OsString {
    fn from(s: String) -> OsString {
        OsString { inner: Wtf8Buf::from_string(s) }
    }
}

impl<'a, T: ?Sized + AsRef<OsStr>> From<&'a T> for OsString {
    fn from(s: &'a T) -> OsString {
        s.as_ref().to_os_string()
    }
}

impl ops::Index<ops::RangeFull> for OsString {
    type Output = OsStr;

    #[inline]
    fn index(&self, _index: ops::RangeFull) -> &OsStr {
        OsStr::from_inner(self.inner.as_slice())
    }
}

impl ops::Deref for OsString {
    type Target = OsStr;

    #[inline]
    fn deref(&self) -> &OsStr {
        &self[..]
    }
}

impl Default for OsString {
    #[inline]
    fn default() -> OsString {
        OsString::new()
    }
}

impl Debug for OsString {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, formatter)
    }
}

impl PartialEq for OsString {
    fn eq(&self, other: &OsString) -> bool {
        &**self == &**other
    }
}

impl PartialEq<str> for OsString {
    fn eq(&self, other: &str) -> bool {
        &**self == other
    }
}

impl PartialEq<OsString> for str {
    fn eq(&self, other: &OsString) -> bool {
        &**other == self
    }
}

impl Eq for OsString {}

impl PartialOrd for OsString {
    #[inline]
    fn partial_cmp(&self, other: &OsString) -> Option<cmp::Ordering> {
        (&**self).partial_cmp(&**other)
    }
    #[inline]
    fn lt(&self, other: &OsString) -> bool { &**self < &**other }
    #[inline]
    fn le(&self, other: &OsString) -> bool { &**self <= &**other }
    #[inline]
    fn gt(&self, other: &OsString) -> bool { &**self > &**other }
    #[inline]
    fn ge(&self, other: &OsString) -> bool { &**self >= &**other }
}

impl PartialOrd<str> for OsString {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        (&**self).partial_cmp(other)
    }
}

impl Ord for OsString {
    #[inline]
    fn cmp(&self, other: &OsString) -> cmp::Ordering {
        (&**self).cmp(&**other)
    }
}

impl Hash for OsString {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&**self).hash(state)
    }
}

impl OsStr {
    /// Coerces into an `OsStr` slice.
    pub fn new<S: AsRef<OsStr> + ?Sized>(s: &S) -> &OsStr {
        s.as_ref()
    }

    fn from_inner(inner: &Wtf8) -> &OsStr {
        unsafe { mem::transmute(inner) }
    }

    /// Yields a `&str` slice if the `OsStr` is valid Unicode.
    ///
    /// This conversion may entail doing a check for UTF-8 validity.
    pub fn to_str(&self) -> Option<&str> {
        self.inner.as_str()
    }

    /// Converts an `OsStr` to a `Cow<str>`.
    ///
    /// Any non-Unicode sequences are replaced with U+FFFD REPLACEMENT CHARACTER.
    pub fn to_string_lossy(&self) -> Cow<str> {
        self.inner.to_string_lossy()
    }

    /// Copies the slice into an owned `OsString`.
    pub fn to_os_string(&self) -> OsString {
        let mut buf = Wtf8Buf::with_capacity(self.inner.len());
        buf.push_wtf8(&self.inner);
        OsString { inner: buf }
    }

    /// Checks whether the `OsStr` is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the length of this `OsStr`.
    ///
    /// Note that this does **not** return the number of bytes in this string
    /// as, for example, OS strings on Windows are encoded as a list of `u16`
    /// rather than a list of bytes. This number is simply useful for passing to
    /// other methods like `OsString::with_capacity` to avoid reallocations.
    ///
    /// See `OsStr` introduction for more information about encoding.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Gets the underlying byte representation.
    ///
    /// Note: it is *crucial* that this API is private, to avoid
    /// revealing the internal, platform-specific encodings.
    fn bytes(&self) -> &[u8] {
        unsafe { mem::transmute(&self.inner) }
    }
    
    /// Re-encodes an `OsStr` as a wide character sequence,
    /// i.e. potentially ill-formed UTF-16.
    /// This is lossless. Note that the encoding does not include a final
    /// null.
    ///
    /// NOTE: This function was copied from the windows implementation of OsStrExt
    pub fn encode_wide(&self) -> EncodeWide {
        self.inner.encode_wide()
    }

}

impl<'a> Default for &'a OsStr {
    #[inline]
    fn default() -> &'a OsStr {
        OsStr::new("")
    }
}

impl PartialEq for OsStr {
    fn eq(&self, other: &OsStr) -> bool {
        self.bytes().eq(other.bytes())
    }
}

impl PartialEq<str> for OsStr {
    fn eq(&self, other: &str) -> bool {
        *self == *OsStr::new(other)
    }
}

impl PartialEq<OsStr> for str {
    fn eq(&self, other: &OsStr) -> bool {
        *other == *OsStr::new(self)
    }
}

impl Eq for OsStr {}

impl PartialOrd for OsStr {
    #[inline]
    fn partial_cmp(&self, other: &OsStr) -> Option<cmp::Ordering> {
        self.bytes().partial_cmp(other.bytes())
    }
    #[inline]
    fn lt(&self, other: &OsStr) -> bool { self.bytes().lt(other.bytes()) }
    #[inline]
    fn le(&self, other: &OsStr) -> bool { self.bytes().le(other.bytes()) }
    #[inline]
    fn gt(&self, other: &OsStr) -> bool { self.bytes().gt(other.bytes()) }
    #[inline]
    fn ge(&self, other: &OsStr) -> bool { self.bytes().ge(other.bytes()) }
}

impl PartialOrd<str> for OsStr {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        self.partial_cmp(OsStr::new(other))
    }
}

// FIXME (#19470): cannot provide PartialOrd<OsStr> for str until we
// have more flexible coherence rules.

impl Ord for OsStr {
    #[inline]
    fn cmp(&self, other: &OsStr) -> cmp::Ordering { self.bytes().cmp(other.bytes()) }
}

macro_rules! impl_cmp {
    ($lhs:ty, $rhs: ty) => {
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool { <OsStr as PartialEq>::eq(self, other) }
        }

        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool { <OsStr as PartialEq>::eq(self, other) }
        }

        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<cmp::Ordering> {
                <OsStr as PartialOrd>::partial_cmp(self, other)
            }
        }

        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<cmp::Ordering> {
                <OsStr as PartialOrd>::partial_cmp(self, other)
            }
        }
    }
}

impl_cmp!(OsString, OsStr);
impl_cmp!(OsString, &'a OsStr);
impl_cmp!(Cow<'a, OsStr>, OsStr);
impl_cmp!(Cow<'a, OsStr>, &'b OsStr);
impl_cmp!(Cow<'a, OsStr>, OsString);

impl Hash for OsStr {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bytes().hash(state)
    }
}

impl Debug for OsStr {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.inner.fmt(formatter)
    }
}

impl Borrow<OsStr> for OsString {
    fn borrow(&self) -> &OsStr { &self[..] }
}

impl ToOwned for OsStr {
    type Owned = OsString;
    fn to_owned(&self) -> OsString { self.to_os_string() }
}

impl AsRef<OsStr> for OsStr {
    fn as_ref(&self) -> &OsStr {
        self
    }
}

impl AsRef<OsStr> for OsString {
    fn as_ref(&self) -> &OsStr {
        self
    }
}

impl AsRef<OsStr> for str {
    fn as_ref(&self) -> &OsStr {
        OsStr::from_inner(Wtf8::from_str(self))
    }
}

impl AsRef<OsStr> for String {
    fn as_ref(&self) -> &OsStr {
        (&**self).as_ref()
    }
}

impl FromInner<Wtf8Buf> for OsString {
    fn from_inner(buf: Wtf8Buf) -> OsString {
        OsString { inner: buf }
    }
}

impl IntoInner<Wtf8Buf> for OsString {
    fn into_inner(self) -> Wtf8Buf {
        self.inner
    }
}

impl AsInner<Wtf8> for OsStr {
    fn as_inner(&self) -> &Wtf8 {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sys::{AsInner, IntoInner};

    #[test]
    fn test_os_string_with_capacity() {
        let os_string = OsString::with_capacity(0);
        assert_eq!(0, os_string.inner.capacity());

        let os_string = OsString::with_capacity(10);
        assert_eq!(10, os_string.inner.capacity());

        let mut os_string = OsString::with_capacity(0);
        os_string.push("abc");
        assert!(os_string.inner.capacity() >= 3);
    }

    #[test]
    fn test_os_string_clear() {
        let mut os_string = OsString::from("abc");
        assert_eq!(3, os_string.inner.len());

        os_string.clear();
        assert_eq!(&os_string, "");
        assert_eq!(0, os_string.inner.len());
    }

    #[test]
    fn test_os_string_capacity() {
        let os_string = OsString::with_capacity(0);
        assert_eq!(0, os_string.capacity());

        let os_string = OsString::with_capacity(10);
        assert_eq!(10, os_string.capacity());

        let mut os_string = OsString::with_capacity(0);
        os_string.push("abc");
        assert!(os_string.capacity() >= 3);
    }

    #[test]
    fn test_os_string_reserve() {
        let mut os_string = OsString::new();
        assert_eq!(os_string.capacity(), 0);

        os_string.reserve(2);
        assert!(os_string.capacity() >= 2);

        for _ in 0..16 {
            os_string.push("a");
        }

        assert!(os_string.capacity() >= 16);
        os_string.reserve(16);
        assert!(os_string.capacity() >= 32);

        os_string.push("a");

        os_string.reserve(16);
        assert!(os_string.capacity() >= 33)
    }

    #[test]
    fn test_os_string_reserve_exact() {
        let mut os_string = OsString::new();
        assert_eq!(os_string.capacity(), 0);

        os_string.reserve_exact(2);
        assert!(os_string.capacity() >= 2);

        for _ in 0..16 {
            os_string.push("a");
        }

        assert!(os_string.capacity() >= 16);
        os_string.reserve_exact(16);
        assert!(os_string.capacity() >= 32);

        os_string.push("a");

        os_string.reserve_exact(16);
        assert!(os_string.capacity() >= 33)
    }

    #[test]
    fn test_os_string_default() {
        let os_string: OsString = Default::default();
        assert_eq!("", &os_string);
    }

    #[test]
    fn test_os_str_is_empty() {
        let mut os_string = OsString::new();
        assert!(os_string.is_empty());

        os_string.push("abc");
        assert!(!os_string.is_empty());

        os_string.clear();
        assert!(os_string.is_empty());
    }

    #[test]
    fn test_os_str_len() {
        let mut os_string = OsString::new();
        assert_eq!(0, os_string.len());

        os_string.push("abc");
        assert_eq!(3, os_string.len());

        os_string.clear();
        assert_eq!(0, os_string.len());
    }

    #[test]
    fn test_os_str_default() {
        let os_str: &OsStr = Default::default();
        assert_eq!("", os_str);
    }
}
