#![allow(non_camel_case_types)]

use std::hash::Hash;
use std::mem::MaybeUninit;
use crate::constants::return_values::*;
use crate::types::{BYTE, HIDS, INT, UINT, void};

/// Enumeration of commands of function [`is_ImageBuffer`].
///
/// # Documentation
/// [`is_ImageBuffer`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_imagebuffer.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IMGBUF_CMD {
    /// Returns the number of iterations which are currently in the camera memory.
    ///
    /// # Parameter type
    /// [`ID_RANGE`]
    IS_IMGBUF_DEVMEM_CMD_GET_AVAILABLE_ITERATIONS      = 1,

    /// Returns the information about a specific iteration,
    /// e.g. the number of images in the iteration.
    ///
    /// # Parameter type
    /// [`IMGBUF_ITERATION_INFO`]
    IS_IMGBUF_DEVMEM_CMD_GET_ITERATION_INFO            = 2,

    /// Transfers an image from an iteration in the camera memory to the user buffer on the PC.
    ///
    /// # Parameter type
    /// [`IMGBUF_ITEM`]
    IS_IMGBUF_DEVMEM_CMD_TRANSFER_IMAGE                = 3,

    /// Releases all iterations up to the given ID in the camera memory.
    ///
    /// # Parameter type
    /// [`INT`]
    IS_IMGBUF_DEVMEM_CMD_RELEASE_ITERATIONS            = 4
}

/// Data type for ID ranges of iterations or image IDs.
///
/// # Documentation
/// [`is_ImageBuffer`: Content of the `ID_RANGE` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_imagebuffer.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ID_RANGE {
    /// First image ID.
    s32First: INT,

    /// Last image ID
    s32Last: INT,
}

impl ID_RANGE {
    #[inline]
    pub fn size(&self) -> INT {
        (self.s32Last - self.s32First).abs()
    }
}

impl PartialOrd for ID_RANGE {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size().partial_cmp(&other.size())
    }
}

impl Ord for ID_RANGE {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size().cmp(&other.size())
    }
}

/// Info structure for capture iterations.
///
/// # Documentation
/// [`is_ImageBuffer`: Content of the `IMGBUF_ITERATION_INFO` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_imagebuffer.html)#[repr(C)]
#[derive(Debug, Eq)]
#[repr(C)]
pub struct IMGBUF_ITERATION_INFO {
    /// Iteration ID.
    pub u32IterationID: UINT,

    /// Image range i.e. the ID of the first and last image.
    pub rangeImageID: ID_RANGE,

    /// (**reserved**)
    bReserved: [BYTE; 52],
}

impl Clone for IMGBUF_ITERATION_INFO {
    fn clone(&self) -> Self {
        // Unsafe allocate clone to avoid zeroing `bReserved`.
        let mut other = unsafe {
            MaybeUninit::<Self>::uninit().assume_init()
        };

        other.u32IterationID = self.u32IterationID;
        other.rangeImageID = self.rangeImageID;
        other
    }
}

impl PartialEq for IMGBUF_ITERATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.u32IterationID == other.u32IterationID && self.rangeImageID == other.rangeImageID
    }
}

impl Hash for IMGBUF_ITERATION_INFO {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.u32IterationID.hash(state);
        self.rangeImageID.hash(state);
    }
}

/// Structure to specify concrete image.
///
/// # Documentation
/// [`is_ImageBuffer`: Content of the `IMGBUF_ITEM` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_imagebuffer.html)#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IMGBUF_ITEM {
    /// Iteration ID.
    pub u32IterationID: UINT,

    /// Image ID.
    pub s32ImageID: INT
}

unsafe extern "C" {
    /// Allows accessing the captured images in the camera memory
    /// (see [Using the camera memory (GigE uEye cameras)](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk_gige_cameramemory.html)).
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`IMGBUF_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_DEVICE_BUSY`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Documentation
    /// [`is_ImageBuffer`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_imagebuffer.html)
    pub fn is_ImageBuffer(hCam: HIDS, nCommand: IMGBUF_CMD, pParam: *mut void, cbSizeOfParam: UINT) -> INT;
}
