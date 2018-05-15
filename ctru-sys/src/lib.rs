#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![feature(const_fn)] 

#![no_std]

#![cfg_attr(feature = "stdbuild", feature(libc))]

extern crate libc;

include!("bindings.rs");
