//! **Obsolete:** Configure scene presets.

#![allow(non_camel_case_types)]

use crate::types::{void, HIDS, INT, UINT};
use bitflags::bitflags;

/// Enumeration of commands for [`is_ScenePreset`].
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum SCENE_CMD {
    /// Get the supported scene presets.
    ///
    /// # Parameter type
    /// [`SCENE_PRESET`], _bitmask_
    SCENE_CMD_GET_SUPPORTED_PRESETS = 1,

    /// Set the scene preset.
    ///
    /// # Parameter type
    /// [`SCENE_PRESET`]
    SCENE_CMD_SET_PRESET = 2,

    /// Get the current sensor scene preset.
    ///
    /// # Parameter type
    /// [`SCENE_PRESET`]
    SCENE_CMD_GET_PRESET = 3,

    /// Get the default sensor scene preset.
    ///
    /// # Parameter type
    /// [`SCENE_PRESET`]
    SCENE_CMD_GET_DEFAULT_PRESET = 4,
}

bitflags! {
    /// Scene presets (_supports bitmask_).
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SCENE_PRESET: UINT {
        const SCENE_INVALID = 0;
        const SCENE_SENSOR_AUTOMATIC = 0x00000001;
        const SCENE_SENSOR_PORTRAIT = 0x00000002;
        const SCENE_SENSOR_SUNNY = 0x00000004;
        const SCENE_SENSOR_ENTERTAINMENT = 0x00000008;
        const SCENE_SENSOR_NIGHT = 0x00000010;
        const SCENE_SENSOR_SPORTS = 0x00000040;
        const SCENE_SENSOR_LANDSCAPE = 0x00000080;
    }
}

unsafe extern "C" {
    /// **Obsolete:** Configure scene presets.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`SCENE_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `nSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    #[deprecated]
    pub fn is_ScenePreset(
        hCam: HIDS,
        nCommand: SCENE_CMD,
        pParam: *mut void,
        nSizeOfParam: UINT,
    ) -> INT;
}
