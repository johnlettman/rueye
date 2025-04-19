//! Configure special camera functions provided by specific _uEye_ models.
//!
//! * [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
//! * [Setting the FPN correction](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturefpn.html)
//! * [Setting the JPEG compression](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturejpegcomp.html)
//! * [Using image effects](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureimageeffect.html)
//! * [Setting the I2C stop bit](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature-i2c-stop-bit.html)
//! * [Using the Log mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelogmode.html)
//! * [Using camera LUT with RAW formats](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelut.html)
//! * [Using level controlled trigger](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelevelconttrigger.html)
//! * [Using the extended pixel clock](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureextendpixelclock.html)
//! * [Displaying black reference](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureblackcol.html)
//! * [Setting the sensor bit depth](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturebitdepth.html)
//! * [Switching the shutter mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureshuttermode.html)
//! * [Getting the sensor temperature](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturesensortemp.html)
//! * [Using the internal image memory](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature-usb3-ueye-cp-rev2-image-memory.html)
//! * [Using the line scan mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelinescan.html)
//! * [Configuring the timestamp](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturetimestamp.html)
//!
//! # Documentation
//! [is_DeviceFeature](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{double, void, BOOL, BYTE, HIDS, INT, UINT, WORD, IS_RANGE_S32};
use std::mem::MaybeUninit;

