//! UDS (local networking) service.
//!
//! The UDS service is used to handle local networking, i.e. peer-to-peer networking used for local multiplayer.
//! This module also covers some functionality used in Download Play (dlp); there is a specific module for DLP, but it can also be implemented manually using UDS.
#![doc(alias = "network")]
#![doc(alias = "dlplay")]

use std::error::Error as StdError;
use std::ffi::CString;
use std::fmt::{Debug, Display};
use std::mem::MaybeUninit;
use std::ops::FromResidual;
use std::ptr::null;
use std::sync::Mutex;

use crate::error::ResultCode;
use crate::services::ServiceReference;

use bitflags::bitflags;
use macaddr::MacAddr6;

bitflags! {
    /// Flags used for sending packets to a network.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SendFlags: u8 {
        /// Unknown function according to `libctru`.
        const Default = ctru_sys::UDS_SENDFLAG_Default as u8;
        /// Broadcast the data frame even when sending to a non-broadcast address.
        const Broadcast = ctru_sys::UDS_SENDFLAG_Broadcast as u8;
    }
}

/// Error enum for generic errors within the [`Uds`] service.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// The provided username was too long.
    UsernameTooLong,
    /// The provided username contained a NULL byte.
    UsernameContainsNull(usize),
    /// Not connected to a network.
    NotConnected,
    /// No context bound.
    NoContext,
    /// Cannot send data on a network as a spectator.
    Spectator,
    /// No network created.
    NoNetwork,
    /// The provided app data buffer was too large.
    TooMuchAppData,
    /// The provided node ID does not reference a specific node.
    NotANode,
    /// ctru-rs error
    Lib(crate::Error),
}

impl From<crate::Error> for Error {
    fn from(value: crate::Error) -> Self {
        Error::Lib(value)
    }
}

impl<T> FromResidual<crate::Error> for Result<T, Error> {
    fn from_residual(residual: crate::Error) -> Self {
        Err(residual.into())
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(value: std::ffi::NulError) -> Self {
        Error::UsernameContainsNull(value.nul_position())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UsernameTooLong =>
                    "provided username was too long (max 10 bytes, not code points)".into(),
                Self::UsernameContainsNull(pos) =>
                    format!("provided username contained a NULL byte at position {pos}"),
                Self::NotConnected => "not connected to a network".into(),
                Self::NoContext => "no context bound".into(),
                Self::Spectator => "cannot send data on a network as a spectator".into(),
                Self::NoNetwork => "not hosting a network".into(),
                Self::TooMuchAppData => "provided too much app data (max 200 bytes)".into(),
                Self::NotANode => "provided node ID was non-specific".into(),
                Self::Lib(e) => format!("ctru-rs error: {e}"),
            }
        )
    }
}

impl StdError for Error {}

/// Possible types of connection to a network.
#[doc(alias = "udsConnectionType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ConnectionType {
    /// A normal client. Can push packets to the network.
    Client = ctru_sys::UDSCONTYPE_Client,
    /// A spectator. Cannot push packets to the network,
    /// but doesn't need the passphrase to join.
    Spectator = ctru_sys::UDSCONTYPE_Spectator,
}

impl From<ConnectionType> for u8 {
    fn from(value: ConnectionType) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for ConnectionType {
    type Error = ();

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value as u32 {
            ctru_sys::UDSCONTYPE_Client => Ok(Self::Client),
            ctru_sys::UDSCONTYPE_Spectator => Ok(Self::Spectator),
            _ => Err(()),
        }
    }
}

/// ID for a node on the network.
#[doc(alias = "NetworkNodeID")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeID {
    /// No node ID set (not connected to a network).
    None,
    /// A normal node on the network, counting from 1 (the host) to 16, inclusive.
    Node(u8),
    /// Broadcast to all nodes
    Broadcast,
}

impl From<NodeID> for u16 {
    fn from(value: NodeID) -> Self {
        match value {
            NodeID::None => 0,
            NodeID::Node(node) => node as u16,
            NodeID::Broadcast => ctru_sys::UDS_BROADCAST_NETWORKNODEID as u16,
        }
    }
}

impl TryFrom<u16> for NodeID {
    type Error = ();

    fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
        match value as u32 {
            0 => Ok(Self::None),
            ctru_sys::UDS_HOST_NETWORKNODEID..=ctru_sys::UDS_MAXNODES => {
                Ok(Self::Node(value as u8))
            }
            ctru_sys::UDS_BROADCAST_NETWORKNODEID => Ok(Self::Broadcast),
            _ => Err(()),
        }
    }
}

/// Information about a network node.
#[doc(alias = "udsNodeInfo")]
#[derive(Copy, Clone)]
pub struct NodeInfo(ctru_sys::udsNodeInfo);

