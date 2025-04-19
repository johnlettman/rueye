//! Check whether the image digitizing process has started.
//!
//! This function is helpful when the [`is_FreezeVideo`] function was called with the
//! [`IS_DONT_WAIT`] parameter.
//!
//! # Documentation
//! [is_HasVideoStarted](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_hasvideostarted.html)

use crate::constants::live_freeze::*;
use crate::constants::return_values::*;
use crate::freeze_video::is_FreezeVideo;
use crate::is_video_finish::is_IsVideoFinish;
use crate::types::{BOOL, HIDS, INT};

unsafe extern "C" {
    /// Check whether the image digitizing process has started.
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
