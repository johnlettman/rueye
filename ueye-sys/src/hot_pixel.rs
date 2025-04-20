//! Configures the correction of sensor hot pixels.
//!
//! The correction is performed by the software. The hot pixel list is stored in the camera's
//! non-volatile camera memory. Some sensor models can also correct hot pixels directly in
//! the sensor. For further information on hot pixel correction, please refer to
//! [Basics: Hot pixels](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/hw_hotpixel.html).
//!
//! There are different hot pixel lists:
//! * `*_SOFTWARE_USER_LIST`: user-defined hot pixel list in the computer
//! * `*_CAMERA_FACTORY_LIST`: factory-set hot pixel list in the non-volatile camera memory
//! * `*_CAMERA_USER_LIST`: user-defined hot pixel list in the non-volatile camera memory
//! * `*_MERGED_CAMERA_LIST`: factory-set and user-defined hot pixel list in the non-volatile
//!     camera memory (`*_CAMERA_FACTORY_LIST` and `*_CAMERA_USER_LIST`)
//!
//! This function is currently not supported by the following camera models:
//! * _UI-1007XS_
//!
//! This correction will not work with subsampling or with binning factors greater than 2.
//!
//! ## Adaptive hot pixel correction
//! From version 4.82 on, the adaptive hot pixel correction is implemented additionally.
//! The adaptive hot pixel correction can determine hot pixels dynamically and correct them
//! adaptively in the current image. Thus, hot pixels can be corrected better which for example
//! result from an increased temperature. In addition, the adaptive hot pixel correction can be
//! executed directly in live operation, thus permitting the best possible optimization on the
//! application conditions.
//!
//! **Note on adaptive hot pixel correction:**
//! * Adaptive hot pixel correction can be used in combination with subsampling, binning or scaler.
//! * Currently, adaptive hot pixel correction can only be used with software color formats or
//!     RAW color formats.
//!
//! # Documentation
//! [is_HotPixel](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_hotpixel.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{void, BOOL, HIDS, INT, NULL, STRING, UINT, WORD, WSTRING};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct HOTPIXEL_MODE: UINT {
        const DISABLED = 0x0000;

        /// Hot pixel correction is possible via the hot pixel list in the
        /// non-volatile camera memory.
        const ENABLE_SENSOR_CORRECTION = 0x0001;

        /// Hot pixel correction is possible via the user-defined hot pixel list.
        const ENABLE_CAMERA_CORRECTION = 0x0002;

        /// Hot pixel correction is possible via the sensor-internal hot pixel correction.
        const ENABLE_SOFTWARE_USER_CORRECTION = 0x0004;
    }
}

