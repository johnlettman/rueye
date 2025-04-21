//! Configure the multicast mode of a _GigE uEye_ camera or open a virtual multicast camera.
//!
//! The camera itself must be opened by the master PC to be additionally available as a virtual
//! multicast camera. Only then the virtual multicast camera is free and available for other
//! client PCs.
//!
//! The commands are divided as active and passive multicast commands. Active commands changes
//! parameters in the camera. For this purpose the camera must be available (but not opened) and
//! device ID must be used in the camera handle. Passive multicast commands are used to open a
//! virtual multicast camera by a client in read-only mode.
//!
//! The camera models _UI-526xCP_, _UI-536xCP_, and _UI-537xCP_ of the
//! [_GigE uEye CP_](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/family-gige-ueye-cp.html)
//! camera family do not support the multicast mode.
//!
//! For multicast mode, you must configure the camera with a persistent IP address
//! ([`is_IpConfig`]) and a multicast IP address. Valid multicast IP addresses are from
//! `224.0.0.1` to `239.255.255.255`. However, the address range `224.0.0.0` to `224.0.0.255`,
//! inclusive, is reserved for use by routing protocols and other low-level topology
//! determination or maintenance protocols.
//!
//! The following restrictions apply to multicast cameras:
//! * The persistent IP address of the _GigE uEye_ camera must be in the subnet of the network
//!     adapter. Possible clients of the virtual multicast camera must also be in this subnet.
//! * If the master changes the camera settings, these changes also apply on the image of the
//!     clients. The clients can only change camera-independent functions
//!     (e.g. software gamma or software LUT).
//! * The software hot pixel correction does not work on client PCs.
//! * If multicast mode is enabled ([`IS_PMC_CMD_INITIALIZE`][IS_PMC_CMD::IS_PMC_CMD_INITIALIZE]),
//!     this may affect the performance of the API application.
//!
//! If using a firewall, the rules must be set in that way that the application can receive
//! network packets. Security programs (e.g. antivirus programs) may have a negative impact on
//! the performance of virtual multicast cameras.
//!
//! # Documentation
//! [is_Multicast](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_multicast.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::eth::is_IpConfig;
use crate::eth::UEYE_ETH_ADDR_IPV4;
use crate::types::{void, BOOL, HIDS, INT, NULL, UINT};
use bitflags::bitflags;

/// Active multicast command flag.
pub const IS_MC_CMD_FLAG_ACTIVE: UINT = 0x1000;

/// Passive multicast command flag.
pub const IS_MC_CMD_FLAG_PASSIVE: UINT = 0x2000;

bitflags! {
    /// Flags for multicast mode support.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct IS_AMC_SUPPORTED_FLAG: UINT {
        /// The camera supports the multicast mode.
        const IS_AMC_SUPPORTED_FLAG_DEVICE = 0x0001;

        /// The camera firmware supports the multicast mode (depends on the firmware version).
        const IS_AMC_SUPPORTED_FLAG_FIRMWARE = 0x0002;
    }
}

/// Error handling modes.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum IS_PMC_ERRORHANDLING {
    /// Refuse incomplete images (incorrect received images are discarded).
    IS_PMC_ERRORHANDLING_REJECT_IMAGES = 0x01,

    /// Ignore image errors
    /// (image errors are ignored, that means not received image parts are black).
    IS_PMC_ERRORHANDLING_IGNORE_MISSING_PARTS = 0x02,

    /// Merge mode - release on complete (if a packet loss occurs,
    /// the image display is delayed until all packets have been received again).
    IS_PMC_ERRORHANDLING_MERGE_IMAGES_RELEASE_ON_COMPLETE = 0x03,

    /// Merge mode - release on new image (new images are displayed immediately;
    /// for missing packets, the previous image data is used).
    IS_PMC_ERRORHANDLING_MERGE_IMAGES_RELEASE_ON_RECEIVED_IMGLEN = 0x04,
}

/// Device descriptor structure.
///
/// # Documentation
/// [is_Multicast: Content of the IS_PMC_READONLYDEVICEDESCRIPTOR structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_multicast.html?q=IS_PMC_READONLYDEVICEDESCRIPTOR)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct IS_PMC_READONLYDEVICEDESCRIPTOR {
    /// Camera IP address.
    pub ipCamera: UEYE_ETH_ADDR_IPV4,

    /// Mutlicast IP address.
    pub ipMulticast: UEYE_ETH_ADDR_IPV4,

    /// Camera ID.
    pub u32CameraId: UINT,

    /// Error handling mode.
    ///
    /// An image is transferred in multiple packets over the network.
    /// Under certain circumstances single packets may be lost.
    /// For this case, you set the error handling.
    pub u32ErrorHandlingMode: IS_PMC_ERRORHANDLING,
}

