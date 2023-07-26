//! OS services used to handle system-specific functionality.
//!
//! Most of the 3DS console's functionalities (when writing user-land homebrew) are accessible via services,
//! which need to be initialized before accessing any particular feature.
//!
//! To ensure safety while using the underlying services, [`ctru-rs`](crate) leverages Rust's lifetime model.
//! After initializing the handle for a specific service (e.g. [`Apt`](apt::Apt)) the service will be accessible as long as there is at least one handle "alive".
//! As such, handles should be dropped *after* the use of a specific service. This is particularly important for services which are necessary for functionality
//! "outside" their associated methods, such as [`RomFS`](romfs::RomFS), which creates an accessible virtual filesystem, or [`Soc`](soc::Soc),
//! which enables all network communications via sockets.
//!
//! In [`ctru-rs`](crate) some services only allow a single handle to be created at a time, to ensure a safe and controlled environment.

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

            // If the feature is set, but no "romfs" directory was found: send an error during compilation.
            #[cfg(feature = "romfs")]
            compile_error!("romfs feature is enabled but no romfs found!");
        }
    }
}

pub(crate) use self::reference::ServiceReference;
