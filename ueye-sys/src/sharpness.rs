//! Enhance or reduce the image sharpness if supported by your _uEye_ model.
//!
//! While [`is_Sharpness`] is a sensor function, [`is_EdgeEnhancement`] performs the
//! image sharpness on software-side.
//!
//! This function is currently supported by the following camera models:
//! * _UI-1007XS_
//!
//! # Documentation
//! [is_Sharpness](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_sharpness.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{void, HIDS, INT, UINT};
use bitflags::bitflags;

/// Enumeration of commands for [`is_Sharpness`].
///
/// # Documentation
/// [is_Sharpness](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_sharpness.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SHARPNESS_CMD {
    /// Returns the function modes supported by the camera.
    ///
    /// # Parameter type
    /// [`SHARPNESS_CAPABILITY_FLAGS`], _bitmask_
    SHARPNESS_CMD_GET_CAPABILITIES = 0,

    /// Returns the current image sharpness setting.
    ///
    /// # Parameter type
    /// [`INT`]
    SHARPNESS_CMD_GET_VALUE = 1,

    /// Returns the minimum image sharpness value.
    ///
    /// # Parameter type
    /// [`INT`]
    SHARPNESS_CMD_GET_MIN_VALUE = 2,

    /// Returns the maximum image sharpness value.
    ///
    /// # Parameter type
    /// [`INT`]
    SHARPNESS_CMD_GET_MAX_VALUE = 3,

    /// Returns the increment for setting the image sharpness.
    ///
    /// # Parameter type
    /// [`UINT`]
    SHARPNESS_CMD_GET_INCREMENT = 4,

    /// Returns the default value for image sharpness.
    ///
    /// # Parameter type
    /// [`INT`]
    SHARPNESS_CMD_GET_DEFAULT_VALUE = 5,

    /// Sets a value for image sharpness.
    ///
    /// # Parameter type
    /// [`INT`]
    SHARPNESS_CMD_SET_VALUE = 6,
}

bitflags! {
    /// Sharpness support compatibility flags.
    ///
    /// # Related commands
    /// * [`SHARPNESS_CMD_GET_CAPABILITIES`][SHARPNESS_CMD::SHARPNESS_CMD_GET_CAPABILITIES]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct SHARPNESS_CAPABILITY_FLAGS: UINT {
        /// The camera does not support setting the image sharpness.
        const SHARPNESS_CAP_INVALID = 0x0000;

        /// The camera supports setting the image sharpness.
        const SHARPNESS_CAP_SHARPNESS_SUPPORTED = 0x0001;
    }
}

unsafe extern "C" {
    /// Enhance or reduce the image sharpness if supported by your _uEye_ model.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`SHARPNESS_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
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
    /// * [`is_EdgeEnhancement`]
    ///
    /// # Documentation
    /// [is_Sharpness](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_sharpness.html)
    pub fn is_Sharpness(
        hCam: HIDS,
        nCommand: SHARPNESS_CMD,
        pParam: *mut void,
        nSizeOfParam: UINT,
    ) -> INT;
}
