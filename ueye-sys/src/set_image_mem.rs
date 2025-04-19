//! Makes the specified image memory the active memory.
//!
//! Only an active image memory can receive image data. When you call [`is_FreezeVideo`], the
//! captured image is stored in the image buffer designated by `pcMem` and `nMemId`. For `pcMem`,
//! you must pass a pointer which was created by [`is_AllocImageMem`], passing any other pointer
//! will result in an error message. You may pass the same pointer multiple times.
//!
//! In the Direct3D or OpenGL modes, there is no need to set an image memory.
//!
//! If the specified image memory is set as active memory, a previously created sequence list is
//! deleted and a new sequence list is created with this image memory. If image memories of the
//! already existing sequence list are locked by the user and thus signaled as used, the image
//! memories cannot be removed from the sequence list. So the sequence list is not deleted.
//! Therefore, before calling [`is_SetImageMem`], **all** image memories locked using
//! [`is_LockSeqBuf`] **have to be** unlocked using [`is_UnlockSeqBuf`] first.
//!
//! # Documentation
//! [is_SetImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_setimagemem.html)

use crate::alloc_image_mem::is_AllocImageMem;
use crate::constants::return_values::*;
use crate::types::{char, HIDS, INT};

unsafe extern "C" {
    /// Makes the specified image memory the active memory.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pcMem` - Pointer to the starting position in the memory.
    /// * `nMemId` - ID of this memory.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SEQ_BUFFER_IS_LOCKED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_AllocImageMem`]
    /// * [`is_FreeImageMem`]
    /// * [`is_AddToSequence`]
    /// * [`is_SetAllocatedImageMem`]
    /// * [`is_GetColorDepth`]
    /// * [`is_GetImageMem`]
    /// * [`is_GetImagePitch`]
    ///
    /// # Documentation
    /// [is_SetImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_setimagemem.html)
    pub fn is_SetImageMem(hCam: HIDS, pcMem: *const char, nMemId: INT) -> INT;
}
