//! Releases an image memory that was allocated using [`is_AllocImageMem`] and removes it from
//! the driver management.
//!
//! If the memory was not allocated using an SDK function, you need to call [`is_FreeImageMem`]
//! as well. Otherwise, there may be errors when the driver keeps trying to access this memory.
//!
//! This does however not release the memory. So you need to make sure that the memory will be
//! released again.
//!
//! If image memories are locked by the user and are thus signaled as used, the image memories
//! cannot be released. Therefore, before calling [`is_FreeImageMem`], image memories locked using
//! [`is_LockSeqBuf`] **has to be** unlocked using [`is_UnlockSeqBuf`] first.
//!
//! # Documentation
//! [is_FreeImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_freeimagemem.html)

use crate::alloc_image_mem::is_AllocImageMem;
use crate::constants::return_values::*;
use crate::types::{char, HIDS, INT};

unsafe extern "C" {
    /// Releases an image memory that was allocated using [`is_AllocImageMem`] and removes it from
    /// the driver management.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pcMem` - Points to the starting address of the memory
    ///     (e.g. set in the [`is_AllocImageMem`] function)
    /// * `nMemId` - ID of this memory.
    ///
    /// # Return values
    /// * [`IS_CANT_ADD_TO_SEQUENCE`]
    /// * [`IS_CANT_CLEANUP_MEMORY`]
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
    /// * [`is_AllocImageMem`]
    ///
    /// # Documentation
    /// [is_FreeImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_freeimagemem.html)
    pub fn is_FreeImageMem(hCam: HIDS, pcMem: *const char, nMemId: INT) -> INT;
}
