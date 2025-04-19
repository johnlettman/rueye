//! Adjust the latency of the image data transfer of GigE _uEye_ cameras.
//!
//! You can adjust the interval between capturing and transferring an image for each individual
//! camera that is connected. This may be useful to divide available bandwidth amongst cameras
//! running simultaneously on slow networks (e.g. transfer in a wireless or 100 Mbit network).
//! Currently, you can adjust the following parameters:
//! * **Image delay:** This value determines the internal camera delay of the image transfer.
//!     When the sensor begins outputting data, the camera waits for the specified duration before
//!     starting to transfer the image data. In this way you can coordinate the data transfer for
//!     several simultaneously triggered cameras.
//! * **Packet interval** GigE _uEye_ cameras transfer image data in several data packets of the
//!     same size. The "Packet interval" value determines the interval between the transfer of two
//!     successive packets, improving the data transfer of one or several cameras on slow networks.
//!
//! The usual value for the "Packet interval" in Gigabit Ethernet networks is around 20 µs.
//! Higher values for
//! [`TRANSFER_CMD_SET_PACKETINTERVAL_US`][TRANSFER_CMD::TRANSFER_CMD_SET_PACKETINTERVAL_US] can
//! reduce the transfer speed of the GigE _uEye_ camera considerably.
//!
//! Additionally, the function can query if the camera supports the memory mode and set its
//! properties (see [Using the camera memory (GigE uEye cameras)](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk_gige_cameramemory.html)).
//!
//! # Documentation
//! [is_Transfer](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_transfer.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{void, HIDS, INT, RANGE_OF_VALUES_U32, UINT};
use bitflags::bitflags;

bitflags! {
    /// Enumeration of transfer engine's capability flags (_supports bitmask_)
    ///
    /// # Documentation
    /// [is_Transfer: Contents of the TRANSFER_CAPABILITY_FLAGS enumeration](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_transfer.html#transfer_capability_flags)
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct TRANSFER_CAPABILITY_FLAGS: UINT {
        /// Setting the image transfer delay is supported.
        const TRANSFER_CAP_IMAGEDELAY = 0x01;

        /// Setting the packet interval during image transfer is supported.
        const TRANSFER_CAP_PACKETINTERVAL = 0x20;
    }
}

bitflags! {
    /// Enumeration of transfer targets (_supports bitmask_).
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct TRANSFER_TARGET: UINT {
        const IS_TRANSFER_DESTINATION_DEVICE_MEMORY = 1;
        const IS_TRANSFER_DESTINATION_USER_MEMORY = 2;
    }
}

/// Enumeration of commands for [`is_Transfer`].
///
/// # Documentation
/// [is_Transfer](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_transfer.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TRANSFER_CMD {
    /// Returns the function modes supported by the camera.
    ///
    /// # Parameter type
    /// [`TRANSFER_CAPABILITY_FLAGS`], _bitflags_
    TRANSFER_CMD_QUERY_CAPABILITIES = 0,

    /// Sets the internal camera delay of the image transfer in microseconds (µs).
    ///
    /// # Parameter type
    /// [`UINT`]
    TRANSFER_CMD_SET_IMAGEDELAY_US = 1000,

    /// Sets the packet interval for the image transfer in microseconds (µs).
    ///
    /// # Parameter type
    /// [`UINT`]
    TRANSFER_CMD_SET_PACKETINTERVAL_US = 1005,

    /// Returns the internal camera delay of the image transfer in microseconds (µs).
    ///
    /// # Parameter type
    /// [`UINT`]
    TRANSFER_CMD_GET_IMAGEDELAY_US = 2000,

    /// Returns the packet interval of the image transfer in microseconds (µs).
    ///
    /// # Parameter type
    /// [`UINT`]
    TRANSFER_CMD_GET_PACKETINTERVAL_US = 2005,

    /// Returns the value range for the internal camera delay of the image transfer in
    /// microseconds (µs).
    ///
    /// # Parameter type
    /// [`RANGE_OF_VALUES_U32`]
    TRANSFER_CMD_GETRANGE_IMAGEDELAY_US = 3000,

    /// Returns the value range for the packet interval of image transfers in microseconds (µs).
    ///
    /// # Parameter type
    /// [`RANGE_OF_VALUES_U32`]
    TRANSFER_CMD_GETRANGE_PACKETINTERVAL_US = 3005,

    /// Sets the current target memory.
    ///
    /// # Parameter type
    /// [`TRANSFER_TARGET`]
    TRANSFER_CMD_SET_IMAGE_DESTINATION = 5000,

    /// Returns the current target memory.
    ///
    /// # Parameter type
    /// [`TRANSFER_TARGET`]
    TRANSFER_CMD_GET_IMAGE_DESTINATION = 5001,

    /// Returns the current target memory.
    ///
    /// # Parameter type
    /// [`TRANSFER_TARGET`], _bitmask_
    TRANSFER_CMD_GET_IMAGE_DESTINATION_CAPABILITIES = 5002,
}

unsafe extern "C" {
    /// Adjust the latency of the image data transfer of GigE _uEye_ cameras.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`TRANSFER_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `nSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_CaptureStatus`]
    /// * [`is_GetImageInfo`]
    ///
    /// # Documentation
    /// [is_Transfer](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_transfer.html)
    pub fn is_Transfer(
        hCam: HIDS,
        nCommand: TRANSFER_CMD,
        pParam: *mut void,
        cbSizeOfParam: UINT,
    ) -> INT;
}
