//! System Applets.
//!
//! Applets are small integrated programs that the OS makes available to the developer to streamline commonly needed functionality.
//! Thanks to these integrations the developer can avoid wasting time re-implementing common features and instead use a more reliable base for their application.
//!
//! Unlike [services](crate::services), applets aren't accessed via a system subprocess (which would require obtaining a special handle at runtime).
//! Instead, the application builds a configuration storing the various parameters which is then used to "launch" the applet.
//!
//! Applets block execution of the thread that launches them as long as the user doesn't close the applet.

pub mod mii_selector;
pub mod swkbd;
