//! Returns the pointer to the starting address of the active image memory.
//!
//! If you use ring buffering, [`is_GetImageMem`] returns the starting address of the image memory
//! last used for image capturing.
//!
//! # Documentation
//! [is_GetImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_getimagemem.html)

use crate::alloc_image_mem::is_AllocImageMem;
use crate::constants::return_values::*;
use crate::set_image_mem::is_SetImageMem;
use crate::types::{void, HIDS, INT};

unsafe extern "C" {
    /// Returns the pointer to the starting address of the active image memory.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pMem` - Pointer to the starting address of the image memory.
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
    /// * [`is_GetImageMemPitch`]
    /// * [`is_AllocImageMem`]
    /// * [`is_AddToSequence`]
    /// * [`is_SetImageMem`]
    /// * [`is_SetAllocatedImageMem`]
    ///
    /// # Documentation
    /// [is_GetImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_getimagemem.html)
    pub fn is_GetImageMem(hCam: HIDS, pMem: *mut *const void) -> INT;
}
