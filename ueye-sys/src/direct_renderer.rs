//! Provides a set of advanced rendering functions and allows inserting overlay data into the
//! camera's live image without flicker.
//!
//! The graphics card functions of the Direct3D library are supported under Windows.
//!
//! **Note on using the function under Linux:**
//! The [`is_DirectRenderer`] functions works under Linux only in OpenGL mode.
//!
//! **Note on system requirements:**
//! * To use the Direct3D functionality, the appropriate version of the Microsoft DirectX Runtime
//!     has to be installed in your PC.
//! * When you are using high-resolution cameras, the maximum texture size supported by the
//!     graphics card should be at least 4096 x 4096 pixels. You can check the maximum texture size
//!     by reading out the [`DR_GET_MAX_OVERLAY_SIZE`][DR_CMD::DR_GET_MAX_OVERLAY_SIZE] parameter.
//! * The Direct3D mode automatically uses the Windows Desktop color depth setting for the display.
//!
//! Please also read the notes on graphics cards which are provided in the
//! [System requirements](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/hw_systemanforderungen.html) chapter.
//!
//! **Note on displaying monochrome or raw data formats:**
//! To display monochrome or Bayer raw data in Direct3D, please set the appropriate constants using
//! the [`is_SetDisplayMode`] function.
//!
//! # Documentation
//! [is_DirectRenderer](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_directrenderer.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{void, HIDS, HWND, INT, UINT};

#[derive(Debug)]
#[repr(C)]
pub struct OPENGL_DISPLAY {
    pub nWindowID: INT,
    pub pDisplay: *mut void,
}

