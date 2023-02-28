//! System services used to handle system-specific functionalities.
//!
//! Most of the 3DS console's functionalities (when writing homebrew) are locked behind services,
//! which need to be initialized before accessing any particular feature.
//!
//! Some include: button input, audio playback, graphics rendering, built-in cameras, etc.

pub mod apt;
pub mod am;
pub mod cam;
pub mod cfgu;
pub mod fs;
pub mod gspgpu;
pub mod hid;
pub mod ndsp;
pub mod ps;
mod reference;
pub mod soc;
pub mod sslc;

pub use self::apt::Apt;
pub use self::hid::Hid;

pub(crate) use self::reference::ServiceReference;
