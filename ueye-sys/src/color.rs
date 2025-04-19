//! Common color functions.

use crate::constants::return_values::*;
use crate::types::{HIDS, INT};

/// Read current color format in function [`is_SetColorMode`].
pub const IS_GET_COLOR_MODE: INT = 0x8000;

/// Get bits per pixel for the current color format in function [`is_SetColorMode`].
pub const IS_GET_BITS_PER_PIXEL: INT = 0x9000;

/// Planar format.
pub const IS_CM_FORMAT_PLANAR: INT = 0x2000;

/// Format mask.
pub const IS_CM_FORMAT_MASK: INT = 0x2000;

/// BGR pixel order.
pub const IS_CM_ORDER_BGR: INT = 0x0000;

/// RGB pixel order.
pub const IS_CM_ORDER_RGB: INT = 0x0080;

/// Pixel order mask.
pub const IS_CM_ORDER_MASK: INT = 0x0080;

/// Raw sensor data (8), for monochrome and color cameras, LUT/gamma not active.
const IS_CM_SENSOR_RAW8: INT = 11;

/// Raw sensor data (10), for monochrome and color cameras, LUT/gamma not active.
const IS_CM_SENSOR_RAW10: INT = 33;

/// Raw sensor data (12), for monochrome and color cameras, LUT/gamma not active.
const IS_CM_SENSOR_RAW12: INT = 27;

/// Raw sensor data (16), for monochrome and color cameras, LUT/gamma not active.
const IS_CM_SENSOR_RAW16: INT = 29;

/// Grayscale (8), for monochrome and color cameras, LUT/gamma active.
const IS_CM_MONO8: INT = 6;

/// Grayscale (10), for monochrome and color cameras, LUT/gamma active.
const IS_CM_MONO10: INT = 34;

/// Grayscale (12), for monochrome and color cameras, LUT/gamma active.
const IS_CM_MONO12: INT = 26;

/// Grayscale (16), for monochrome and color cameras, LUT/gamma active.
const IS_CM_MONO16: INT = 28;

/// BGR (5 5 5), for monochrome and color cameras, LUT/gamma active.
const IS_CM_BGR5_PACKED: INT = (3 | IS_CM_ORDER_BGR);

/// BGR (5 6 5), for monochrome and color cameras, LUT/gamma active.
const IS_CM_BGR565_PACKED: INT = (2 | IS_CM_ORDER_BGR);

/// RGB (8 8 8), for monochrome and color cameras, LUT/gamma active.
const IS_CM_RGB8_PACKED: INT = (1 | IS_CM_ORDER_RGB);

/// BGR (8 8 8), for monochrome and color cameras, LUT/gamma active.
const IS_CM_BGR8_PACKED: INT = (1 | IS_CM_ORDER_BGR);

/// RGB (8 8 8), for monochrome and color cameras, LUT/gamma active.
const IS_CM_RGBA8_PACKED: INT = (0 | IS_CM_ORDER_RGB);

/// BGR (8 8 8), for monochrome and color cameras, LUT/gamma active.
const IS_CM_BGRA8_PACKED: INT = (0 | IS_CM_ORDER_BGR);

/// RGBY (8 8 8 8), for monochrome and color cameras, LUT/gamma active.
const IS_CM_RGBY8_PACKED: INT = (24 | IS_CM_ORDER_RGB);

/// BGRY (8 8 8), for monochrome and color cameras, LUT/gamma active.
const IS_CM_BGRY8_PACKED: INT = (24 | IS_CM_ORDER_BGR);

/// RGB (10 10 10), for monochrome and color cameras, LUT/gamma active.
const IS_CM_RGB10_PACKED: INT = (25 | IS_CM_ORDER_RGB);

/// BGR (10 10 10), for monochrome and color cameras, LUT/gamma active.
const IS_CM_BGR10_PACKED: INT = (25 | IS_CM_ORDER_BGR);

/// Unpacked RGB (10 10 10), for monochrome and color cameras, LUT/gamma active.
const IS_CM_RGB10_UNPACKED: INT = (35 | IS_CM_ORDER_RGB);

/// BGR (10 10 10), for monochrome and color cameras, LUT/gamma active.
const IS_CM_BGR10_UNPACKED: INT = (35 | IS_CM_ORDER_BGR);

/// Unpacked RGB (12 12 12), for monochrome and color cameras, LUT/gamma active.
const IS_CM_RGB12_UNPACKED: INT = (30 | IS_CM_ORDER_RGB);

/// Unpacked BGR (12 12 12), for monochrome and color cameras, LUT/gamma active.
const IS_CM_BGR12_UNPACKED: INT = (30 | IS_CM_ORDER_BGR);

/// Unpacked RGB (12 12 12), for monochrome and color cameras, LUT/gamma active.
const IS_CM_RGBA12_UNPACKED: INT = (31 | IS_CM_ORDER_RGB);

