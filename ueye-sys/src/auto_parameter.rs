//! Manage different automatic controls.
//!
//! * [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html):
//!     With this function, you set the automatic controls for auto exposure shutter,
//!     auto gain and black level.
//! * [Configuring the auto white balance](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautowhite.html):
//!     This functions enables/disables the auto white balance.
//!     With this function, you can require all supported types for white balance.
//!     In addition to the older white balance with the Gray-World algorithm, there is also a
//!     color temperature control according to Kelvin. In addition to the function the
//!     supported color spaces are queried and set.

#![allow(non_camel_case_types)]

use crate::color_temperature::RGB_COLOR_MODELS;
use crate::constants::return_values::*;
use crate::types::{double, void, CHAR, HIDS, INT, IS_RANGE_S32, IS_RECT, UINT};
use bitflags::bitflags;
use std::mem::MaybeUninit;

bitflags! {
    /// AES modes (_supports bitmask_).
    ///
    /// # Documentation
    /// [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct AES_MODE: UINT {
        /// Peak mode.
        const IS_AES_MODE_PEAK = 0x01;

        /// Mean mode.
        const IS_AES_MODE_MEAN = 0x02;
    }
}

/// AES configuration used by [`is_AutoParameter`].
///
/// # Documentation
/// [Configuring the auto control: Content of the `AES_CONFIGURATION` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct AES_CONFIGURATION {
    /// Modes of the auto control.
    pub nMode: AES_MODE,

    pub pConfiguration: [CHAR; 1],
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AES_PEAK_CONFIGURATION {
    /// Specifies the AOI to be used for analysis.
    ///
    /// If no AOI should be used, all values must be set to `0`.
    ///
    /// For details on the AOI grids of the individual camera models,
    /// please see [Camera and sensor data](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/hw_sensoren.html).
    pub rectUserAOI: IS_RECT,

    /// Sets the number of frames to be skipped during automatic image control.
    pub nFrameSkip: UINT,

    /// Sets the hysteresis value.
    ///
    /// The hysteresis value must always be smaller than the target value of the auto control by
    /// `1` increment of the set granularity. An incorrect value is automatically adjusted.
    pub nHysteresis: UINT,

    /// Sets the brightness reference value for auto control.
    pub nReference: UINT,

    /// Sets the channels to be used for analysis.
    ///
    /// This parameter has no effect on monochrome color formats.
    pub nChannel: AES_CHANNEL,

    /// Sets the maximum exposure value for the control (`0` = no limit).
    pub f64Maximum: double,

    /// Sets the minimum exposure value for the control (`0` = no limit).
    pub f64Minimum: double,

    /// Sets the mode which is used for analysis.
    pub nMode: AES_PEAK_MODE,

    /// Sets the granularity which is used for analysis.
    ///
    /// The granularity has an effect on the following values:
    /// * [`AES_PEAK_CONFIGURATION::nHysteresis`]
    /// * [`AES_PEAK_CONFIGURATION::nReference`]
    /// * [`AES_PEAK_CONFIGURATION_RANGE::rangeHysteresis`]
    /// * [`AES_PEAK_CONFIGURATION_RANGE::rangeReference`]
    pub nGranularity: AES_GRANULARITY,
}

/// AES peak white configuration range used by [`is_AutoParameter`].
///
/// # Documentation
#[derive(Debug, Eq)]
#[repr(C)]
pub struct AES_PEAK_WHITE_CONFIGURATION_RANGE {
    /// Range for the number of frames to be skipped.
    pub rangeFrameSkip: IS_RANGE_S32,

    /// Range of hysteresis value.
    pub rangeHysteresis: IS_RANGE_S32,

    /// Range of brightness reference value.
    pub rangeReference: IS_RANGE_S32,

    /// (**reserved**)
    reserved: [CHAR; 32],
}

impl Clone for AES_PEAK_WHITE_CONFIGURATION_RANGE {
    fn clone(&self) -> Self {
        // Unsafe allocate clone to avoid zeroing `reserved`.
        let mut other = unsafe { MaybeUninit::<Self>::uninit().assume_init() };

        other.rangeFrameSkip = self.rangeFrameSkip;
        other.rangeHysteresis = self.rangeHysteresis;
        other.rangeReference = self.rangeReference;
        other
    }
}

impl PartialEq for AES_PEAK_WHITE_CONFIGURATION_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.rangeFrameSkip == other.rangeFrameSkip
            && self.rangeHysteresis == other.rangeHysteresis
            && self.rangeReference == other.rangeReference
    }
}

