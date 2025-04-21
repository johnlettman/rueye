#![allow(non_camel_case_types)]

use crate::types::{INT, UINT, void};
use crate::constants::return_values::*;

/// Enumeration of CPU idle defines used by [`is_Configuration`].
///
/// # Related commands
/// * [`CONFIGURATION_CMD::IS_CONFIG_CPU_IDLE_STATES_CMD_GET_ENABLE`]
/// * [`CONFIGURATION_CMD::IS_CONFIG_CPU_IDLE_STATES_CMD_GET_DISABLE_ON_OPEN`]
/// * [`CONFIGURATION_CMD::IS_CONFIG_CPU_IDLE_STATES_CMD_SET_DISABLE_ON_OPEN`]
///
/// # Documentation
/// [Processor operating states under Windows: Contents of the `CONFIGURATION_SEL` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationidlestates.html#configuration_sel)
#[cfg(target_os = "windows")]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum CONFIGURATION_SEL_CPU_IDLE {
    /// Set/recover processor operating states for power supply unit operation.
    IS_CONFIG_CPU_IDLE_STATES_BIT_AC_VALUE = 0x01,

    /// Set/recover processor operating states for battery operation.
    IS_CONFIG_CPU_IDLE_STATES_BIT_DC_VALUE = 0x02,
}

/// Enumeration of IPO thread defines used by [`is_Configuration`].
///
/// # Related commands
/// * [`CONFIGURATION_CMD::IS_CONFIG_IPO_CMD_SET_ALLOWED`]
/// * [`CONFIGURATION_CMD::IS_CONFIG_IPO_CMD_GET_ALLOWED`]
///
/// # Documentation
/// [Allowing IPO thread: Contents of the `CONFIGURATION_SEL` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationipothread.html#configuration_sel)
#[cfg(target_os = "windows")]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum CONFIGURATION_SEL_IPO {
    /// IPO thread not allowed.
    IS_CONFIG_IPO_NOT_ALLOWED                      = 0,

    /// IPO thread allowed.
    IS_CONFIG_IPO_ALLOWED                          = 1,
}

#[cfg(target_os = "windows")]
impl From<bool> for CONFIGURATION_SEL_IPO {
    #[inline]
    fn from(value: bool) -> Self {
        if value { Self::IS_CONFIG_IPO_ALLOWED } else { Self::IS_CONFIG_IPO_NOT_ALLOWED }
    }
}

/// Enumeration of OpenMP defines used by [`is_Configuration`].
///
/// # Related commands
/// * [`CONFIGURATION_CMD::IS_CONFIG_OPEN_MP_CMD_SET_ENABLE`]
/// * [`CONFIGURATION_CMD::IS_CONFIG_OPEN_MP_CMD_GET_ENABLE`]
/// * [`CONFIGURATION_CMD::IS_CONFIG_OPEN_MP_CMD_GET_ENABLE_DEFAULT`]
///
/// # Documentation
/// [Activating OpenMP: Contents of the `CONFIGURATION_SEL` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationopenmp.html#configuration_sel)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum CONFIGURATION_SEL_OPEN_MP {
    /// OpenMP support disabled.
    IS_CONFIG_OPEN_MP_DISABLE                      = 0,

    /// OpenMP support enabled.
    IS_CONFIG_OPEN_MP_ENABLE                          = 1,
}

impl From<bool> for CONFIGURATION_SEL_OPEN_MP {
    #[inline]
    fn from(value: bool) -> Self {
        if value { Self::IS_CONFIG_OPEN_MP_ENABLE } else { Self::IS_CONFIG_OPEN_MP_DISABLE }
    }
}

