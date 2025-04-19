//! Returns the line increment (in bytes) of the active image memory.
//!
//! The line increment is defined as the number of bytes from the beginning of a line to the
//! beginning of the next line. The line increment is calculated as:
//! ```txt
//! lineinc = width * int[(bitspixel + 7) / 8]
//! ```
//!
//! # Documentation
//! [is_GetImageMemPitch](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_getimagemempitch.html)

use crate::alloc_image_mem::is_AllocImageMem;
use crate::constants::return_values::*;
use crate::set_image_mem::is_SetImageMem;
use crate::get_image_mem::is_GetImageMem;
use crate::types::{HIDS, INT};

unsafe extern "C" {
    /// Returns the line increment (in bytes) of the active image memory.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pPitch` - Pointer to the variable containing the line increment
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_GetImageMem`]
    /// * [`is_AllocImageMem`]
    /// * [`is_AddToSequence`]
    /// * [`is_SetImageMem`]
    /// * [`is_SetAllocatedImageMem`]
    ///
    /// # Documentation
    /// [is_GetImageMemPitch](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_getimagemempitch.html)
    pub fn is_GetImageMemPitch(hCam: HIDS, pPitch: * INT) -> INT;
}
