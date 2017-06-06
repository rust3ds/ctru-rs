#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![feature(const_fn)] 
#![feature(untagged_unions)]

#![no_std]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