impl Debug for NodeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeInfo(")?;

        f.debug_struct("udsNodeInfo")
            .field("uds_friendcodeseed", &self.uds_friendcodeseed())
            .field("username", &self.username())
            .field("flag", &self.flag())
            .field("NetworkNodeID", &self.node_id())
            .finish()?;

        write!(f, ")")
    }
}

impl From<ctru_sys::udsNodeInfo> for NodeInfo {
    fn from(value: ctru_sys::udsNodeInfo) -> Self {
        Self(value)
    }
}

impl NodeInfo {
    /// Friend code seed associated with this network node.
    pub fn uds_friendcodeseed(&self) -> u64 {
        self.0.uds_friendcodeseed
    }

    /// Username associated with this network node.
    pub fn username(&self) -> String {
        String::from_utf16_lossy(unsafe { &self.0.__bindgen_anon_1.__bindgen_anon_1.username })
    }

    /// Flag associated with this network node.
    pub fn flag(&self) -> u8 {
        unsafe { self.0.__bindgen_anon_1.__bindgen_anon_1.flag }
    }

    /// Node ID associated with this network node.
    pub fn node_id(&self) -> NodeID {
        self.0
            .NetworkNodeID
            .try_into()
            .expect("UDS service should always provide a valid NetworkNodeID")
    }
}

/// Information returned from scanning for networks.
#[doc(alias = "udsNetworkScanInfo")]
#[derive(Copy, Clone)]
pub struct NetworkScanInfo(ctru_sys::udsNetworkScanInfo);

impl Debug for NetworkScanInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NetworkScanInfo(")?;

        f.debug_struct("udsNetworkScanInfo")
            .field("datareply_entry", &self.datareply_entry())
            .field("network", &self.network())
            .field("nodes", &self.nodes())
            .finish()?;

        write!(f, ")")
    }
}

impl From<ctru_sys::udsNetworkScanInfo> for NetworkScanInfo {
    fn from(value: ctru_sys::udsNetworkScanInfo) -> Self {
        Self(value)
    }
}

impl NetworkScanInfo {
    /// NWM output structure.
    pub fn datareply_entry(&self) -> ctru_sys::nwmBeaconDataReplyEntry {
        self.0.datareply_entry
    }

    /// Get a reference to the NWM output structure.
    pub fn datareply_entry_ref(&self) -> &ctru_sys::nwmBeaconDataReplyEntry {
        &self.0.datareply_entry
    }

    /// Get a mutable reference to the NWM output structure.
    pub fn datareply_entry_mut(&mut self) -> &mut ctru_sys::nwmBeaconDataReplyEntry {
        &mut self.0.datareply_entry
    }

    /// Information about the network.
    pub fn network(&self) -> ctru_sys::udsNetworkStruct {
        self.0.network
    }

    /// Get a reference to the information about the network.
    pub fn network_ref(&self) -> &ctru_sys::udsNetworkStruct {
        &self.0.network
    }

    /// Get a mutable reference to the information about the network.
    pub fn network_mut(&mut self) -> &mut ctru_sys::udsNetworkStruct {
        &mut self.0.network
    }

    /// All nodes on the network (first node is the server,
    /// max 16, `None` means no node connected).
    pub fn nodes(&self) -> [Option<NodeInfo>; 16] {
        self.0.nodes.map(|n| {
            if n.uds_friendcodeseed != 0 {
                Some(n.into())
            } else {
                None
            }
        })
    }
}

/// Possible raw connection status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[non_exhaustive]
pub enum ConnectionStatusInfo {
    /// Not connected to any network.
    Disconnected = 3,
    /// Connected as a host.
    Host = 6,
    /// Connected as a client.
    Client = 9,
    /// Connected as a spectator.
    Spectator = 10,
    /// Unknown
    Unknown = 11,
}

impl From<ConnectionStatusInfo> for u32 {
    fn from(value: ConnectionStatusInfo) -> Self {
        value as Self
    }
}

impl TryFrom<u32> for ConnectionStatusInfo {
    type Error = ();

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            3 => Ok(Self::Disconnected),
            6 => Ok(Self::Host),
            9 => Ok(Self::Client),
            10 => Ok(Self::Spectator),
            11 => Ok(Self::Unknown),
            _ => Err(()),
        }
    }
}

/// Status of the connection.
#[doc(alias = "udsConnectionStatus")]
#[derive(Clone, Copy)]
pub struct ConnectionStatus(ctru_sys::udsConnectionStatus);

