#![allow(non_camel_case_types)]

use bitflags::{bitflags, Flags};

use crate::constants::return_values::*;
use crate::types::{double, void, BOOL, HIDS, INT, TRUE, UINT};

pub const IS_LUT_64: usize = 64;
pub const IS_LUT_128: usize = 128;

/// LUT presets.
///
/// # Documentation
/// [`is_LUT`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_lut.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum LUT_PRESET {
    /// Linear LUT, no image modifications.
    IS_LUT_PRESET_ID_IDENTITY = 0,

    /// Inverts the image.
    IS_LUT_PRESET_ID_NEGATIVE = 1,

    /// False-color display of the image.
    IS_LUT_PRESET_ID_GLOW1 = 2,

    /// False-color display of the image.
    IS_LUT_PRESET_ID_GLOW2 = 3,

    /// False-color display of the image.
    IS_LUT_PRESET_ID_ASTRO1 = 4,

    /// False-color display of the image.
    IS_LUT_PRESET_ID_RAINBOW1 = 5,

    /// False-color display of the image.
    IS_LUT_PRESET_ID_MAP1 = 6,

    /// False-color display of the image.
    IS_LUT_PRESET_ID_HOT = 7,

    /// Uses sepia toning for coloring the image.
    IS_LUT_PRESET_ID_SEPIC = 8,

    /// Shows only the red channel of the image.
    IS_LUT_PRESET_ID_ONLY_RED = 9,

    /// Shows only the green channel of the image.
    IS_LUT_PRESET_ID_ONLY_GREEN = 10,

    /// Shows only the blue channel of the image.
    IS_LUT_PRESET_ID_ONLY_BLUE = 11,

    /// Digital brightness correction.
    IS_LUT_PRESET_ID_DIGITAL_GAIN_2X = 12,

    /// Digital brightness correction.
    IS_LUT_PRESET_ID_DIGITAL_GAIN_4X = 13,

    /// Digital brightness correction.
    IS_LUT_PRESET_ID_DIGITAL_GAIN_8X = 14,
}

/// Structure with LUT values.
///
/// # Documentation
/// [`is_LUT`: Contents of the `IS_LUT_CONFIGURATION_64` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_lut.html)
#[derive(Debug, Clone)]
#[repr(C)]
pub struct IS_LUT_CONFIGURATION_64 {
    /// Defines a LUT with 64 knee points.
    /// This results in 32 sections with a start and end point each.
    pub dblValues: [[double; 3]; IS_LUT_64],

    /// If [`TRUE`], the same LUT is applied to all three channels.
    pub bAllChannelsAreEqual: BOOL,
}

/// Structure for LUT presets.
///
/// # Documentation
/// [`is_LUT`: Contents of the `IS_LUT_CONFIGURATION_PRESET_64` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_lut.html)
#[derive(Debug, Clone)]
#[repr(C)]
pub struct IS_LUT_CONFIGURATION_PRESET_64 {
    /// ID of the predefined LUT.
    pub predefinedLutID: LUT_PRESET,

    /// Predefined LUT.
    pub lutConfiguration: IS_LUT_CONFIGURATION_64,
}

bitflags! {
    /// Defines for the state of the LUT (_supports bitmask_).
    ///
    /// # Documentation
    /// [`is_LUT`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_lut.html)
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[repr(transparent)]
    pub struct LUT_STATE_ID: UINT {
        const IS_LUT_STATE_ID_FLAG_HARDWARE = 0x0010;
        const IS_LUT_STATE_ID_FLAG_SOFTWARE = 0x0020;
        const IS_LUT_STATE_ID_FLAG_GAMMA = 0x0100;
        const IS_LUT_STATE_ID_FLAG_LUT = 0x0200;

        const IS_LUT_STATE_ID_INACTIVE = 0x0000;
        const IS_LUT_STATE_ID_NOT_SUPPORTED = 0x0001;
    }
}

pub const IS_LUT_STATE_ID_HARDWARE_LUT: LUT_STATE_ID =
    LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_HARDWARE.union(LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_LUT);

pub const IS_LUT_STATE_ID_HARDWARE_GAMMA: LUT_STATE_ID =
    LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_HARDWARE.union(LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_GAMMA);

pub const IS_LUT_STATE_ID_HARDWARE_LUTANDGAMMA: LUT_STATE_ID =
    LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_HARDWARE
        .union(LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_LUT)
        .union(LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_GAMMA);

pub const IS_LUT_STATE_ID_SOFTWARE_LUT: LUT_STATE_ID =
    LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_SOFTWARE.union(LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_LUT);

pub const IS_LUT_STATE_ID_SOFTWARE_GAMMA: LUT_STATE_ID =
    LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_SOFTWARE.union(LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_GAMMA);

pub const IS_LUT_STATE_ID_SOFTWARE_LUTANDGAMMA: LUT_STATE_ID =
    LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_SOFTWARE
        .union(LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_LUT)
        .union(LUT_STATE_ID::IS_LUT_STATE_ID_FLAG_GAMMA);

