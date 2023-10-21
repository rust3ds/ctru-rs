//! Utilities to get information about the operating system and hardware state.

/// System version information. This struct is used for both kernel and firmware versions.
///
/// # Example
/// ```
/// # let _runner = test_runner::GdbRunner::default();
/// let firm_version = ctru::os::firm_version();
/// assert_ne!(firm_version.major(), 0);
///
/// let kernel_version = ctru::os::kernel_version();
/// assert_ne!(kernel_version.major(), 0);
/// ```
#[derive(Clone, Copy)]
pub struct Version(u32);

impl Version {
    /// Pack a system version from its components
    pub fn new(major: u8, minor: u8, revision: u8) -> Self {
        let major = u32::from(major);
        let minor = u32::from(minor);
        let revision = u32::from(revision);

        Self(major << 24 | minor << 16 | revision << 8)
    }

    /// Get the major version from a packed system version.
    pub fn major(&self) -> u8 {
        (self.0 >> 24).try_into().unwrap()
    }

    /// Get the minor version from a packed system version.
    pub fn minor(&self) -> u8 {
        (self.0 >> 16 & 0xFF).try_into().unwrap()
    }

    /// Get the revision from a packed system version.
    pub fn revision(&self) -> u8 {
        (self.0 >> 8 & 0xFF).try_into().unwrap()
    }
}

/// Get the system's FIRM version.
pub fn firm_version() -> Version {
    Version(unsafe { ctru_sys::osGetFirmVersion() })
}

/// Get the system's kernel version.
pub fn kernel_version() -> Version {
    Version(unsafe { ctru_sys::osGetKernelVersion() })
}

// TODO: I can't seem to find good documentation on it, but we could probably
// define enums for firmware type (NATIVE_FIRM, SAFE_FIRM etc.) as well as
// application memory layout. Leaving those as future enhancements for now

/// A region of memory. Most applications will only use [`Application`](MemRegion::Application)
/// memory, but the other types can be used to query memory usage information.
/// See <https://www.3dbrew.org/wiki/Memory_layout#FCRAM_memory-regions_layout>
/// for more details on the different types of memory.
///
/// # Example
/// ```
/// # let _runner = test_runner::GdbRunner::default();
/// let all_memory = ctru::os::MemRegion::All;
///
/// assert!(all_memory.size() > 0);
/// assert!(all_memory.used() > 0);
/// assert!(all_memory.free() > 0);
/// ```
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u32)]
pub enum MemRegion {
    /// All memory regions.
    All = ctru_sys::MEMREGION_ALL,
    /// APPLICATION memory.
    Application = ctru_sys::MEMREGION_APPLICATION,
    /// SYSTEM memory.
    System = ctru_sys::MEMREGION_SYSTEM,
    /// BASE memory.
    Base = ctru_sys::MEMREGION_BASE,
}

impl MemRegion {
    /// Get the total size of this memory region, in bytes.
    pub fn size(&self) -> usize {
        unsafe { ctru_sys::osGetMemRegionSize(*self as u32) }
            .try_into()
            .unwrap()
    }

    /// Get the number of bytes used within this memory region.
    pub fn used(&self) -> usize {
        unsafe { ctru_sys::osGetMemRegionUsed(*self as u32) }
            .try_into()
            .unwrap()
    }

    /// Get the number of bytes free within this memory region.
    pub fn free(&self) -> usize {
        unsafe { ctru_sys::osGetMemRegionFree(*self as u32) }
            .try_into()
            .unwrap()
    }
}

/// WiFi signal strength. This enum's `u8` representation corresponds with
/// the number of bars displayed in the Home menu.
///
/// # Example
///
/// ```
/// let _runner = test_runner::GdbRunner::default();
/// let strength = ctru::os::WifiStrength::current();
/// assert!((strength as u8) < 4);
/// ```
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum WifiStrength {
    /// This may indicate a very poor signal quality even worse than `Bad`,
    /// or that no network is connected at all.
    Disconnected = 0,
    /// Poor signal strength.
    Bad = 1,
    /// Medium signal strength.
    Decent = 2,
    /// Good signal strength.
    Good = 3,
}

impl WifiStrength {
    /// Get the current WiFi signal strength.
    pub fn current() -> Self {
        match unsafe { ctru_sys::osGetWifiStrength() } {
            0 => Self::Disconnected,
            1 => Self::Bad,
            2 => Self::Decent,
            3 => Self::Good,
            other => panic!("Got unexpected WiFi strength value {other}"),
        }
    }
}

/// Get the current value of the stereoscopic 3D slider on a scale from 0.0­–­1.0.
pub fn current_3d_slider_state() -> f32 {
    unsafe { ctru_sys::osGet3DSliderState() }
}

/// Whether or not a headset is currently plugged into the device.
pub fn is_headset_connected() -> bool {
    unsafe { ctru_sys::osIsHeadsetConnected() }
}