impl Debug for ConnectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionStatus(")?;

        f.debug_struct("udsConnectionStatus")
            .field("status", &self.status())
            .field("cur_node_id", &self.cur_node_id())
            .field("total_nodes", &self.total_nodes())
            .field("max_nodes", &self.max_nodes())
            .field("node_bitmask", &self.node_bitmask())
            .finish()?;

        write!(f, ")")
    }
}

impl From<ctru_sys::udsConnectionStatus> for ConnectionStatus {
    fn from(value: ctru_sys::udsConnectionStatus) -> Self {
        Self(value)
    }
}

impl ConnectionStatus {
    /// Raw status information.
    pub fn status(&self) -> Option<ConnectionStatusInfo> {
        self.0.status.try_into().ok()
    }

    /// Network node ID for the current device.
    pub fn cur_node_id(&self) -> NodeID {
        self.0
            .cur_NetworkNodeID
            .try_into()
            .expect("UDS service should always provide a valid NetworkNodeID")
    }

    /// Number of nodes connected to the network.
    pub fn total_nodes(&self) -> u8 {
        self.0.total_nodes
    }

    /// Maximum nodes allowed on this network.
    pub fn max_nodes(&self) -> u8 {
        self.0.max_nodes
    }

    /// Bitmask for which of the 16 possible nodes are connected
    /// to this network; bit 0 is the server, bit 1 is the first
    /// original client, etc.
    pub fn node_bitmask(&self) -> u16 {
        self.0.node_bitmask
    }
}

/// Status of the service handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    /// Not connected to or hosting a network.
    Disconnected,
    /// Connected to a network.
    Client,
    /// Hosting a network.
    Server,
}

/// Handle to the UDS service.
pub struct Uds {
    _service_handler: ServiceReference,
    context: Option<ctru_sys::udsBindContext>,
    network: Option<ctru_sys::udsNetworkStruct>,
    scan_buf: Box<[u8; Self::SCAN_BUF_SIZE]>,
}

static UDS_ACTIVE: Mutex<()> = Mutex::new(());

impl Uds {
    /// Size of one frame.
    const RECV_FRAME_SIZE: usize = ctru_sys::UDS_DATAFRAME_MAXSIZE as usize;

    /// Size of receive buffer; max frame size * 8.
    const RECV_BUF_SIZE: u32 = ctru_sys::UDS_DEFAULT_RECVBUFSIZE;

    /// Shared memory size; must be slightly larger
    /// than `RECV_BUF_SIZE`.
    const SHAREDMEM_SIZE: usize = 0x3000;

    /// Buffer used while scanning for networks.
    /// This value is taken from the devkitPRO example.
    const SCAN_BUF_SIZE: usize = 0x4000;

    /// The maximum number of nodes that can ever be connected
    /// to a network (16). Can be further limited.
    const MAX_NODES: u8 = ctru_sys::UDS_MAXNODES as u8;

    /// The maximum amount of app data any server can provide.
    /// Limited by the size of a struct in libctru.
    const MAX_APPDATA_SIZE: usize = Self::size_of_call(|s: ctru_sys::udsNetworkStruct| s.appdata);

    const fn size_of_call<T, U>(_: fn(T) -> U) -> usize {
        std::mem::size_of::<U>()
    }

    /// Retrieve the current status of the service.
    pub fn service_status(&self) -> ServiceStatus {
        match (self.context, self.network) {
            (None, None) => ServiceStatus::Disconnected,
            (Some(_), None) => ServiceStatus::Client,
            (Some(_), Some(_)) => ServiceStatus::Server,
            _ => unreachable!(),
        }
    }

    /// Initialise a new service handle.
    /// No `new_with_buffer_size` function is provided, as there isn't really a
    /// reason to use any size other than the default.
    ///
    /// The `username` parameter should be a max 10-byte (not 10 code point!) UTF-8 string, converted to UTF-16 internally.
    /// Pass `None` to use the 3DS's configured username.
    ///
    /// # Errors
    ///
    /// This function will return an error if the [`Uds`] service is already being used,
    /// or if the provided username is invalid (longer than 10 bytes or contains a NULL byte).
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::Uds;
    ///
    /// let uds = Uds::new(None)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsInit")]
    pub fn new(username: Option<&str>) -> Result<Self, Error> {
        if let Some(n) = username {
            if n.len() > 10 {
                return Err(Error::UsernameTooLong);
            }
        }
        let cstr = username.map(CString::new).transpose()?;
        let handler = ServiceReference::new(
            &UDS_ACTIVE,
            || {
                let ptr = cstr.map(|c| c.as_ptr()).unwrap_or(null());

                ResultCode(unsafe { ctru_sys::udsInit(Self::SHAREDMEM_SIZE, ptr) })?;

                Ok(())
            },
            || unsafe {
                ctru_sys::udsExit();
            },
        )?;

        Ok(Self {
            _service_handler: handler,
            context: None,
            network: None,
            scan_buf: Box::new([0; Self::SCAN_BUF_SIZE]),
        })
    }