/// Enumeration of LUT calculation modes.
///
/// # Documentation
/// [`is_LUT`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_lut.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum LUT_MODE {
    /// Default mode: If supported the LUT/gamma is done on hardware otherwise in software.
    IS_LUT_MODE_ID_DEFAULT = 0,

    /// LUT/gamma in hardware.
    ///
    /// _If not possible:_ [`LUT_STATE_ID::IS_LUT_STATE_ID_NOT_SUPPORTED`].
    IS_LUT_MODE_ID_FORCE_HARDWARE = 1,

    /// LUT/gamma in software.
    ///
    /// _If not possible:_ [`LUT_STATE_ID::IS_LUT_STATE_ID_NOT_SUPPORTED`].
    IS_LUT_MODE_ID_FORCE_SOFTWARE = 2,
}

/// Structure for the state of the LUT [`is_LUT`].
///
/// # Documentation
/// [`is_LUT`: Contents of the `IS_LUT_STATE` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_lut.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IS_LUT_STATE {
    /// LUT is enabled.
    pub bLUTEnabled: BOOL,

    /// ID of the LUT status information.
    pub nLUTStateID: LUT_STATE_ID,

    /// ID of the LUT mode.
    pub nLUTModeID: LUT_MODE,

    /// Used bits of the LUT.
    pub nLUTBits: INT,
}

/// Enabled state of the LUT.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[allow(u32)]
pub enum LUT_ENABLED {
    /// LUT is disabled.
    IS_LUT_DISABLED = 0,

    /// LUT is enabled.
    ///
    /// If no other LUT is defined, the linear LUT [`LUT_PRESET::IS_CAMERA_LUT_IDENTITY`] is set.
    IS_LUT_ENABLED = 1,
}

impl From<bool> for LUT_ENABLED {
    fn from(value: bool) -> Self {
        if value {
            Self::IS_LUT_ENABLED
        } else {
            Self::IS_LUT_DISABLED
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct IS_LUT_SUPPORT_INFO {
    /// Hardware LUT is supported.
    pub bSupportLUTHardware: BOOL,

    /// Software LUT is supported.
    pub bSupportLUTSoftware: BOOL,

    /// Used bits of the hardware LUT.
    pub nBitsHardware: INT,

    /// Used bits of the software LUT.
    pub nBitsSoftware: INT,

    /// Supported channels for hardware LUT.
    pub nChannelsHardware: INT,

    /// Supported channels for software LUT.
    pub nChannelsSoftware: INT,
}

/// Commands for the [`is_LUT`] function.
///
/// # Documentation
/// [`is_LUT`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_lut.html)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum LUT_CMD {
    IS_LUT_CMD_SET_ENABLED = 0x0001,
    IS_LUT_CMD_SET_MODE = 0x0002,
    IS_LUT_CMD_GET_STATE = 0x0005,
    IS_LUT_CMD_GET_SUPPORT_INFO = 0x0006,
    IS_LUT_CMD_SET_USER_LUT = 0x0010,
    IS_LUT_CMD_GET_USER_LUT = 0x0011,
    IS_LUT_CMD_GET_COMPLETE_LUT = 0x0012,
    IS_LUT_CMD_GET_PRESET_LUT = 0x0013,
    IS_LUT_CMD_LOAD_FILE = 0x0100,
    IS_LUT_CMD_SAVE_FILE = 0x0101,
}

unsafe extern "C" {

    /// Enable a hardware or software LUT for uEye cameras which will be applied to the image in
    /// the camera.
    ///
    /// Each lookup table (LUT) for the uEye camera contains modification values for the image
    /// brightness and contrast parameters. When a LUT is used, each brightness value in the image
    /// will be replaced by a value from the table. LUTs are typically used to enhance the image
    /// contrast or the gamma curve.
    ///
    /// A number of predefined LUTs are available. Alternatively, you define your own LUT. A LUT
    /// consists of 32 sections. Each section is defined by start and end points. The values for
    /// the start and end points must be in the range between `0.0` and `1.0`. If you want to
    /// create a linear LUT, the start and end points of successive sections must have the
    /// same values.
    ///
    /// You can also define a LUT without enabling it at the same time. You can also query the
    /// current LUT used by the camera.
    ///
    /// For further information on LUTs, please refer to the
    /// [LUT properties](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/hw_lut_gamma.html) section.
    ///
    /// ## LUT and automatic exposure (AES)/gain control (AGC)
    /// If you use LUT in combination with automatic exposure (AES) or gain control (AGC) undesired
    /// side effects may occur. For example, if you use a negative LUT (the image brightness is
    /// inverted) but at the same time the automatic control lightens the image. For this reason,
    /// disable the automatic controls if you use a discontinuous (jumps) or not completely
    /// positive (gradient) LUT.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`LUT_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_FILE_READ_OPEN_ERROR`]
    /// * [`IS_FILE_WRITE_OPEN_ERROR`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Documentation
    /// [`is_LUT`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_lut.html)
    pub fn is_LUT(hCam: HIDS, nCommand: LUT_CMD, pParam: *mut void, cbSizeOfParams: UINT) -> INT;
}
