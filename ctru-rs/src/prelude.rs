//! `use ctru::prelude::*;` to import common services, members and functions.
//!
//! Particularly useful when writing very small applications.

pub use crate::console::Console;
pub use crate::services::{
    apt::Apt,
    gfx::Gfx,
    hid::{Hid, KeyPad},
    soc::Soc,
};