/// Enumeration of commands for [`is_DirectRenderer`].
///
/// # Documentation
/// [is_DirectRenderer](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_directrenderer.html)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum DR_CMD {
    /// _Direct3D only:_ Returns the device context (DC) handle to the overlay area of the
    /// graphics card.
    ///
    /// In Direct3D mode, the [`DR_GET_OVERLAY_DC`][DR_CMD::DR_GET_OVERLAY_DC] mode returns the
    /// device context (DC) handle of the overlay area. Using this handle, it is possible to access
    /// the overlay using the Windows GDI functionality. Thus, all Windows graphics commands
    /// (e.g. Line, Circle, Rectangle, TextOut) are available. To transfer the drawn elements to
    /// the overlay, release the DC handle by calling
    /// [`DR_RELEASE_OVERLAY_DC`][DR_CMD::DR_RELEASE_OVERLAY_DC].
    ///
    /// # Parameter type
    /// [`HDC`]
    DR_GET_OVERLAY_DC = 1,

    /// Returns the width X and height Y of the maximum overlay area supported by the graphics card.
    ///
    /// # Parameter type
    /// _Array of:_
    /// * Width: [`UINT`]
    /// * Height: [`UINT`]
    DR_GET_MAX_OVERLAY_SIZE = 2,

    /// Returns the RGB values of the current key color (_default: black_).
    ///
    /// # Parameter type
    /// _Array of:_
    /// * R: [`UINT`]
    /// * G: [`UINT`]
    /// * B: [`UINT`]
    DR_GET_OVERLAY_KEY_COLOR = 3,

    /// _Direct3D only:_ Releases the device context (DC) handle.
    ///
    /// Using [`DR_RELEASE_OVERLAY_DC`][DR_CMD::DR_RELEASE_OVERLAY_DC], you can release the DC
    /// handle and update the overlay data.
    ///
    /// # Parameter type
    /// [`NULL`]
    DR_RELEASE_OVERLAY_DC = 4,

    /// Enables overlay display on top of the current camera image.
    ///
    /// # Parameter type
    /// [`NULL`]
    DR_SHOW_OVERLAY = 5,

    /// Disables overlay display.
    ///
    /// # Parameter type
    /// [`NULL`]
    DR_HIDE_OVERLAY = 6,

    /// Defines the size of the overlay area (_default: current camera image size_).
    ///
    /// # Parameter type
    /// _Array of:_
    /// * Width: [`UINT`]
    /// * Height: [`UINT`]
    DR_SET_OVERLAY_SIZE = 7,

    /// Defines the position of the overlay area.
    ///
    /// # Parameter type
    /// _Array of:_
    /// * X: [`UINT`]
    /// * Y: [`UINT`]
    DR_SET_OVERLAY_POSITION = 8,

    /// Defines the RGB values of the key color.
    ///
    /// # Parameter type
    /// _Array of:_
    /// * R: [`UINT`]
    /// * G: [`UINT`]
    /// * B: [`UINT`]
    DR_SET_OVERLAY_KEY_COLOR = 9,

    /// Sets a new window handle for image output in Direct3D.
    ///
    /// # Parameter type
    /// [`HWND`]
    DR_SET_HWND = 10,

    /// Enables real-time scaling of the image to the size of the display window.
    ///
    /// The overlay is scaled together with the camera image.
    ///
    /// # Parameter type
    /// [`NULL`]
    DR_ENABLE_SCALING = 11,

    /// Disables real-time scaling.
    ///
    /// # Parameter type
    /// [`NULL`]
    DR_DISABLE_SCALING = 12,

    /// Deletes the data of the overlay area by filling it with black color.
    ///
    /// # Parameter type
    /// [`NULL`]
    DR_CLEAR_OVERLAY = 13,

    /// Enables a semi-transparent display of the overlay area.
    ///
    /// In semi-transparent mode, the values of the camera image and the overlay data are added up
    /// for each pixel. Since black has the value `0`, the complete camera image will be visible if
    /// the overlay is black; if the overlay is white, only the overlay will be visible.
    /// With all other colors, the camera image will be visible with the overlay superimposed.
    ///
    /// The key color has no effect in semi-transparent mode!
    ///
    /// # Parameter type
    /// [`NULL`]
    DR_ENABLE_SEMI_TRANSPARENT_OVERLAY = 14,

    /// Disables the semi-transparent display of the overlay area.
    ///
    /// # Parameter type
    /// [`NULL`]
    DR_DISABLE_SEMI_TRANSPARENT_OVERLAY = 15,

    /// Returns whether the graphics card supports the _uEye_ Direct3D functions.
    ///
    /// # Parameter type
    /// [`NULL`]
    DR_CHECK_COMPATIBILITY = 16,

    /// Disables image display synchronization. The image is displayed immediately.
    ///
    /// # Parameter type
    /// [`NULL`]
    DR_SET_VSYNC_OFF = 17,

    /// Enables synchronization of the image display with the monitor's image rendering.
    ///
    /// The image is displayed upon the monitor's next VSYNC signal.
    ///
    /// # Parameter type
    /// [`NULL`]
    DR_SET_VSYNC_AUTO = 18,

    /// _Direct3D only:_ Enables synchronization of the image display with a monitor pixel row
    /// specified by the user.
    ///
    /// When displaying very large camera images, the auto VSYNC function might not always
    /// optimally synchronize image rendering. In this case, you can eliminate flicker by manually
    /// setting a suitable position for synchronization. The position needs to be determined
    /// individually, based on the camera type and the graphics card.
    ///
    /// # Parameter type
    /// [`INT`]
    DR_SET_USER_SYNC = 19,

    /// _Direct3D only:_ Returns the minimum and maximum row position for
    /// [`DR_SET_USER_SYNC`][DR_CMD::DR_SET_USER_SYNC].
    ///
    /// # Parameter type
    /// _Array of:_
    /// * [`INT`]: range
    /// * [`INT`]: set position
    DR_GET_USER_SYNC_POSITION_RANGE = 20,

    /// _Direct3D only:_ Loads a bitmap image (*.BMP file) into the overlay area.
    ///
    /// If the bitmap image is larger than the overlay area, the bitmap image is clipped.
    /// The bitmap image must be saved in 24-bit without color space information.
    ///
    /// # Parameter type
    /// [`STR`]
    DR_LOAD_OVERLAY_FROM_FILE = 21,

    /// Copies the next image to the active user memory (Steal function).
    ///
    /// # Possible values
    /// Using the `pParam` parameter, you specify when the function should return:
    /// * [`IS_WAIT`]: The function waits until the image save is complete.
    /// * [`IS_DONT_WAIT`]: The function returns immediately.
    ///
    /// # Parameter type
    /// [`UINT`]
    DR_STEAL_NEXT_FRAME = 22,

    /// Defines the color format for the Steal function.
    ///
    /// For a list of all available color formats, see the function description for
    /// [`is_SetColorMode`]. The default is IS_CM_BGRA8_PACKED (RGB 32).
    ///
    /// # Parameter type
    /// TODO
    DR_SET_STEAL_FORMAT = 23,

    /// Returns the color format setting for the Steal function.
    ///
    /// # Parameter type
    /// TODO
    DR_GET_STEAL_FORMAT = 24,

    /// _Direct3D only:_ Enables real-time scaling of the image to the size of the display window.
    ///
    /// The overlay is not scaled (see: [`DR_ENABLE_SCALING`][DR_CMD::DR_ENABLE_SCALING]).
    DR_ENABLE_IMAGE_SCALING = 25,

    /// Returns the size of the overlay area.
    ///
    /// # Parameter type
    /// _Array of:_
    /// * Width: [`UINT`]
    /// * Height: [`UINT`]
    DR_GET_OVERLAY_SIZE = 26,
    DR_CHECK_COLOR_MODE_SUPPORT = 27,

    /// _OpenGL only:_ Returns a pointer to the overlay.
    DR_GET_OVERLAY_DATA = 28,

    /// _OpenGL only:_ Updates the overlay.
    DR_UPDATE_OVERLAY_DATA = 29,

    /// Returns either if Direct3D or OpenGL is supported by the graphics card.
    DR_GET_SUPPORTED = 30,
}