    /// Scan the UDS service for all available beacons broadcasting with the given IDs.
    ///
    /// This function must be called to obtain network objects that can later be connected to.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::Uds;
    /// let mut uds = Uds::new(None)?;
    ///
    /// let networks = uds.scan(b"HBW\x10", None, None)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsScanBeacons")]
    pub fn scan(
        &mut self,
        comm_id: &[u8; 4],
        additional_id: Option<u8>,
        whitelist_macaddr: Option<MacAddr6>,
    ) -> crate::Result<Vec<NetworkScanInfo>> {
        self.scan_buf.fill(0);

        let mut networks = MaybeUninit::uninit();
        let mut total_networks = MaybeUninit::uninit();

        ResultCode(unsafe {
            ctru_sys::udsScanBeacons(
                self.scan_buf.as_mut_ptr().cast(),
                Self::SCAN_BUF_SIZE,
                networks.as_mut_ptr(),
                total_networks.as_mut_ptr(),
                u32::from_be_bytes(*comm_id),
                additional_id.unwrap_or(0),
                whitelist_macaddr
                    .map(|m| m.as_bytes().as_ptr())
                    .unwrap_or(null()),
                self.service_status() == ServiceStatus::Client,
            )
        })?;

        let networks = unsafe { networks.assume_init() };
        let total_networks = unsafe { total_networks.assume_init() };

        let networks = if total_networks > 0 {
            // Safety: `networks` is malloced in application memory with size = `total_networks`
            unsafe { Vec::from_raw_parts(networks, total_networks, total_networks) }
                .into_iter()
                .map(NetworkScanInfo::from)
                .collect()
        } else {
            vec![]
        };

        Ok(networks)
    }

    /// Retrieve app data for a network which the service is not connected to.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::Uds;
    /// let mut uds = Uds::new(None)?;
    ///
    /// let networks = uds.scan(b"HBW\x10", None, None)?;
    /// let appdata = uds.network_appdata(&networks[0], None)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsGetNetworkStructApplicationData")]
    pub fn network_appdata(
        &self,
        network: &NetworkScanInfo,
        max_size: Option<usize>,
    ) -> crate::Result<Vec<u8>> {
        let mut appdata_buffer = vec![
            0u8;
            max_size
                .unwrap_or(Self::MAX_APPDATA_SIZE)
                .min(Self::MAX_APPDATA_SIZE)
        ];

        let mut actual_size = MaybeUninit::uninit();

        ResultCode(unsafe {
            ctru_sys::udsGetNetworkStructApplicationData(
                network.network_ref() as *const _,
                appdata_buffer.as_mut_ptr().cast(),
                appdata_buffer.len(),
                actual_size.as_mut_ptr(),
            )
        })?;

        let actual_size = unsafe { actual_size.assume_init() };

        appdata_buffer.truncate(actual_size);
        appdata_buffer.shrink_to_fit();

        Ok(appdata_buffer)
    }

    /// Retrieve app data for the currently connected network.
    ///
    /// # Errors
    ///
    /// This function will return an error if the service is not connected to a network.
    /// See [`Uds::connect_network()`] to connect to a network.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::{ConnectionType, Uds};
    /// let mut uds = Uds::new(None)?;
    ///
    /// let networks = uds.scan(b"HBW\x10", None, None)?;
    /// uds.connect_network(&networks[0], b"udsdemo passphrase c186093cd2652741\0", ConnectionType::Client, 1)?;
    /// let appdata = uds.appdata(None)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsGetApplicationData")]
    pub fn appdata(&self, max_size: Option<usize>) -> Result<Vec<u8>, Error> {
        if self.service_status() == ServiceStatus::Disconnected {
            return Err(Error::NotConnected);
        }

        let mut appdata_buffer = vec![
            0u8;
            max_size
                .unwrap_or(Self::MAX_APPDATA_SIZE)
                .min(Self::MAX_APPDATA_SIZE)
        ];

        let mut actual_size = MaybeUninit::uninit();

        ResultCode(unsafe {
            ctru_sys::udsGetApplicationData(
                appdata_buffer.as_mut_ptr().cast(),
                appdata_buffer.len(),
                actual_size.as_mut_ptr(),
            )
        })?;

        let actual_size = unsafe { actual_size.assume_init() };

        appdata_buffer.truncate(actual_size);
        appdata_buffer.shrink_to_fit();

        Ok(appdata_buffer)
    }

