//! Stops live mode or cancels a hardware triggered image capture in case the exposure has
//! not yet started.
//!
//! If you call [`is_FreezeVideo`] with the [`IS_WAIT`] parameter, you have to simulate a trigger
//! signal using [`is_ForceTrigger`] to cancel the acquisition.
//!
//! # Documentation
//! [is_StopLiveVideo](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_stoplivevideo.html)

use crate::constants::live_freeze::*;
use crate::constants::return_values::*;
use crate::types::{HIDS, INT};

unsafe extern "C" {
    /// Stops live mode or cancels a hardware triggered image capture in case the exposure has
    /// not yet started.
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
    pub fn is_StopLiveVideo(hCam: HIDS, Wait: INT) -> INT;
}