/// Enumeration of commands for [`is_DeviceFeature`].
///
/// # Documentation
/// [is_DeviceFeature](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DEVICE_FEATURE_CMD {
    /// Returns the functions supported by the camera.
    ///
    /// # Parameter type
    /// [`DEVICE_FEATURE_MODE_CAPS`]
    ///
    /// # Documentation
    /// [is_DeviceFeature](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature.html)
    IS_DEVICE_FEATURE_CMD_GET_SUPPORTED_FEATURES = 1,

    /// Sets the line scan mode.
    ///
    /// # Parameter type
    /// [`UINT`], _one of:_
    /// * [`DEVICE_FEATURE_MODE_CAPS::IS_DEVICE_FEATURE_CAP_LINESCAN_MODE_FAST`]
    /// * [`DEVICE_FEATURE_MODE_CAPS::IS_DEVICE_FEATURE_CAP_LINESCAN_NUMBER`]
    ///
    /// # Documentation
    /// [Using the line scan mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelinescan.html)
    IS_DEVICE_FEATURE_CMD_SET_LINESCAN_MODE = 2,

    /// Returns the current set line scan mode.
    ///
    /// # Parameter type
    /// [`UINT`], _one of:_
    /// * [`DEVICE_FEATURE_MODE_CAPS::IS_DEVICE_FEATURE_CAP_LINESCAN_MODE_FAST`]
    /// * [`DEVICE_FEATURE_MODE_CAPS::IS_DEVICE_FEATURE_CAP_LINESCAN_NUMBER`]
    ///
    /// # Documentation
    /// [Using the line scan mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelinescan.html)
    IS_DEVICE_FEATURE_CMD_GET_LINESCAN_MODE = 3,

    /// Sets the scan line used for the line scan mode.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Using the line scan mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelinescan.html)
    IS_DEVICE_FEATURE_CMD_SET_LINESCAN_NUMBER = 4,

    /// Returns the scan line used for the line scan mode.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Using the line scan mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelinescan.html)
    IS_DEVICE_FEATURE_CMD_GET_LINESCAN_NUMBER = 5,

    /// Sets the shutter mode.
    ///
    /// # Parameter type
    /// [`UINT`], _one of:_
    /// * [`DEVICE_FEATURE_MODE_CAPS::IS_DEVICE_FEATURE_CAP_SHUTTER_MODE_ROLLING`]
    /// * [`DEVICE_FEATURE_MODE_CAPS::IS_DEVICE_FEATURE_CAP_SHUTTER_MODE_ROLLING_GLOBAL_START`]
    /// * [`DEVICE_FEATURE_MODE_CAPS::IS_DEVICE_FEATURE_CAP_SHUTTER_MODE_GLOBAL`]
    /// * [`DEVICE_FEATURE_MODE_CAPS::IS_DEVICE_FEATURE_CAP_SHUTTER_MODE_GLOBAL_ALTERNATIVE_TIMING`]
    ///
    /// # Documentation
    /// [Switching the shutter mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureshuttermode.html)
    IS_DEVICE_FEATURE_CMD_SET_SHUTTER_MODE = 6,

    /// Returns the shutter mode.
    ///
    /// # Parameter type
    /// [`UINT`], _one of:_
    /// * [`DEVICE_FEATURE_MODE_CAPS::IS_DEVICE_FEATURE_CAP_SHUTTER_MODE_ROLLING`]
    /// * [`DEVICE_FEATURE_MODE_CAPS::IS_DEVICE_FEATURE_CAP_SHUTTER_MODE_ROLLING_GLOBAL_START`]
    /// * [`DEVICE_FEATURE_MODE_CAPS::IS_DEVICE_FEATURE_CAP_SHUTTER_MODE_GLOBAL`]
    /// * [`DEVICE_FEATURE_MODE_CAPS::IS_DEVICE_FEATURE_CAP_SHUTTER_MODE_GLOBAL_ALTERNATIVE_TIMING`]
    ///
    /// # Documentation
    /// [Switching the shutter mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureshuttermode.html)
    IS_DEVICE_FEATURE_CMD_GET_SHUTTER_MODE = 7,
    IS_DEVICE_FEATURE_CMD_SET_PREFER_XS_HS_MODE = 8,
    IS_DEVICE_FEATURE_CMD_GET_PREFER_XS_HS_MODE = 9,
    IS_DEVICE_FEATURE_CMD_GET_DEFAULT_PREFER_XS_HS_MODE = 10,

    /// Returns the default settings for the Log mode.
    ///
    /// # Parameter type
    /// [`LOG_MODES`]
    ///
    /// # Documentation
    /// [Using the Log mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelogmode.html)
    IS_DEVICE_FEATURE_CMD_GET_LOG_MODE_DEFAULT = 11,

    /// Returns the current Log mode.
    ///
    /// # Parameter type
    /// [`LOG_MODES`]
    ///
    /// # Documentation
    /// [Using the Log mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelogmode.html)
    IS_DEVICE_FEATURE_CMD_GET_LOG_MODE = 12,

    /// Sets the Log mode.
    ///
    /// # Parameter type
    /// [`LOG_MODES`]
    ///
    /// # Documentation
    /// [Using the Log mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelogmode.html)
    IS_DEVICE_FEATURE_CMD_SET_LOG_MODE = 13,

    /// Returns the default setting for the manual value of the Log mode.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Using the Log mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelogmode.html)
    IS_DEVICE_FEATURE_CMD_GET_LOG_MODE_MANUAL_VALUE_DEFAULT = 14,

    /// Returns the range of the manual value of the Log mode.
    ///
    /// # Parameter type
    /// [`IS_RANGE_S32`]
    ///
    /// # Documentation
    /// [Using the Log mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelogmode.html)
    IS_DEVICE_FEATURE_CMD_GET_LOG_MODE_MANUAL_VALUE_RANGE = 15,

    /// Returns the current manual value of the Log mode.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Using the Log mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelogmode.html)
    IS_DEVICE_FEATURE_CMD_GET_LOG_MODE_MANUAL_VALUE = 16,

    /// Sets the manual value of the Log mode.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Using the Log mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelogmode.html)
    IS_DEVICE_FEATURE_CMD_SET_LOG_MODE_MANUAL_VALUE = 17,

    /// Returns the default settings for the manual gain for the Log mode.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Using the Log mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelogmode.html)
    IS_DEVICE_FEATURE_CMD_GET_LOG_MODE_MANUAL_GAIN_DEFAULT = 18,

    /// Returns the range for the manual gain of the Log mode.
    ///
    /// # Parameter type
    /// [`IS_RANGE_S32`]
    ///
    /// # Documentation
    /// [Using the Log mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelogmode.html)
    IS_DEVICE_FEATURE_CMD_GET_LOG_MODE_MANUAL_GAIN_RANGE = 19,

    /// Returns the current manual gain of the Log mode.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Using the Log mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelogmode.html)
    IS_DEVICE_FEATURE_CMD_GET_LOG_MODE_MANUAL_GAIN = 20,

    /// Sets the manual gain of the Log mode.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Using the Log mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelogmode.html)
    IS_DEVICE_FEATURE_CMD_SET_LOG_MODE_MANUAL_GAIN = 21,

    /// Returns the default value of the AOI merge mode.
    ///
    /// # Parameter type
    /// [`VERTICAL_AOI_MERGE_MODES`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_MODE_DEFAULT = 22,

    /// Returns the current set AOI merge mode.
    ///
    /// # Parameter type
    /// [`VERTICAL_AOI_MERGE_MODES`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_MODE = 23,

    /// Sets the AOI merge mode.
    ///
    /// # Parameter type
    /// [`VERTICAL_AOI_MERGE_MODES`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_SET_VERTICAL_AOI_MERGE_MODE = 24,

    /// Returns the default value for the AOI merge position.
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_POSITION_DEFAULT = 25,

    /// Returns the range for the AOI merge position.
    ///
    /// # Parameter type
    /// [`IS_RANGE_S32`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_POSITION_RANGE = 26,

    /// Returns the position of the two sensor lines (default = `0`, i.e. the two top lines).
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_POSITION = 27,

    /// Sets the position of the two sensor lines.
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_SET_VERTICAL_AOI_MERGE_POSITION = 28,

    /// Returns the default setting of the FPN (fixed pattern noise) correction
    /// (enabled by default).
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Setting the FPN correction](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturefpn.html)
    IS_DEVICE_FEATURE_CMD_GET_FPN_CORRECTION_MODE_DEFAULT = 29,

    /// Returns the current setting of the FPN correction.
    ///
    /// # Parameter type
    /// [`FPN_CORRECTION_MODES`]
    ///
    /// # Documentation
    /// [Setting the FPN correction](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturefpn.html)
    IS_DEVICE_FEATURE_CMD_GET_FPN_CORRECTION_MODE = 30,

    /// Sets the settings of the FPN correction.
    ///
    /// # Parameter type
    /// [`FPN_CORRECTION_MODES`]
    ///
    /// # Documentation
    /// [Setting the FPN correction](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturefpn.html)
    IS_DEVICE_FEATURE_CMD_SET_FPN_CORRECTION_MODE = 31,

    /// Returns the range for the sensor source gain (default = `0`).
    ///
    /// The value may also be negative.
    ///
    /// # Parameter type
    /// [`IS_RANGE_S32`]
    ///
    /// # Documentation
    /// [Setting the sensor source gain](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturesourcegain.html)
    IS_DEVICE_FEATURE_CMD_GET_SENSOR_SOURCE_GAIN_RANGE = 32,

    /// Returns the default value of the sensor source gain.
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Setting the sensor source gain](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturesourcegain.html)
    IS_DEVICE_FEATURE_CMD_GET_SENSOR_SOURCE_GAIN_DEFAULT = 33,

    /// Returns the current set sensor source gain.
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Setting the sensor source gain](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturesourcegain.html)
    IS_DEVICE_FEATURE_CMD_GET_SENSOR_SOURCE_GAIN = 34,

    /// Sets the sensor source gain.
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Setting the sensor source gain](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturesourcegain.html)
    IS_DEVICE_FEATURE_CMD_SET_SENSOR_SOURCE_GAIN = 35,

    /// Returns the default setting for the black reference.
    ///
    /// # Parameter type
    /// [`BLACK_REFERENCE_MODES`]
    ///
    /// # Documentation
    /// [Displaying black reference](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureblackcol.html)
    IS_DEVICE_FEATURE_CMD_GET_BLACK_REFERENCE_MODE_DEFAULT = 36,

    /// Returns the current setting of the black reference.
    ///
    /// # Parameter type
    /// [`BLACK_REFERENCE_MODES`]
    ///
    /// # Documentation
    /// [Displaying black reference](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureblackcol.html)
    IS_DEVICE_FEATURE_CMD_GET_BLACK_REFERENCE_MODE = 37,

    /// Sets the settings of black reference.
    ///
    /// # Parameter type
    /// [`BLACK_REFERENCE_MODES`]
    ///
    /// # Documentation
    /// [Displaying black reference](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureblackcol.html)
    IS_DEVICE_FEATURE_CMD_SET_BLACK_REFERENCE_MODE = 38,

    /// Returns if the camera LUT can be used in combination with RAW formats.
    /// (`0` = `false` or `1` = `true`).
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Documentation
    /// [Using camera LUT with RAW formats](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelut.html)
    IS_DEVICE_FEATURE_CMD_GET_ALLOW_RAW_WITH_LUT = 39,

    /// Sets if the camera LUT can be used in combination with RAW formats.
    ///
    /// If the value `1` is passed, the camera LUT can also be used with RAW formats.
    /// The default value is `0`, i.e. the feature is disabled.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Documentation
    /// [Using camera LUT with RAW formats](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelut.html)
    IS_DEVICE_FEATURE_CMD_SET_ALLOW_RAW_WITH_LUT = 40,

    /// Returns the supported sensor bit depth as a bitmask.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Setting the sensor bit depth](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturebitdepth.html)
    IS_DEVICE_FEATURE_CMD_GET_SUPPORTED_SENSOR_BIT_DEPTHS = 41,

    /// Returns the default setting for the sensor bit depth.
    ///
    /// # Parameter type
    /// [`SENSOR_BIT_DEPTH`]
    ///
    /// # Documentation
    /// [Setting the sensor bit depth](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturebitdepth.html)
    IS_DEVICE_FEATURE_CMD_GET_SENSOR_BIT_DEPTH_DEFAULT = 42,

    /// Returns the current sensor bit depth.
    ///
    /// # Parameter type
    /// [`SENSOR_BIT_DEPTH`]
    ///
    /// # Documentation
    /// [Setting the sensor bit depth](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturebitdepth.html)
    IS_DEVICE_FEATURE_CMD_GET_SENSOR_BIT_DEPTH = 43,

    /// Sets the sensor bit depth.
    ///
    /// # Parameter type
    /// [`SENSOR_BIT_DEPTH`]
    ///
    /// # Documentation
    /// [Setting the sensor bit depth](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturebitdepth.html)
    IS_DEVICE_FEATURE_CMD_SET_SENSOR_BIT_DEPTH = 44,
    IS_DEVICE_FEATURE_CMD_GET_TEMPERATURE = 45,

    /// Returns the current value of the JPEG compression.
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Setting the JPEG compression](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturejpegcomp.html)
    IS_DEVICE_FEATURE_CMD_GET_JPEG_COMPRESSION = 46,

    /// Sets the value for the JPEG compression.
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Setting the JPEG compression](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturejpegcomp.html)
    IS_DEVICE_FEATURE_CMD_SET_JPEG_COMPRESSION = 47,

    /// Returns the default value for the JPEG compression. The default value is `2`.
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Setting the JPEG compression](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturejpegcomp.html)
    IS_DEVICE_FEATURE_CMD_GET_JPEG_COMPRESSION_DEFAULT = 48,

    /// Returns the range for the JPEG compression (`1..9`).
    ///
    /// The higher the value, the more the image data is compressed.
    ///
    /// # Parameter type
    /// [`IS_RANGE_S32`]
    ///
    /// # Documentation
    /// [Setting the JPEG compression](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturejpegcomp.html)
    IS_DEVICE_FEATURE_CMD_GET_JPEG_COMPRESSION_RANGE = 49,
    IS_DEVICE_FEATURE_CMD_GET_NOISE_REDUCTION_MODE = 50,
    IS_DEVICE_FEATURE_CMD_SET_NOISE_REDUCTION_MODE = 51,
    IS_DEVICE_FEATURE_CMD_GET_NOISE_REDUCTION_MODE_DEFAULT = 52,

    /// Returns the current configuration of the timestamp.
    ///
    /// # Parameter type
    /// [`IS_TIMESTAMP_CONFIGURATION`]
    ///
    /// # Documentation
    /// [Configuring the timestamp](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturetimestamp.html)
    IS_DEVICE_FEATURE_CMD_GET_TIMESTAMP_CONFIGURATION = 53,

    /// Sets the new configuration of the timestamp.
    ///
    /// Note that initially all parameters must be set.
    ///
    /// # Parameter type
    /// [`IS_TIMESTAMP_CONFIGURATION`]
    ///
    /// # Documentation
    /// [Configuring the timestamp](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturetimestamp.html)
    IS_DEVICE_FEATURE_CMD_SET_TIMESTAMP_CONFIGURATION = 54,

    /// Returns the default value of the AOI merge height (`2` for color, `1` for monochrome).
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_HEIGHT_DEFAULT = 55,

    /// Returns the number of list elements with the allowed AOI merge heights.
    ///
    /// The image height has to be a multiple of the AOI merge height.
    /// Therefore, the list of the current image height changes.
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_HEIGHT_NUMBER = 56,

    /// Returns the list with the allowed AOI merge heights.
    ///
    /// # Parameter type
    /// _List of_ [`INT`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_HEIGHT_LIST = 57,

    /// Returns the current set AOI merge height.
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_HEIGHT = 58,

    /// Sets the AOI merge height.
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_SET_VERTICAL_AOI_MERGE_HEIGHT = 59,

    /// Returns the default value for the additional AOI merge position.
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_ADDITIONAL_POSITION_DEFAULT = 60,

    /// Returns the range for the additional AOI merge position.
    ///
    /// # Parameter type
    /// [`IS_RANGE_S32`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_ADDITIONAL_POSITION_RANGE = 61,

    /// Returns the position of the additional used sensor line(s).
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_ADDITIONAL_POSITION = 62,

    /// Sets the position of the additional used sensor line(s).
    ///
    /// # Parameter type
    /// [`INT`]
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_SET_VERTICAL_AOI_MERGE_ADDITIONAL_POSITION = 63,

    /// Returns the content of the sensor register.
    ///
    /// This value is not the absolute temperature.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Getting the sensor temperature](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturesensortemp.html)
    IS_DEVICE_FEATURE_CMD_GET_SENSOR_TEMPERATURE_NUMERICAL_VALUE = 64,

    /// Sets an image effect.
    ///
    /// # Parameter type
    /// [`IMAGE_EFFECT_MODE`]
    ///
    /// # Documentation
    /// [Using image effects](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureimageeffect.html)
    IS_DEVICE_FEATURE_CMD_SET_IMAGE_EFFECT = 65,

    /// Returns the current set image effect.
    ///
    /// # Parameter type
    /// [`IMAGE_EFFECT_MODE`]
    ///
    /// # Documentation
    /// [Using image effects](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureimageeffect.html)
    IS_DEVICE_FEATURE_CMD_GET_IMAGE_EFFECT = 66,

    /// Returns the default setting for the image effect.
    ///
    /// # Parameter type
    /// [`IMAGE_EFFECT_MODE`]
    ///
    /// # Documentation
    /// [Using image effects](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureimageeffect.html)
    IS_DEVICE_FEATURE_CMD_GET_IMAGE_EFFECT_DEFAULT = 67,

    /// Returns the default settings for the extended pixel clock range
    /// (default = [`IS_EXTENDED_PIXELCLOCK_RANGE::EXTENDED_PIXELCLOCK_RANGE_OFF`]).
    ///
    /// # Parameter type
    /// [`IS_EXTENDED_PIXELCLOCK_RANGE`]
    ///
    /// # Documentation
    /// [Using the extended pixel clock](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureextendpixelclock.html)
    IS_DEVICE_FEATURE_CMD_GET_EXTENDED_PIXELCLOCK_RANGE_ENABLE_DEFAULT = 68,

    /// Returns the current setting for the extended pixel clock range.
    ///
    /// # Parameter type
    /// [`IS_EXTENDED_PIXELCLOCK_RANGE`]
    ///
    /// # Documentation
    /// [Using the extended pixel clock](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureextendpixelclock.html)
    IS_DEVICE_FEATURE_CMD_GET_EXTENDED_PIXELCLOCK_RANGE_ENABLE = 69,

    /// Enables/disables the use of the extended pixel clock range.
    ///
    /// # Parameter type
    /// [`IS_EXTENDED_PIXELCLOCK_RANGE`]
    ///
    /// # Documentation
    /// [Using the extended pixel clock](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureextendpixelclock.html)
    IS_DEVICE_FEATURE_CMD_SET_EXTENDED_PIXELCLOCK_RANGE_ENABLE = 70,

    /// Returns a structure with the minimum and maximum setting ranges in multi integration mode.
    ///
    /// # Parameter type
    /// [`IS_MULTI_INTEGRATION_SCOPE`]
    ///
    /// # Documentation
    /// [Using the multi integration mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturemultiintmode.html)
    IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_GET_SCOPE = 71,

    /// Returns the set parameter for multi integration mode.
    ///
    /// # Parameter type
    /// [`IS_MULTI_INTEGRATION_CYCLES`]
    ///
    /// # Documentation
    /// [Using the multi integration mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturemultiintmode.html)
    IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_GET_PARAMS = 72,

    /// Sets the parameter for multi integration mode.
    ///
    /// # Parameter type
    /// [`IS_MULTI_INTEGRATION_CYCLES`]
    ///
    /// # Documentation
    /// [Using the multi integration mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturemultiintmode.html)
    IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_SET_PARAMS = 73,

    /// Returns the default setting for multi integration mode.
    ///
    /// # Parameter type
    /// [`IS_MULTI_INTEGRATION_MODE`]
    ///
    /// # Documentation
    /// [Using the multi integration mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturemultiintmode.html)
    IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_GET_MODE_DEFAULT = 74,

    /// Returns the current setting for multi integration mode.
    ///
    /// # Parameter type
    /// [`IS_MULTI_INTEGRATION_MODE`]
    ///
    /// # Documentation
    /// [Using the multi integration mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturemultiintmode.html)
    IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_GET_MODE = 75,

    /// Sets the multi integration mode.
    ///
    /// # Parameter type
    /// [`IS_MULTI_INTEGRATION_MODE`]
    ///
    /// # Documentation
    /// [Using the multi integration mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturemultiintmode.html)
    IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_SET_MODE = 76,
    IS_DEVICE_FEATURE_CMD_SET_I2C_TARGET = 77,
    IS_DEVICE_FEATURE_CMD_SET_WIDE_DYNAMIC_RANGE_MODE = 78,
    IS_DEVICE_FEATURE_CMD_GET_WIDE_DYNAMIC_RANGE_MODE = 79,
    IS_DEVICE_FEATURE_CMD_GET_WIDE_DYNAMIC_RANGE_MODE_DEFAULT = 80,
    IS_DEVICE_FEATURE_CMD_GET_SUPPORTED_BLACK_REFERENCE_MODES = 81,

    /// Enables/disables the use of the level controlled trigger.
    ///
    /// # Parameter type
    /// [`LEVEL_CONTROLLED_TRIGGER_INPUT_MODES`]
    ///
    /// # Documentation
    /// [Using level controlled trigger](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelevelconttrigger.html)
    IS_DEVICE_FEATURE_CMD_SET_LEVEL_CONTROLLED_TRIGGER_INPUT_MODE = 82,

    /// Returns the current state of the level controlled trigger.
    ///
    /// # Parameter type
    /// [`LEVEL_CONTROLLED_TRIGGER_INPUT_MODES`]
    ///
    /// # Documentation
    /// [Using level controlled trigger](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelevelconttrigger.html)
    IS_DEVICE_FEATURE_CMD_GET_LEVEL_CONTROLLED_TRIGGER_INPUT_MODE = 83,

    /// Returns the default settings for the level controlled trigger.
    ///
    /// # Parameter type
    /// [`LEVEL_CONTROLLED_TRIGGER_INPUT_MODES`]
    ///
    /// # Documentation
    /// [Using level controlled trigger](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelevelconttrigger.html)
    IS_DEVICE_FEATURE_CMD_GET_LEVEL_CONTROLLED_TRIGGER_INPUT_MODE_DEFAULT = 84,

    /// Returns the supported modes of the AOI merge mode
    /// (freerun, software trigger or hardware trigger).
    /// See: [`VERTICAL_AOI_MERGE_MODE_LINE_TRIGGER`].
    ///
    /// # Documentation
    /// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_MODE_SUPPORTED_LINE_MODES = 85,

    /// Enables/disables the I<sup>2</sup>C stop bit between the write and read command.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Documentation
    /// [Setting the I2C stop bit](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature-i2c-stop-bit.html)
    IS_DEVICE_FEATURE_CMD_SET_REPEATED_START_CONDITION_I2C = 86,

    /// Returns the current setting for the function.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Documentation
    /// [Setting the I2C stop bit](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature-i2c-stop-bit.html)
    IS_DEVICE_FEATURE_CMD_GET_REPEATED_START_CONDITION_I2C = 87,

    /// Returns the default setting for the function.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Documentation
    /// [Setting the I2C stop bit](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature-i2c-stop-bit.html)
    IS_DEVICE_FEATURE_CMD_GET_REPEATED_START_CONDITION_I2C_DEFAULT = 88,

    /// Returns the last signaled temperature state.
    ///
    /// # Parameter type
    /// [`IS_TEMPERATURE_CONTROL_STATUS`]
    ///
    /// # Documentation
    /// [Querying the temperature state](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature-status-temperature.html)
    IS_DEVICE_FEATURE_CMD_GET_TEMPERATURE_STATUS = 89,

    /// Returns if the image memory of the camera is enabled.
    ///
    /// # Parameter type
    /// [`IS_MEMORY_MODE`]
    ///
    /// # Documentation
    /// [Using the internal image memory](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature-usb3-ueye-cp-rev2-image-memory.html)
    IS_DEVICE_FEATURE_CMD_GET_MEMORY_MODE_ENABLE = 90,

    /// Enables/disables the image memory of the camera.
    ///
    /// # Parameter type
    /// [`IS_MEMORY_MODE`]
    ///
    /// # Documentation
    /// [Using the internal image memory](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature-usb3-ueye-cp-rev2-image-memory.html)
    IS_DEVICE_FEATURE_CMD_SET_MEMORY_MODE_ENABLE = 91,

    /// Returns the default setting.
    ///
    /// # Parameter type
    /// [`IS_MEMORY_MODE`]
    ///
    /// # Documentation
    /// [Using the internal image memory](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature-usb3-ueye-cp-rev2-image-memory.html)
    IS_DEVICE_FEATURE_CMD_GET_MEMORY_MODE_ENABLE_DEFAULT = 92,
    IS_DEVICE_FEATURE_CMD_93 = 93,
    IS_DEVICE_FEATURE_CMD_94 = 94,
    IS_DEVICE_FEATURE_CMD_95 = 95,
    IS_DEVICE_FEATURE_CMD_96 = 96,
    IS_DEVICE_FEATURE_CMD_GET_SUPPORTED_EXTERNAL_INTERFACES = 97,
    IS_DEVICE_FEATURE_CMD_GET_EXTERNAL_INTERFACE = 98,
    IS_DEVICE_FEATURE_CMD_SET_EXTERNAL_INTERFACE = 99,
    IS_DEVICE_FEATURE_CMD_EXTENDED_AWB_LIMITS_GET = 100,
    IS_DEVICE_FEATURE_CMD_EXTENDED_AWB_LIMITS_SET = 101,

    /// Returns if the internal image memory is supported.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Documentation
    /// [Using the internal image memory](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature-usb3-ueye-cp-rev2-image-memory.html)
    IS_DEVICE_FEATURE_CMD_GET_MEMORY_MODE_ENABLE_SUPPORTED = 102,
    IS_DEVICE_FEATURE_CMD_SET_SPI_TARGET = 103,

    /// Returns if the camera is calibrated
    /// (_UI-313x, UI-314x, UI-316x, UI-318x only_).
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [Setting the FPN correction](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturefpn.html)
    IS_DEVICE_FEATURE_CMD_GET_FPN_CORRECTION_IS_CALIBRATED = 104,

    /// Sets the setting for loading the FPN correction data
    /// (_UI-313x, UI-314x, UI-316x, UI-318x only_).
    ///
    /// # Parameter type
    /// [`FPN_CORRECTION_DATA_LOADING`]
    ///
    /// # Documentation
    /// [Setting the FPN correction](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturefpn.html)
    IS_DEVICE_FEATURE_CMD_SET_FPN_CORRECTION_DATA_LOADING = 105,

    /// Returns the current setting for loading the FPN correction data
    /// (_UI-313x, UI-314x, UI-316x, UI-318x only_).
    ///
    /// # Parameter type
    /// [`FPN_CORRECTION_DATA_LOADING`]
    ///
    /// # Documentation
    /// [Setting the FPN correction](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturefpn.html)
    IS_DEVICE_FEATURE_CMD_GET_FPN_CORRECTION_DATA_LOADING = 106,
    IS_DEVICE_FEATURE_CMD_GET_MEMORY_MODE_BUFFER_LIMIT = 107,
    IS_DEVICE_FEATURE_CMD_GET_MEMORY_MODE_BUFFER_LIMIT_DEFAULT = 108,
    IS_DEVICE_FEATURE_CMD_SET_MEMORY_MODE_BUFFER_LIMIT = 109,

    /// Returns the default setting for loading the FPN correction data
    /// (_UI-313x, UI-314x, UI-316x, UI-318x only_).
    ///
    /// # Parameter type
    /// [`FPN_CORRECTION_DATA_LOADING`]
    ///
    /// # Documentation
    /// [Setting the FPN correction](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturefpn.html)
    IS_DEVICE_FEATURE_CMD_GET_FPN_CORRECTION_DATA_LOADING_DEFAULT = 110,
    IS_DEVICE_FEATURE_CMD_GET_BLACKLEVEL_OFFSET_CORRECTION = 111,
    IS_DEVICE_FEATURE_CMD_SET_BLACKLEVEL_OFFSET_CORRECTION = 112,
    IS_DEVICE_FEATURE_CMD_GET_ALTERNATIVE_TRIGGER_MODE = 113,
    IS_DEVICE_FEATURE_CMD_SET_ALTERNATIVE_TRIGGER_MODE = 114,
    IS_DEVICE_FEATURE_CMD_GET_AOI_CONSTANT_FRAMERATE_ENABLE = 115,
    IS_DEVICE_FEATURE_CMD_SET_AOI_CONSTANT_FRAMERATE_ENABLE = 116,
}