/// Enumeration of initialization camera parameters defines used by [`is_Configuration`].
///
/// # Related commands
/// * [`CONFIGURATION_CMD::IS_CONFIG_INITIAL_PARAMETERSET_CMD_GET`]
/// * [`CONFIGURATION_CMD::IS_CONFIG_INITIAL_PARAMETERSET_CMD_SET`]
///
/// # Documentation
/// [Loading camera parameters during initializing: Contents of the `CONFIGURATION_SEL` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationcameraparameter.html#configuration_sel)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
#[repr(u32)]
pub enum CONFIGURATION_SEL_INITIAL_PARAMETERSET {
    /// Load camera parameters during initialization disabled.
    IS_CONFIG_INITIAL_PARAMETERSET_NONE            = 0,

    /// Load camera parameter set 1 during initialization.
    IS_CONFIG_INITIAL_PARAMETERSET_1               = 1,

    /// Load camera parameter set 2 during initialization.
    IS_CONFIG_INITIAL_PARAMETERSET_2               = 2,
}

/// Enumeration of ETH daemon defines used by [`is_Configuration`].
///
/// # Related commands
/// * [`CONFIGURATION_CMD::IS_CONFIG_ETH_CONFIGURATION_MODE_CMD_GET_ENABLE`]
/// * [`CONFIGURATION_CMD::IS_CONFIG_ETH_CONFIGURATION_MODE_CMD_SET_ENABLE`]
///
/// # Documentation
/// [Configuration mode under Linux: Contents of the `CONFIGURATION_SEL` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationlinuxconfigmode.html#configuration_sel)
#[cfg(target_os = "linux")]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum CONFIGURATION_SEL_ETH_CONFIGURATION {
    /// Disables the configuration mode for the ETH daemon.
    IS_CONFIG_ETH_CONFIGURATION_MODE_OFF           = 0,

    /// Enables the configuration mode for the ETH daemon.
    IS_CONFIG_ETH_CONFIGURATION_MODE_ON            = 1,
}

#[cfg(target_os = "linux")]
impl From<bool> for CONFIGURATION_SEL_ETH_CONFIGURATION {
    #[inline]
    fn from(value: bool) -> Self {
        if value { Self::IS_CONFIG_ETH_CONFIGURATION_MODE_ON } else { Self::IS_CONFIG_ETH_CONFIGURATION_MODE_OFF }
    }
}

/// Enumeration of trusted pairing mode defines used by [`is_Configuration`].
///
/// # Related commands
/// * [`CONFIGURATION_CMD::IS_CONFIG_CMD_TRUSTED_PAIRING_SET`]
/// * [`CONFIGURATION_CMD::IS_CONFIG_CMD_TRUSTED_PAIRING_GET`]
/// * [`CONFIGURATION_CMD::IS_CONFIG_CMD_TRUSTED_PAIRING_GET_DEFAULT`]
///
/// # Documentation
/// [Trusted pairing mode for GigE uEye cameras: Contents of the `CONFIGURATION_SEL` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationtrustedpairing.html#configuration_sel)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum CONFIGURATION_SEL_TRUSTED_PAIRING {
    /// Disables the trusted pairing mode.
    IS_CONFIG_TRUSTED_PAIRING_OFF                  = 0,

    /// Enables the trusted pairing mode.
    IS_CONFIG_TRUSTED_PAIRING_ON                   = 1,
}

impl From<bool> for CONFIGURATION_SEL_TRUSTED_PAIRING {
    #[inline]
    fn from(value: bool) -> Self {
        if value { Self::IS_CONFIG_TRUSTED_PAIRING_ON } else { Self::IS_CONFIG_TRUSTED_PAIRING_OFF }
    }
}

