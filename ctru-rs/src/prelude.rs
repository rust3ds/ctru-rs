//! `use ctru::prelude::*;` to import common services, members and functions.
//!
//! Particularly useful when writing very small applications.

pub use crate::console::Console;
pub use crate::services::apt::Apt;
pub use crate::services::gfx::Gfx;
pub use crate::services::hid::{Hid, KeyPad};
pub use crate::services::soc::Soc;
