//! Query the exposure time ranges available in your camera and set new exposure times.
//!
//! * [Setting the exposure time](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposuresetexposure.html)
//! * [Exposure time with fine increments](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposurefineincrement.html)
//! * [Setting the long exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposurelongexposure.html)
//! * [Setting the dual exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposuredualexposure.html)
//!
//! **Note on dependencies on other settings:**
//! The use of the following functions will affect the exposure time:
//! * [`is_PixelClock`]
//! * [`is_SetOptimalCameraTiming`]
//! * [`is_SetFrameRate`]
//! * [`is_AOI`]
//! * [`is_SetSubSampling`]
//! * [`is_SetBinning`]
//!
//! Changes made to the image size, the frame rate or the pixel clock frequency also affect the
//! exposure time. For this reason, you need to call [`is_Exposure`] again after such changes.
//!
//! **Note on new driver versions:**
//! Newer driver versions sometimes allow an extended value range for the exposure time setting.
//! We recommend querying the value range every time and set the exposure time explicitly.
//!
//! # Documentation
//! [is_Exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposure.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{double, void, BOOL, HCAM, INT, IS_RANGE_S32, UINT};

/// Enumeration of commands for [`is_Exposure`].
///
/// # Documentation
/// [is_Exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposure.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EXPOSURE_CMD {
    /// Returns the supported function modes.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [is_Exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposure.html)
    IS_EXPOSURE_CMD_GET_CAPS = 1,

    /// Returns the default setting for the exposure time.
    ///
    /// # Parameter type
    /// [`double`]
    ///
    /// # Documentation
    /// [Setting the exposure time](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposuresetexposure.html)
    IS_EXPOSURE_CMD_GET_EXPOSURE_DEFAULT = 2,

    /// Returns the minimum exposure time.
    ///
    /// # Parameter type
    /// [`double`]
    ///
    /// # Documentation
    /// [Setting the exposure time](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposuresetexposure.html)
    IS_EXPOSURE_CMD_GET_EXPOSURE_RANGE_MIN = 3,

    /// Returns the maximum exposure time.
    ///
    /// # Parameter type
    /// [`double`]
    ///
    /// # Documentation
    /// [Setting the exposure time](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposuresetexposure.html)
    IS_EXPOSURE_CMD_GET_EXPOSURE_RANGE_MAX = 4,

    /// Returns the exposure time increment.
    ///
    /// # Parameter type
    /// [`double`]
    ///
    /// # Documentation
    /// [Setting the exposure time](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposuresetexposure.html)
    IS_EXPOSURE_CMD_GET_EXPOSURE_RANGE_INC = 5,

    /// Returns the exposure time range.
    ///
    /// # Parameter type
    /// _Array of (in order):_
    /// * `minimum`: [`double`]
    /// * `maximum`: [`double`]
    /// * `increment`: [`double`]
    ///
    /// # Documentation
    /// [Setting the exposure time](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposuresetexposure.html)
    IS_EXPOSURE_CMD_GET_EXPOSURE_RANGE = 6,

    /// Returns the currently set exposure time (in ms).
    ///
    /// # Parameter type
    /// [`double`]
    ///
    /// # Documentation
    /// [Setting the exposure time](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposuresetexposure.html)
    IS_EXPOSURE_CMD_GET_EXPOSURE = 7,

    /// Returns the minimum exposure time in fine increments for some sensors.
    ///
    /// # Parameter type
    /// [`double`]
    ///
    /// # Documentation
    /// [Exposure time with fine increments](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposurefineincrement.html)
    IS_EXPOSURE_CMD_GET_FINE_INCREMENT_RANGE_MIN = 8,

    /// Returns the maximum exposure time in fine increments for some sensors.
    ///
    /// # Parameter type
    /// [`double`]
    ///
    /// # Documentation
    /// [Exposure time with fine increments](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposurefineincrement.html)
    IS_EXPOSURE_CMD_GET_FINE_INCREMENT_RANGE_MAX = 9,

    /// Returns the exposure time increment in fine increments for some sensors.
    ///
    /// # Parameter type
    /// [`double`]
    ///
    /// # Documentation
    /// [Exposure time with fine increments](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposurefineincrement.html)
    IS_EXPOSURE_CMD_GET_FINE_INCREMENT_RANGE_INC = 10,

    /// Returns the exposure time range in fine increments for some sensors.
    ///
    /// # Parameter type
    /// _Array of (in order):_
    /// * `minimum`: [`double`]
    /// * `maximum`: [`double`]
    /// * `increment`: [`double`]
    ///
    /// # Documentation
    /// [Exposure time with fine increments](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposurefineincrement.html)
    IS_EXPOSURE_CMD_GET_FINE_INCREMENT_RANGE = 11,

    /// Sets the exposure time (in ms).
    ///
    /// # Parameter type
    /// [`double`]
    ///
    /// After setting the exposure time this value contains the actually set exposure time.
    /// Depending on the sensor the set exposure time may vary slightly from the desired
    /// exposure time.
    ///
    /// # Documentation
    /// [Setting the exposure time](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposuresetexposure.html)
    IS_EXPOSURE_CMD_SET_EXPOSURE = 12,

    /// Returns the minimum long exposure time.
    ///
    /// # Parameter type
    /// [`double`]
    ///
    /// # Documentation
    /// [Setting the long exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposurelongexposure.html)
    IS_EXPOSURE_CMD_GET_LONG_EXPOSURE_RANGE_MIN = 13,

    /// Returns the maximum long exposure time.
    ///
    /// # Parameter type
    /// [`double`]
    ///
    /// # Documentation
    /// [Setting the long exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposurelongexposure.html)
    IS_EXPOSURE_CMD_GET_LONG_EXPOSURE_RANGE_MAX = 14,

    /// Returns the increments for long exposure.
    ///
    /// # Parameter type
    /// [`double`]
    ///
    /// # Documentation
    /// [Setting the long exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposurelongexposure.html)
    IS_EXPOSURE_CMD_GET_LONG_EXPOSURE_RANGE_INC = 15,

    /// Returns the value range for long exposure.
    ///
    /// # Parameter type
    /// _Array of (in order):_
    /// * `minimum`: [`double`]
    /// * `maximum`: [`double`]
    /// * `increment`: [`double`]
    ///
    /// # Documentation
    /// [Setting the long exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposurelongexposure.html)
    IS_EXPOSURE_CMD_GET_LONG_EXPOSURE_RANGE = 16,

    /// Returns the current settings for long exposure.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Documentation
    /// [Setting the long exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposurelongexposure.html)
    IS_EXPOSURE_CMD_GET_LONG_EXPOSURE_ENABLE = 17,

    /// Enables/Disables long exposure.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Documentation
    /// [Setting the long exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposurelongexposure.html)
    IS_EXPOSURE_CMD_SET_LONG_EXPOSURE_ENABLE = 18,

    /// Returns the default ratio for dual exposure.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Setting the dual exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposuredualexposure.html)
    IS_EXPOSURE_CMD_GET_DUAL_EXPOSURE_RATIO_DEFAULT = 19,

    /// Returns the range for dual exposure.
    ///
    /// # Parameter type
    /// [`IS_RANGE_S32`]
    ///
    /// # Documentation
    /// [Setting the dual exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposuredualexposure.html)
    IS_EXPOSURE_CMD_GET_DUAL_EXPOSURE_RATIO_RANGE = 20,

    /// Returns the ratio between exposure times for dual exposure.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Setting the dual exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposuredualexposure.html)
    IS_EXPOSURE_CMD_GET_DUAL_EXPOSURE_RATIO = 21,

    /// Sets the ratio between exposure times for dual exposure.
    ///
    /// For model _UI-336x/UI-536x_ or _UI-337x/UI-537x_, select a percentage value between
    /// `10` and `100`. E.g. `50` means that odd lines are exposed at the selected exposure time
    /// and even lines are exposed with 50% of the selected exposure time.
    ///
    /// The dual exposure cannot be used when the camera is operated with minimum exposure time.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Setting the dual exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposuredualexposure.html)
    IS_EXPOSURE_CMD_SET_DUAL_EXPOSURE_RATIO = 22,
    IS_EXPOSURE_CMD_GET_LONG_EXPOSURE_FRAMERATE_LIMIT = 23,
    IS_EXPOSURE_CMD_SET_LONG_EXPOSURE_FRAMERATE_LIMIT = 24,
}

