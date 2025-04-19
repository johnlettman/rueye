//! Defines the settings of the internal image memory queue.
//!
//! You can configure the queue size (in MB) and the number of image memories.
//!
//! # Documentation
//! [is_CaptureConfiguration](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_captureconfiguration.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{void, HIDS, INT, UINT};

/// Enumeration of commands for [`is_CaptureConfiguration`].
///
/// # Documentation
/// [is_CaptureConfiguration](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_captureconfiguration.html)
pub enum CAPTURE_CONFIGURATION_CMD {
    /// Limits the size of the internal image memory queue.
    ///
    /// The queue corresponds to the number of internal image memories.
    /// Default is `0`, i.e. with `0` the queue size is unlimited.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_CAPTURE_CONFIGURATION_CMD_SET_QUEUE_BUFFER_COUNT = 1,

    /// Returns the current setting of the queue.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_CAPTURE_CONFIGURATION_CMD_GET_QUEUE_BUFFER_COUNT = 2,

    /// Sets the current size of the total image memory (in MB).
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_CAPTURE_CONFIGURATION_CMD_SET_INTERNAL_BUFFER_SIZE = 3,

    /// Returns the current size of the total image memory (in MB).
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_CAPTURE_CONFIGURATION_CMD_GET_INTERNAL_BUFFER_SIZE = 4,

    /// Returns the default size of the total image memory (_default: 16 MB_).
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_CAPTURE_CONFIGURATION_CMD_GET_INTERNAL_BUFFER_SIZE_DEFAULT = 5,

    /// Sets the minimum number of image memories (_range: `5`…`256`_).
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_CAPTURE_CONFIGURATION_CMD_SET_INTERNAL_BUFFER_COUNT = 6,

    /// Returns the minimum number of image memories.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_CAPTURE_CONFIGURATION_CMD_GET_INTERNAL_BUFFER_COUNT = 7,

    /// Returns the default value (_default: `5`_).
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_CAPTURE_CONFIGURATION_CMD_GET_INTERNAL_BUFFER_COUNT_DEFAULT = 8,
}

unsafe extern "C" {
    /// Defines the settings of the internal image memory queue.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`CAPTURE_CONFIGURATION_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_CAPTURE_RUNNING`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NULL_POINTER`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_ImageQueue`]
    ///
    /// # Documentation
    /// [is_CaptureConfiguration](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_captureconfiguration.html)
    pub fn is_CaptureConfiguration(
        hCam: HIDS,
        nCommand: CAPTURE_CONFIGURATION_CMD,
        pParam: *mut void,
        cbSizeOfParam: UINT,
    ) -> INT;
}