    /// Connect to a network.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::{ConnectionType, Uds};
    /// let mut uds = Uds::new(None)?;
    ///
    /// let networks = uds.scan(b"HBW\x10", None, None)?;
    /// uds.connect_network(&networks[0], b"udsdemo passphrase c186093cd2652741\0", ConnectionType::Client, 1)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsConnectNetwork")]
    pub fn connect_network(
        &mut self,
        network: &NetworkScanInfo,
        passphrase: &[u8],
        connection_type: ConnectionType,
        channel: u8,
    ) -> crate::Result<()> {
        let mut context = MaybeUninit::uninit();

        ResultCode(unsafe {
            ctru_sys::udsConnectNetwork(
                network.network_ref() as *const _,
                passphrase.as_ptr().cast(),
                passphrase.len(),
                context.as_mut_ptr(),
                NodeID::Broadcast.into(),
                connection_type as u32,
                channel,
                Self::RECV_BUF_SIZE,
            )
        })?;

        let context = unsafe { context.assume_init() };

        self.context.replace(context);

        Ok(())
    }

    /// Disconnect from a network.
    ///
    /// # Errors
    ///
    /// This function will return an error if the service is not connected to a network.
    /// See [`Uds::connect_network()`] to connect to a network.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::{ConnectionType, Uds};
    /// let mut uds = Uds::new(None)?;
    ///
    /// let networks = uds.scan(b"HBW\x10", None, None)?;
    /// uds.connect_network(&networks[0], b"udsdemo passphrase c186093cd2652741\0", ConnectionType::Client, 1)?;
    /// uds.disconnect_network()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsDisconnectNetwork")]
    pub fn disconnect_network(&mut self) -> Result<(), Error> {
        if self.service_status() != ServiceStatus::Client {
            return Err(Error::NotConnected);
        }

        if self.context.is_some() {
            self.unbind_context()?;
        }

        ResultCode(unsafe { ctru_sys::udsDisconnectNetwork() })?;

        Ok(())
    }

    /// Unbind the connection context.
    ///
    /// Normally, there's no reason to call this function,
    /// since [`Uds::disconnect_network()`] and [`Uds::destroy_network()`] both automatically unbind their contexts.
    ///
    /// # Errors
    ///
    /// This function will return an error if no context is currently bound (i.e. the service is neither connected to nor hosting a network).
    /// See [`Uds::connect_network()`] to connect to a network or [`Uds::create_network()`] to create one.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::{ConnectionType, Uds};
    /// let mut uds = Uds::new(None)?;
    ///
    /// let networks = uds.scan(b"HBW\x10", None, None)?;
    /// uds.connect_network(&networks[0], b"udsdemo passphrase c186093cd2652741\0", ConnectionType::Client, 1)?;
    /// uds.unbind_context()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsUnbind")]
    pub fn unbind_context(&mut self) -> Result<(), Error> {
        if let Some(mut ctx) = self.context {
            ResultCode(unsafe { ctru_sys::udsUnbind(&mut ctx as *mut _) })?;
        } else {
            return Err(Error::NoContext);
        }

        self.context = None;

        Ok(())
    }

    /// Returns the Wi-Fi channel currently in use.
    ///
    /// # Errors
    ///
    /// This function will return an error if the service is currently neither connected to nor hosting a network.
    /// See [`Uds::connect_network()`] to connect to a network or [`Uds::create_network()`] to create one.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::{ConnectionType, Uds};
    /// let mut uds = Uds::new(None)?;
    ///
    /// let networks = uds.scan(b"HBW\x10", None, None)?;
    /// uds.connect_network(&networks[0], b"udsdemo passphrase c186093cd2652741\0", ConnectionType::Client, 1)?;
    /// let channel = uds.channel()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsGetChannel")]
    pub fn channel(&self) -> Result<u8, Error> {
        if self.service_status() == ServiceStatus::Disconnected {
            return Err(Error::NotConnected);
        }

        let mut channel = MaybeUninit::uninit();

        ResultCode(unsafe { ctru_sys::udsGetChannel(channel.as_mut_ptr()) })?;

        let channel = unsafe { channel.assume_init() };

        Ok(channel)
    }

