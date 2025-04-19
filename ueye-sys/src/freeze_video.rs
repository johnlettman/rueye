//! Acquires a single image from the camera.
//!
//! In DIB mode, the image is stored in the active image memory. If ring buffering is used in DIB
//! mode, the captured image is transferred to the next available image memory of the sequence.
//! Once the last available sequence memory has been filled, the sequence event or message
//! will be triggered.
//!
//! **Note:**
//! The [`is_FreezeVideo`] function is not recommended for multiple or continuous capturing.
//! Use the [`is_CaptureVideo`] function instead.
//!
//! In Direct3D or OpenGL mode, the image is directly copied to the graphics card buffer and
//! then displayed.
//!
//! Image capture will be started by a trigger if you previously enabled the trigger mode using
//! [`is_SetExternalTrigger`]. A hardware triggered image acquisition can be cancelled using
//! [`is_StopLiveVideo`] if exposure has not started yet. If you call [`is_FreezeVideo`] with the
//! [`IS_WAIT`] parameter, you have to simulate a trigger signal using [`is_ForceTrigger`] to
//! cancel the acquisition.
//!
//! For further information on the image capture modes of the _uEye_ camera, see the
//! [How to proceed: Image capture](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk-programming-image-capture-modes.html) section.
//!
//! # Documentation
//! [is_FreezeVideo](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_freezevideo.html)

use crate::constants::live_freeze::*;
use crate::constants::return_values::*;
use crate::stop_live_video::is_StopLiveVideo;
use crate::types::{HIDS, INT};

unsafe extern "C" {
    /// Stops live mode or cancels a hardware triggered image capture in case the exposure has
    /// not yet started.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `Wait` - Timeout value for image capture (see also the
    ///     [How to proceed: Timeout values for image capture](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk-programming-image-capture-timeout.html)
    ///     section).
    ///     **Note:**  If a timeout value `t` is passed and the camera does not return after `t` +
    ///     10 ms, the function returns with [`IS_TIMED_OUT`] and image acquisition is aborted.
    ///     * [`IS_WAIT`]
    ///     * [`IS_DONT_WAIT`]
    ///     * Time `t` (_in milliseconds_)
    ///
    /// # Return values
    /// * [`IS_BAD_STRUCTURE_SIZE`]
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CAPTURE_RUNNING`]
    /// * [`IS_INVALID_BUFFER_SIZE`]
    /// * [`IS_INVALID_CAMERA_TYPE`]
    /// * [`IS_INVALID_EXPOSURE_TIME`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_ACTIVE_IMG_MEM`]
    /// * [`IS_NO_USB20`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_CALIBRATED`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_OUT_OF_MEMORY`]
    /// * [`IS_TIMED_OUT`]
    /// * [`IS_SUCCESS`]
    /// * [`IS_TRANSFER_ERROR`]
    ///
    /// # Related functions
    /// * [`is_HasVideoStarted`]
    /// * [`is_IsVideoFinish`]
    /// * [`is_SetExternalTrigger`]
    /// * [`is_ForceTrigger`]
    /// * [`is_CaptureVideo`]
    /// * [`is_SetTimeout`]
    /// * [`is_CaptureStatus`]
    ///
    /// # Documentation
    /// [is_FreezeVideo](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_freezevideo.html)
    pub fn is_FreezeVideo(hCam: HIDS, Wait: INT) -> INT;
}