/// AES peak configuration range used by [`is_AutoParameter`].
///
/// # Documentation
/// [Configuring the auto control: Content of the `AES_PEAK_CONFIGURATION_RANGE` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct AES_PEAK_CONFIGURATION_RANGE {
    /// Range for the number of frames to be skipped.
    pub rangeFrameSkip: IS_RANGE_S32,

    /// Range of hysteresis value.
    pub rangeHysteresis: IS_RANGE_S32,

    /// Range of brightness reference value.
    pub rangeReference: IS_RANGE_S32,
}

/// AES channel enumeration used by [`AES_PEAK_CONFIGURATION::nChannel`].
///
/// # Documentation
/// [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum AES_CHANNEL {
    IS_AES_CHANNEL_MONO = 0x00,
    IS_AES_CHANNEL_RED = 0x01,
    IS_AES_CHANNEL_GREEN = 0x02,
    IS_AES_CHANNEL_BLUE = 0x04,
}

/// AES peak channel mode enumeration used by [`AES_PEAK_CONFIGURATION::nMode`].
///
/// # Documentation
/// [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum AES_PEAK_MODE {
    /// Controls to the target value of the color channel set by
    /// [`nChannel`][AES_PEAK_CONFIGURATION::nChannel].
    IS_AES_PEAK_MODE_SELECTED_CHANNELS = 0x00,

    /// Controls to the target value of the dominant color channel.
    ///
    /// The [`nChannel`][AES_PEAK_CONFIGURATION::nChannel] parameter is not taken into account
    /// in this mode.
    IS_AES_PEAK_MODE_LEADING_CHANNEL = 0x01,

    /// Controls to target value of the sum for all color channels in saturation.
    ///
    /// The [`nChannel`][AES_PEAK_CONFIGURATION::nChannel] parameter is not taken into account
    /// in this mode.
    IS_AES_PEAK_MODE_ACCUMULATED_CHANNELS = 0x02,
}

/// AES peak granularity enumeration used by [`AES_PEAK_CONFIGURATION::nGranularity`].
///
/// # Documentation
/// [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
#[repr(u32)]
pub enum AES_GRANULARITY {
    /// In percent, that is, with increment `1`.
    IS_AES_GRANULARITY_PER_100 = 0x00,

    /// In per-thousand, that is, with increment `0.1`.
    IS_AES_GRANULARITY_PER_1000 = 0x01,

    /// In 1/10 per-thousand, that is, with increment `0.01`
    IS_AES_GRANULARITY_PER_10000 = 0x02,
}

