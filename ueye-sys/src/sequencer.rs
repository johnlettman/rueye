//! Configure and activate the sequencer mode.
//!
//! In sequencer mode, you can define different parameters for image acquisition via sequencer
//! sets that are processed one after another. For example, you can create imaging sequences
//! with different exposure times. Therefore, fast parameter changes are possible.
//!
//! Each sequencer set has the following settings:
//! * Exposure time
//! * Gain (master, red, green and blue)
//! * X and Y position of the area of interest (AOI)
//! * Flash settings
//! * Reference to the following sequencer set using a sequencer path
//!
//! The sequencer sets are edited and saved in configuration mode.
//!
//! ## Conditions for the use of the sequencer mode
//! * The sequencer mode is supported in trigger mode only.
//! * The sequencer mode must be activated before the image acquisition starts.
//! * The internal image memory must be activated.
//! * The following functions cannot be used in combination with the sequencer mode:
//!     multi AOI function, sequence AOI mode, and IDS line scan (AOI merge mode).
//!
//! The sequencer mode is not supported by the _UI-359xCP Rev.2_ camera model.
//!
//! # Documentation
//! [is_Sequencer](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_sequencer.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::io::IO_FLASH_MODE;
use crate::types::{void, BOOL, FALSE, HIDS, INT, STRING, TRUE, UINT};
use bitflags::bitflags;

/// Enumeration of commands for [`is_Sequencer`].
///
/// # Documentation
/// [is_Sequencer](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_sequencer.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SEQUENCER_CMD {
    /// Enables or disables the sequencer mode.
    ///
    /// The sequencer mode can be activated only if at least one sequencer set has been saved.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Possible values
    /// * [`TRUE`] = Sequencer mode is active.
    /// * [`FALSE`] = Disables the sequencer mode.
    IS_SEQUENCER_MODE_ENABLED_SET = 1,

    /// Returns if the sequencer mode is active.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Possible values
    /// * [`TRUE`] = Sequencer mode is active.
    /// * [`FALSE`] = Sequencer mode is inactive.
    IS_SEQUENCER_MODE_ENABLED_GET = 2,

    /// Returns if the configuration mode is active.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Possible values
    /// * [`TRUE`] = Enables the configuration mode.
    /// * [`FALSE`] = Disables the configuration mode.
    IS_SEQUENCER_CONFIGURATION_ENABLED_SET = 3,

    /// Returns if the configuration mode is active.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Possible values
    /// * [`TRUE`] = Configuration mode is active.
    /// * [`FALSE`] = Configuration mode is inactive.
    IS_SEQUENCER_CONFIGURATION_ENABLED_GET = 4,

    /// Returns if the camera supports the sequencer mode.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Possible values
    /// * [`TRUE`] = Sequencer mode is supported.
    /// * [`FALSE`] = Sequencer mode is not supported.
    IS_SEQUENCER_MODE_SUPPORTED_GET = 5,

    /// Resets the sequencer mode.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_SEQUENCER_RESET = 6,

    /// Loads a sequencer configuration from a specified file.
    ///
    /// # Parameter type
    /// [`STRING`]
    IS_SEQUENCER_CONFIGURATION_LOAD = 7,

    /// Saves the current sequencer configuration in the specified file.
    ///
    /// # Parameter type
    /// [`STRING`]
    IS_SEQUENCER_CONFIGURATION_SAVE = 8,

    /// Saves the current parameters for the selected sequencer set.
    ///
    /// The configuration mode must be enabled.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_SEQUENCER_SET_SAVE = 10,

    /// Sets the first sequencer set that is used to start the sequencer
    /// (_value range: `0`…`31`_).
    ///
    /// The configuration mode must be enabled.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_SEQUENCER_SET_START_SET = 11,

    /// Returns the first sequencer set that is used to start the sequencer.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_SEQUENCER_SET_START_GET = 12,

    /// Sets the sequencer set that is used for further settings
    /// (_value range: `0`…`31`_).
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_SEQUENCER_SET_SELECTED_SET = 13,

    /// Returns the current sequencer set that is used for further settings
    /// (_value range: `0`…`31`_).
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_SEQUENCER_SET_SELECTED_GET = 14,

    /// Returns the configuration of the sequencer path for the selected sequencer set.
    ///
    /// # Parameter type
    /// [`IS_SEQUENCER_PATH`]
    IS_SEQUENCER_SET_PATH_SET = 15,

    /// Sets the configuration of the sequencer path for the selected sequencer set.
    ///
    /// The configuration mode must be enabled.
    ///
    /// # Parameter type
    /// [`IS_SEQUENCER_PATH`]
    IS_SEQUENCER_SET_PATH_GET = 16,

    /// Returns the maximum count of possible sets.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_SEQUENCER_SET_MAX_COUNT_GET = 17,

    /// Sets the parameter that is controlled by the sequencer.
    ///
    /// # Parameter type
    /// [`IS_SEQUENCER_FEATURE`]
    IS_SEQUENCER_FEATURE_SELECTED_SET = 20,

    /// Returns the selected parameter that is controlled by the sequencer.
    ///
    /// # Parameter type
    /// [`IS_SEQUENCER_FEATURE`]
    IS_SEQUENCER_FEATURE_SELECTED_GET = 21,

    /// Enables or disables the selected parameter and transfers the setting for all sequencer sets.
    ///
    /// The configuration mode must be enabled.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Possible values
    /// * [`TRUE`] = Parameter is active.
    /// * [`FALSE`] = Parameter is inactive.
    IS_SEQUENCER_FEATURE_ENABLED_SET = 22,

    /// Returns if the selected parameter is active.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Possible values
    /// * [`TRUE`] = Parameter is active.
    /// * [`FALSE`] = Parameter is inactive.
    IS_SEQUENCER_FEATURE_ENABLED_GET = 23,

    /// Returns a bit mask with the supported parameters.
    ///
    /// # Parameter type
    /// [`IS_SEQUENCER_FEATURE`], _bitmask_
    IS_SEQUENCER_FEATURE_SUPPORTED_GET = 24,

    /// Returns the saved value for the selected parameter.
    ///
    /// # Parameter type
    /// [`IS_SEQUENCER_FEATURE`]
    IS_SEQUENCER_FEATURE_VALUE_GET = 25,

    /// Returns the maximum count of possible paths per set.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_SEQUENCER_PATH_MAX_COUNT_GET = 30,

    /// Returns a list of the supported trigger sources.
    ///
    /// # Parameter type
    /// [`IS_SEQUENCER_TRIGGER_SOURCE`], _bitmask_
    IS_SEQUENCER_TRIGGER_SOURCE_SUPPORTED_GET = 31,
}

