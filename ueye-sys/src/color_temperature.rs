//! Fix a setting (in kelvins) for the color temperature of an image when you are using a
//! color camera.
//!
//! The function will use the sensor's hardware gain controls for the setting, as far as possible.
//! In addition, you can choose between different color spaces. A specific color temperature will
//! result in slightly differing RGB values, depending on the selected color space.
//!
//! Additionally, you can set or return the color temperature for the _UI-1007XS_ lens correction.
//!
//! Note that the current LUT values are calculated at the first function call of
//! [`is_ColorTemperature`]. Depending on the system used, the first function call may take a
//! little longer.
//!
//! The color temperature is the temperature to which a black body radiator has to be heated to
//! glow and give off light in the corresponding color. Warm light (reddish) has a lower color
//! temperature than cold light (bluish). The following table lists a few example values:
//!
//! | Light source                  | Color temperature |
//! |-------------------------------|-------------------|
//! | Light bulb (100 W)            | 2800              |
//! | Halogen lamp                  | 3200              |
//! | Fluorescent lamp (cold white) | 4000              |
//! | Morning and evening sunlight  | 5000              |
//! | Noon sunlight                 | 5500-5800         |
//! | Flash strobe                  | 6000              |
//! | Overcast daylight             | 6500-7500         |
//! | Fog                           | 8000              |
//!
//! <div class="warning">
//!
//! The [`is_ColorTemperature`] function cannot be used simultaneously with the automatic white
//! balance function in [`is_SetAutoParameter`]/[`is_AutoParameter`].
//!
//! </div>
//!
//! # Documentation
//! [is_ColorTemperature](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_colortemperature.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{void, HIDS, INT, UINT};
use bitflags::bitflags;

bitflags! {
    /// Color spaces (_supports bitmask_).
    ///
    /// # Documentation
    /// [is_ColorTemperature: Color Spaces](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_colortemperature.html#farbraeume)
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct RGB_COLOR_MODELS: UINT {
        /// sRGB (standard RGB) color space with a white point of 5000 kelvins (warm light).
        const RGB_COLOR_MODEL_SRGB_D50        = 0x0001;

        /// sRGB (standard RGB) color space with a white point of 6500 kelvins (mid-daylight).
        const RGB_COLOR_MODEL_SRGB_D65        = 0x0002;

        /// CIE-RGB color space with standard illumination E.
        const RGB_COLOR_MODEL_CIE_RGB_E       = 0x0004;

        /// ECI-RGB color space with a white point of 5000 kelvins (warm light).
        const RGB_COLOR_MODEL_ECI_RGB_D50     = 0x0008;

        /// Adobe RGB color space with a white point of 6500 kelvins (mid-daylight).
        ///
        /// The Adobe RGB color space is larger than the sRGB color space,
        /// but not all devices can render it.
        const RGB_COLOR_MODEL_ADOBE_RGB_D65   = 0x0010;
    }
}

bitflags! {
    /// Lens shading models (_supports bitmask_).
    ///
    /// # Documentation
    /// [is_ColorTemperature](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_colortemperature.html)
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct LENS_SHADING_MODELS: UINT {
        /// Color space with a white point of 3000 kelvins.
        const LSC_MODEL_AGL = 0x0001;

        /// Color space with a white point of 4000 kelvins.
        const LSC_MODEL_TL84 = 0x0002;

        /// Color space with a white point of 5000 kelvins.
        const LSC_MODEL_D50 = 0x0004;

        /// Color space with a white point of 6500 kelvins.
        const LSC_MODEL_D65 = 0x0008;
    }
}

/// Enumeration of commands for [`is_ColorTemperature`].
///
/// # Documentation
/// [is_ColorTemperature](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_colortemperature.html)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum COLOR_TEMPERATURE_CMD {
    /// Sets a color temperature.
    ///
    /// # Parameter type
    /// [`UINT`]
    COLOR_TEMPERATURE_CMD_SET_TEMPERATURE = 0,

    /// Sets a [color space][RGB_COLOR_MODELS].
    ///
    /// # Parameter type
    /// [`RGB_COLOR_MODELS`]
    COLOR_TEMPERATURE_CMD_SET_RGB_COLOR_MODEL = 1,

    /// Returns the supported [color spaces][RGB_COLOR_MODELS].
    ///
    /// # Parameter type
    /// [`RGB_COLOR_MODELS`]
    COLOR_TEMPERATURE_CMD_GET_SUPPORTED_RGB_COLOR_MODELS = 2,

    /// Returns the set color temperature.
    ///
    /// # Parameter type
    /// [`UINT`]
    COLOR_TEMPERATURE_CMD_GET_TEMPERATURE = 3,

    /// Returns the set [color space][RGB_COLOR_MODELS].
    ///
    /// # Parameter type
    /// [`RGB_COLOR_MODELS`]
    COLOR_TEMPERATURE_CMD_GET_RGB_COLOR_MODEL = 4,

    /// Returns the minimum value for the color temperature.
    ///
    /// # Parameter type
    /// [`UINT`]
    COLOR_TEMPERATURE_CMD_GET_TEMPERATURE_MIN = 5,

    /// Returns the maximum value for the color temperature.
    ///
    /// # Parameter type
    /// [`UINT`]
    COLOR_TEMPERATURE_CMD_GET_TEMPERATURE_MAX = 6,

    /// Returns the increment for setting the color temperature.
    ///
    /// # Parameter type
    /// [`UINT`]
    COLOR_TEMPERATURE_CMD_GET_TEMPERATURE_INC = 7,

    /// Returns the default value for the color temperature.
    ///
    /// # Parameter type
    /// [`UINT`]
    COLOR_TEMPERATURE_CMD_GET_TEMPERATURE_DEFAULT = 8,

    /// Returns the default color space.
    ///
    /// # Parameter type
    /// [`RGB_COLOR_MODELS`]
    COLOR_TEMPERATURE_CMD_GET_RGB_COLOR_MODEL_DEFAULT = 9,

    /// Sets the new color temperature for lens correction.
    ///
    /// # Parameter type
    /// [`LENS_SHADING_MODELS`]
    COLOR_TEMPERATURE_CMD_SET_LENS_SHADING_MODEL = 10,

    /// Returns the set color temperature for lens correction.
    ///
    /// # Parameter type
    /// [`LENS_SHADING_MODELS`]
    COLOR_TEMPERATURE_CMD_GET_LENS_SHADING_MODEL = 11,

    /// Returns the supported color temperature for lens correction.
    ///
    /// # Parameter type
    /// [`LENS_SHADING_MODELS`], _bitmask_
    COLOR_TEMPERATURE_CMD_GET_LENS_SHADING_MODEL_SUPPORTED = 12,

    /// Returns the default value for the color temperature for lens correction.
    ///
    /// # Parameter type
    /// [`LENS_SHADING_MODELS`]
    COLOR_TEMPERATURE_CMD_GET_LENS_SHADING_MODEL_DEFAULT = 13,
}

unsafe extern "C" {
    /// Fix a setting (in kelvins) for the color temperature of an image when you are using a
    /// color camera.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`COLOR_TEMPERATURE_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `nSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_SetHardwareGain`]
    /// * [`is_AutoParameter`]
    /// * [`is_SetAutoParameter`]
    ///
    /// # Documentation
    /// [is_ColorTemperature](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_colortemperature.html)
    pub fn is_ColorTemperature(
        hCam: HIDS,
        nCommand: COLOR_TEMPERATURE_CMD,
        pParam: *mut void,
        nSizeOfParam: UINT,
    ) -> INT;
}