/// Enumeration of image memory compatibility mode defines used by [`is_Configuration`].
///
/// # Related commands
/// * [`CONFIGURATION_CMD::IS_CONFIG_CMD_SET_IMAGE_MEMORY_COMPATIBILIY_MODE`]
/// * [`CONFIGURATION_CMD::IS_CONFIG_CMD_GET_IMAGE_MEMORY_COMPATIBILIY_MODE`]
/// * [`CONFIGURATION_CMD::IS_CONFIG_CMD_GET_IMAGE_MEMORY_COMPATIBILIY_MODE_DEFAULT`]
///
/// # Documentation
/// [Image memory compatibility mode: Contents of the `CONFIGURATION_SEL` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationimagememory.html#configuration_sel)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum CONFIGURATION_SEL_IMAGE_MEMORY_COMPATIBILITY_MODE {
    /// Disables image memory compatibility mode.
    IS_CONFIG_IMAGE_MEMORY_COMPATIBILITY_MODE_OFF  = 0,

    /// Enables image memory compatibility mode.
    IS_CONFIG_IMAGE_MEMORY_COMPATIBILITY_MODE_ON   = 1
}

impl From<bool> for CONFIGURATION_SEL_IMAGE_MEMORY_COMPATIBILITY_MODE {
    #[inline]
    fn from(value: bool) -> Self {
        if value { Self::IS_CONFIG_IMAGE_MEMORY_COMPATIBILITY_MODE_ON } else { Self::IS_CONFIG_IMAGE_MEMORY_COMPATIBILITY_MODE_OFF }
    }
}