/// Enumeration of exposure capabilities.
///
/// # Related commands
/// * [`IS_EXPOSURE_CMD_GET_CAPS`][EXPOSURE_CMD::IS_EXPOSURE_CMD_GET_CAPS]
///
/// # Documentation
/// [is_Exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposure.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EXPOSURE_CAPS {
    /// The exposure time setting is supported.
    IS_EXPOSURE_CAP_EXPOSURE = 0x00000001,

    /// Fine exposure time increments are supported.
    IS_EXPOSURE_CAP_FINE_INCREMENT = 0x00000002,

    /// Long time exposure is supported.
    ///
    /// Depending on the camera model long time exposure is only supported in trigger mode but
    /// not in freerun mode.
    IS_EXPOSURE_CAP_LONG_EXPOSURE = 0x00000004,

    /// The sensor supports dual exposure (_UI-336x/UI-536x and UI-337x/UI-537x only_).
    ///
    /// Odd and even lines can be exposed with different exposure times.
    IS_EXPOSURE_CAP_DUAL_EXPOSURE = 0x00000008,
}

unsafe extern "C" {
    /// Query the exposure time ranges available in your camera and set new exposure times.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`EXPOSURE_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_INVALID_CAMERA_TYPE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MODE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_CALIBRATED`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    /// * [`IS_TIMED_OUT`]
    ///
    /// # Related functions
    /// * [`is_SetFrameRate`]
    /// * [`is_PixelClock`]
    /// * [`is_SetOptimalCameraTiming`]
    /// * [`is_SetAutoParameter`]
    /// * [`is_AutoParameter`]
    /// * [`is_SetHardwareGain`]
    ///
    /// # Documentation
    /// [is_Exposure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_exposure.html)
    pub fn is_Exposure(
        hCam: HCAM,
        nCommand: EXPOSURE_CMD,
        pParam: *mut void,
        cbSizeOfParam: UINT,
    ) -> INT;
}