bitflags! {
    /// Modes of the adaptive hot pixel correction.
    ///
    /// [is_HotPixel: Modes of the adaptive hot pixel correction](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_hotpixel.html#mode)
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct HOTPIXEL_ADAPTIVE_CORRECTION_MODE: UINT {
        /// The hot pixels are determined once and stored in a list.
        ///
        /// The adaptive hot pixel correction works with this list. This mode is suitable for all
        /// applications with static light conditions which do not require dynamic adaptation.
        const IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_ONCE = 0x0000;

        /// The hot pixel list is dynamically determined for each image.
        ///
        /// So, this allows a better response for changing light conditions.
        /// In this mode, the full frame rate may not be achieved.
        const IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_DYNAMIC = 0x0001;

        /// An adaptive cluster correction can be applied which also corrects two or more
        /// adjacent hot pixels.
        ///
        /// The hot pixel clusters are determined once and stored in the cluster list.
        /// The cluster list is independent of the hot pixel list.
        ///
        /// Note that the cluster detection could detect larger image content as a cluster.
        /// In this case, calibrate the camera against a low-contrast background or darken the
        /// camera before calibration.
        const IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_ONCE_CLUSTER = 0x0002;

        /// The cluster list is dynamically determined for each image.
        ///
        /// So, this allows a better response for changing light conditions.
        /// In this mode, the full frame rate may not be achieved.
        const IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_DYNAMIC_CLUSTER = 0x0004;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum HOTPIXEL_ADAPTIVE_CORRECTION_ENABLE {
    /// Enables the adaptive hot pixel correction.
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_DISABLE = 0,

    /// Disables the adaptive hot pixel correction.
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_ENABLE = 1,
}

/// Enumeration of commands for [`is_HotPixel`].
///
/// # Documentation
/// [is_HotPixel](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_hotpixel.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_HOTPIXEL_CMD {
    /// Disables hot pixel correction.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_HOTPIXEL_DISABLE_CORRECTION = 0x0000,

    /// Enables sensor's own hot pixel correction function (if available).
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_HOTPIXEL_ENABLE_SENSOR_CORRECTION = 0x0001,

    /// Enables hot pixel correction using the hot pixel list(s) stored in the
    /// non-volatile camera memory.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_HOTPIXEL_ENABLE_CAMERA_CORRECTION = 0x0002,

    /// Enables hot pixel correction using the user's hot pixel list stored in the computer.
    ///
    /// This requires the user's hot pixel list to be set
    /// ([`IS_HOTPIXEL_SET_SOFTWARE_USER_LIST`][HOTPIXEL_CMD::IS_HOTPIXEL_SET_SOFTWARE_USER_LIST]).
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_HOTPIXEL_ENABLE_SOFTWARE_USER_CORRECTION = 0x0004,

    /// Disables the sensor's own hot pixel correction function.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_HOTPIXEL_DISABLE_SENSOR_CORRECTION = 0x0008,

    /// Returns the currently set hot pixel correction mode.
    ///
    /// # Parameter type
    /// [`HOTPIXEL_MODE`], _bitmask_
    IS_HOTPIXEL_GET_CORRECTION_MODE = 0x8000,

    /// Returns the supported hot pixel correction modes.
    ///
    /// # Parameter type
    /// [`HOTPIXEL_MODE`], _bitmask_
    IS_HOTPIXEL_GET_SUPPORTED_CORRECTION_MODES = 0x8001,

    /// Indicates whether the user-defined hot pixel list exists in the computer.
    ///
    /// # Parameter type
    /// [`BOOL`]
    IS_HOTPIXEL_GET_SOFTWARE_USER_LIST_EXISTS = 0x8100,

    /// Returns the number of hot pixels in the user-defined hot pixel list stored in the computer
    /// (_max possible number = 0.5% of the sensor resolution_).
    ///
    /// # Parameter type
    /// [`INT`]
    IS_HOTPIXEL_GET_SOFTWARE_USER_LIST_NUMBER = 0x8101,

    /// Returns the user-defined hot pixel list stored in the computer.
    ///
    /// # Parameter type
    /// _Array of:_
    /// * [`WORD`]: X
    /// * [`WORD`]: Y
    IS_HOTPIXEL_GET_SOFTWARE_USER_LIST = 0x8102,

    /// Sets the user-defined hot pixel list that is stored in the computer.
    ///
    /// # Parameter type
    /// _Array of:_
    /// * [`WORD`]: X
    /// * [`WORD`]: Y
    IS_HOTPIXEL_SET_SOFTWARE_USER_LIST = 0x8103,

    /// Saves the user-defined hot pixel list to a file.
    ///
    /// The function can also be used with Unicode file names.
    ///
    /// The user-defined hot pixel list is saved as a binary file.
    /// The file should only be edited with the _uEye Hotpixel Editor_ or
    /// the corresponding API commands.
    ///
    /// # Parameter type
    /// [`STRING`]
    IS_HOTPIXEL_SAVE_SOFTWARE_USER_LIST = 0x8104,

    /// Loads the user-defined hot pixel list from a file.
    ///
    /// The function can also be used with Unicode file names.
    ///
    /// A user-defined hot pixel list can be created and saved using the
    /// _uEye Hotpixel Editor_. Alternatively, create the list using
    /// [`IS_HOTPIXEL_SET_SOFTWARE_USER_LIST`][HOTPIXEL_CMD::IS_HOTPIXEL_SET_SOFTWARE_USER_LIST]
    /// and save the list with
    /// [`IS_HOTPIXEL_SAVE_SOFTWARE_USER_LIST`][HOTPIXEL_CMD::IS_HOTPIXEL_SAVE_SOFTWARE_USER_LIST].
    ///
    /// # Parameter type
    /// [`STRING`]
    IS_HOTPIXEL_LOAD_SOFTWARE_USER_LIST = 0x8105,

    /// Indicates whether the factory-set hot pixel list exists.
    ///
    /// # Parameter type
    /// [`BOOL`]
    IS_HOTPIXEL_GET_CAMERA_FACTORY_LIST_EXISTS = 0x8106,

    /// Returns the number of hot pixels in the factory-set hot pixel list.
    ///
    /// # Parameter type
    /// [`INT`]
    IS_HOTPIXEL_GET_CAMERA_FACTORY_LIST_NUMBER = 0x8107,

    /// Returns the factory-set hot pixel list.
    ///
    /// # Parameter type
    /// _Array of:_
    /// * [`WORD`]: X
    /// * [`WORD`]: Y
    IS_HOTPIXEL_GET_CAMERA_FACTORY_LIST = 0x8108,

    /// Indicates whether the user-defined hot pixel list exists in the non-volatile camera memory.
    ///
    /// # Parameter type
    /// [`BOOL`]
    IS_HOTPIXEL_GET_CAMERA_USER_LIST_EXISTS = 0x8109,

    /// Returns the number of hot pixels in the user-defined hot pixel list stored in the
    /// non-volatile camera memory.
    ///
    /// # Parameter type
    /// [`INT`]
    IS_HOTPIXEL_GET_CAMERA_USER_LIST_NUMBER = 0x810A,

    /// Returns the user-defined hot pixel list stored in the non-volatile camera memory.
    ///
    /// # Parameter type
    /// _Array of:_
    /// * [`WORD`]: X
    /// * [`WORD`]: Y
    IS_HOTPIXEL_GET_CAMERA_USER_LIST = 0x810B,

    /// Sets the user-defined hot pixel list stored in the non-volatile camera memory.
    ///
    /// # Parameter type
    /// _Array of:_
    /// * [`WORD`]: X
    /// * [`WORD`]: Y
    IS_HOTPIXEL_SET_CAMERA_USER_LIST = 0x810C,

    /// Returns the maximum number of hot pixels that the user can store in the
    /// non-volatile camera memory.
    ///
    /// # Parameter type
    /// [`INT`]
    IS_HOTPIXEL_GET_CAMERA_USER_LIST_MAX_NUMBER = 0x810D,

    /// Deletes the user-defined hot pixel list from the non-volatile camera memory.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_HOTPIXEL_DELETE_CAMERA_USER_LIST = 0x810E,

    /// Returns the number of hot pixels in a merged list that combines the entries from the
    /// factory-set hot pixel list with those of the user-defined hot pixels list stored in the
    /// non-volatile camera memory.
    ///
    /// # Parameter type
    /// [`INT`]
    IS_HOTPIXEL_GET_MERGED_CAMERA_LIST_NUMBER = 0x810F,

    /// Returns the merged list.
    ///
    /// # Parameter type
    /// _Array of:_
    /// * [`WORD`]: X
    /// * [`WORD`]: Y
    IS_HOTPIXEL_GET_MERGED_CAMERA_LIST = 0x8110,

    /// Saves the user-defined hot pixel list to a file.
    ///
    /// The function can also be used with Unicode file names.
    ///
    /// The user-defined hot pixel list is saved as a binary file.
    /// The file should only be edited with the _uEye Hotpixel Editor_ or
    /// the corresponding API commands.
    ///
    /// # Parameter type
    /// [`WSTRING`]
    IS_HOTPIXEL_SAVE_SOFTWARE_USER_LIST_UNICODE = 0x8111,

    /// Loads the user-defined hot pixel list from a file.
    ///
    /// The function can also be used with Unicode file names.
    ///
    /// A user-defined hot pixel list can be created and saved using the
    /// _uEye Hotpixel Editor_. Alternatively, create the list using
    /// [`IS_HOTPIXEL_SET_SOFTWARE_USER_LIST`][HOTPIXEL_CMD::IS_HOTPIXEL_SET_SOFTWARE_USER_LIST]
    /// and save the list with
    /// [`IS_HOTPIXEL_SAVE_SOFTWARE_USER_LIST`][HOTPIXEL_CMD::IS_HOTPIXEL_SAVE_SOFTWARE_USER_LIST].
    ///
    /// # Parameter type
    /// [`WSTRING`]
    IS_HOTPIXEL_LOAD_SOFTWARE_USER_LIST_UNICODE = 0x8112,

    /// Returns if the adaptive hot pixel correction is enabled.
    ///
    /// # Parameter type
    /// [`HOTPIXEL_ADAPTIVE_CORRECTION_ENABLE`]
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_GET_ENABLE = 0x8113,

    /// Returns the default settings for the adaptive hot pixel correction.
    ///
    /// # Parameter type
    /// [`HOTPIXEL_ADAPTIVE_CORRECTION_ENABLE`]
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_GET_ENABLE_DEFAULT = 0x8114,

    /// Enables/disables the adaptive hot pixel correction.
    ///
    /// # Parameter type
    /// [`HOTPIXEL_ADAPTIVE_CORRECTION_ENABLE`]
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_SET_ENABLE = 0x8115,

    /// Returns the mode of the adaptive hot pixel correction.
    ///
    /// # Parameter type
    /// [`HOTPIXEL_ADAPTIVE_CORRECTION_MODE`]
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_GET_MODE = 0x8116,

    /// Returns the default settings for the mode of the adaptive hot pixel correction.
    ///
    /// # Parameter type
    /// [`HOTPIXEL_ADAPTIVE_CORRECTION_MODE`]
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_GET_MODE_DEFAULT = 0x8117,

    /// Sets the mode of the adaptive hot pixel correction.
    ///
    /// # Parameter type
    /// [`HOTPIXEL_ADAPTIVE_CORRECTION_MODE`]
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_SET_MODE = 0x8118,

    /// Returns the value for the sensitivity of the adaptive hot pixel correction:
    /// `1` (_lowest sensitivity_) … `5` (_maximum sensitivity_).
    ///
    /// # Parameter type
    /// [`INT`]
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_GET_SENSITIVITY = 0x8119,

    /// Returns the default value for the sensitivity of the adaptive hot pixel correction.
    ///
    /// # Parameter type
    /// [`INT`]
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_GET_SENSITIVITY_DEFAULT = 0x8120,

    /// Returns the minimum value for the sensitivity.
    ///
    /// # Parameter type
    /// [`INT`]
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_GET_SENSITIVITY_MIN = 0x8121,

    /// Returns the maximum value for the sensitivity.
    ///
    /// # Parameter type
    /// [`INT`]
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_GET_SENSITIVITY_MAX = 0x8122,

    /// Sets the value for the sensitivity of the adaptive hot pixel correction.
    ///
    /// # Parameter type
    /// [`INT`]
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_SET_SENSITIVITY = 0x8123,

    /// Resets the hot pixel list and determines it with the next image.
    /// The cluster list is not reset.
    ///
    /// This command has only an effect in combination with the
    /// [`IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_ONCE`][HOTPIXEL_ADAPTIVE_CORRECTION_MODE::IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_ONCE]
    /// parameter.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_RESET_DETECTION = 0x8124,

    /// Returns the number of corrected hot pixels in the last image.
    ///
    /// If the hot pixels are determined once
    /// ([`IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_ONCE`][HOTPIXEL_ADAPTIVE_CORRECTION_MODE::IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_ONCE])
    /// the value will not change until the next determination. For dynamic determination
    /// ([`IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_DYNAMIC`][HOTPIXEL_ADAPTIVE_CORRECTION_MODE::IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_DYNAMIC]),
    /// the value may change with each image.
    ///
    /// # Parameter type
    /// [`INT`]
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_GET_NUMBER_DETECTED = 0x8125,

    /// Resets the hot pixel list and the cluster list and determines both lists with the
    /// next image.
    ///
    /// This command has only an effect in combination with the
    /// [`IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_ONCE_CLUSTER`][HOTPIXEL_ADAPTIVE_CORRECTION_MODE::IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_ONCE_CLUSTER]
    /// parameter.
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_RESET_DETECTION_CLUSTER = 0x8126,

    /// Returns the number of corrected hot pixel clusters in the last image.
    ///
    /// If the hot pixels are determined once
    /// ([`IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_ONCE_CLUSTER`][HOTPIXEL_ADAPTIVE_CORRECTION_MODE::IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_ONCE_CLUSTER])
    /// the value will not change until the next determination. For dynamic determination
    /// ([`IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_DYNAMIC_CLUSTER`][HOTPIXEL_ADAPTIVE_CORRECTION_MODE::IS_HOTPIXEL_ADAPTIVE_CORRECTION_DETECT_DYNAMIC_CLUSTER]),
    /// the value may change with each image.
    IS_HOTPIXEL_ADAPTIVE_CORRECTION_GET_NUMBER_DETECTED_CLUSTER = 0x8127,
}

unsafe extern "C" {
    /// Configures the correction of sensor hot pixels.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nMode` - Command. See [`IS_HOTPIXEL_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `SizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_TYPE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_CALIBRATED`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_NULL_POINTER`]
    /// * [`IS_OUT_OF_MEMORY`]
    /// * [`IS_SUCCESS`]
    /// * [`IS_TIMED_OUT`]
    ///
    /// # Documentation
    /// [is_HotPixel](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_hotpixel.html)
    pub fn is_HotPixel(
        hCam: HIDS,
        nMode: IS_HOTPIXEL_CMD,
        pParam: *mut void,
        SizeOfParam: UINT,
    ) -> INT;
}
