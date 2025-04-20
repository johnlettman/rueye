//! **Obsolete:** Configure optimal camera timing.

#![allow(non_camel_case_types)]

use crate::types::{double, void, HCAM, INT, UINT};

/// Enumeration of commands for [`is_OptimalCameraTiming`].
pub enum IS_OPTIMAL_CAMERA_TIMING_CMD {
    /// Get optimal camera timing (pixel clock).
    IS_OPTIMAL_CAMERA_TIMING_CMD_GET_PIXELCLOCK = 0x00000001,

    /// Get optimal camera timing (framerate).
    IS_OPTIMAL_CAMERA_TIMING_CMD_GET_FRAMERATE = 0x00000002,
}

/// I/O structure of the optimal camera parameters.
pub struct IS_OPTIMAL_CAMERA_TIMING {
    s32Mode: INT,

    /// The period (in milliseconds) during which no transfer error may occur.
    ///
    /// The adjustable range is between `4` and `20` seconds.
    /// The higher the value you set for this parameter, the more stable the determined pixel clock
    /// value will be. This, in turn, increases the runtime of the function correspondingly.
    s32TimeoutFineTuning: INT,

    /// Pixel clock frequency (in MHz).
    ps32PixelClock: *mut INT,

    /// Frame rate (in FPS).
    pdFramerate: *mut double,
}

unsafe extern "C" {
    /// Generic interface to the optimal camera timing functionality.
    pub fn is_OptimalCameraTiming(
        hCam: HCAM,
        u32Command: IS_OPTIMAL_CAMERA_TIMING_CMD,
        pParam: *mut void,
        u32SizeOfParam: UINT,
    ) -> INT;
}