/// Enumeration of commands for [`is_Multicast`].
///
/// # Documentation
/// [is_Multicast](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_multicast.html)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum MULTICAST_CMD {
    // Passive multicast commands
    /// Initializes the passive multicast mode.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_PMC_CMD_INITIALIZE = (0x0001 | IS_MC_CMD_FLAG_PASSIVE),

    /// Deinitializes the passive multicast mode.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_PMC_CMD_DEINITIALIZE = (0x0002 | IS_MC_CMD_FLAG_PASSIVE),

    /// Creates a new virtual multicast camera.
    ///
    /// # Parameter type
    /// [`IS_PMC_READONLYDEVICEDESCRIPTOR`]
    IS_PMC_CMD_ADDMCDEVICE = (0x0003 | IS_MC_CMD_FLAG_PASSIVE),

    /// Removes a virtual multicast camera.
    ///
    /// # Parameter type
    /// [`IS_PMC_READONLYDEVICEDESCRIPTOR`]
    IS_PMC_CMD_REMOVEMCDEVICE = (0x0004 | IS_MC_CMD_FLAG_PASSIVE),

    /// Saves all created multicast cameras in the system configuration.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_PMC_CMD_STOREDEVICES = (0x0005 | IS_MC_CMD_FLAG_PASSIVE),

    /// Loads all multicast cameras from the system configuration.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_PMC_CMD_LOADDEVICES = (0x0006 | IS_MC_CMD_FLAG_PASSIVE),

    /// Enables/disables the passive multicast mode for the entire system.
    ///
    /// The multicast mode must be enabled for the entire system to use all other commands.
    ///
    /// # Parameter type
    /// [`BOOL`]
    IS_PMC_CMD_SYSTEM_SET_ENABLE = (0x0007 | IS_MC_CMD_FLAG_PASSIVE),

    /// Returns the status of the multicast mode for the entire system.
    ///
    /// # Parameter type
    /// [`BOOL`]
    IS_PMC_CMD_SYSTEM_GET_ENABLE = (0x0008 | IS_MC_CMD_FLAG_PASSIVE),

    /// Removes all virtual multicast cameras.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_PMC_CMD_REMOVEALLMCDEVICES = (0x0009 | IS_MC_CMD_FLAG_PASSIVE),

    // Active muticast commands
    /// Sets the multicast IP of a camera.
    ///
    /// # Parameter type
    /// [`UEYE_ETH_ADDR_IPV4`]
    IS_AMC_CMD_SET_MC_IP = (0x0010 | IS_MC_CMD_FLAG_ACTIVE),

    /// Returns the multicast IP of a camera.
    ///
    /// # Parameter type
    /// [`UEYE_ETH_ADDR_IPV4`]
    IS_AMC_CMD_GET_MC_IP = (0x0011 | IS_MC_CMD_FLAG_ACTIVE),

    /// Enables/disables the multicast mode of a camera.
    ///
    /// # Parameter type
    /// [`BOOL`]
    IS_AMC_CMD_SET_MC_ENABLED = (0x0012 | IS_MC_CMD_FLAG_ACTIVE),

    /// Returns the status of the multicast mode of the camera.
    ///
    /// # Parameter type
    /// [`BOOL`]
    IS_AMC_CMD_GET_MC_ENABLED = (0x0013 | IS_MC_CMD_FLAG_ACTIVE),

    /// Returns if the camera supports the multicast mode.
    ///
    /// # Parameter type
    /// [`IS_AMC_SUPPORTED_FLAG`]
    IS_AMC_CMD_GET_MC_SUPPORTED = (0x0014 | IS_MC_CMD_FLAG_ACTIVE),
}

unsafe extern "C" {
    /// Configure the multicast mode of a _GigE uEye_ camera or open a virtual multicast camera.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`MULTICAST_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParams` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_OUT_OF_MEMORY`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Documentation
    /// [is_Multicast](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_multicast.html)
    pub fn is_Multicast(
        hCam: HIDS,
        nCommand: MULTICAST_CMD,
        pParam: *mut void,
        cbSizeOfParams: UINT,
    ) -> INT;
}