/// Enumeration of commands of function [`is_Configuration`].
///
/// # Documentation
/// * [`is_Configuration`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configuration.html)
/// * [Processor operating states under Windows](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationidlestates.html)
/// * [IP address of the network adapter](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationipaddressnetwork.html)
/// * [Allowing IPO thread](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationipothread.html)
/// * [Image memory compatibility mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationimagememory.html)
/// * [Activating OpenMP](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationopenmp.html)
/// * [Loading camera parameters during initializing](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationcameraparameter.html)
/// * [Trusted pairing mode for GigE uEye cameras](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationtrustedpairing.html)
/// * [Configuration mode under Linux](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationlinuxconfigmode.html)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum CONFIGURATION_CMD {
    /// Returns the configuration options supported by the system.
    ///
    /// # Parameter type
    /// [`UINT`]
    ///
    /// # Documentation
    /// [`is_Configuration`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configuration.html)
    IS_CONFIG_CMD_GET_CAPABILITIES                         = 1,

    /// Returns whether the current settings allow low power consumption operating states
    /// (unequal `C0`).
    ///
    /// # Parameter type
    /// [`UINT`], _flags of:_ [`CONFIGURATION_SEL_CPU_IDLE`]
    ///
    /// # Documentation
    /// [Processor operating states under Windows](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationidlestates.html)
    #[cfg(target_os = "windows")]
    IS_CONFIG_CPU_IDLE_STATES_CMD_GET_ENABLE               = 2,

    /// Changes the energy settings of the operating system so that low power consumption
    /// operating states (unequal `C0`) are disabled.
    ///
    /// To apply a new setting, you must close all open USB uEye cameras and then reopen at least
    /// one camera.
    ///
    /// # Parameter type
    /// [`UINT`], _flags of:_ [`CONFIGURATION_SEL_CPU_IDLE`]
    ///
    /// # Documentation
    /// [Processor operating states under Windows](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationidlestates.html)
    #[cfg(target_os = "windows")]
    IS_CONFIG_CPU_IDLE_STATES_CMD_SET_DISABLE_ON_OPEN      = 4,

    /// Returns the current setting for
    /// [`CONFIGURATION_CMD::IS_CONFIG_CPU_IDLE_STATES_CMD_SET_DISABLE_ON_OPEN`].
    ///
    /// # Parameter type
    /// [`UINT`], _flags of:_ [`CONFIGURATION_SEL_CPU_IDLE`]
    ///
    /// # Documentation
    /// [Processor operating states under Windows](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationidlestates.html)
    #[cfg(target_os = "windows")]
    IS_CONFIG_CPU_IDLE_STATES_CMD_GET_DISABLE_ON_OPEN      = 5,

    /// Returns whether OpenMP support is enabled.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_OPEN_MP`]
    ///
    /// # Documentation
    /// [Activating OpenMP](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationopenmp.html)
    IS_CONFIG_OPEN_MP_CMD_GET_ENABLE                       = 6,

    /// Enables OpenMP support.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_OPEN_MP`]
    ///
    /// # Documentation
    /// [Activating OpenMP](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationopenmp.html)
    IS_CONFIG_OPEN_MP_CMD_SET_ENABLE                       = 7,

    /// Returns the default setting for OpenMP support.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_OPEN_MP`]
    ///
    /// # Documentation
    /// [Activating OpenMP](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationopenmp.html)
    IS_CONFIG_OPEN_MP_CMD_GET_ENABLE_DEFAULT               = 8,

    /// Sets the parameter set to read and apply from the non-volatile camera memory
    /// when the camera is opened.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_INITIAL_PARAMETERSET`]
    ///
    /// # Documentation
    /// [Loading camera parameters during initializing](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationcameraparameter.html)
    IS_CONFIG_INITIAL_PARAMETERSET_CMD_SET                 = 9,

    /// Returns which parameter set will be read and applied from the non-volatile camera memory
    /// when the camera is opened.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_INITIAL_PARAMETERSET`]
    ///
    /// # Documentation
    /// [Loading camera parameters during initializing](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationcameraparameter.html)
    IS_CONFIG_INITIAL_PARAMETERSET_CMD_GET                 = 10,

    /// Switches the ETH daemon into a configuration mode to detect wrong configured camera and
    /// set the IP configuration.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_ETH_CONFIGURATION`]
    ///
    /// # Documentation
    /// [Configuration mode under Linux](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationlinuxconfigmode.html)
    #[cfg(target_os = "linux")]
    IS_CONFIG_ETH_CONFIGURATION_MODE_CMD_SET_ENABLE        = 11,

    /// Switches the ETH daemon into a configuration mode and returns the current settings.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_ETH_CONFIGURATION`]
    ///
    /// # Documentation
    /// [Configuration mode under Linux](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationlinuxconfigmode.html)
    #[cfg(target_os = "linux")]
    IS_CONFIG_ETH_CONFIGURATION_MODE_CMD_GET_ENABLE        = 12,

    /// Returns if the `NoIpo` registry value exists.
    /// If the value is set to other than `0` the IPO thread is prevented from running.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_IPO`]
    ///
    /// # Documentation
    /// [Allowing IPO thread](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationipothread.html)
    #[cfg(target_os = "windows")]
    IS_CONFIG_IPO_CMD_GET_ALLOWED                          = 13,

    /// Sets or deletes the `NoIpo` registry value.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_IPO`]
    ///
    /// # Documentation
    /// [Allowing IPO thread](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationipothread.html)
    #[cfg(target_os = "windows")]
    IS_CONFIG_IPO_CMD_SET_ALLOWED                          = 14,

    /// Enables/disables the trusted pairing mode.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_TRUSTED_PAIRING`]
    ///
    /// # Documentation
    /// [Trusted pairing mode for GigE uEye cameras](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationtrustedpairing.html)
    IS_CONFIG_CMD_TRUSTED_PAIRING_SET                      = 15,

    /// Returns the current settings for the trusted pairing mode.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_TRUSTED_PAIRING`]
    ///
    /// # Documentation
    /// [Trusted pairing mode for GigE uEye cameras](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationtrustedpairing.html)
    IS_CONFIG_CMD_TRUSTED_PAIRING_GET                      = 16,

    /// Returns the default settings for the trusted pairing mode.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_TRUSTED_PAIRING`]
    ///
    /// # Documentation
    /// [Trusted pairing mode for GigE uEye cameras](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationtrustedpairing.html)
    IS_CONFIG_CMD_TRUSTED_PAIRING_GET_DEFAULT              = 17,

    /// (**reserved**)
    IS_CONFIG_CMD_RESERVED_1                               = 18,

    /// Changes the settings of the image memory compatibility mode.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_IMAGE_MEMORY_COMPATIBILITY_MODE`]
    ///
    /// # Documentation
    /// [Image memory compatibility mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationimagememory.html)
    IS_CONFIG_CMD_SET_IMAGE_MEMORY_COMPATIBILIY_MODE         = 19,

    /// Returns the settings of the image memory compatibility mode.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_IMAGE_MEMORY_COMPATIBILITY_MODE`]
    ///
    /// # Documentation
    /// [Image memory compatibility mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationimagememory.html)
    IS_CONFIG_CMD_GET_IMAGE_MEMORY_COMPATIBILIY_MODE         = 20,

    /// Returns the standard settings of the image memory compatibility mode.
    ///
    /// # Parameter type
    /// [`CONFIGURATION_SEL_IMAGE_MEMORY_COMPATIBILITY_MODE`]
    ///
    /// # Documentation
    /// [Image memory compatibility mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationimagememory.html)
    IS_CONFIG_CMD_GET_IMAGE_MEMORY_COMPATIBILIY_MODE_DEFAULT = 21,

    /// Reload the IP address of the network adapter.
    ///
    /// # Parameter type
    /// [`NULL`]
    ///
    /// # Documentation
    /// [IP address of the network adapter](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationipaddressnetwork.html)
    #[cfg(target_os = "windows")]
    IS_CONFIG_CMD_UPDATE_TCPIP_SETUP = 22
}

