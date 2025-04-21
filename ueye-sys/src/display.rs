//! Common display functions.

#![allow(non_camel_case_types)]

use crate::color::is_SetColorMode;
use crate::constants::return_values::*;
use crate::types::{long, void, BOOL, FALSE, HDC, HIDS, HWND, INT, TRUE, ULONG};
use bitflags::bitflags;

/// Returns the R channel.
const IS_GET_KC_RED: INT = 0x8000;

/// Returns the G channel.
const IS_GET_KC_GREEN: INT = 0x8001;

/// Returns the B channel.
const IS_GET_KC_BLUE: INT = 0x8002;

/// Returns the RGB.
const IS_GET_KC_RGB: INT = 0x8003;
const IS_GET_KC_INDEX: INT = 0x8004;

/// Returns the X offset.
const IS_GET_KEYOFFSET_X: INT = 0x8000;

/// Returns the Y offset.
const IS_GET_KEYOFFSET_Y: INT = 0x8001;

/// Initializes the manual steal mode.
const IS_INIT_STEAL_VIDEO: INT = 1;

/// Deinitializes the steal mode.
const IS_EXIT_STEAL_VIDEO: INT = 2;

/// Initializes the manual steal mode.
const IS_INIT_STEAL_VIDEO_MANUAL: INT = 3;

/// Initializes the automatic steal mode.
const IS_INIT_STEAL_VIDEO_AUTO: INT = 4;

/// Sets the proportion from number of images to VGA card and/or main memory
/// (OR-operation with first three constants).
const IS_SET_STEAL_RATIO: INT = 64;

/// Acquisition of the Steal-Image in the same size of the corresponding image memory which is
/// allocated for the image.
const IS_USE_MEM_IMAGE_SIZE: INT = 128;
const IS_STEAL_MODES_MASK: INT = 7;
const IS_SET_STEAL_COPY: INT = 0x1000;
const IS_SET_STEAL_NORMAL: INT = 0x2000;

bitflags! {
    /// Render modes for [`is_RenderBitmap`].
    ///
    /// # Documentation
    /// [is_RenderBitmap](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_renderbitmap.html)
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct IS_RENDER_MODE: INT {
        const IS_RENDER_DISABLED = 0x0000;

        /// The image is rendered normally.
        /// It will be displayed in 1:1 scale as stored in the image memory.
        const IS_RENDER_NORMAL = 0x0001;

        /// The image size is adjusted to fit the output window.
        const IS_RENDER_FIT_TO_WINDOW = 0x0002;

        /// Displays the image at 50% of its original size.
        const IS_RENDER_DOWNSCALE_1_2 = 0x0004;

        /// Mirrors the displayed image along the horizontal axis.
        const IS_RENDER_MIRROR_UPDOWN = 0x0010;

        /// Renders the red color component of the planar format in red.
        const IS_RENDER_PLANAR_COLOR_RED = 0x0080;

        /// Renders the green color component of the planar format in green.
        const IS_RENDER_PLANAR_COLOR_GREEN = 0x0100;

        /// Renders the blue color component of the planar format in blue.
        const IS_RENDER_PLANAR_COLOR_BLUE = 0x0200;

        /// Renders the red color component of the planar format in gray shades.
        const IS_RENDER_PLANAR_MONO_RED = 0x0400;

        /// Renders the green color component of the planar format in gray shades.
        const IS_RENDER_PLANAR_MONO_GREEN = 0x0800;

        /// Renders the blue color component of the planar format in gray shades.
        const IS_RENDER_PLANAR_MONO_BLUE = 0x1000;

        /// Rotates the image clockwise 90 degrees.
        const IS_RENDER_ROTATE_90 = 0x0020;

        /// Rotates the image clockwise 180 degrees
        const IS_RENDER_ROTATE_180 = 0x0040;

        /// Rotates the image clockwise 270 degrees
        const IS_RENDER_ROTATE_270 = 0x2000;
    }
}