/// Enumeration of device feature capability flags.
///
/// Used with: [`IS_DEVICE_FEATURE_CMD_GET_SUPPORTED_FEATURES`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_SUPPORTED_FEATURES].
///
/// # Documentation
/// [is_DeviceFeature: Status flags from DEVICE_FEATURE_MODE_CAPS](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature.html#device_feature_mode_caps)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DEVICE_FEATURE_MODE_CAPS {
    /// Rolling shutter mode is supported/Set mode.
    IS_DEVICE_FEATURE_CAP_SHUTTER_MODE_ROLLING = 0x00000001,

    /// Global shutter mode is supported/Set mode.
    IS_DEVICE_FEATURE_CAP_SHUTTER_MODE_GLOBAL = 0x00000002,

    /// Fast line scan mode is supported/Set mode.
    IS_DEVICE_FEATURE_CAP_LINESCAN_MODE_FAST = 0x00000004,

    /// Line number at fast line scan mode is supported/Set number.
    IS_DEVICE_FEATURE_CAP_LINESCAN_NUMBER = 0x00000008,
    IS_DEVICE_FEATURE_CAP_PREFER_XS_HS_MODE = 0x00000010,

    /// Log mode is supported/Set mode.
    IS_DEVICE_FEATURE_CAP_LOG_MODE = 0x00000020,

    /// Rolling shutter mode with global start is supported/Set mode.
    IS_DEVICE_FEATURE_CAP_SHUTTER_MODE_ROLLING_GLOBAL_START = 0x00000040,

    /// Global shutter mode with different timing parameters is supported/Set mode.
    IS_DEVICE_FEATURE_CAP_SHUTTER_MODE_GLOBAL_ALTERNATIVE_TIMING = 0x00000080,

    /// Special [AOI merge mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
    /// which combines the lines of an AOI to a new image.
    IS_DEVICE_FEATURE_CAP_VERTICAL_AOI_MERGE = 0x00000100,

    /// Fixed pattern noise correction is supported.
    IS_DEVICE_FEATURE_CAP_FPN_CORRECTION = 0x00000200,

    /// Analog sensor source gain is supported.
    IS_DEVICE_FEATURE_CAP_SENSOR_SOURCE_GAIN = 0x00000400,

    /// Displays the black level reference in the image which can be used to calibrate the
    /// black level.
    IS_DEVICE_FEATURE_CAP_BLACK_REFERENCE = 0x00000800,

    /// Sets the bit depth of the sensor.
    ///
    /// With the [`SENSOR_BIT_DEPTH::IS_SENSOR_BIT_DEPTH_AUTO`] setting the software selects the
    /// appropriate sensor bit depth to the chosen image format. The bit depth can also be selected
    /// independently from the image format.
    ///
    /// <div class="warning">
    /// As you can choose combinations that does not fit, this function should be used by experts
    /// only. In most cases the auto control are sufficient.
    /// </div>
    IS_DEVICE_FEATURE_CAP_SENSOR_BIT_DEPTH = 0x00001000,

    /// Display of the internal camera temperature is supported.
    IS_DEVICE_FEATURE_CAP_TEMPERATURE = 0x00002000,

    /// JPEG compression is supported.
    IS_DEVICE_FEATURE_CAP_JPEG_COMPRESSION = 0x00004000,

    /// Noise suppression is supported.
    IS_DEVICE_FEATURE_CAP_NOISE_REDUCTION = 0x00008000,

    /// Configuration of the timestamp
    /// (e.g. reset the timestamp to `0` at a signal on the trigger pin).
    IS_DEVICE_FEATURE_CAP_TIMESTAMP_CONFIGURATION = 0x00010000,

    /// Image effects are supported.
    IS_DEVICE_FEATURE_CAP_IMAGE_EFFECT = 0x00020000,

    /// The use of the extended pixel clock range is supported.
    IS_DEVICE_FEATURE_CAP_EXTENDED_PIXELCLOCK_RANGE = 0x00040000,

    /// Multi integration mode is supported.
    IS_DEVICE_FEATURE_CAP_MULTI_INTEGRATION = 0x00080000,

    /// The use of the wide dynamic range is supported.
    IS_DEVICE_FEATURE_CAP_WIDE_DYNAMIC_RANGE = 0x00100000,

    /// The use of the level controlled trigger is supported.
    IS_DEVICE_FEATURE_CAP_LEVEL_CONTROLLED_TRIGGER = 0x00200000,

    /// Enabling/disabling the I<sup>2</sup>C stop bit for the read command is supported.
    IS_DEVICE_FEATURE_CAP_REPEATED_START_CONDITION_I2C = 0x00400000,

    /// Monitoring the temperature status is supported.
    IS_DEVICE_FEATURE_CAP_TEMPERATURE_STATUS = 0x00800000,

    /// Internal image memory is supported.
    IS_DEVICE_FEATURE_CAP_MEMORY_MODE = 0x01000000,
    IS_DEVICE_FEATURE_CAP_SEND_EXTERNAL_INTERFACE_DATA = 0x02000000,

    /// The camera model supports the [`IS_SET_EVENT_END_OF_EXPOSURE`] event.
    /// See [`is_Event`].
    IS_DEVICE_FEATURE_CAP_END_OF_EXPOSURE = 0x04000000,
}

