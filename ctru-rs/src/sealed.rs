//! This is a private module to prevent users from implementing certain traits.
//! This is done by requiring a `Sealed` trait implementation, which can only be
//! done in this crate.

use crate::console::Console;
use crate::services::gfx::{BottomScreen, TopScreen, TopScreen3D, TopScreenLeft, TopScreenRight};

pub trait Sealed {}

impl Sealed for TopScreen {}
impl Sealed for TopScreen3D<'_> {}
impl Sealed for TopScreenLeft {}
impl Sealed for TopScreenRight {}
impl Sealed for BottomScreen {}
impl Sealed for Console<'_> {}