/// Unpacked BGR (12 12 12), for monochrome and color cameras, LUT/gamma active.
const IS_CM_BGRA12_UNPACKED: INT = (31 | IS_CM_ORDER_BGR);

/// JPEG for USB _uEye XS_.
const IS_CM_JPEG: INT = 32;

/// YUV 4:2:2 (8 8), for monochrome and color cameras, LUT/gamma active.
const IS_CM_UYVY_PACKED: INT = 12;

/// YUV 4:2:2 (8 8), for monochrome and color cameras, LUT/gamma active.
const IS_CM_UYVY_MONO_PACKED: INT = 13;

/// YUV 4:2:2 (8 8), for monochrome and color cameras, LUT/gamma active.
const IS_CM_UYVY_BAYER_PACKED: INT = 14;

/// YCbCr 4:2:2 (8 8), for monochrome and color cameras, LUT/gamma active.
const IS_CM_CBYCRY_PACKED: INT = 23;

const IS_CM_RGB8_PLANAR: INT = (1 | IS_CM_ORDER_RGB | IS_CM_FORMAT_PLANAR);

/// All possible color modes.
pub const IS_CM_ALL_POSSIBLE: INT = 0xFFFF;

/// Color mode mask.
pub const IS_CM_MODE_MASK: INT = 0x007F;

/// This flag indicates whether a packed source pixel format should be used
/// (also for the debayered pixel formats).
pub const IS_CM_PREFER_PACKED_SOURCE_FORMAT: INT = 0x4000;

unsafe extern "C" {
    /// Sets the color mode to be used when image data are saved or displayed by the graphics card.
    ///
    /// For this purpose, the allocated image memory must be large enough to accommodate the data
    /// with the selected color mode. When images are transferred directly to the graphics card
    /// memory, make sure that the display settings match the color mode settings. Otherwise,
    /// the images will be displayed with altered colors or are not clearly visible.
    ///
    /// **Note on display modes:**
    /// This function is only supported in the bitmap (DIB) display mode.
    /// Use the [`is_SetDisplayMode`] function to display other color formats in
    /// Direct3D or OpenGL mode.
    ///
    /// ## _UI-1007XS_ color formats
    /// The _UI-1007XS_ performs color conversion internally and currently supports the following
    /// color formats:
    /// * [`IS_CM_BGR8_PACKED`]
    /// * [`IS_CM_BGRA8_PACKED`]
    /// * [`IS_CM_CBYCRY_PACKED`]
    /// * [`IS_CM_JPEG`]
    ///
    /// ## Note on bit depth
    /// Color formats with a bit depth of more than 8 bits per channel are only supported by the
    /// following models:
    /// * GigE uEye series
    /// * uEye USB 3.1 series
    /// * USB 3 uEye series
    /// * USB 2.0 models with rev. 3
    ///
    /// Using color formats with higher bit depth increases the bandwidth used by a camera.
    ///
    /// ## Note on RGB15/16
    /// For the RGB16 and RGB15 data formats, the MSBs of the internal 8-bit
    /// R, G and B colors are used.
    ///
    /// ## Support
    /// The following color formats are currently not supported by GigE uEye cameras:
    /// * [`IS_CM_SENSOR_RAW10`]
    /// * [`IS_CM_MONO10`]
    /// * [`IS_CM_RGB10_UNPACKED`]
    /// * [`IS_CM_BGR10_UNPACKED`]
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `Mode` - Color mode to be set. For a list of all available color formats and the
    ///     associated input parameters, see the
    ///     [Appendix: Color and memory formats](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk_allgemeines_farbformate.html) section.
    ///
    /// # Return values
    /// * When used together with [`IS_GET_COLOR_MODE`]:
    ///     Current setting
    /// * When used together with [`IS_GET_BITS_PER_PIXEL`]:
    ///     Number of bits per pixel of the currently set color mode
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CAPTURE_RUNNING`]
    /// * [`IS_INVALID_CAMERA_TYPE`]
    /// * [`IS_INVALID_COLOR_FORMAT`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MODE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_IR_FILTER`]
    /// * [`IS_NOT_CALIBRATED`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_NULL_POINTER`]
    /// * [`IS_OUT_OF_MEMORY`]
    /// * [`IS_SUCCESS`]
    /// * [`IS_TIMED_OUT`]
    ///
    /// # Related functions
    /// * [`is_SetDisplayMode`]
    /// * [`is_SetColorConverter`]
    /// * [`is_SetColorCorrection`]
    /// * [`is_GetColorDepth`]
    /// * [`is_AllocImageMem`]
    /// * [`is_RenderBitmap`]
    ///
    /// # Documentation
    /// [is_SetColorMode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_setcolormode.html)
    pub fn is_SetColorMode(hCam: HIDS, Mode: INT) -> INT;
}