unsafe extern "C" {
    /// Provides a set of advanced rendering functions and allows inserting overlay data into the
    /// camera's live image without flicker.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nMode` - Command. See [`DR_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `nSize` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_DR_DEVICE_CAPS_INSUFFICIENT`]: When used with
    ///     [`DR_CHECK_COMPATIBILITY`][DR_CMD::DR_CHECK_COMPATIBILITY]:
    ///     The graphics hardware does not fully support the _uEye_ Direct3D functions.
    /// * [`IS_DR_CANNOT_CREATE_SURFACE`]
    /// * [`IS_DR_CANNOT_CREATE_TEXTURE`]
    /// * [`IS_DR_CANNOT_CREATE_VERTEX_BUFFER`]
    /// * [`IS_DR_CANNOT_GET_OVERLAY_DC`]
    /// * [`IS_DR_CANNOT_LOCK_OVERLAY_SURFACE`]
    /// * [`IS_DR_CANNOT_RELEASE_OVERLAY_DC`]
    /// * [`IS_DR_CANNOT_UNLOCK_OVERLAY_SURFACE`]
    /// * [`IS_DR_DEVICE_CAPS_INSUFFICIENT`]
    /// * [`IS_DR_DEVICE_OUT_OF_MEMORY`]
    /// * [`IS_DR_NOT_ALLOWED_WHILE_DC_IS_ACTIVE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    /// * [`IS_TIMED_OUT`]
    ///
    /// # Related functions
    /// * [`is_SetDisplayMode`]
    /// * [`is_SetColorMode`]
    /// * [`is_SetImageMem`]
    /// * [`is_RenderBitmap`]
    ///
    /// # Documentation
    /// [is_DirectRenderer](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_directrenderer.html)
    pub fn is_DirectRenderer(hCam: HIDS, nMode: DR_CMD, pParam: *mut void, nSize: UINT) -> INT;
}
