//! Enables/disables a software edge filter.
//!
//! Due to Bayer format color conversion, the original edges of a color image may easily
//! become blurred. By enabling the digital edge filter, you can optimize edge representation.
//! This function causes a higher CPU load.
//!
//! For _UI-1007XS_, please use the [`is_Sharpness`] function instead.
//!
//! <div class="warning">
//!
//! The [`is_EdgeEnhancement`] function cannot be used if you have set raw Bayer as
//! [color format](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_setcolormode.html).
//!
//! </div>
//!
//! # Documentation
//! [is_EdgeEnhancement](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_edgeenhancement.html)

#![allow(non_camel_case_types)]

use crate::color::is_SetColorMode;
use crate::sharpness::is_Sharpness;
use crate::types::{void, HIDS, INT, IS_RANGE_U32, UINT};

/// Enumeration of commands for [`is_EdgeEnhancement`].
///
/// # Documentation
/// [is_EdgeEnhancement](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_edgeenhancement.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_EDGE_ENHANCEMENT_CMD {
    /// Returns the range of the edge enhancement.
    ///
    /// # Parameter type
    /// [`IS_RANGE_U32`]
    IS_EDGE_ENHANCEMENT_CMD_GET_RANGE = 1,

    /// Returns the standard value of the edge enhancement.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_EDGE_ENHANCEMENT_CMD_GET_DEFAULT = 2,

    /// Returns the current set edge enhancement.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_EDGE_ENHANCEMENT_CMD_GET = 3,

    /// Sets the edge enhancement. `0` = no edge enhancement.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_EDGE_ENHANCEMENT_CMD_SET = 4,
}

unsafe extern "C" {
    /// Enables/disables a software edge filter.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`IS_EDGE_ENHANCEMENT_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_SetColorMode`]
    /// * [`is_SetColorConverter`]
    ///
    /// # Documentation
    /// [is_EdgeEnhancement](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_edgeenhancement.html)
    pub fn is_EdgeEnhancement(
        hCam: HIDS,
        nCommand: IS_EDGE_ENHANCEMENT_CMD,
        pParam: *mut void,
        cbSizeOfParam: UINT,
    ) -> INT;
}