/// Enumeration of temperature states.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_GET_TEMPERATURE_STATUS`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_TEMPERATURE_STATUS]
///
/// # Documentation
/// [Querying the temperature state](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature-status-temperature.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_TEMPERATURE_CONTROL_STATUS {
    /// Temperature is below 75°C.
    TEMPERATURE_CONTROL_STATUS_NORMAL = 0,

    /// Temperature is above 75°C, but below 80°C.
    TEMPERATURE_CONTROL_STATUS_WARNING = 1,

    /// Temperature is above 80°C, at a critical operating temperature.
    TEMPERATURE_CONTROL_STATUS_CRITICAL = 2,
}

/// Enumeration of Noise modes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum NOISE_REDUCTION_MODES {
    IS_NOISE_REDUCTION_OFF = 0,
    IS_NOISE_REDUCTION_ADAPTIVE = 1,
}

/// Enumeration of Log modes.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_GET_LOG_MODE_DEFAULT`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_LOG_MODE_DEFAULT]
/// * [`IS_DEVICE_FEATURE_CMD_GET_LOG_MODE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_LOG_MODE]
/// * [`IS_DEVICE_FEATURE_CMD_SET_LOG_MODE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_SET_LOG_MODE]
///
/// # Documentation
/// [Using the Log mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelogmode.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum LOG_MODES {
    /// Factory setting for the Log mode.
    IS_LOG_MODE_FACTORY_DEFAULT = 0,

    /// Log mode off.
    IS_LOG_MODE_OFF = 1,

    /// Manual Log mode. In this case the Log mode value and the Log mode gain are effective.
    IS_LOG_MODE_MANUAL = 2,

    /// Automatic Log mode (default setting).
    IS_LOG_MODE_AUTO = 3,
}

