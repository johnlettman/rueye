//! Activates the camera's live video mode (free run mode).
//!
//! The driver transfers the images to an allocated image memory or, if Direct3D/OpenGL is used,
//! to the graphics card. The image data (DIB mode) is stored in the memory created using
//! [`is_AllocImageMem`] and designated as active image memory using [`is_SetImageMem`].
//! Using [`is_GetImageMem`], you can query the memory address.
//!
//! If ring buffering is used, the image capturing function cycles through all image memories used
//! for storing the images of a capture sequence in an endless loop. Sequence memories locked by
//! [`is_LockSeqBuf`] will be skipped. If the last available sequence memory has been filled,
//! the sequence event or message will be triggered. Capturing always starts with the first
//! element of the sequence.
//!
//! When you call [`is_CaptureVideo`] in trigger mode, the camera is set to continuous trigger
//! standby. An image is recorded on each electrical trigger signal and afterward the camera is
//! ready to trigger again.
//!
//! For further information on the image capture modes, see the
//! [How to proceed: Image capture](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk-programming-image-capture-modes.html) section.
//!
//! # Documentation
//! [is_CaptureVideo](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_capturevideo.html)

use crate::constants::live_freeze::*;
use crate::constants::return_values::*;
use crate::freeze_video::is_FreezeVideo;
use crate::stop_live_video::is_StopLiveVideo;
use crate::types::{HIDS, INT, TRUE};

unsafe extern "C" {
    /// Activates the camera's live video mode (free run mode).
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
    ///     * [`IS_GET_LIVE`] = Returns if live capture is enabled.
    ///
    /// # Return values
    /// * [`TRUE`], _when used with [`IS_GET_LIVE`]_
    /// * [`IS_BAD_STRUCTURE_SIZE`]
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CAPTURE_RUNNING`]
    /// * [`IS_INVALID_CAMERA_TYPE`]
    /// * [`IS_INVALID_EXPOSURE_TIME`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_ACTIVE_IMG_MEM`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_CALIBRATED`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_OUT_OF_MEMORY`]
    /// * [`IS_SUCCESS`]
    /// * [`IS_TIMED_OUT`]
    /// * [`IS_TRIGGER_ACTIVATED`]
    ///
    /// # Related functions
    /// * [`is_FreezeVideo`]
    /// * [`is_StopLiveVideo`]
    /// * [`is_SetExternalTrigger`]
    /// * [`is_ForceTrigger`]
    /// * [`is_SetTimeout`]
    /// * [`is_CaptureStatus`]
    ///
    /// # Documentation
    /// [is_CaptureVideo](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_capturevideo.html)
    pub fn is_CaptureVideo(hCam: HIDS, Wait: INT) -> INT;
}
