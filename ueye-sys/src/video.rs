//! Common video functions.

use crate::constants::return_values::*;
use crate::constants::video_finish::{IS_VIDEO_FINISH, IS_VIDEO_NOT_FINISH};
use crate::types::{BOOL, HIDS, INT, TRUE, UINT};

pub const IS_GET_LIVE: UINT = 0x8000;

/// The function waits.
pub const IS_WAIT: UINT = 0x0001;

/// The function returns immediately.
pub const IS_DONT_WAIT: UINT = 0x0000;

/// Digitizing is stopped immediately.
pub const IS_FORCE_VIDEO_STOP: UINT = 0x4000;
pub const IS_FORCE_VIDEO_START: UINT = 0x4000;
pub const IS_USE_NEXT_MEM: UINT = 0x8000;

unsafe extern "C" {
    /// Stops live mode or cancels a hardware triggered image capture in case the exposure has
    /// not yet started.
    ///
    /// If you call [`is_FreezeVideo`] with the [`IS_WAIT`] parameter, you have to simulate a
    /// trigger signal using [`is_ForceTrigger`] to cancel the acquisition.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `Wait` - Waiting period.
    ///     * [`IS_FORCE_VIDEO_STOP`] = Digitizing is stopped immediately.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    /// * [`IS_TIMED_OUT`]
    ///
    /// # Documentation
    /// [is_StopLiveVideo](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_stoplivevideo.html)
    pub fn is_StopLiveVideo(hCam: HIDS, Wait: INT) -> INT;

    /// Stops live mode or cancels a hardware triggered image capture in case the exposure has
    /// not yet started.
    ///
    /// In DIB mode, the image is stored in the active image memory. If ring buffering is used in DIB
    /// mode, the captured image is transferred to the next available image memory of the sequence.
    /// Once the last available sequence memory has been filled, the sequence event or message
    /// will be triggered.
    ///
    /// **Note:**
    /// The [`is_FreezeVideo`] function is not recommended for multiple or continuous capturing.
    /// Use the [`is_CaptureVideo`] function instead.
    ///
    /// In Direct3D or OpenGL mode, the image is directly copied to the graphics card buffer and
    /// then displayed.
    ///
    /// Image capture will be started by a trigger if you previously enabled the trigger mode using
    /// [`is_SetExternalTrigger`]. A hardware triggered image acquisition can be cancelled using
    /// [`is_StopLiveVideo`] if exposure has not started yet. If you call [`is_FreezeVideo`] with the
    /// [`IS_WAIT`] parameter, you have to simulate a trigger signal using [`is_ForceTrigger`] to
    /// cancel the acquisition.
    ///
    /// For further information on the image capture modes of the _uEye_ camera, see the
    /// [How to proceed: Image capture](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk-programming-image-capture-modes.html) section.
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

    /// Activates the camera's live video mode (free run mode).
    ///
    /// The driver transfers the images to an allocated image memory or, if Direct3D/OpenGL is used,
    /// to the graphics card. The image data (DIB mode) is stored in the memory created using
    /// [`is_AllocImageMem`] and designated as active image memory using [`is_SetImageMem`].
    /// Using [`is_GetImageMem`], you can query the memory address.
    ///
    /// If ring buffering is used, the image capturing function cycles through all image memories
    /// used for storing the images of a capture sequence in an endless loop. Sequence memories
    /// locked by [`is_LockSeqBuf`] will be skipped. If the last available sequence memory has been
    /// filled, the sequence event or message will be triggered. Capturing always starts with the
    /// first element of the sequence.
    ///
    /// When you call [`is_CaptureVideo`] in trigger mode, the camera is set to continuous trigger
    /// standby. An image is recorded on each electrical trigger signal and afterward the camera is
    /// ready to trigger again.
    ///
    /// For further information on the image capture modes, see the
    /// [How to proceed: Image capture](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk-programming-image-capture-modes.html) section.
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

    /// Check whether an image has been captured and stored completely in the image memory.
    ///
    /// This function is used in combination with [`is_FreezeVideo`] and the parameter
    /// [`IS_DONT_WAIT`].
    ///
    /// By setting the `*pbo` == [`IS_CAPTURE_STATUS`] parameter before calling
    /// [`is_IsVideoFinish`], you can also check whether a transfer or post-processing error
    /// occurred.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pbo` -
    ///     * By setting `*pbo` != [`IS_CAPTURE_STATUS`] before calling the function,
    ///         `pbo` contains the following digitizing status:
    ///         * [`IS_VIDEO_NOT_FINISH`]
    ///         * [`IS_VIDEO_FINISH`]
    ///     * By setting `*pbo` == [`IS_CAPTURE_STATUS`] before calling the function,
    ///         `pbo` contains the following digitizing status:
    ///         * [`IS_VIDEO_NOT_FINISH`]
    ///         * [`IS_VIDEO_FINISH`]
    ///         * [`IS_CAPTURE_STATUS`] = Transfer error or conversion problem
    ///             (e.g. destination memory is invalid)
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_FreezeVideo`]
    /// * [`is_HasVideoStarted`]
    ///
    /// # Documentation
    /// [is_IsVideoFinish](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_isvideofinish.html)
    pub fn is_IsVideoFinish(hCam: HIDS, pbo: *mut INT) -> INT;

    /// Check whether the image digitizing process has started.
    ///
    /// This function is helpful when the [`is_FreezeVideo`] function was called with the
    /// [`IS_DONT_WAIT`] parameter.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pbo` - Returns the digitizing status:
    ///     * [`FALSE`] = Image capturing has not started yet.
    ///     * [`TRUE`] = Image capturing has started.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_FreezeVideo`]
    /// * [`is_IsVideoFinish`]
    ///
    /// # Documentation
    /// [is_HasVideoStarted](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_hasvideostarted.html)
    pub fn is_HasVideoStarted(hCam: HIDS, pbo: *mut BOOL) -> INT;
}
