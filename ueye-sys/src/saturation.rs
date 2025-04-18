//! Enhance or reduce the color saturation if supported by your _uEye_ model.
//!
//! This function is currently supported by the following camera models:
//! * _UI-1007XS_
//!
//! # Documentation
//! [is_Saturation](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_saturation.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{void, HIDS, INT, UINT};
use bitflags::bitflags;

/// Enumeration of commands for [`is_Saturation`].
///
/// # Documentation
/// [is_Saturation](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_saturation.html)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum SATURATION_CMD {
    /// Returns the function modes supported by the camera.
    ///
    /// # Parameter type
    /// [`SATURATION_CAPABILITY_FLAGS`], _bitmask_
    SATURATION_CMD_GET_CAPABILITIES = 0,

    /// Returns the current color saturation setting.
    ///
    /// # Parameter type
    /// [`INT`]
    SATURATION_CMD_GET_VALUE = 1,

    /// Returns the minimum value for color saturation.
    ///
    /// # Parameter type
    /// [`INT`]
    SATURATION_CMD_GET_MIN_VALUE = 2,

    /// Returns the maximum value for color saturation.
    ///
    /// # Parameter type
    /// [`INT`]
    SATURATION_CMD_GET_MAX_VALUE = 3,

    /// Returns the increment for setting the color saturation.
    ///
    /// # Parameter type
    /// [`INT`]
    SATURATION_CMD_GET_INCREMENT = 4,

    /// Returns the default value for color saturation.
    ///
    /// # Parameter type
    /// [`INT`]
    SATURATION_CMD_GET_DEFAULT_VALUE = 5,

    /// Sets a value for color saturation.
    ///
    /// # Parameter type
    /// [`INT`]
    SATURATION_CMD_SET_VALUE = 6,
}

bitflags! {
    /// Saturation support compatibility flags (_supports bitmask_).
    ///
    /// # Related commands
    /// * [`SATURATION_CMD_GET_CAPABILITIES`][SATURATION_CMD::SATURATION_CMD_GET_CAPABILITIES]
    ///
    /// # Documentation
    /// [is_Saturation: Status flags from SATURATION_CAPABILITY_FLAGS](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_saturation.html#saturation_capability_flags)
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct SATURATION_CAPABILITY_FLAGS: UINT {
        /// The camera does not support setting the color saturation.
        const SATURATION_CAP_INVALID = 0x0000;

        /// The camera supports setting the color saturation.
        const SATURATION_CAP_SATURATION_SUPPORTED = 0x0001;
    }
}

unsafe extern "C" {
    /// Enhance or reduce the color saturation if supported by your _uEye_ model.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`SATURATION_CMD`].
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
    /// # Related functions
    /// * [`is_SetSaturation`]
    ///
    /// # Documentation
    /// [is_Saturation](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_saturation.html)
    pub fn is_Saturation(
        hCam: HIDS,
        nCommand: SATURATION_CMD,
        pParam: *mut void,
        nSizeOfParam: UINT,
    ) -> INT;
}