/// Enumeration of configuration command capability flags for [`is_Configuration`].
///
/// # Related commands
/// * [`CONFIGURATION_CMD::IS_CONFIG_CMD_GET_CAPABILITIES`]
///
/// # Documentation
/// [`is_Configuration`: Contents of the `CONFIGURATION_CAPS` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configuration.html#configuration_caps)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum CONFIGURATION_CAPS {
    /// Function parameters for setting the processor operating states are supported.
    IS_CONFIG_CPU_IDLE_STATES_CAP_SUPPORTED                = 0x00000001,

    /// Function parameters to configure OpenMP are supported.
    IS_CONFIG_OPEN_MP_CAP_SUPPORTED                        = 0x00000002,

    /// Function parameters to load camera parameters during initialization are supported.
    IS_CONFIG_INITIAL_PARAMETERSET_CAP_SUPPORTED           = 0x00000004,

    /// Function parameters for setting the IPO thread are supported.
    IS_CONFIG_IPO_CAP_SUPPORTED                            = 0x00000008,

    /// Function parameters for setting the trusted pairing mode are supported.
    IS_CONFIG_TRUSTED_PAIRING_CAP_SUPPORTED                = 0x00000010
}

unsafe extern "C" {


    /// Command to set general configuration parameters (e.g. the CPU idle state settings).
    ///
    /// System-wide options:
    /// * [Processor operating states under Windows](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationidlestates.html)
    /// * [IP address of the network adapter](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationipaddressnetwork.html)
    /// * [Allowing IPO thread](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationipothread.html)
    /// * [Image memory compatibility mode](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationimagememory.html)
    /// * [Activating OpenMP](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationopenmp.html)
    /// * [Loading camera parameters during initializing](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationcameraparameter.html)
    /// * [Trusted pairing mode for GigE uEye cameras](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationtrustedpairing.html)
    /// * [Configuration mode under Linux](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_configurationlinuxconfigmode.html)
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`CONFIGURATION_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_CANT_OPEN_REGISTRY`]
    /// * [`IS_CANT_READ_REGISTRY`]
    /// * [`IS_ERROR_CPU_IDLE_STATES_CONFIGURATION`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_IMAGE_MEM_ALLOCATED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_OPERATING_SYSTEM_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    pub fn is_Configuration(nCommand: CONFIGURATION_CMD, pParam: *mut void, cbSizeOfParam: UINT) -> INT;

}