/// Enumeration of Vertical AOI merge modes.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_MODE_DEFAULT`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_MODE_DEFAULT]
/// * [`IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_MODE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_MODE]
/// * [`IS_DEVICE_FEATURE_CMD_SET_VERTICAL_AOI_MERGE_MODE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_SET_VERTICAL_AOI_MERGE_MODE]
///
/// # Documentation
/// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum VERTICAL_AOI_MERGE_MODES {
    /// Disables the AOI merge mode.
    ///
    /// It is recommended to first stop the image acquisition and disable the AOI merge mode.
    IS_VERTICAL_AOI_MERGE_MODE_OFF = 0,

    /// The sensor runs with maximum speed.
    IS_VERTICAL_AOI_MERGE_MODE_FREERUN = 1,

    /// The sensor is triggered via software.
    IS_VERTICAL_AOI_MERGE_MODE_TRIGGERED_SOFTWARE = 2,

    /// The sensor is triggered on GPIO 1 (falling edge).
    IS_VERTICAL_AOI_MERGE_MODE_TRIGGERED_FALLING_GPIO1 = 3,

    /// The sensor is triggered on GPIO 1 (rising edge).
    IS_VERTICAL_AOI_MERGE_MODE_TRIGGERED_RISING_GPIO1 = 4,

    /// The sensor is triggered on GPIO 2 (falling edge).
    IS_VERTICAL_AOI_MERGE_MODE_TRIGGERED_FALLING_GPIO2 = 5,

    /// The sensor is triggered on GPIO 2 (rising edge).
    IS_VERTICAL_AOI_MERGE_MODE_TRIGGERED_RISING_GPIO2 = 6,
}