bitflags! {
    /// Auto White Balance modes (_supports bitmask_).
    ///
    /// # Documentation
    /// [Configuring the auto white balance](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautowhite.html)
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct AWB_MODE: UINT {
        /// Older white balance with the Gray-World algorithm.
        const IS_AWB_GREYWORLD = 0x0001;

        /// Color temperature control according to Kelvin.
        const IS_AWB_COLOR_TEMPERATURE = 0x0002;
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum IS_AUTOPARAMETER_ENABLE {
    IS_AUTOPARAMETER_DISABLE = 0,
    IS_AUTOPARAMETER_ENABLE = 1,
    IS_AUTOPARAMETER_ENABLE_RUNONCE = 2,
}

/// Enumeration of commands of function [`is_AutoParameter`].
///
///
pub enum IS_AUTOPARAMETER_CMD {
    /// Returns the supported types for auto white balance.
    ///
    /// # Parameter type
    /// [`AWB_MODE`], _bitmask_
    ///
    /// # Documentation
    /// [Configuring the auto white balance](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautowhite.html)
    IS_AWB_CMD_GET_SUPPORTED_TYPES = 1,

    /// Returns the current set type of the auto white balance.
    ///
    /// # Parameter type
    /// [`AWB_MODE`]
    ///
    /// # Documentation
    /// [Configuring the auto white balance](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautowhite.html)
    IS_AWB_CMD_GET_TYPE = 2,

    /// Sets the type of the auto white balance.
    ///
    /// # Parameter type
    /// [`AWB_MODE`]
    ///
    /// # Documentation
    /// [Configuring the auto white balance](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautowhite.html)
    IS_AWB_CMD_SET_TYPE = 3,

    /// Returns if the auto white balance is enabled.
    ///
    /// # Parameter type
    /// [`IS_AUTOPARAMETER_ENABLE`]
    ///
    /// # Documentation
    /// [Configuring the auto white balance](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautowhite.html)
    IS_AWB_CMD_GET_ENABLE = 4,

    /// Enables/disables the auto white balance.
    ///
    /// # Parameter type
    /// [`IS_AUTOPARAMETER_ENABLE`]
    ///
    /// # Documentation
    /// [Configuring the auto white balance](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautowhite.html)
    IS_AWB_CMD_SET_ENABLE = 5,

    /// Returns the supported color spaces for the auto white balance.
    ///
    /// # Parameter type
    /// [`RGB_COLOR_MODELS`], _bitmask_
    ///
    /// # Documentation
    /// [Configuring the auto white balance](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautowhite.html)
    IS_AWB_CMD_GET_SUPPORTED_RGB_COLOR_MODELS = 6,

    /// Returns the current color space for the auto white balance.
    ///
    /// # Parameter type
    /// [`RGB_COLOR_MODELS`]
    ///
    /// # Documentation
    /// [Configuring the auto white balance](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautowhite.html)
    IS_AWB_CMD_GET_RGB_COLOR_MODEL = 7,

    /// Sets the color space for the auto white balance.
    ///
    /// # Parameter type
    /// [`RGB_COLOR_MODELS`]
    ///
    /// # Documentation
    /// [Configuring the auto white balance](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautowhite.html)
    IS_AWB_CMD_SET_RGB_COLOR_MODEL = 8,

    /// Returns the supported mode for the auto control.
    ///
    /// # Parameter type
    /// [`AES_MODE`], _bitmask_
    ///
    /// # Documentation
    /// [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
    IS_AES_CMD_GET_SUPPORTED_TYPES = 9,

    /// Enables/disables the auto control.
    ///
    /// # Parameter type
    /// [`IS_AUTOPARAMETER_ENABLE`]
    ///
    /// # Documentation
    /// [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
    IS_AES_CMD_SET_ENABLE = 10,

    /// Returns if the auto control is enabled.
    ///
    /// # Parameter type
    /// [`IS_AUTOPARAMETER_ENABLE`]
    ///
    /// # Documentation
    /// [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
    IS_AES_CMD_GET_ENABLE = 11,

    /// Sets the mode for the auto control.
    ///
    /// # Parameter type
    /// [`AES_MODE`]
    ///
    /// # Documentation
    /// [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
    IS_AES_CMD_SET_TYPE = 12,

    /// Returns the current set mode for the auto control.
    ///
    /// # Parameter type
    /// [`AES_MODE`]
    ///
    /// # Documentation
    /// [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
    IS_AES_CMD_GET_TYPE = 13,

    /// Set the configuration for the auto control.
    ///
    /// # Parameter type
    /// _Glued together:_
    /// * [`AES_CONFIGURATION`]
    /// * [`AES_PEAK_CONFIGURATION`]
    ///
    /// # Documentation
    /// [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
    IS_AES_CMD_SET_CONFIGURATION = 14,

    /// Returns the current configuration for the auto control.
    ///
    /// # Parameter type
    /// _Glued together:_
    /// * [`AES_CONFIGURATION`]
    /// * [`AES_PEAK_CONFIGURATION`]
    ///
    /// # Documentation
    /// [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
    IS_AES_CMD_GET_CONFIGURATION = 15,

    /// Returns the default configuration for the auto control.
    ///
    /// # Parameter type
    /// _Glued together:_
    /// * [`AES_CONFIGURATION`]
    /// * [`AES_PEAK_CONFIGURATION`]
    ///
    /// # Documentation
    /// [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
    IS_AES_CMD_GET_CONFIGURATION_DEFAULT = 16,

    /// Returns the range of the possible parameters of the auto control.
    ///
    /// # Parameter type
    /// _Glued together:_
    /// * [`AES_CONFIGURATION`]
    /// * [`AES_PEAK_CONFIGURATION_RANGE`]
    ///
    /// # Documentation
    /// [Configuring the auto control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameterautoexposure.html)
    IS_AES_CMD_GET_CONFIGURATION_RANGE = 17,
}

unsafe extern "C" {
    /// Manage different automatic controls.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`IS_AUTOPARAMETER_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Documentation
    /// [is_AutoParameter](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_autoparameter.html)
    pub fn is_AutoParameter(
        hCam: HIDS,
        nCommand: IS_AUTOPARAMETER_CMD,
        pParam: *mut void,
        cbSizeOfParam: UINT,
    ) -> INT;
}
