//! Read or write the non-volatile user memory of the camera.
//!
//! Besides the hard-coded factory information, the user memory of the _uEye_ camera can hold
//! additional user data which is not to be stored cyclically but permanently, e.g.
//! camera parameters for camera opening, one-time determined, permanent calibration data, etc.
//!
//! <div class="warning">
//! Note that images may be lost if you write to the non-volatile user memory during
//! image acquisition.
//! </div>
//!
//! Depending on the camera model, an additional extended 64 kB memory is provided beside the
//! old 64 Byte user memory, which can also be used storing user data.
//!
//! The following overview shows which memories are supported by each camera family.
//!
//! | Camera family           | User memory                     |
//! |-------------------------|---------------------------------|
//! | _uEye LE USB 3.1 Gen 1_ | 64 Byte + 64 kB extended memory |
//! | _uEye SE USB 3.1 Gen 1_ | 64 Byte + 64 kB extended memory |
//! | _USB 3 uEye CP Rev. 2_  | 64 Byte + 64 kB extended memory |
//! | _USB 3 uEye CP_         | 64 Byte                         |
//! | _USB 3 uEye LE_         | 64 Byte + 64 kB extended memory |
//! | _USB 3 uEye ML_         | 64 Byte + 64 kB extended memory |
//! | _GigE uEye CP Rev. 2_   | 64 Byte                         |
//! | _GigE uEye CP_          | 64 Byte                         |
//! | _GigE uEye FA_          | 64 Byte                         |
//! | _GigE uEye LE_          | 64 Byte                         |
//! | _GigE uEye RE PoE_      | 64 Byte                         |
//! | _GigE uEye SE Rev. 4_   | 64 Byte                         |
//! | _GigE uEye SE_          | 64 Byte                         |
//! | _USB uEye LE_           | 64 Byte                         |
//! | _USB uEye ML_           | 64 Byte                         |
//! | _USB uEye SE_           | 64 Byte                         |
//! | _USB uEye XS_           | 64 Byte                         |
//!
//! # Documentation
//! [is_PersistentMemory](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_persistentmemory.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{char, void, HIDS, INT, UINT};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IS_PERSISTENT_MEMORY {
    /// Address offset within the range.
    pub u32Offset: UINT,

    /// Number of characters to read/written.
    pub u32Count: UINT,

    pub s32Option: INT,

    /// Pointer to the buffer with the data to be read/written.
    pub pu8Memory: *mut char,
}

/// Enumeration of commands for [`is_PersistentMemory`].
///
/// # Documentation
/// [is_PersistentMemory](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_persistentmemory.html)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum PERSISTENT_MEMORY_CMD {
    /// Reads the data from the extended user memory.
    ///
    /// # Parameter type
    /// [`IS_PERSISTENT_MEMORY`]
    IS_PERSISTENT_MEMORY_READ_USER_EXTENDED = 1,

    /// Writes the data in the extended user memory.
    ///
    /// # Parameter type
    /// [`IS_PERSISTENT_MEMORY`]
    IS_PERSISTENT_MEMORY_WRITE_USER_EXTENDED = 2,

    /// Returns the size of the extended user memory (in bytes).
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_PERSISTENT_MEMORY_GET_SIZE_USER_EXTENDED = 3,

    /// Reads the data from the user memory.
    ///
    /// # Parameter type
    /// [`IS_PERSISTENT_MEMORY`]
    IS_PERSISTENT_MEMORY_READ_USER = 4,

    /// Writes the data in the user memory.
    ///
    /// # Parameter type
    /// [`IS_PERSISTENT_MEMORY`]
    IS_PERSISTENT_MEMORY_WRITE_USER = 5,

    /// Returns the size of the user memory (in bytes).
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_PERSISTENT_MEMORY_GET_SIZE_USER = 6,

    /// Reads the data from the protected user memory.
    ///
    /// # Parameter type
    /// [`IS_PERSISTENT_MEMORY`]
    IS_PERSISTENT_MEMORY_READ_USER_PROTECTED = 7,

    /// Writes the data in the protected user memory.
    ///
    /// # Parameter type
    /// [`IS_PERSISTENT_MEMORY`]
    IS_PERSISTENT_MEMORY_WRITE_USER_PROTECTED = 8,

    /// Returns the size of the protected user memory (in bytes).
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_PERSISTENT_MEMORY_GET_SIZE_USER_PROTECTED = 9,
}

unsafe extern "C" {

    /// Read or write the non-volatile user memory of the camera.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`PERSISTENT_MEMORY_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Documentation
    /// [is_PersistentMemory](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_persistentmemory.html)
    pub fn is_PersistentMemory(
        hCam: HIDS,
        nCommand: PERSISTENT_MEMORY_CMD,
        pParam: *mut void,
        cbSizeOfParam: UINT,
    ) -> INT;

}