bitflags! {
    /// Direct rendering support flags (_supports bitmask_)
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct IS_SET_DM: INT {
        /// Image in system memory (RAM).
        const IS_SET_DM_DIB = 1;

        /// DirectDraw mode.
        const IS_SET_DM_DIRECTDRAW = 2;

        /// Direct3D mode.
        const IS_SET_DM_DIRECT3D = 4;

        /// OpenGL mode.
        const IS_SET_DM_OPENGL = 8;

        /// Monochrome format.
        const IS_SET_DM_MONO = 0x800;

        /// Bayer format.
        const IS_SET_DM_BAYER = 0x1000;

        /// YCbCr mode.
        const IS_SET_DM_YCBCR = 0x4000;

        /// Return the display mode.
        const IS_GET_DISPLAY_MODE = 0x8000;

        const IS_SET_DM_ALLOW_SYSMEM = 0x40;
        const IS_SET_DM_ALLOW_PRIMARY = 0x80;
        const IS_GET_DD_OVERLAY_SCALE = 0x8000;
        const IS_SET_DM_ALLOW_OVERLAY = 0x100;
        const IS_SET_DM_ALLOW_SCALING = 0x200;
        const IS_SET_DM_ALLOW_FIELDSKIP = 0x400;
        const IS_SET_DM_BACKBUFFER = 0x2000;
    }
}

/// Enumeration of color bar modes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum IS_CBARS_MODE {
    /// Returns the current setting.
    IS_GET_CBARS_MODE = 0x8000,

    /// Disables the color bar generator.
    IS_SET_CBARS_OFF = 0,

    /// Enables the color bar generator.
    IS_SET_CBARS_ON = 1,
}

