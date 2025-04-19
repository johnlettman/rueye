//! Returns the pointer to the starting address and the ID number of the active image memory.
//!
//! If a Direct3D mode is active and image memory was nevertheless allocated, the pointer to the
//! image memory and its ID will be returned. However, in Direct3D mode, the image will not be
//! copied automatically to this image memory.
//!
//! # Documentation
//! [is_GetActiveImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_getactiveimagemem.html)

use crate::alloc_image_mem::is_AllocImageMem;
use crate::constants::return_values::*;
use crate::set_image_mem::is_SetImageMem;
use crate::get_image_mem::is_GetImageMem;
use crate::types::{char, HIDS, INT};

unsafe extern "C" {
    /// Returns the pointer to the starting address of the active image memory.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `ppcMem` - Returns the pointer to the starting address of the active image memory.
    /// * `pnMemId` - Returns the ID of the active image memory.
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_ACTIVE_IMG_MEM`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_AllocImageMem`]
    /// * [`is_GetImageMem`]
    /// * [`is_SetImageMem`]
    /// * [`is_SetAllocatedImageMem`]
    ///
    /// # Documentation
    /// [is_GetActiveImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_getactiveimagemem.html)
    pub fn is_GetActiveImageMem(hCam: HIDS, ppcMem: *mut *const char, pnMemId: * INT) -> INT;
}
