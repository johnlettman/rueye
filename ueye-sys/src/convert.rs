#![allow(non_camel_case_types)]

use crate::types::{void, BYTE, CHAR, HIDS, INT, UINT};

/// Buffer conversion parameters.
///
/// # Documentation
/// [`is_Convert`: Contents of the `BUFFER_CONVERSION_PARAMS` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_convert.html)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BUFFER_CONVERSION_PARAMS {
    /// Pointer to the raw Bayer buffer which was created with the [`is_AllocImageMem`] function.
    pub pSourceBuffer: *mut CHAR,

    /// Pointer to the target buffer with the converted data which was created with the
    /// [`is_AllocImageMem`] function.
    pub pDestBuffer: *mut CHAR,

    /// Color mode of the target image.
    ///
    /// See [`is_SetColorMode`] for the possible modes.
    pub nDestPixelFormat: INT,

    /// Conversion mode of the target image.
    ///
    /// See [`is_SetColorConverter`] for the possible modes
    pub nDestPixelConverter: INT,

    /// Sets the gamma correction.
    ///
    /// See [`is_Gamma`].
    pub nDestGamma: INT,

    /// Sets the edge enhancement.
    ///
    /// See [`is_EdgeEnhancement`].
    pub nDestEdgeEnhancement: INT,

    /// Sets the color correction.
    ///
    /// See [`is_SetColorCorrection`].
    pub nDestColorCorrectionMode: INT,

    /// Sets the color saturation (saturation U).
    ///
    /// See [`is_SetColorCorrection`].
    pub nDestSaturationU: INT,

    /// Sets the color saturation (saturation V).
    ///
    /// See [`is_SetColorCorrection`].
    pub nDestSaturationV: INT,

    /// (**reserved**)
    reserved: [BYTE; 32]
}

/// Enumeration of commands of function [`is_Convert`].
///
/// # Documentation
/// [`is_Convert`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_convert.html)
pub enum CONVERT_CMD {
    /// Converts a raw Bayer buffer with the passed conversion parameters.
    IS_CONVERT_CMD_APPLY_PARAMS_AND_CONVERT_BUFFER = 1
}

unsafe extern "C" {

    /// General function to convert a raw Bayer image to the desired format.
    ///
    /// You can set all parameters, which are important for software conversion:
    /// * Pixel format
    /// * Pixel converter (3×3, 5×5)
    /// * Color correction
    /// * Gamma
    /// * Saturation
    /// * Edge enhancement
    ///
    /// The target buffer must be allocated with the [`is_AllocImageMem`] function and
    /// must have the right size.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`CONVERT_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_BUFFER_SIZE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_SetColorMode`]
    /// * [`is_SetColorConverter`]
    ///
    /// # Documentation
    /// [`is_Convert`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_convert.html)
    pub fn is_Convert(hCam: HIDS, nCommand: CONVERT_CMD, pParam: *mut void, cbSizeOfParam: UINT) -> INT;
}
