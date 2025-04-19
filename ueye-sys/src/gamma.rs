//! Enables digital gamma correction which applies a gamma characteristic to the image.
//!
//! When hardware color conversion is used on _USB 3 uEye CP/LE/ML_ cameras, the gamma
//! correction is performed in the camera hardware as well. When the color conversion is
//! performed in the PC (software conversion) the gamma correction is performed in software.
//!
//! Typical values for gamma range between `1.6` (`160`) and `2.2` (`220`).
//!
//! When the color format is set to Raw Bayer the gamma correction can not be used.
//!
//! # Documentation
//! [is_Gamma](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_gamma.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{void, HIDS, INT, UINT};

/// Minimum gamma value.
pub const IS_GAMMA_VALUE_MIN: INT = 1;

/// Maximum gamma value.
pub const IS_GAMMA_VALUE_MAX: INT = 1000;

/// Enumeration of commands for [`is_Gamma`].
///
/// # Documentation
/// [is_Gamm](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_gamma.html)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum GAMMA_CMD {
    /// Gamma value to be set, multiplied by `100`.
    ///
    /// # Parameter type
    /// [`INT`], _range of:_ [`IS_GAMMA_VALUE_MIN`]…[`IS_GAMMA_VALUE_MAX`]
    IS_GAMMA_CMD_SET = 0x0001,

    /// Returns the standard gamma value.
    ///
    /// Default = `100`, corresponds to a gamma value of `1.0`.
    ///
    /// # Parameter type
    /// [`INT`], _range of:_ [`IS_GAMMA_VALUE_MIN`]…[`IS_GAMMA_VALUE_MAX`]
    IS_GAMMA_CMD_GET_DEFAULT = 0x0002,

    /// Returns the current set gamma value.
    ///
    /// # Parameter type
    /// [`INT`], _range of:_ [`IS_GAMMA_VALUE_MIN`]…[`IS_GAMMA_VALUE_MAX`]
    IS_GAMMA_CMD_GET = 0x0003,
}

unsafe extern "C" {
    /// Enables digital gamma correction which applies a gamma characteristic to the image.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`GAMMA_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParams` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_SetHardwareGamma`]
    ///
    /// # Documentation
    /// [is_Gamma](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_gamma.html)
    pub fn is_Gamma(
        hCam: HIDS,
        nCommand: GAMMA_CMD,
        pParam: *mut void,
        cbSizeOfParams: UINT,
    ) -> INT;
}