/// Enumeration of supported Vertical AOI merge mode line trigger modes.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_MODE_SUPPORTED_LINE_MODES`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_VERTICAL_AOI_MERGE_MODE_SUPPORTED_LINE_MODES]
///
/// # Documentation
/// [Configuring the IDS line scan ("AOI merge mode")](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureaoimergemode.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum VERTICAL_AOI_MERGE_MODE_LINE_TRIGGER {
    /// AOI merge mode in freerun mode is supported.
    IS_VERTICAL_AOI_MERGE_MODE_LINE_FREERUN = 1,

    /// AOI merge mode with software trigger is supported.
    IS_VERTICAL_AOI_MERGE_MODE_LINE_SOFTWARE_TRIGGER = 2,

    /// AOI merge mode with hardware trigger is supported.
    IS_VERTICAL_AOI_MERGE_MODE_LINE_GPIO_TRIGGER = 4,
}

/// Enumeration of level controlled trigger input modes.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_SET_LEVEL_CONTROLLED_TRIGGER_INPUT_MODE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_SET_LEVEL_CONTROLLED_TRIGGER_INPUT_MODE]
/// * [`IS_DEVICE_FEATURE_CMD_GET_LEVEL_CONTROLLED_TRIGGER_INPUT_MODE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_LEVEL_CONTROLLED_TRIGGER_INPUT_MODE]
/// * [`IS_DEVICE_FEATURE_CMD_GET_LEVEL_CONTROLLED_TRIGGER_INPUT_MODE_DEFAULT`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_LEVEL_CONTROLLED_TRIGGER_INPUT_MODE_DEFAULT]
///
/// # Documentation
/// [Using level controlled trigger](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturelevelconttrigger.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum LEVEL_CONTROLLED_TRIGGER_INPUT_MODES {
    /// Disables the level controlled trigger.
    IS_LEVEL_CONTROLLED_TRIGGER_INPUT_OFF = 0,

    /// Enables the level controlled trigger.
    IS_LEVEL_CONTROLLED_TRIGGER_INPUT_ON = 1,
}

/// Enumeration of FPN correction modes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum FPN_CORRECTION_MODES {
    /// Disables the FPN correction in the camera.
    IS_FPN_CORRECTION_MODE_OFF = 0,

    /// Enables the FPN correction.
    IS_FPN_CORRECTION_MODE_HARDWARE = 1,
}

/// Enumeration of FPN persistent modes.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_GET_FPN_CORRECTION_DATA_LOADING`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_FPN_CORRECTION_DATA_LOADING]
/// * [`IS_DEVICE_FEATURE_CMD_SET_FPN_CORRECTION_DATA_LOADING`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_SET_FPN_CORRECTION_DATA_LOADING]
/// * [`IS_DEVICE_FEATURE_CMD_GET_FPN_CORRECTION_DATA_LOADING_DEFAULT`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_FPN_CORRECTION_DATA_LOADING_DEFAULT]
///
/// # Documentation
/// [Setting the FPN correction](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturefpn.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum FPN_CORRECTION_DATA_LOADING {
    /// FPN correction data is not loaded (FPN correction is inactive).
    IS_FPN_CORRECTION_DATA_LOADING_OFF = 0,

    /// FPN correction data is loaded (FPN correction is active).
    IS_FPN_CORRECTION_DATA_LOADING_ON = 1,
}

/// Enumeration of black level reference pixel modes.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_GET_BLACK_REFERENCE_MODE_DEFAULT`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_BLACK_REFERENCE_MODE_DEFAULT]
/// * [`IS_DEVICE_FEATURE_CMD_GET_BLACK_REFERENCE_MODE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_BLACK_REFERENCE_MODE]
/// * [`IS_DEVICE_FEATURE_CMD_SET_BLACK_REFERENCE_MODE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_SET_BLACK_REFERENCE_MODE]
///
/// # Documentation
/// [Displaying black reference](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureblackcol.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum BLACK_REFERENCE_MODES {
    /// Disables the black level reference.
    IS_BLACK_REFERENCE_MODE_OFF = 0x00000000,

    /// Enables the black level reference columns (left image side).
    IS_BLACK_REFERENCE_MODE_COLUMNS_LEFT = 0x00000001,

    /// Enables the black level reference rows (top).
    IS_BLACK_REFERENCE_MODE_ROWS_TOP = 0x00000002,
}

