//! Allows the measurement of the sharpness in a defined AOI of the current image.
//!
//! To get a sharpness value the edges in the image are evaluated. The sharpness can only be
//! indicated as a relative value as it depends on the edges in the current image. An image
//! with less edges will not reach the sharpness value of an image with a lot of edges.
//!
//! The higher the value, the better the sharpness. The value can be used in comparative
//! measurements to detect changes in the image acquisition of the same object,
//! e.g. caused by readjusted lenses.
//!
//! # Documentation
//! [is_Measure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_measure.html)
#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{float, IS_RECT, UINT, char, HIDS, INT, void};

/// Enumeration of sharpening calculation algorithms for `is_Measure`.
///
/// # Documentation
/// [is_Measure: Content of the `MEASURE_SHARPNESS_CALCULATION_ALGORITHM` enumeration](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_measure.html#measuresharpnesscalculationalgorithm)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum MEASURE_SHARPNESS_CALCULATION_ALGORITHM {
    /// Tenengrad - contrast-based sharpness algorithm (convolution).
    IS_MEASURE_SHARPNESS_CALCULATION_ALGORITHM_TENENGRAD = 0x01,

    /// Mean Score - contrast-based sharpness algorithm (mean value).
    IS_MEASURE_SHARPNESS_CALCULATION_ALGORITHM_MEAN_SCORE = 0x02,

    /// Varianz - statistics-based sharpness algorithm
    IS_MEASURE_SHARPNESS_CALCULATION_ALGORITHM_HISTOGRAM_VARIANCE = 0x04,

    /// Sobel - contrast-based sharpness algorithm (convolution) / default setting.
    IS_MEASURE_SHARPNESS_CALCULATION_ALGORITHM_SOBEL = 0x10
}

impl Default for MEASURE_SHARPNESS_CALCULATION_ALGORITHM {
    #[inline]
    fn default() -> Self {
        Self::IS_MEASURE_SHARPNESS_CALCULATION_ALGORITHM_SOBEL
    }
}

/// Info structure about the calculated sharpness value (**deprecated**).
///
/// # Documentation
/// [is_Measure: Content of the MEASURE_SHARPNESS_INFO structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_measure.html#measuresharpnessinfo)
#[deprecated]
#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct MEASURE_SHARPNESS_AOI_INFO {
    /// ID of the AOI.
    pub u32NumberAOI: UINT,

    /// Relative sharpness value in the defined AOI.
    pub u32SharpnessValue: UINT,

    /// Position and size of the AOI.
    ///
    /// # Values
    /// * `s32X` - X position.
    /// * `s32Y` - Y position.
    /// * `s32Width` - AOI width.
    /// * `s32Height` - AOI height.
    pub rcAOI: IS_RECT
}

/// Info structure about the calculated sharpness value.
///
/// # Documentation
/// [is_Measure: Content of the MEASURE_SHARPNESS_INFO structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_measure.html#measuresharpnessinfo)
#[derive(Debug, Clone)]
#[repr(C)]
pub struct MEASURE_SHARPNESS_INFO {
    /// ID of the AOI.
    pub u32NumberAOI: UINT,

    /// Relative sharpness value in the defined AOI.
    pub fSharpnessValue: float,

    /// Position and size of the AOI.
    ///
    /// # Values
    /// * `s32X` - X position.
    /// * `s32Y` - Y position.
    /// * `s32Width` - AOI width.
    /// * `s32Height` - AOI height.
    pub rcAOI: IS_RECT,

    /// Start address of the image buffer to be used for the calculation.
    ///
    /// * If pcImageMem is valid, the selected buffer is used.
    /// * If pcImageMem = [`NULL`] or invalid, the active image buffer is used for the calculation.
    pub pcImageMem: *mut char
}

/// Sharpness AOI presets.
///
/// # Documentation
/// [is_Measure: Content of the MEASURE_SHARPNESS_AOI_PRESETS enumeration](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_measure.html#measuresharpnessaoipresets)
pub enum MEASURE_SHARPNESS_AOI_PRESETS {
    /// Predefined AOI for the sharpness measurement
    /// (5 AOIs, in each of the four image corners and in the center, each of the 5 AOIs has a size
    /// of ⅓ × "total image width" and ⅓ × "total image height")
    IS_MEASURE_SHARPNESS_AOI_PRESET_1 = 1
}

/// Commands for [`is_Measure`].
///
/// # Documentation
/// [is_Measure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_measure.html)
pub enum MEASURE_CMD {
    /// Sets an AOI in which the sharpness is measured.
    ///
    /// In the image are up to 5 AOIs possible. These AOIs can also overlap.
    ///
    /// # Parameter type
    /// [`MEASURE_SHARPNESS_INFO`]
    IS_MEASURE_CMD_SHARPNESS_AOI_SET                   = 1,

    /// Returns information of the AOI, e.g. the sharpness.
    ///
    /// # Parameter type
    /// [`MEASURE_SHARPNESS_INFO`]
    IS_MEASURE_CMD_SHARPNESS_AOI_INQUIRE               = 2,

    /// Sets different predefined AOIs in the image.
    ///
    /// # Parameter type
    /// [`MEASURE_SHARPNESS_AOI_PRESETS`]
    IS_MEASURE_CMD_SHARPNESS_AOI_SET_PRESET            = 3,

    /// _uEye LE USB 3.1 Gen 1 AF:_ Sets the algorithm for the sharpening calculation.
    ///
    /// # Parameter type
    /// [`MEASURE_SHARPNESS_CALCULATION_ALGORITHM`]
    IS_MEASURE_CMD_SHARPNESS_CALCULATION_ALGORITHM_SET = 4
}

unsafe extern "C" {
    /// Allows the measurement of the sharpness in a defined AOI of the current image.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`MEASURE_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_ACTIVE_IMG_MEM`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Documentation
    /// [is_Measure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_measure.html)
    pub fn is_Measure(hCam: HIDS, nCommand: MEASURE_CMD, pParam: *mut void, cbSizeOfParam: UINT) -> INT;
}


