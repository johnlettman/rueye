#![allow(non_camel_case_types)]

use bitflags::bitflags;
use crate::constants::return_values::*;
use crate::types::{HIDS, INT, UINT, void, IS_RANGE_S32};

/// Enumeration of modes of function [`is_Blacklevel`].
///
/// # Documentation
/// [`is_Blacklevel`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_blacklevel.html)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum BLACKLEVEL_MODES {
    /// The automatic black level mode is switched off.
    IS_AUTO_BLACKLEVEL_OFF = 0,

    /// The automatic black level mode is switched on.
    IS_AUTO_BLACKLEVEL_ON  = 1
}

impl From<bool> for BLACKLEVEL_MODES {
    #[inline]
    fn from(value: bool) -> Self {
        if value { Self::IS_AUTO_BLACKLEVEL_ON } else { Self::IS_AUTO_BLACKLEVEL_OFF }
    }
}

bitflags! {
    /// Enumeration of capabilities of function [`is_Blacklevel`].
    ///
    /// # Documentation
    /// [`is_Blacklevel`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_blacklevel.html)
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct BLACKLEVEL_CAPS: UINT {
        /// The state of the automatic black level can be changed.
        ///
        /// The flag does not indicate whether the camera is running with auto black level by
        /// default or not.For this purpose, use IS_BLACKLEVEL_CMD_GET_MODE_DEFAULT.
        const     IS_BLACKLEVEL_CAP_SET_AUTO_BLACKLEVEL   = 1;

        /// The offset can be changed.
        ///
        /// The flag does not indicate whether the camera has set an offset by default or not.
        /// For this purpose, use IS_BLACKLEVEL_CMD_GET_OFFSET_DEFAULT.
        const IS_BLACKLEVEL_CAP_SET_OFFSET            = 2;
    }
}

/// Enumeration of commands of function [`is_Blacklevel`].
///
/// # Documentation
/// [`is_Blacklevel`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_blacklevel.html)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum BLACKLEVEL_CMD {
    /// Returns the black level feature of the camera.
    IS_BLACKLEVEL_CMD_GET_CAPS           = 1,

    /// Returns the default black level mode.
    ///
    /// # Parameter type
    /// [`BLACKLEVEL_MODES`]
    IS_BLACKLEVEL_CMD_GET_MODE_DEFAULT   = 2,

    /// Returns the current black level mode.
    ///
    /// # Parameter type
    /// [`BLACKLEVEL_MODES`]
    IS_BLACKLEVEL_CMD_GET_MODE           = 3,

    /// Sets the black level mode.
    ///
    /// # Parameter type
    /// [`BLACKLEVEL_MODES`]
    IS_BLACKLEVEL_CMD_SET_MODE           = 4,

    /// Returns the default offset.
    ///
    /// # Parameter type
    /// [`INT`]
    IS_BLACKLEVEL_CMD_GET_OFFSET_DEFAULT = 5,

    /// Returns the range of the offset.
    ///
    /// # Parameter type
    /// [`IS_RANGE_S32`]
    IS_BLACKLEVEL_CMD_GET_OFFSET_RANGE   = 6,

    /// Returns the current offset.
    ///
    /// # Parameter type
    /// [`INT`]
    IS_BLACKLEVEL_CMD_GET_OFFSET         = 7,

    /// Sets the offset.
    ///
    /// # Parameter type
    /// [`INT`]
    IS_BLACKLEVEL_CMD_SET_OFFSET         = 8
}



unsafe extern "C" {
    /// Controls the black level correction of the camera which might improve the image quality
    /// under certain circumstances.
    ///
    /// By default, the sensor adjusts the black level value for each pixel automatically.
    /// If the environment is very bright, it can be necessary to adjust the black level manually.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`BLACKLEVEL_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Documentation
    /// [`is_Blacklevel`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_blacklevel.html)
    pub fn is_Blacklevel(hCam: HIDS, nCommand: BLACKLEVEL_CMD, pParam: *mut void, cbSizeOfParam: UINT) -> INT;
}
