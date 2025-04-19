//! Allocates an image memory for an image having its dimensions defined by `width` and `height`
//! and its color depth defined by `bitspixel`.
//!
//! The memory size is:
//! ```txt
//! size = [width * int((bitspixel + 7) / 8)] * height
//! ```
//!
//! The line increment is calculated as:
//! ```txt
//! lineinc = width * int[(bitspixel + 7) / 8]
//! ```
//!
//! To read out the line increment, you can use the [`is_GetImgMemPitch`] function.
//!
//! The starting address of the memory area is returned in `ppcMem`.
//! `pid` returns an ID for the allocated memory. A newly allocated memory is not directly active,
//! i.e. digitised images will not be stored immediately in this new memory. It must first be made
//! active using [`is_SetImageMem`].
//!
//! The returned pointer must be write-protected and may not be altered because it will be used for
//! all further `ImageMem` functions. To release the memory, you can use [`is_FreeImageMem`].
//!
//! In the Direct3D or OpenGL modes, image memory allocation is not necessary.
//!
//! `RGB16` and `RGB15` require the same amount of memory, but can be distinguished by the
//! `bitspixel` parameter. For information on the bit depths of different color formats please
//! refer to the
//! [Appendix: Color and memory formats](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk_allgemeines_farbformate.html) chapter.
//!
//! <div class="warning">
//! In case the operating system is short of physical memory, today's OS versions swap individual
//! areas of the RAM that have not been used for some time out to the slower hard disk. This can
//! slow down image capture if more image memory has been allocated than can be provided by the
//! RAM at a time.
//! </div>
//!
//! # Documentation
//! [is_AllocImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_allocimagemem.html)

use crate::constants::return_values::*;
use crate::types::{char, HIDS, INT};

unsafe extern "C" {
    /// Allocates an image memory for an image having its dimensions defined by `width` and `height`
    /// and its color depth defined by `bitspixel`.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `width` - Image width.
    /// * `height` - Image height.
    /// * `bitspixel` - Image bit depth (bits per pixel).
    /// * `ppcMem` - Returns the pointer to the memory starting address.
    /// * `pnMemId` - Returns the ID of this memory.
    ///
    /// # Return values
    /// * [`IS_CANT_ADD_TO_SEQUENCE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_OUT_OF_MEMORY`]
    /// * [`IS_SEQUENCE_BUF_ALREADY_LOCKED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_FreeImageMem`]
    /// * [`is_AddToSequence`]
    /// * [`is_SetImageMem`]
    /// * [`is_SetAllocatedImageMem`]
    /// * [`is_GetColorDepth`]
    /// * [`is_GetImgMemPitch`]
    ///
    /// # Documentation
    /// [is_AllocImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_allocimagemem.html)
    pub fn is_AllocImageMem(
        hCam: HIDS,
        width: INT,
        height: INT,
        bitspixel: INT,
        ppcMem: *mut *const char,
        pnMemId: *mut INT,
    ) -> INT;
}