/// Enumeration of sensor bit depths.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_GET_SENSOR_BIT_DEPTH_DEFAULT`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_SENSOR_BIT_DEPTH_DEFAULT]
/// * [`IS_DEVICE_FEATURE_CMD_GET_SENSOR_BIT_DEPTH`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_SENSOR_BIT_DEPTH]
/// * [`IS_DEVICE_FEATURE_CMD_SET_SENSOR_BIT_DEPTH`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_SET_SENSOR_BIT_DEPTH]
///
/// # Documentation
/// [Setting the sensor bit depth](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturebitdepth.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SENSOR_BIT_DEPTH {
    /// Sets the sensor bit depth to auto.
    IS_SENSOR_BIT_DEPTH_AUTO = 0x00000000,

    /// Sets the sensor bit depth to 8-bit.
    IS_SENSOR_BIT_DEPTH_8_BIT = 0x00000001,

    /// Sets the sensor bit depth to 10-bit.
    IS_SENSOR_BIT_DEPTH_10_BIT = 0x00000002,

    /// Sets the sensor bit depth to 12-bit.
    IS_SENSOR_BIT_DEPTH_12_BIT = 0x00000004,
}

/// Enumeration of timestamp configuration modes.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_GET_TIMESTAMP_CONFIGURATION`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_TIMESTAMP_CONFIGURATION]
/// * [`IS_DEVICE_FEATURE_CMD_SET_TIMESTAMP_CONFIGURATION`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_SET_TIMESTAMP_CONFIGURATION]
///
/// # Documentation
/// [Configuring the timestamp](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturetimestamp.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TIMESTAMP_CONFIGURATION_MODE {
    /// Sets once the camera timestamp to `0`.
    IS_RESET_TIMESTAMP_ONCE = 1,

    /// Used when parts of an already set configuration are re-set.
    ///
    /// This must not be used during initial configuration.
    IS_IGNORE_PARAMETER = -1,
}

/// Enumeration of timestamp configuration pins.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_GET_TIMESTAMP_CONFIGURATION`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_TIMESTAMP_CONFIGURATION]
/// * [`IS_DEVICE_FEATURE_CMD_SET_TIMESTAMP_CONFIGURATION`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_SET_TIMESTAMP_CONFIGURATION]
///
/// # Documentation
/// [Configuring the timestamp](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturetimestamp.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TIMESTAMP_CONFIGURATION_PIN {
    /// Timestamp is not reset.
    TIMESTAMP_CONFIGURATION_PIN_NONE = 0,

    /// Timestamp is set by signal on the trigger pin.
    TIMESTAMP_CONFIGURATION_PIN_TRIGGER = 1,

    /// Timestamp is set by signal on GPIO 1.
    TIMESTAMP_CONFIGURATION_PIN_GPIO_1 = 2,

    /// Timestamp is set by signal on GPIO 2.
    TIMESTAMP_CONFIGURATION_PIN_GPIO_2 = 3,
}

/// Enumeration of timestamp configuration edges.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_GET_TIMESTAMP_CONFIGURATION`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_TIMESTAMP_CONFIGURATION]
/// * [`IS_DEVICE_FEATURE_CMD_SET_TIMESTAMP_CONFIGURATION`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_SET_TIMESTAMP_CONFIGURATION]
///
/// # Documentation
/// [Configuring the timestamp](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturetimestamp.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TIMESTAMP_CONFIGURATION_EDGE {
    /// Falling edge.
    TIMESTAMP_CONFIGURATION_EDGE_FALLING = 0,

    /// Rising edge.
    TIMESTAMP_CONFIGURATION_EDGE_RISING = 1,
}

/// Timestamp configuration data.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_GET_TIMESTAMP_CONFIGURATION`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_TIMESTAMP_CONFIGURATION]
/// * [`IS_DEVICE_FEATURE_CMD_SET_TIMESTAMP_CONFIGURATION`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_SET_TIMESTAMP_CONFIGURATION]
///
/// # Documentation
/// [Configuring the timestamp: Contents of the `IS_TIMESTAMP_CONFIGURATION` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturetimestamp.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IS_TIMESTAMP_CONFIGURATION {
    /// Mode to be set.
    pub s32Mode: TIMESTAMP_CONFIGURATION_MODE,

    /// Pin for setting the timestamp.
    pub s32Pin: TIMESTAMP_CONFIGURATION_PIN,

    /// The timestamp is set by rising or falling edge.
    pub s32Edge: TIMESTAMP_CONFIGURATION_EDGE,
}

/// Enumeration of image effect configuration.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_SET_IMAGE_EFFECT`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_SET_IMAGE_EFFECT]
/// * [`IS_DEVICE_FEATURE_CMD_GET_IMAGE_EFFECT`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_IMAGE_EFFECT]
/// * [`IS_DEVICE_FEATURE_CMD_GET_IMAGE_EFFECT_DEFAULT`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_IMAGE_EFFECT_DEFAULT]
///
/// # Documentation
/// [Using image effects](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureimageeffect.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IMAGE_EFFECT_MODE {
    /// Disables image effects.
    IS_IMAGE_EFFECT_DISABLE = 0,

    /// Enables the sepia effect.
    IS_IMAGE_EFFECT_SEPIA = 1,

    /// Enables the monochrome effect.
    IS_IMAGE_EFFECT_MONOCHROME = 2,

    /// Enables the negative effect.
    IS_IMAGE_EFFECT_NEGATIVE = 3,

    /// Enables the crosshairs effect.
    IS_IMAGE_EFFECT_CROSSHAIRS = 4,
}

/// Enumeration of the extended pixel clock ranges.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_GET_EXTENDED_PIXELCLOCK_RANGE_ENABLE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_EXTENDED_PIXELCLOCK_RANGE_ENABLE]
/// * [`IS_DEVICE_FEATURE_CMD_SET_EXTENDED_PIXELCLOCK_RANGE_ENABLE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_SET_EXTENDED_PIXELCLOCK_RANGE_ENABLE]
/// * [`IS_DEVICE_FEATURE_CMD_GET_EXTENDED_PIXELCLOCK_RANGE_ENABLE_DEFAULT`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_EXTENDED_PIXELCLOCK_RANGE_ENABLE_DEFAULT]
///
/// # Documentation
/// [Using the extended pixel clock](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeatureextendpixelclock.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_EXTENDED_PIXELCLOCK_RANGE {
    EXTENDED_PIXELCLOCK_RANGE_OFF = 0,
    EXTENDED_PIXELCLOCK_RANGE_ON = 1,
}

/// Enumeration of the multi integration modes.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_GET_MODE_DEFAULT`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_GET_MODE_DEFAULT]
/// * [`IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_GET_MODE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_GET_MODE]
/// * [`IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_SET_MODE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_SET_MODE]
///
/// # Documentation
/// [Using the multi integration mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturemultiintmode.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_MULTI_INTEGRATION_MODE {
    /// Disables the multi integration mode.
    MULTI_INTEGRATION_MODE_OFF = 0,

    /// The multi integration mode is controlled by software.
    ///
    /// In this case, the exposure sequence is generated by the camera-internal
    /// pulse-width modulation (PWM).
    /// In this mode, the parameters are effective.
    MULTI_INTEGRATION_MODE_SOFTWARE = 1,

    /// Sets the multi integration mode on GPIO 1.
    ///
    /// The exposure sequence is generated via the GPIO 1 input by the user.
    /// In this mode, the parameters are not effective.
    MULTI_INTEGRATION_MODE_GPIO1 = 2,

    /// Sets the multi integration mode on GPIO 2.
    ///
    /// The exposure sequence is generated via the GPIO 2 input by the user.
    /// In this mode, the parameters are not effective.
    MULTI_INTEGRATION_MODE_GPIO2 = 3,
}

