//! Activates the burst trigger mode in GigE and USB 3 _uEye_ cameras.
//!
//! In burst trigger mode, the camera captures a series of images in rapid succession on receipt
//! of a single trigger signal. The trigger signal can be transmitted via external hardware trigger
//! via the digital input of the camera. [`is_CaptureVideo`] is used for image acquisition because
//! multiple images have to be transferred.
//!
//! The burst images are captured and transferred at maximum speed. The maximum speed depends on
//! the parameters pixel clock (see [`is_PixelClock`]) and exposure time (see [`is_Exposure`]).
//! For GigE _uEye_ cameras, [`is_Transfer`] allows adjusting the latency of image data transfer.
//!
//! The burst trigger mode can only be used in hardware trigger. For image acquisition,
//! [`is_CaptureVideo`] is used. A trigger is activated either via the camera's trigger pin or via
//! a call to [`is_ForceTrigger`].
//!
//! The _UI-359xLE_ model and the _uEye LE USB 3.1 Gen 1_ camera family currently do not support
//! the burst trigger mode.
//!
//! ## Note on trigger delay in burst trigger mode
//! If you set a trigger delay with the [`is_SetTriggerDelay`] function, the delay will only apply
//! to the first image after each trigger signal.
//!
//! ## Setting a trigger prescaler
//! I n addition, you can set a trigger prescaler with this function. Via the trigger prescaler
//! (frequency divider) you can set for some uEye models that the trigger signal is divided by the
//! set value. This is necessary when the trigger signal delivers more pulses as needed for the
//! captures. With the set value, e.g. `40`, you define that the camera captures an image only
//! every 40th trigger signal
//! (see uEye Cockpit: [Properties - Trigger](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/hw_trigger.html)).
//!
//! The trigger prescaler is currently supported by the following models in hardware trigger mode:
//! * _USB 3 uEye CP Rev. 2_
//! * _USB 3 uEye CP_
//! * _USB 3 uEye LE_
//! * _USB 3 uEye ML_
//! * _GigE uEye CP Rev. 2_
//! * _GigE uEye CP_
//! * _GigE uEye FA_
//! * _GigE uEye SE Rev. 4_
//!
//! # Documentation
//! [is_Trigger](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_trigger.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::exposure::is_Exposure;
use crate::pixel_clock::is_PixelClock;
use crate::transfer::is_Transfer;
use crate::trigger_debounce::is_TriggerDebounce;
use crate::types::{void, BOOL, HCAM, INT, RANGE_OF_VALUES_U32, UINT};

/// Enumeration of commands for [`is_Trigger`].
///
/// # Documentation
/// [is_Trigger](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_trigger.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TRIGGER_CMD {
    /// Returns if the camera supports the burst trigger mode.
    ///
    /// # Parameter type
    /// [`BOOL`]
    IS_TRIGGER_CMD_GET_BURST_SIZE_SUPPORTED = 1,

    /// Returns the value range, the default value and the increment for the number of images
    /// in a burst.
    ///
    /// # Parameter type
    /// [`RANGE_OF_VALUES_U32`]
    IS_TRIGGER_CMD_GET_BURST_SIZE_RANGE = 2,

    /// Returns the currently set number of images in a burst.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_TRIGGER_CMD_GET_BURST_SIZE = 3,

    /// Sets the number of images in a burst.
    ///
    /// The maximum number is 1,023 images.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_TRIGGER_CMD_SET_BURST_SIZE = 4,

    /// Returns if a trigger prescaler for image recordings is supported.
    ///
    /// # Parameter type
    /// [`BOOL`]
    IS_TRIGGER_CMD_GET_FRAME_PRESCALER_SUPPORTED = 5,

    /// Returns the value range, the increment and the default value for the trigger prescaler for
    /// image recordings.
    ///
    /// # Parameter type
    /// [`RANGE_OF_VALUES_U32`]
    IS_TRIGGER_CMD_GET_FRAME_PRESCALER_RANGE = 6,

    /// Returns the current set trigger prescaler for image recordings.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_TRIGGER_CMD_GET_FRAME_PRESCALER = 7,

    /// Sets the trigger prescaler for image recordings.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_TRIGGER_CMD_SET_FRAME_PRESCALER = 8,

    /// Returns if a trigger prescaler for line recordings is supported.
    ///
    /// # Parameter type
    /// [`BOOL`]
    IS_TRIGGER_CMD_GET_LINE_PRESCALER_SUPPORTED = 9,

    /// Returns the value range, the increment and the default value for the trigger prescaler for
    /// line recordings.
    ///
    /// # Parameter type
    /// [`RANGE_OF_VALUES_U32`]
    IS_TRIGGER_CMD_GET_LINE_PRESCALER_RANGE = 10,

    /// Returns the current set trigger prescaler for line recordings.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_TRIGGER_CMD_GET_LINE_PRESCALER = 11,

    /// Sets the trigger prescaler for line recordings.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_TRIGGER_CMD_SET_LINE_PRESCALER = 12,
}

unsafe extern "C" {
    /// Activates the burst trigger mode in GigE and USB 3 _uEye_ cameras.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`TRIGGER_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_SetExternalTrigger`]
    /// * [`is_ForceTrigger`]
    /// * [`is_SetTriggerDelay`]
    /// * [`is_TriggerDebounce`]
    ///
    /// # Documentation
    /// [is_Trigger](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_trigger.html)
    pub fn is_Trigger(
        hCam: HCAM,
        nCommand: TRIGGER_CMD,
        pParam: *mut void,
        cbSizeOfParam: UINT,
    ) -> INT;
}
