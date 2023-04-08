//! System services used to handle system-specific functionalities.
//!
//! Most of the 3DS console's functionalities (when writing homebrew) are locked behind services,
//! which need to be initialized before accessing any particular feature.
//!
//! Some include: button input, audio playback, graphics rendering, built-in cameras, etc.

pub mod am;
pub mod apt;
pub mod cam;
pub mod cfgu;
pub mod fs;
pub mod gfx;
pub mod gspgpu;
pub mod hid;
pub mod ndsp;
pub mod ps;
mod reference;
pub mod soc;
pub mod sslc;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "romfs", romfs_exists))] {
        pub mod romfs;
    } else {
        pub mod romfs {
            //! The RomFS folder has not been detected and/or the `romfs` feature has not been enabled.
            //!
            //! Configure the path in Cargo.toml (the default path is "romfs"). Paths are relative to the
            //! `CARGO_MANIFEST_DIR` environment variable, which is the directory containing the manifest of
            //! your package.
            //!
            //! ```toml
            //! [package.metadata.cargo-3ds]
            //! romfs_dir = "romfs"
            //! ```

            compile_error!("romfs feature is enabled but no romfs found!");
        }
    }
}

pub use self::apt::Apt;
pub use self::hid::Hid;

pub(crate) use self::reference::ServiceReference;
