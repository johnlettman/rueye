#![allow(non_camel_case_types)]

use crate::return_values::*;
use crate::types::{void, HIDS, INT, UINT, IS_RANGE_U32};

/// Enumeration of commands of function [`is_PixelClock`].
///
/// # Documentation
/// [`is_PixelClock`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_pixelclock.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PIXELCLOCK_CMD {
    /// Returns the number of discrete pixel clocks which are supported by the camera.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_PIXELCLOCK_CMD_GET_NUMBER    = 1,

    /// Returns the list with discrete pixel clocks.
    ///
    /// # Parameter type
    /// _Array of:_ [`UINT`]
    IS_PIXELCLOCK_CMD_GET_LIST      = 2,

    /// Returns the range for the pixel clock.
    ///
    /// If [`PIXELCLOCK_CMD::IS_PIXELCLOCK_CMD_GET_RANGE`] returns the increment value `0`,
    /// the camera has only discrete pixel clocks. The discrete pixel clocks can be queried with
    /// [`PIXELCLOCK_CMD::IS_PIXELCLOCK_CMD_GET_NUMBER`] and
    /// [`PIXELCLOCK_CMD::IS_PIXELCLOCK_CMD_GET_LIST`].
    ///
    /// # Parameter type
    /// [`IS_RANGE_U32`]
    IS_PIXELCLOCK_CMD_GET_RANGE     = 3,

    /// Returns the default pixel clock.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_PIXELCLOCK_CMD_GET_DEFAULT   = 4,

    /// Returns the current set pixel clock in MHz.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_PIXELCLOCK_CMD_GET           = 5,

    /// Sets the pixel clock in MHz.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_PIXELCLOCK_CMD_SET           = 6
}

unsafe extern "C" {
    /// Returns the adjustable pixel clock range and sets the pixel clock.
    ///
    /// Due to an excessive pixel clock for USB cameras, images may get lost during the transfer.
    /// If you change the pixel clock on-the-fly, the current image capturing process will
    /// be aborted.
    ///
    /// The pixel clock limit values can vary, depending on the camera model and operating mode.
    /// For detailed information on the pixel clock range of a specific camera model, please refer
    /// to the [Camera and sensor data](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/hw_sensoren.html)
    /// chapter.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`PIXELCLOCK_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_MODE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_GetFramesPerSecond`]
    /// * [`is_GetFrameTimeRange`]
    /// * [`is_Exposure`]
    /// * [`is_SetOptimalCameraTiming`]
    /// * [`is_SetFrameRate`]
    /// * [`is_SetAutoParameter`]
    /// * [`is_SetBinning`]
    /// * [`is_SetSubSampling`]
    /// * [`is_AOI`]
    ///
    /// # Documentation
    /// [`is_PixelClock`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_pixelclock.html)
    pub fn is_PixelClock(hCam: HIDS, nCommand: PIXELCLOCK_CMD, pParam: *mut void, cbSizeOfParam: UINT) -> INT;
}