    /// Wait for a ConnectionStatus event to occur.
    ///
    /// If `next` is `true`, discard the current event (if any) and wait for the next one.
    ///
    /// If `wait` is `true`, block until an event is signalled, else return `false` if no event.
    ///
    /// Always returns `true`, unless `wait` is `false` and no event has been signalled.
    ///
    /// # Errors
    ///
    /// This function will return an error if the service is currently neither connected to nor hosting a network.
    /// See [`Uds::connect_network()`] to connect to a network or [`Uds::create_network()`] to create one.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::{ConnectionType, Uds};
    /// let mut uds = Uds::new(None)?;
    ///
    /// let networks = uds.scan(b"HBW\x10", None, None)?;
    /// uds.connect_network(&networks[0], b"udsdemo passphrase c186093cd2652741\0", ConnectionType::Client, 1)?;
    /// if uds.wait_status_event(false, false)? {
    ///     println!("Event signalled");
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsWaitConnectionStatusEvent")]
    pub fn wait_status_event(&self, next: bool, wait: bool) -> Result<bool, Error> {
        if self.service_status() == ServiceStatus::Disconnected {
            return Err(Error::NotConnected);
        }

        Ok(unsafe { ctru_sys::udsWaitConnectionStatusEvent(next, wait) })
    }

    /// Returns the current [`ConnectionStatus`] struct.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::{ConnectionType, Uds};
    /// let mut uds = Uds::new(None)?;
    ///
    /// let networks = uds.scan(b"HBW\x10", None, None)?;
    /// uds.connect_network(&networks[0], b"udsdemo passphrase c186093cd2652741\0", ConnectionType::Client, 1)?;
    /// if uds.wait_status_event(false, false)? {
    ///     println!("Connection status event signalled");
    ///     let status = uds.connection_status()?;
    ///     println!("Status: {status:#X?}");
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsGetConnectionStatus")]
    pub fn connection_status(&self) -> crate::Result<ConnectionStatus> {
        let mut status = MaybeUninit::uninit();

        ResultCode(unsafe { ctru_sys::udsGetConnectionStatus(status.as_mut_ptr()) })?;

        let status = unsafe { status.assume_init() };

        Ok(status.into())
    }

    /// Send a packet to the network.
    ///
    /// TODO: max size?
    ///
    /// # Errors
    ///
    /// This function will return an error if the service is currently neither connected to nor hosting a network.
    /// See [`Uds::connect_network()`] to connect to a network or [`Uds::create_network()`] to create one.
    /// It will also return an error if the service is currently connected to a network as a spectator, as spectators cannot send data, only receive it.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::{ConnectionType, NodeID, SendFlags, Uds};
    /// let mut uds = Uds::new(None)?;
    ///
    /// let networks = uds.scan(b"HBW\x10", None, None)?;
    /// uds.connect_network(&networks[0], b"udsdemo passphrase c186093cd2652741\0", ConnectionType::Client, 1)?;
    /// uds.send_packet(b"Hello, World!", NodeID::Broadcast, 1, SendFlags::Default)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsSendTo")]
    pub fn send_packet(
        &self,
        packet: &[u8],
        address: NodeID,
        channel: u8,
        flags: SendFlags,
    ) -> Result<(), Error> {
        if self.service_status() == ServiceStatus::Disconnected {
            return Err(Error::NotConnected);
        }

        if self.context.unwrap().spectator {
            return Err(Error::Spectator);
        }

        let code = ResultCode(unsafe {
            ctru_sys::udsSendTo(
                address.into(),
                channel,
                flags.bits(),
                packet.as_ptr().cast(),
                packet.len(),
            )
        });

        if code.0
            != ctru_sys::MAKERESULT(
                ctru_sys::RL_STATUS as _,
                ctru_sys::RS_OUTOFRESOURCE as _,
                ctru_sys::RM_UDS as _,
                ctru_sys::RD_BUSY as _,
            )
        {
            code?;
        }

        Ok(())
    }

    /// Pull a packet from the network.
    ///
    /// # Errors
    ///
    /// This function will return an error if the service is currently neither connected to nor hosting a network.
    /// See [`Uds::connect_network()`] to connect to a network or [`Uds::create_network()`] to create one.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::{ConnectionType, Uds};
    /// let mut uds = Uds::new(None)?;
    ///
    /// let networks = uds.scan(b"HBW\x10", None, None)?;
    /// uds.connect_network(&networks[0], b"udsdemo passphrase c186093cd2652741\0", ConnectionType::Client, 1)?;
    /// let packet = uds.pull_packet()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsPullPacket")]
    pub fn pull_packet(&self) -> Result<Option<(Vec<u8>, NodeID)>, Error> {
        if self.service_status() == ServiceStatus::Disconnected {
            return Err(Error::NotConnected);
        }

        let mut frame = MaybeUninit::<[u8; Self::RECV_FRAME_SIZE]>::zeroed();

        let mut actual_size = MaybeUninit::uninit();
        let mut src_node_id = MaybeUninit::uninit();

        ResultCode(unsafe {
            ctru_sys::udsPullPacket(
                &self.context.unwrap() as *const _,
                frame.as_mut_ptr().cast(),
                Self::RECV_FRAME_SIZE,
                actual_size.as_mut_ptr(),
                src_node_id.as_mut_ptr(),
            )
        })?;

        let frame = unsafe { frame.assume_init() };
        let actual_size = unsafe { actual_size.assume_init() };
        let src_node_id = unsafe { src_node_id.assume_init() };

        Ok(if actual_size == 0 {
            None
        } else {
            // TODO: to_vec() first, then truncate() and shrink_to_fit()?
            Some((
                frame[..actual_size].to_vec(),
                src_node_id
                    .try_into()
                    .expect("UDS service should always provide a valid NetworkNodeID"),
            ))
        })
    }