/// Multi integration parameters.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_GET_PARAMS`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_GET_PARAMS]
/// * [`IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_SET_PARAMS`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_SET_PARAMS]
///
/// # Documentation
/// [Using the multi integration mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturemultiintmode.html)
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct IS_MULTI_INTEGRATION_CYCLES {
    /// Pulse (exposure time) in milliseconds.
    pub dblIntegration_ms: double,

    /// Pause in milliseconds.
    pub dblPause_ms: double,
}

/// Maximum and minimum setting ranges in multi integration mode.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_GET_SCOPE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_MULTI_INTEGRATION_GET_SCOPE]
///
/// # Documentation
/// [Using the multi integration mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeaturemultiintmode.html)
#[derive(Debug)]
#[repr(C)]
pub struct IS_MULTI_INTEGRATION_SCOPE {
    /// Minimum pulse duration (exposure time) in milliseconds.
    pub dblMinIntegration_ms: double,

    /// Maximum pulse duration (exposure time) in milliseconds.
    pub dblMaxIntegration_ms: double,

    /// Pulse duration (exposure time) granularity in milliseconds.
    pub dblIntegrationGranularity_ms: double,

    /// Minimum pause duration in milliseconds.
    pub dblMinPause_ms: double,

    /// Maximum pause duration in milliseconds.
    pub dblMaxPause_ms: double,

    /// Pause duration granularity in milliseconds.
    pub dblPauseGranularity_ms: double,

    /// Minimum cycle duration (pulse and pause duration) in milliseconds.
    pub dblMinCycle_ms: double,

    /// Maximum cycle duration (pulse and pause duration) in milliseconds.
    pub dblMaxCycle_ms: double,

    /// Cycle granularity in milliseconds.
    pub dblCycleGranularity_ms: double,

    /// Minimum duration of trigger cycle (first cycle = trigger pulse + tripper pause)
    /// in milliseconds.
    pub dblMinTriggerCycle_ms: double,

    /// Minimum duration of the first pulse (trigger pulse) in milliseconds.
    pub dblMinTriggerDuration_ms: double,

    /// Minimum number of cycles.
    pub nMinNumberOfCycles: UINT,

    /// Maximum number of cycles.
    pub nMaxNumberOfCycles: UINT,

    /// (**reserved**)
    m_bReserved: [BYTE; 32],
}

impl Clone for IS_MULTI_INTEGRATION_SCOPE {
    fn clone(&self) -> IS_MULTI_INTEGRATION_SCOPE {
        // Unsafe allocate clone to avoid zeroing `m_bReserved`.
        let mut other = unsafe { MaybeUninit::<Self>::uninit().assume_init() };

        other.dblMinIntegration_ms = self.dblMinIntegration_ms;
        other.dblMaxIntegration_ms = self.dblMaxIntegration_ms;
        other.dblIntegrationGranularity_ms = self.dblIntegrationGranularity_ms;
        other.dblMinPause_ms = self.dblMinPause_ms;
        other.dblMaxPause_ms = self.dblMaxPause_ms;
        other.dblPauseGranularity_ms = self.dblPauseGranularity_ms;
        other.dblMinCycle_ms = self.dblMinCycle_ms;
        other.dblMaxCycle_ms = self.dblMaxCycle_ms;
        other.dblCycleGranularity_ms = self.dblCycleGranularity_ms;
        other.dblMinTriggerCycle_ms = self.dblMinTriggerCycle_ms;
        other.dblMinTriggerDuration_ms = self.dblMinTriggerDuration_ms;
        other.nMinNumberOfCycles = self.nMinNumberOfCycles;
        other.nMaxNumberOfCycles = self.nMaxNumberOfCycles;

        other
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_I2C_TARGET {
    I2C_TARGET_DEFAULT = 0,
    I2C_TARGET_SENSOR_1 = 1,
    I2C_TARGET_SENSOR_2 = 2,
    I2C_TARGET_LOGIC_BOARD = 4,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_SPI_TARGET {
    SPI_TARGET_DEFAULT = 0,
    SPI_TARGET_SENSOR_1 = 1,
    SPI_TARGET_SENSOR_2 = 2,
}

/// Enumeration of camera image memory modes.
///
/// # Related commands
/// * [`IS_DEVICE_FEATURE_CMD_GET_MEMORY_MODE_ENABLE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_MEMORY_MODE_ENABLE]
/// * [`IS_DEVICE_FEATURE_CMD_SET_MEMORY_MODE_ENABLE`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_SET_MEMORY_MODE_ENABLE]
/// * [`IS_DEVICE_FEATURE_CMD_GET_MEMORY_MODE_ENABLE_DEFAULT`][DEVICE_FEATURE_CMD::IS_DEVICE_FEATURE_CMD_GET_MEMORY_MODE_ENABLE_DEFAULT]
///
/// # Documentation
/// [Using the internal image memory](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature-usb3-ueye-cp-rev2-image-memory.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_MEMORY_MODE {
    /// Image memory of the camera is disabled.
    IS_MEMORY_MODE_OFF = 0,

    /// Image memory of the camera is enabled.
    IS_MEMORY_MODE_ON = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_EXTERNAL_INTERFACE_TYPE {
    IS_EXTERNAL_INTERFACE_TYPE_NONE = 0,
    IS_EXTERNAL_INTERFACE_TYPE_I2C = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_EXTERNAL_INTERFACE_REGISTER_TYPE {
    IS_EXTERNAL_INTERFACE_REGISTER_TYPE_8BIT = 0,
    IS_EXTERNAL_INTERFACE_REGISTER_TYPE_16BIT = 1,
    IS_EXTERNAL_INTERFACE_REGISTER_TYPE_NONE = 2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_EXTERNAL_INTERFACE_EVENT {
    IS_EXTERNAL_INTERFACE_EVENT_RISING_VSYNC = 0,
    IS_EXTERNAL_INTERFACE_EVENT_FALLING_VSYNC = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_EXTERNAL_INTERFACE_DATA {
    IS_EXTERNAL_INTERFACE_DATA_USER = 0,
    IS_EXTERNAL_INTERFACE_DATA_TIMESTAMP_FULL = 1,
    IS_EXTERNAL_INTERFACE_DATA_TIMESTAMP_LOWBYTE = 2,
    IS_EXTERNAL_INTERFACE_DATA_TIMESTAMP_HIGHBYTE = 3,
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct IS_EXTERNAL_INTERFACE_I2C_CONFIGURATION {
    pub bySlaveAddress: BYTE,
    pub wRegisterAddress: WORD,
    pub byRegisterAddressType: BYTE,
    pub byAckPolling: BYTE,

    byReserved: [BYTE; 11],
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct IS_EXTERNAL_INTERFACE_CONFIGURATION {
    pub wInterfaceType: WORD,
    pub sInterfaceConfiguration: [BYTE; 16],
    pub wSendEvent: WORD,
    pub wDataSelection: WORD,
}

unsafe extern "C" {
    /// Interface to control various device features.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`DEVICE_FEATURE_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAPTURE_MODE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_OUT_OF_MEMORY`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Documentation
    /// [is_DeviceFeature](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature.html)
    pub fn is_DeviceFeature(
        hCam: HIDS,
        nCommand: DEVICE_FEATURE_CMD,
        pParam: *mut void,
        cbSizeOfParam: UINT,
    ) -> INT;
}