bitflags! {
    /// Sequencer trigger source (_supports bitmask_).
    ///
    /// Specifies the internal signal or physical input line to use as the sequencer trigger source.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct IS_SEQUENCER_TRIGGER_SOURCE: UINT {
        /// Disables the sequencer trigger source, which means that no further set is started after
        /// this sequencer set.
        const IS_TRIGGER_SOURCE_OFF          = 0;

        /// Triggers the next sequencer set with the end of image readout.
        const IS_TRIGGER_SOURCE_FRAME_END = 0x01;

        /// Triggers the next sequencer set with the beginning of image readout.
        const IS_TRIGGER_SOURCE_FRAME_START = 0x02;

        /// Triggers the next sequencer set with end of exposure.
        const IS_TRIGGER_SOURCE_EXPOSURE_END = 0x04;
    }
}

/// Sequencer path configuration.
///
/// [is_Sequencer: Contents of the IS_SEQUENCER_PATH structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_sequencer.html#is_sequencer_path)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IS_SEQUENCER_PATH {
    /// Path index to which the configuration will apply (_possible values:_ `0` or `1`).
    pub u32PathIndex: UINT,

    /// Specifies the next sequencer set.
    pub u32NextIndex: UINT,

    /// Specifies the internal signal or physical input line to use as the sequencer trigger source.
    pub u32TriggerSource: IS_SEQUENCER_TRIGGER_SOURCE,

    /// Specifies the activation mode of the sequencer trigger (_possible values:_ `0`).
    pub u32TriggerActivation: UINT,
}

/// Sequencer gain configuration for the gain feature.
///
/// # Documentation
/// [is_Sequencer: Contents of the IS_SEQUENCER_GAIN_CONFIGURATION structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_sequencer.html#is_sequencer_gain_configuration)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IS_SEQUENCER_GAIN_CONFIGURATION {
    /// Sets the master gain factor (`0`…`100`).
    pub Master: UINT,

    ///  Sets the red channel gain factor (`0`…`100`).
    pub Red: UINT,

    ///  Sets the green channel gain factor (`0`…`100`).
    pub Green: UINT,

    ///  Sets the blue channel gain factor (`0`…`100`).
    pub Blue: UINT,
}

/// Sequencer flash configuration for the flash feature.
///
/// # Documentation
/// [is_Sequencer: Contents of the IS_SEQUENCER_FLASH_CONFIGURATION structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_sequencer.html#is_sequencer_flash_configuration)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IS_SEQUENCER_FLASH_CONFIGURATION {
    /// Flash mode.
    pub u32Mode: IO_FLASH_MODE,

    /// Flash duration (in μs).
    ///
    /// If `0` is passed, the flash output will be active until the end of the exposure time.
    /// For sensors with Global Start Shutter, this is the time until the end of exposure of the
    /// first sensor row.
    pub u32Duration: UINT,

    /// Flash delay (in μs).
    pub u32Delay: UINT,
}

bitflags! {
    /// Sequencer features (_supports bitmask_).
    ///
    /// Specifies the sequencer features that can be part of a device sequencer set.
    /// All the device's sequencer sets have the same features.
    pub struct IS_SEQUENCER_FEATURE: UINT {
        /// Exposure time in milliseconds ([`double`])
        ///
        /// Note that the possible exposure time range depends on the pixel clock and frame rate.
        /// A low frame rate setting allows long exposure times. A high frame rate setting reduces
        /// the maximum possible exposure time.
        const IS_FEATURE_EXPOSURE     = 0x01;

        /// Gain in percent ([`IS_SEQUENCER_GAIN_CONFIGURATION`]).
        const IS_FEATURE_GAIN         = 0x02;

        /// Offset for the X position of the AOI ([`INT`]).
        const IS_FEATURE_AOI_OFFSET_X = 0x04;

        /// Offset for the Y position of the AOI ([`INT`]).
        const IS_FEATURE_AOI_OFFSET_Y = 0x08;

        /// Flash settings ([`IS_SEQUENCER_FLASH_CONFIGURATION`]).
        const IS_FEATURE_FLASH        = 0x10;
    }
}

unsafe extern "C" {
    /// Configure and activate the sequencer mode.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`SEQUENCER_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParams` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_CAPTURE_RUNNING`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_GetImageInfo`]
    ///
    /// # Documentation
    /// [is_Sequencer](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_sequencer.html)
    pub fn is_Sequencer(
        hCam: HIDS,
        nCommand: SEQUENCER_CMD,
        pParam: *mut void,
        cbSizeOfParams: UINT,
    ) -> INT;
}