    /// Create a new network.
    ///
    /// # Errors
    ///
    /// This function will return an error if the [`Uds`] service is already being used.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::Uds;
    /// let mut uds = Uds::new(None)?;
    ///
    /// uds.create_network(b"HBW\x10", None, None, b"udsdemo passphrase c186093cd2652741\0", 1)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsCreateNetwork")]
    pub fn create_network(
        &mut self,
        comm_id: &[u8; 4],
        additional_id: Option<u8>,
        max_nodes: Option<u8>,
        passphrase: &[u8],
        channel: u8,
    ) -> crate::Result<()> {
        let mut network = MaybeUninit::uninit();
        unsafe {
            ctru_sys::udsGenerateDefaultNetworkStruct(
                network.as_mut_ptr(),
                u32::from_be_bytes(*comm_id),
                additional_id.unwrap_or(0),
                max_nodes.unwrap_or(Self::MAX_NODES).min(Self::MAX_NODES),
            )
        };

        let network = unsafe { network.assume_init() };

        let mut context = MaybeUninit::uninit();

        ResultCode(unsafe {
            ctru_sys::udsCreateNetwork(
                &network as *const _,
                passphrase.as_ptr().cast(),
                passphrase.len(),
                context.as_mut_ptr(),
                channel,
                Self::RECV_BUF_SIZE,
            )
        })?;

        let context = unsafe { context.assume_init() };

        self.network.replace(network);

        self.context.replace(context);

        Ok(())
    }

    /// Destroy the current network.
    ///
    /// # Errors
    ///
    /// This function will return an error if no network has been created.
    /// See [`Uds::create_network()`] to create a network.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::Uds;
    /// let mut uds = Uds::new(None)?;
    ///
    /// uds.create_network(b"HBW\x10", None, None, b"udsdemo passphrase c186093cd2652741\0", 1)?;
    /// uds.destroy_network()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsDestroyNetwork")]
    pub fn destroy_network(&mut self) -> Result<(), Error> {
        if self.service_status() != ServiceStatus::Server {
            return Err(Error::NoNetwork);
        }

        // should always be true
        if self.context.is_some() {
            self.unbind_context()?;
        }

        ResultCode(unsafe { ctru_sys::udsDestroyNetwork() })?;

        self.network = None;

        Ok(())
    }

    /// Set the app data for the currently hosted network.
    ///
    /// # Errors
    ///
    /// This function will return an error if no network has been created.
    /// See [`Uds::create_network()`] to create a network.
    /// This function will also return an error if the provided buffer is too large (see [`Uds::MAX_APPDATA_SIZE`]).
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::Uds;
    /// let mut uds = Uds::new(None)?;
    ///
    /// uds.create_network(b"HBW\x10", None, None, b"udsdemo passphrase c186093cd2652741\0", 1)?;
    /// uds.set_appdata(b"Test appdata.\0")?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsSetApplicationData")]
    pub fn set_appdata(&self, data: &[u8]) -> Result<(), Error> {
        if self.service_status() != ServiceStatus::Server {
            return Err(Error::NoNetwork);
        }

        if data.len() > Self::MAX_APPDATA_SIZE {
            return Err(Error::TooMuchAppData);
        }

        ResultCode(unsafe { ctru_sys::udsSetApplicationData(data.as_ptr().cast(), data.len()) })?;

        Ok(())
    }

