//! Suppress disturbances at the trigger input when you are running a _uEye_ camera in trigger mode.
//!
//! This function cannot be used if GPIO is used for triggering.
//! [`is_TriggerDebounce`] delays the start of a triggered image capture by the selected time
//! (`DELAY_TIME`).
//!
//! The signal at the trigger input is only recognized as a trigger if the signal level remains
//! constant at the target level for a user-selectable time. The signal edge and a delay
//! (`DELAY_TIME`) can be set as parameters. It is recommended to use automatic signal edge
//! selection.
//!
//! **Example:** mode set to "rising edge"
//! ([`TRIGGER_DEBOUNCE_MODE_RISING_EDGE`][TRIGGER_DEBOUNCE_MODE::TRIGGER_DEBOUNCE_MODE_RISING_EDGE])
//! and delay set to 50 µs. The camera will not trigger the image capture on the rising edge until
//! the digital signal has remained at the high level for longer than 50 µs without interruption.
//! If this is not the case, the signal is regarded as a disturbance and ignored.
//!
//! This function is currently only supported by the USB 3 and GigE _uEye_ camera series but not by
//! the _UI-359xLE_ camera model or the _uEye LE USB 3.1 Gen 1_ camera family.
//!
//! # Documentation
//! [is_TriggerDebounce](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_triggerdebounce.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{void, HIDS, INT, UINT};
use bitflags::bitflags;

bitflags! {
    /// Trigger debounce modes.
    ///
    /// # Related commands
    /// * [`TRIGGER_DEBOUNCE_CMD_SET_MODE`][TRIGGER_DEBOUNCE_CMD::TRIGGER_DEBOUNCE_CMD_SET_MODE]
    /// * [`TRIGGER_DEBOUNCE_CMD_GET_MODE`][TRIGGER_DEBOUNCE_CMD::TRIGGER_DEBOUNCE_CMD_GET_MODE]
    /// * [`TRIGGER_DEBOUNCE_CMD_GET_MODE_DEFAULT`][TRIGGER_DEBOUNCE_CMD::TRIGGER_DEBOUNCE_CMD_GET_MODE_DEFAULT]
    /// * [`TRIGGER_DEBOUNCE_CMD_GET_SUPPORTED_MODES`][TRIGGER_DEBOUNCE_CMD::TRIGGER_DEBOUNCE_CMD_GET_SUPPORTED_MODES]
    ///
    /// # Documentation
    /// [is_TriggerDebounce: Function modes](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_triggerdebounce.html#funktionsmodi)
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct TRIGGER_DEBOUNCE_MODE: UINT {
        /// Disables debouncing the trigger input.
        const TRIGGER_DEBOUNCE_MODE_NONE = 0x0000;

        /// Debounces the trigger input for falling edge signals.
        const TRIGGER_DEBOUNCE_MODE_FALLING_EDGE = 0x0001;

        /// Debounces the trigger input for rising edge signals.
        const TRIGGER_DEBOUNCE_MODE_RISING_EDGE = 0x0002;

        /// Debounces the trigger input for rising or falling edge signals.
        const TRIGGER_DEBOUNCE_MODE_BOTH_EDGES = 0x0004;

        /// Debounces the trigger input with automatic edge selection (_recommended_).
        ///
        /// The edge is selected based on the set trigger edge (see [`is_SetExternalTrigger`]).
        const TRIGGER_DEBOUNCE_MODE_AUTOMATIC = 0x0008;
    }
}

/// Enumeration of commands for [`is_TriggerDebounce`].
///
/// # Documentation
/// [is_TriggerDebounce](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_triggerdebounce.html)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum TRIGGER_DEBOUNCE_CMD {
    /// Sets a [function mode][TRIGGER_DEBOUNCE_MODE].
    ///
    /// # Parameter type
    /// [`TRIGGER_DEBOUNCE_MODE`]
    TRIGGER_DEBOUNCE_CMD_SET_MODE = 0,

    /// Sets a delay time (in µs).
    ///
    /// # Parameter type
    /// [`UINT`]
    TRIGGER_DEBOUNCE_CMD_SET_DELAY_TIME = 1,

    /// Returns the [function modes][TRIGGER_DEBOUNCE_MODE] supported by the camera.
    ///
    /// # Parameter type
    /// [`TRIGGER_DEBOUNCE_MODE`], _bitmask_
    TRIGGER_DEBOUNCE_CMD_GET_SUPPORTED_MODES = 2,

    /// Returns the set [function mode][TRIGGER_DEBOUNCE_MODE].
    ///
    /// # Parameter type
    /// [`TRIGGER_DEBOUNCE_MODE`]
    TRIGGER_DEBOUNCE_CMD_GET_MODE = 3,

    /// Returns the set delay time (in µs).
    ///
    /// # Parameter type
    /// [`UINT`]
    TRIGGER_DEBOUNCE_CMD_GET_DELAY_TIME = 4,

    /// Returns the minimum value for the delay (in µs).
    ///
    /// # Parameter type
    /// [`UINT`]
    TRIGGER_DEBOUNCE_CMD_GET_DELAY_TIME_MIN = 5,

    /// Returns the maximum value for the delay (in µs).
    ///
    /// # Parameter type
    /// [`UINT`]
    TRIGGER_DEBOUNCE_CMD_GET_DELAY_TIME_MAX = 6,

    /// Returns the increment for setting the delay.
    ///
    /// # Parameter type
    /// [`UINT`]
    TRIGGER_DEBOUNCE_CMD_GET_DELAY_TIME_INC = 7,

    /// Returns the default mode.
    ///
    /// # Parameter type
    /// [`TRIGGER_DEBOUNCE_MODE`]
    TRIGGER_DEBOUNCE_CMD_GET_MODE_DEFAULT = 8,

    /// Returns the default value for the delay.
    ///
    /// # Parameter type
    /// [`UINT`]
    TRIGGER_DEBOUNCE_CMD_GET_DELAY_TIME_DEFAULT = 9,
}

unsafe extern "C" {
    /// Suppress disturbances at the trigger input when you are running a _uEye_ camera in
    /// trigger mode.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`TRIGGER_DEBOUNCE_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `nSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_SetExternalTrigger`]
    /// * [`is_SetTriggerDelay`]
    ///
    /// # Documentation
    /// [is_TriggerDebounce](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_triggerdebounce.html)
    pub fn is_TriggerDebounce(
        hCam: HIDS,
        nCommand: TRIGGER_DEBOUNCE_CMD,
        pParam: *mut void,
        nSizeOfParam: UINT,
    ) -> INT;
}
