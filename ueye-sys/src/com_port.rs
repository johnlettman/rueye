//! Common COM port functions.

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{HCAM, INT, UINT, WORD};

/// COM port configuration structure.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C, packed(1))]
pub struct UEYE_COMPORT_CONFIGURATION {
    /// COM port number.
    pub wComportNumber: WORD,
}

unsafe extern "C" {
    /// Returns the current COM port number of a USB 3 uEye CP/ML camera.
    ///
    /// The default port number is `100`. You can change the port number in the IDS Camera Manager.
    /// For further information, please also refer to the
    /// [Camera basics: Serial interface](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/hw_serielleschnittstelle_rs-232.html)
    /// chapter.
    ///
    /// <div class="warning">
    ///
    /// The [`is_GetComportNumber`] function is only supported by cameras of the
    /// USB 3 uEye CP/ML series.
    ///
    /// </div>
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pComportNumber` - Pointer to the variable that is supposed to contain the port number.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Documentation
    /// [is_GetComportNumber](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_getcomportnumber.html)
    pub fn is_GetComportNumber(hCam: HCAM, pComportNumber: *mut UINT) -> INT;
}
