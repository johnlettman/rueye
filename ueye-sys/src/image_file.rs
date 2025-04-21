//! Loads and save an image from or to a file.
//!
//! The image must be BMP, JPEG or PNG format. The image is loaded into the active image memory
//! or read-out from the active image memory.
//!
//! **Note for Linux systems:**
//! For saving images as JPEG or PNG you must install specific libraries under Linux
//! (see <https://en.ids-imaging.com/download-ueye.html>).
//!
//! The bitmap is stored with the color depth that was used when allocating the image memory
//! (in DIB mode) or that was set for the current color mode (in Direct3D mode). You can save
//! images with a bit depth of more than 8 bit in the PNG format. 12 bit formats are converted
//! into 16 bit. JPEG files are always saved with a color depth of 8 or 24 bits.
//!
//! When saving an image [`is_FreezeVideo`] should not be called with the [`IS_DONT_WAIT`]
//! parameter, because the image acquisition might not be completed at the time of saving.
//!
//! In Direct3D or OpenGL mode, overlay data is not saved.
//!
//! # Documentation
//! [is_ImageFile](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_imagefile.html)

#![allow(non_camel_case_types)]

use crate::constants::{return_values::*, IMG};
use crate::types::{char, void, wchar_t, BYTE, HCAM, INT, NULL, UINT};

/// Image file parameters for [`is_ImageFile`].
///
/// # Documentation
/// [`is_ImageFile`: Contents of the `IMAGE_FILE_PARAMS` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_imagefile.html)
#[derive(Debug)]
#[repr(C)]
pub struct IMAGE_FILE_PARAMS {
    /// Name of the file to be loaded/saved (Unicode/[`wchar_t`]).
    ///
    /// If [`NULL`] is passed, the "Open file"/"Save as" dialog opens.
    pub pwchFileName: *mut wchar_t,

    /// File type to be saved/loaded.
    pub nFileType: IMG,

    /// Image quality.
    ///
    /// * _PNG/JPEG_: Sets the image quality. The higher the value, the better the quality.
    ///     `100` = best quality (no compression).
    ///     If the parameter is set to `0`, the default value of `75` is used.
    /// * _BMP_: The parameter is ignored.
    pub nQuality: UINT,

    /// Pointer to an image memory.
    ///
    /// * **When loading:**
    ///     * If this and [`pnImageID`][IMAGE_FILE_PARAMS::pnImageID] are [`NULL`],
    ///         the image is loaded into the active image memory.
    ///     * If this and [`pnImageID`][IMAGE_FILE_PARAMS::pnImageID] are valid,
    ///         a new memory is allocated.
    ///         This memory must be released with [`is_FreeImageMem`].
    /// * **When saving:**
    ///     * If this and [`pnImageID`][IMAGE_FILE_PARAMS::pnImageID] are [`NULL`],
    ///         the image is saved from the active image memory.
    ///     * If this and [`pnImageID`][IMAGE_FILE_PARAMS::pnImageID] are valid,
    ///         corresponding memory is used.
    pub ppcImageMem: *mut *mut char,

    /// Image memory ID.
    ///
    /// * **When loading:**
    ///     * If this and [`ppcImageMem`][IMAGE_FILE_PARAMS::ppcImageMem] are [`NULL`],
    ///         the image is loaded into the active image memory.
    ///     * If this and [`ppcImageMem`][IMAGE_FILE_PARAMS::ppcImageMem] are valid,
    ///         a new memory is allocated.
    ///         This memory must be released with [`is_FreeImageMem`].
    /// * **When saving:**
    ///     * If this and [`ppcImageMem`][IMAGE_FILE_PARAMS::ppcImageMem] are [`NULL`],
    ///         the image is saved from the active image memory.
    ///     * If this and [`ppcImageMem`][IMAGE_FILE_PARAMS::ppcImageMem] are valid,
    ///         corresponding memory is used.
    pub pnImageID: *mut UINT,

    /// (**reserved**)
    reserved: [BYTE; 32],
}

/// Enumeration of commands of function [`is_ImageFile`].
///
/// # Documentation
/// [`is_ImageFile`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_imagefile.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IMAGE_FILE_CMD {
    /// Loads an image file (bmp, jpg, png).
    ///
    /// The function can be used with UNICODE file names.
    ///
    /// # Parameter type
    /// [`IMAGE_FILE_PARAMS`]
    IS_IMAGE_FILE_CMD_LOAD = 1,

    /// Saves an image file (bmp, jpg, png).
    ///
    /// The function can be used with UNICODE file names.
    ///
    /// # Parameter type
    /// [`IMAGE_FILE_PARAMS`]
    IS_IMAGE_FILE_CMD_SAVE = 2,
}

unsafe extern "C" {
    /// Loads and save an image from or to a file.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`IMAGE_FILE_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_FILE_READ_INVALID_BMP_ID`]
    /// * [`IS_FILE_READ_OPEN_ERROR`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_GetImageMem`]
    /// * [`is_SetImageMem`]
    ///
    /// # Documentation
    /// [is_ImageFile](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_imagefile.html)
    pub fn is_ImageFile(
        hCam: HCAM,
        nCommand: IMAGE_FILE_CMD,
        pParam: *mut void,
        cbSizeOfParam: UINT,
    ) -> INT;
}
