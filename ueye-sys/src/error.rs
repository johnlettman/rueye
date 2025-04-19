//! Common error functions.

use crate::types::{HIDS, INT};

/// Current status of error reporting.
pub const IS_GET_ERR_REP_MODE: INT = 0x8000;

/// Enables error reporting.
pub const IS_ENABLE_ERR_REP: INT = 1;

/// Disables error reporting.
pub const IS_DISABLE_ERR_REP: INT = 0;

unsafe extern "C" {
    /// Queries the last error that occurred and returns the associated error code and message.
    ///
    /// It is recommended to call this function after the previous function has returned the error
    /// [`IS_NO_SUCCESS`]. Each error message will be overwritten when a new error occurs.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pErr` - Pointer to the variable containing the error code.
    /// * `ppcErr` - Pointer to the string containing the error text.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_CaptureStatus`]
    /// * [`is_SetErrorReport`]
    /// * [`is_CameraStatus`]
    ///
    /// # Documentation
    /// [is_GetError](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_geterror.html)
    pub fn is_GetError(hCam: HIDS, pErr: *mut INT, ppcErr: *mut *const char) -> INT;

    /// Enable/disable error event logging.
    ///
    /// If error reporting is enabled, errors will automatically be displayed in a dialog box.
    /// Cancelling the dialog box disables the error report. Even with disabled error reporting,
    /// you can still query errors using the [`is_GetError`] function.
    ///
    /// [`is_SetErrorReport`] can be called before calling [`is_InitCamera`].
    /// You only need to enable the [`is_SetErrorReport`] function once for all cameras in the
    /// application.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `Mode` - Mode.
    ///
    /// # Return values
    /// * When used with, [`IS_GET_ERR_REP_MODE`]: Current setting
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_GetError`]
    /// * [`is_CaptureStatus`]
    /// * [`is_CameraStatus`]
    ///
    /// # Documentation
    /// [is_SetErrorReport](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_seterrorreport.html)
    pub fn is_SetErrorReport(hCam: HIDS, Mode: INT) -> INT;
}
