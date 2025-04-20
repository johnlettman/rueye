//! **Obsolete:** Configure image stabilization.

#![allow(non_camel_case_types)]

use crate::types::{void, HIDS, INT, UINT};
use bitflags::bitflags;

bitflags! {
    /// Image stabilization compatibility flags.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct IMGSTAB_CAPABILITY_FLAGS: UINT {
        const IMGSTAB_CAP_INVALID = 0;

        /// Image stabilization supported.
        const IMGSTAB_CAP_IMAGE_STABILIZATION_SUPPORTED = 0x00000001;
    }
}

/// Enumeration of commands for [`is_ImageStabilization`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_IMGSTAB_CMD {
    /// Get the capabilities for image stabilization.
    IMGSTAB_CMD_GET_CAPABILITIES = 0,

    /// Disable image stabilization.
    IMGSTAB_CMD_SET_DISABLE = 1,

    /// Enable image stabilization.
    IMGSTAB_CMD_SET_ENABLE = 2,

    /// Returns whether image stabilization is enabled.
    IMGSTAB_CMD_GET_ENABLE = 3,
}

unsafe extern "C" {
    /// Configure image stabilization.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`IS_IMGSTAB_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `nSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    pub fn is_ImageStabilization(
        hCam: HIDS,
        nCommand: IS_IMGSTAB_CMD,
        pParam: *mut void,
        nSizeOfParam: UINT,
    ) -> INT;
}