unsafe extern "C" {
    /// Output an image from an image memory in the specified window.
    ///
    /// For the display, Windows bitmap functionality is used. The image is displayed in the format
    /// you specified when allocating the image memory.
    ///
    /// The `bitspixel` parameter of the [`is_AllocImageMem`] function defines the color depth and
    /// display type. RGB16 and RGB15 require the same amount of memory but can be distinguished by
    /// the `bitspixel` parameter.
    ///
    /// [`is_RenderBitmap`] can render Y8 and RGB formats. For displaying YUV/YCbCr formats please
    /// use the [`is_DirectRenderer`] function
    /// (see also [Color and memory formats](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk_allgemeines_farbformate.html)).
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nMemId` - ID of the image memory whose contents is to be displayed.
    /// * `hwnd` - Output window handle.
    /// * `nMode` - Mode of the display. See [`IS_RENDER_MODE`].
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related function
    /// * [`is_AllocImageMem`]
    /// * [`is_SetColorMode`]
    /// * [`is_SetDisplayMode`]
    /// * [`is_DirectRenderer`]
    ///
    /// # Documentation
    /// [is_RenderBitmap](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_renderbitmap.html)
    #[cfg(target_os = "windows")]
    pub fn is_RenderBitmap(hCam: HIDS, nMemId: INT, hwnd: HWND, nMode: INT) -> INT;

    /// Set the way in which images will be displayed on the screen.
    ///
    /// For live videos including overlays, you can use the Direct3D or OpenGL mode. These modes
    /// are not supported by all graphics cards. The graphics card must have sufficient extended
    /// memory because the overlay mode requires additional memory up to the size needed for the
    /// current screen resolution.
    ///
    /// For further information on the display modes of the _uEye_ camera, see the
    /// [How to proceed: Image display](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk_funktionsbloecke_darstellung.html) section.
    ///
    /// The Direct3D display mode is not available on Linux operating systems.
    ///
    /// **Thread safety:**
    /// We recommend that you call the following function exclusively from a single thread in order
    /// to avoid unpredictable behavior of the application: [`is_SetDisplayMode`].
    ///
    /// See also
    /// [General: Thread programming](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk_allgemeines_threads.html).
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `Mode` - Display mode:
    ///     * [`IS_SET_DM_DIB`][IS_SET_DM::IS_SET_DM_DIB]:
    ///         Captures an image in system memory (RAM).
    ///         Using [`is_RenderBitmap`], you can define the image display (default).
    ///     * [`IS_SET_DM_DIRECT3D`][IS_SET_DM::IS_SET_DM_DIRECT3D]:
    ///         Image display in Direct3D mode.
    ///     * [`IS_SET_DM_DIRECT3D`][IS_SET_DM::IS_SET_DM_DIRECT3D] `|`
    ///         [`IS_SET_DM_MONO`][IS_SET_DM::IS_SET_DM_MONO]:
    ///         Monochrome image display in Direct3D mode.
    ///     * [`IS_SET_DM_DIRECT3D`][IS_SET_DM::IS_SET_DM_DIRECT3D] `|`
    ///         [`IS_SET_DM_BAYER`][IS_SET_DM::IS_SET_DM_BAYER]:
    ///         Raw Bayer format image display in Direct3D mode.
    ///     * [`IS_SET_DM_OPENGL`][IS_SET_DM::IS_SET_DM_OPENGL]:
    ///         Image display in OpenGL mode.
    ///     * [`IS_SET_DM_OPENGL`][IS_SET_DM::IS_SET_DM_OPENGL] `|`
    ///         [`IS_SET_DM_MONO`][IS_SET_DM::IS_SET_DM_MONO]:
    ///         Monochrome image display in OpenGL mode.
    ///     * [`IS_SET_DM_OPENGL`][IS_SET_DM::IS_SET_DM_OPENGL] `|`
    ///         [`IS_SET_DM_BAYER`][IS_SET_DM::IS_SET_DM_BAYER]:
    ///         Raw Bayer format image display in OpenGL mode.
    ///     * [`IS_GET_DISPLAY_MODE`][IS_SET_DM::IS_GET_DISPLAY_MODE]:
    ///         Returns the current setting.
    ///
    /// The new Direct3D mode completely replaces the "BackBuffer" and "Overlay Surface" display
    /// modes from DirectDraw. It is advisable not to use these modes any longer
    /// (see also [Obsolete functions](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk_veraltete_funktionen.html)).
    /// To activate the obsolete modes, do the following:
    /// * [`IS_SET_DM_DIRECTDRAW`][IS_SET_DM::IS_SET_DM_DIRECTDRAW] `|`
    ///     [`IS_SET_DM_BACKBUFFER`][IS_SET_DM::IS_SET_DM_BACKBUFFER]:
    ///     Image display in DirectDraw BackBuffer mode.
    /// * [`IS_SET_DM_DIRECTDRAW`][IS_SET_DM::IS_SET_DM_DIRECTDRAW] `|`
    ///     [`IS_SET_DM_BACKBUFFER`][IS_SET_DM::IS_SET_DM_BACKBUFFER]:
    ///     Image display in DirectDraw Overlay Surface mode.
    /// * [`IS_SET_DM_DIRECTDRAW`][IS_SET_DM::IS_SET_DM_DIRECTDRAW]:
    ///     Real-time scaling in Overlay Surface mode.
    ///
    /// # Return values
    /// * _When used with [`IS_GET_DISPLAY_MODE`][IS_SET_DM::IS_GET_DISPLAY_MODE]:_
    ///     Current setting.
    /// * [`IS_CANT_ADD_TO_SEQUENCE`]
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_CAPTURE_RUNNING`]
    /// * [`IS_DR_CANNOT_CREATE_SURFACE`]
    /// * [`IS_DR_CANNOT_CREATE_TEXTURE`]
    /// * [`IS_DR_CANNOT_CREATE_VERTEX_BUFFER`]
    /// * [`IS_DR_DEVICE_OUT_OF_MEMORY`]
    /// * [`IS_DR_LIBRARY_NOT_FOUND`]
    /// * [`IS_INVALID_CAMERA_TYPE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_MODE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_IR_FILTER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_CALIBRATED`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_NULL_POINTER`]
    /// * [`IS_OUT_OF_MEMORY`]
    /// * [`IS_SEQUENCE_BUF_ALREADY_LOCKED`]
    /// * [`IS_SUCCESS`]
    /// * [`IS_TIMED_OUT`]
    ///
    /// # Related functions
    /// * [`is_RenderBitmap`]
    /// * [`is_SetColorMode`]
    /// * [`is_SetDisplayMode`]
    ///
    /// # Documentation
    /// [is_SetDisplayMode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_setdisplaymode.html)
    pub fn is_SetDisplayMode(hwnd: HWND, Mode: IS_SET_DM) -> INT;

    /// Move an area of interest when rendering images using [`is_RenderBitmap`].
    ///
    /// The function moves the camera image by the selected offset within the output window.
    /// The image memory remains unchanged.
    ///
    /// To set the size and position of an area of interest in memory, use the [`is_AOI`] functions.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `x` - Offset in X direction, measured from the top left corner of the output window.
    /// * `y` - Offset in Y direction, measured from the top left corner of the output window.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_AOI`]
    /// * [`is_RenderBitmap`]
    /// * [`is_SetDisplayMode`]
    ///
    /// # Documentation
    /// [is_SetDisplayPos](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_setdisplaypos.html)
    #[cfg(target_os = "windows")]
    pub fn is_SetDisplayPos(hCam: HIDS, x: INT, y: INT) -> INT;

    /// **Obsolete:** Sets a new window handle for the image output with DirectDraw.
    ///
    /// The new handle and the image output is only activated when [`is_SetDisplayMode`] is called.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `hwnd` - Handle to a window.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated(note = "Use is_DirectRenderer instead")]
    pub fn is_SetHwnd(hCam: HIDS, hwnd: HWND) -> INT;

    /// Reads out the VSYNC counter.
    ///
    /// It will be incremented by 1 each time the sensor starts capturing an image.\
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pIntr` - Current VSYNC count.
    /// * `pActIntr` - Current Frame SYNC count.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_GetFramesPerSecond`]
    ///
    /// # Documentation
    /// [is_GetVsyncCount](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_getvsynccount.html)
    pub fn is_GetVsyncCount(hCam: HIDS, pIntr: *mut long, pActIntr: *mut long) -> INT;

    /// **Obsolete:** Updates manually the display in DirectDraw modes.
    ///
    /// Normally the driver does the update automatically. In some cases it can be necessary to
    /// update manually the display because of changed conditions (see [`is_SetDisplayMode`]).
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated(note = "Use is_DirectRenderer instead")]
    pub fn is_UpdateDisplay(hf: HIDS) -> INT;

    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_SetDisplaySize(hf: HIDS, x: INT, y: INT) -> INT;

    /// **Obsolete:** Enables access to the overlay buffer in DirectDraw back buffer mode.
    ///
    /// In DirectDraw mode [`is_LockDDOverlayMem`] gives access to the image memory and returns the
    /// address pointer to the beginning of the image memory. And thus the overlay buffer can be
    /// directly written to, without having to use the Windows GDI functions. `pPitch` returns the
    /// line offset in bytes from the beginning of one line to the beginning of the following line.
    /// Access must be disabled as soon as possible with the [`is_UnlockDDOverlayMem`] function.
    /// Within a `LockDDMem` – `UnlockDDMem` block there are no updates of the back buffer on the
    /// display.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    /// * `ppMem` - Pointer to variable, which will contain address pointer.
    /// * `pPitch` - Pointer to variable, which will contain the pitch value.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_LockDDOverlayMem(hf: HIDS, ppMem: *mut *const void, pPitch: *mut INT) -> INT;

    /// **Obsolete:** Disables access to the overlay buffer in DirectDraw back buffer mode.
    ///
    /// The contents of the overlay buffer are updated on the display
    /// (if [`is_ShowDDOverlay`] has faded on the overlay).
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_UnlockDDOverlayMem(hf: HIDS) -> INT;

    /// **Obsolete:** Gives access to the image memory and returns the address pointer to the
    /// beginning of the image memory.
    ///
    /// In most cases the image memory is on the VGA card. Using the pointer the image memory can
    /// be accessed directly. Access is disabled with [`is_UnlockDDMem`] – this must be done as
    /// soon as possible.
    ///
    /// Digitizing to memory cannot be terminated with [`is_LockDDMem`].
    ///
    /// When in DirectDraw back buffer mode, within a `LockDDMem` – `UnlockDDMem` block there are
    /// no updates of the back buffer on the display.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    /// * `ppMem` - Pointer to variable, which will contain address pointer.
    /// * `pPitch` - Pointer to variable, which will contain the pitch value.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_LockDDMem(hf: HIDS, ppMem: *mut *const void, pPitch: *mut INT) -> INT;

    /// **Obsolete:** Disables access to the image memory in DirectDraw mode.
    ///
    /// Then the contents of the back buffer are updated on the display.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_UnlockDDMem(hf: HIDS) -> INT;

    /// **Obsolete:** Returns the pointer to the internal DirectDraw surface, and thus, functions
    /// from the DirectDraw surface interface can be used.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    /// * `ppDDSurf` - Contains pointer to the DirectDraw surface interface.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_GetDDOvlSurface(hf: HIDS, ppDDSurf: *mut *const void) -> INT;

    /// **Obsolete:** Identify the keying color for the DirectDraw overlay surface mode.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    ///     * [`IS_GET_KC_RED`] = Returns the R channel.
    ///     * [`IS_GET_KC_GREEN`] = Returns the G channel.
    ///     * [`IS_GET_KC_BLUE`] = Returns the B channel.
    ///     * [`IS_GET_KC_RGB`] = Returns the RGB.
    /// * `r` - Red channel of keying color (`0`…`255`).
    /// * `g` - Green channel of keying color (`0`…`255`).
    /// * `b` - Blue channel of keying color (`0`…`255`).
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_SetKeyColor(hf: HIDS, r: INT, g: INT, b: INT) -> INT;

    /// **Obsolete:** The position where the image is faded on can be moved by ±1 pixels in
    /// X and Y directions.
    ///
    /// Normally it should not be necessary to perform a move, but this depends upon the type of
    /// VGA card used and type of driver.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    /// * `nOffsetX` - X offset (_possible values: `-1`, `0`, `1`_) -- or:
    ///     * [`IS_GET_KEYOFFSET_X`] = Returns the X offset.
    ///     * [`IS_GET_KEYOFFSET_Y`] = Returns the Y offset.
    /// * `nOffsetY` - Y offset (_possible values: `-1`, `0`, `1`_).
    ///
    /// # Return values
    /// * _When called with [`IS_GET_KEYOFFSET_X`] and [`IS_GET_KEYOFFSET_Y`]:_ Current setting
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_SetKeyOffset(hf: HIDS, nOffsetX: INT, nOffsetY: INT) -> INT;

    /// **Obsolete:** Sets the timer interval for the update cycle of the video image in
    /// DirectDraw mode.
    ///
    /// Valid values are between 20 ms and 2000 ms (20 ms is unrealistic).
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    /// * `ms` - Time in milliseconds.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_SetDDUpdateTime(hf: HIDS, ms: INT) -> INT;

    /// **Obsolete:** Activates the live overlay mode.
    ///
    /// In overlay mode three non-visible image buffers are used: Back buffer, overlay buffer and
    /// mix buffer. The video image is digitized in the back buffer. The graphics data can be
    /// written in the overlay buffer and thus the overlay data is overlaid on the video image in
    /// the mix buffer. The mix buffer is then copied to the visible area of the VGA card.
    /// The size of the three buffers is:
    /// ```txt
    /// video_x * video_y * color depth (in bytes per pixel)
    /// ```
    ///
    /// The driver tries to allocate the buffer directly on the VGA card, (making best use of the
    /// high speed image transfer that the VGA card can offer) when mixing the three buffers.
    /// If the buffer cannot be allocated in the VGA card, then it must be stored in the
    /// system memory. The image transfer from the system memory is slower and, depending on the
    /// graphics card, sometimes not at all possible. The overlay is not always faded on.
    /// It has to be made visible with [`is_ShowDDOverlay`]. As its key color, the overlay uses
    /// black, and thus an overlay cannot contain any black color.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_EnableDDOverlay(hf: HIDS) -> INT;

    /// **Obsolete:** Fades on the overlay in DirectDraw back buffer mode.
    ///
    /// The last data received in the overlay buffer will be displayed. The display is created with
    /// only three image buffers. Depending upon the VGA card, the image refresh can be smaller
    /// than the overlay display.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_ShowDDOverlay(hf: HIDS) -> INT;

    /// **Obsolete:** Fades out the overlay in DirectDraw back buffer mode.
    ///
    /// Then only the image buffer is displayed, so that the image refresh frequency is higher than
    /// then the overlay, which is being superimposed. When the overlay is faded out the overlay
    /// data is not lost.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_HideDDOverlay(hf: HIDS) -> INT;

    /// **Obsolete:** Determines whether the overlay in DirectDraw back buffer mode is scaled,
    /// if the image is scaled, or not.
    ///
    /// If so, the quality of the overlay might be worse. Scaling is normally switched off.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    /// * `boScale` - Mode:
    ///     * [`FALSE`] = No scaling.
    ///     * [`TRUE`] = Scaling.
    ///
    /// # Return values
    /// * _With [`IS_GET_DD_OVERLAY_SCALE`]:_ Current setting.
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_ScaleDDOverlay(hf: HIDS, boScale: BOOL) -> INT;

    /// **Obsolete:** Returns the overlay buffer’s device context handle.
    ///
    /// Using this handle Windows GDI functions can access the overlay. All graphics commands such
    /// as `line`, `circle`, `rectangle` and `text` out from Windows are available. The device
    /// context handle must be released as soon as possible with the [`is_ReleaseDC`] function.
    /// Within the `GetDC` - `ReleaseDC` blocks there are no updates of the overlay buffer on the
    /// display.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    /// * `phDC` - Pointer to the variable, which the device handle takes over.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_GetDC(hf: HIDS, phDC: *mut HDC) -> INT;

    /// **Obsolete:** Releases the overlay buffer’s device context handle.
    ///
    /// Once the handle has been released, an update of the overlay buffer on the display follows
    /// (if the overlay has been blended on with [`is_ShowDDOverlay`]).
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    /// * `hDC` - Device context handle from [`is_GetDC`].
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_ReleaseDC(hf: HIDS, hDC: HDC) -> INT;

    /// **Obsolete:** In overlay surface mode: switches off the overlay surface when the output
    /// window is moved, its size is changed, or when other applications are activated.
    ///
    /// While the overlay surface is switched off, the standard background color of the window
    /// is displayed.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    /// * `boMode` - Mode:
    ///     * [`TRUE`] = Switch off overlay when window is moved, etc.
    ///     * [`FALSE`] = Overlay always on (_default_).
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_OvlSurfaceOffWhileMove(hf: HIDS, boMode: BOOL) -> INT;

    /// **Obsolete:** Prepares image acquisition in an image memory which has been allocated with
    /// [`is_AllocImageMem`].
    ///
    /// During the display of a live image in DirectDraw mode, a number of images can be stolen
    /// from the live image acquisition and be copied to an allocated area of memory. The live
    /// image acquisition is halted only for this number of images.
    ///
    /// As an alternative an automatic steal mode can be used. And thus the number of images which
    /// are alternately sent to VGA card and main memory can be set separately between
    /// `1` and `255`. When starting the automatic steal mode the first image is sent to the VGA
    /// card following alternately the set number of frames to the main memory and VGA card.
    /// Image acquisition in steal mode can be combined with an image reflection around the
    /// horizontal axis (see also [`is_SetRopEffect`]). For this purpose parameter `Mode` has to be
    /// logical "or"-operated with one of the `MIRROR` constants. The mode set with function
    /// [`is_SetRopEffect`] does not affect the “stealing” of the image. A desired ROP effect must
    /// be set in [`is_PrepareStealVideo`] through this OR-Operation mentioned above.
    ///
    /// The steal mode can only be used in connection with the image acquisition modes
    /// `IS_SET_CM_FRAME` or `IS_SET_CM_NEXT_FRAME` in the interlaced or non-interlaced
    /// representation (see also [`is_SetCaptureMode`]).
    ///
    /// Using the sequence event (see also [`is_InitEvent`]) more events as the set number of
    /// images acquisited in the image memory can be signalized in connection with the DirectDraw
    /// primary surface mode.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `boMode` - Initialization or deinitialization of the mode:
    ///     * Modes:
    ///         * [`IS_INIT_STEAL_VIDEO`]
    ///         * [`IS_INIT_STEAL_VIDEO_MANUAL`]
    ///         * [`IS_INIT_STEAL_VIDEO_AUTO`]
    ///         * [`IS_SET_STEAL_RATIO`]
    ///         * [`IS_EXIT_STEAL_VIDEO`]
    ///     * Additional reflection around the horizontal axis (OR-operation):
    ///         * [`IS_SET_ROP_MIRROR_UPDOWN`]
    ///         * [`IS_SET_ROP_MIRROR_UPDOWN_ODD`]
    ///         * [`IS_SET_ROP_MIRROR_UPDOWN_EVEN`]
    ///     * Steal-image size independent of the current settings (or-linked):
    ///         * [`IS_USE_MEM_IMAGE_SIZE`]
    /// * `StealColorMode` - Color mode with which the image should be stolen.
    ///     (Color mode see [`is_SetColorMode`]). A corresponding image memory must be allocated.
    ///
    /// The number of images to the VGA card and main memory can be set in case of
    /// [`IS_SET_STEAL_RATIO`] with parameter `StealColorMode`:
    /// * Bits `31`…`24`: _reserved_
    /// * Bits `23`…`16`: Number of frames to VGA card.
    /// * Bits `15`…`8`: Number of frames to main memory.
    /// * Bits `7`…`0`: Steal color mode.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_PrepareStealVideo(hCam: HIDS, Mode: INT, StealColorMode: ULONG) -> INT;

    /// **Obsolete:** Steals images from DirectDraw live mode.
    ///
    /// The set number of full frames after the call of the function is redirected to the allocated
    /// image memory. Besides steal mode there is an automatic steal mode
    /// (see [`is_PrepareStealVideo`]). Using the automatic steal mode the number of images, which
    /// are transmitted alternately into the VGA card or into the main memory, can be set
    /// separately within the range of `1`…`255`. After starting the automatic steal mode the first
    /// image is sent to the VGA card, following alternate the adjusted number of frames into the
    /// main memory and the VGA card.
    ///
    /// <div class="warning">
    ///
    /// [`is_StealVideo`] cannot be used in Direct3D mode.
    ///
    /// </div>
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    /// * `Wait` - Mode:
    ///     * [`IS_WAIT`] = Function waits until the image has been acquired.
    ///     * [`IS_DONT_WAIT`] = Function returns immediately.
    ///     * [`IS_EXIT_STEAL_VIDEO`] = Stops the automatic steal mode.
    ///
    /// # Return values
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Obsolete replacement
    /// * [`is_DirectRenderer`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_StealVideo(hf: HIDS, Wait: INT) -> INT;

    /// Displays color bars on your monitor.
    ///
    /// Depending upon your display mode, these will appear in either monochrome or color.
    /// The color bars are produced by the FALCON/EAGLE with the color bar generator. These can be
    /// used to check that the frame grabber is working in harmony with your computer. The color
    /// bar generator displays an onboard video source.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    /// * `Mode` - Mode. See [`IS_CBARS_MODE`].
    ///
    /// # Return values
    /// * _When called with [`IS_GET_CBARS_MODE`][IS_CBARS_MODE::IS_GET_CBARS_MODE]:_
    ///     Current setting.
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    #[cfg(target_os = "windows")]
    #[deprecated]
    pub fn is_ShowColorBars(hf: HIDS, Mode: IS_CBARS_MODE) -> INT;
}
