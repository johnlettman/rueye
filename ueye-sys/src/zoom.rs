//! Control the zoom function of the lens or sensor (digital zoom) if supported by your camera.
//!
//! This function is currently supported by the following camera models:
//! * _UI-1007XS_
//!
//! # Documentation
//! [is_Zoom](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_zoom.html)

use crate::constants::return_values::*;
use crate::types::{void, DOUBLE, HIDS, INT, UINT};
use bitflags::bitflags;

/// Enumeration of commands for [`is_Zoom`].
///
/// # Documentation
/// [is_Zoom](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_zoom.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ZOOM_CMD {
    /// Returns the zoom functions supported by the camera.
    ///
    /// # Parameter type
    /// [`ZOOM_CAPABILITY_FLAGS`], _bitmask_
    ZOOM_CMD_GET_CAPABILITIES = 0,

    /// Returns the number of list entries.
    ///
    /// # Parameter type
    /// [`UINT`]
    ZOOM_CMD_DIGITAL_GET_NUM_LIST_ENTRIES = 1,

    /// Returns a list of supported zoom factors.
    ///
    /// # Parameter type
    /// _Array of:_ [`DOUBLE`]
    ZOOM_CMD_DIGITAL_GET_LIST = 2,

    /// Sets a zoom factor.
    ///
    /// # Parameter type
    /// [`DOUBLE`]
    ZOOM_CMD_DIGITAL_SET_VALUE = 3,

    /// Returns the current zoom factor.
    ///
    /// # Parameter type
    /// [`DOUBLE`]
    ZOOM_CMD_DIGITAL_GET_VALUE = 4,

    /// Returns the digital zoom factor range.
    ZOOM_CMD_DIGITAL_GET_VALUE_RANGE = 5,

    /// Returns the default value of the digital zoom.
    ///
    /// # Parameter type
    /// [`DOUBLE`]
    ZOOM_CMD_DIGITAL_GET_VALUE_DEFAULT = 6,
}

bitflags! {
    /// Zoom functions supported by the camera.
    ///
    /// # Related commands
    /// [`ZOOM_CMD_GET_CAPABILITIES`][ZOOM_CMD::ZOOM_CMD_GET_CAPABILITIES]
    ///
    /// # Documentation
    /// [is_Zoom: Status flags from ZOOM_CAPABILITY_FLAGS](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_zoom.html#zoom_capability_flags)
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct ZOOM_CAPABILITY_FLAGS: UINT {
        /// The camera does not support the zoom function.
        const ZOOM_CAP_INVALID = 0;

        /// The camera supports digital zoom.
        const ZOOM_CAP_DIGITAL_ZOOM = 0x00001;
    }
}

unsafe extern "C" {
    /// Control the zoom function of the lens or sensor (digital zoom) if supported by your camera.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`ZOOM_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `nSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_NULL_POINTER`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Documentation
    /// [is_Zoom](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_zoom.html)
    pub fn is_Zoom(hCam: HIDS, nCommand: ZOOM_CMD, pParam: *mut void, nSizeOfParam: UINT) -> INT;
}