    /// Wait for a bind event to occur.
    ///
    /// If `next` is `true`, discard the current event (if any) and wait for the next one.
    ///
    /// If `wait` is `true`, block until an event is signalled, else return `false` if no event.
    ///
    /// Always returns `true`, unless `wait` is `false` and no event has been signalled.
    ///
    /// # Errors
    ///
    /// This function will return an error if the service is currently neither connected to nor hosting a network.
    /// See [`Uds::connect_network()`] to connect to a network or [`Uds::create_network()`] to create one.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::{ConnectionType, Uds};
    /// let mut uds = Uds::new(None)?;
    ///
    /// let networks = uds.scan(b"HBW\x10", None, None)?;
    /// uds.connect_network(&networks[0], b"udsdemo passphrase c186093cd2652741\0", ConnectionType::Client, 1)?;
    /// if uds.wait_data_available(false, false)? {
    ///     println!("Data available");
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsWaitConnectionStatusEvent")]
    pub fn wait_data_available(&self, next: bool, wait: bool) -> Result<bool, Error> {
        if self.service_status() == ServiceStatus::Disconnected {
            return Err(Error::NotConnected);
        }

        Ok(unsafe {
            ctru_sys::udsWaitDataAvailable(&self.context.unwrap() as *const _, next, wait)
        })
    }

    /// Eject a client from the network.
    ///
    /// # Errors
    ///
    /// This function will return an error if no network has been created.
    /// See [`Uds::create_network()`] to create a network.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::{NodeID, Uds};
    /// let mut uds = Uds::new(None)?;
    ///
    /// uds.create_network(b"HBW\x10", None, None, b"udsdemo passphrase c186093cd2652741\0", 1)?;
    /// uds.eject_client(NodeID::Node(2))?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsEjectClient")]
    pub fn eject_client(&self, address: NodeID) -> Result<(), Error> {
        if self.service_status() != ServiceStatus::Server {
            return Err(Error::NoNetwork);
        }

        ResultCode(unsafe { ctru_sys::udsEjectClient(address.into()) })?;

        Ok(())
    }

    /// Allow or disallow spectators on the network.
    ///
    /// Disallowing spectators will disconnect all spectators currently observing the network.
    ///
    /// # Errors
    ///
    /// This function will return an error if no network has been created.
    /// See [`Uds::create_network()`] to create a network.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::Uds;
    /// let mut uds = Uds::new(None)?;
    ///
    /// uds.create_network(b"HBW\x10", None, None, b"udsdemo passphrase c186093cd2652741\0", 1)?;
    /// uds.allow_spectators(false)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsEjectSpectator")]
    #[doc(alias = "udsAllowSpectators")]
    pub fn allow_spectators(&mut self, allow: bool) -> Result<(), Error> {
        if self.service_status() != ServiceStatus::Server {
            return Err(Error::NoNetwork);
        }

        ResultCode(unsafe {
            if allow {
                ctru_sys::udsAllowSpectators()
            } else {
                ctru_sys::udsEjectSpectator()
            }
        })?;

        Ok(())
    }

    /// Allow or disallow new clients on the network.
    ///
    /// Disallowing new clients will not disconnect any currently connected clients.
    ///
    /// # Errors
    ///
    /// This function will return an error if no network has been created.
    /// See [`Uds::create_network()`] to create a network.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::Uds;
    /// let mut uds = Uds::new(None)?;
    ///
    /// uds.create_network(b"HBW\x10", None, None, b"udsdemo passphrase c186093cd2652741\0", 1)?;
    /// uds.allow_new_clients(false)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsSetNewConnectionsBlocked")]
    pub fn allow_new_clients(&mut self, allow: bool) -> Result<(), Error> {
        if self.service_status() != ServiceStatus::Server {
            return Err(Error::NoNetwork);
        }

        ResultCode(unsafe { ctru_sys::udsSetNewConnectionsBlocked(!allow, true, false) })?;

        Ok(())
    }

    /// Returns the [`NodeInfo`] struct for the specified network node.
    ///
    /// # Errors
    ///
    /// This function will return an error if [`NodeID::None`] or [`NodeID::Broadcast`] is passed.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::uds::{NodeID, Uds};
    /// let mut uds = Uds::new(None)?;
    ///
    /// uds.create_network(b"HBW\x10", None, None, b"udsdemo passphrase c186093cd2652741\0", 1)?;
    /// let node_info = uds.node_info(NodeID::Node(2))?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "udsGetNodeInformation")]
    pub fn node_info(&self, address: NodeID) -> Result<NodeInfo, Error> {
        let NodeID::Node(node) = address else {
            return Err(Error::NotANode);
        };

        let mut info = MaybeUninit::uninit();

        ResultCode(unsafe { ctru_sys::udsGetNodeInformation(node as u16, info.as_mut_ptr()) })?;

        let info = unsafe { info.assume_init() };

        Ok(info.into())
    }
}

impl Drop for Uds {
    #[doc(alias = "udsExit")]
    fn drop(&mut self) {
        match self.service_status() {
            ServiceStatus::Client => self.disconnect_network().unwrap(),
            ServiceStatus::Server => self.destroy_network().unwrap(),
            _ => {}
        };
        // ctru_sys::udsExit() is called by the ServiceHandle
    }
}
