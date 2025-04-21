#![allow(non_camel_case_types)]

use crate::types::{HIDS, INT, UINT, void, BOOL, NULL, wchar_t};

/// Enumeration of commands of function [`is_ParameterSet`].
///
/// # Documentation
/// [`is_ParameterSet`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_parameterset.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum PARAMETERSET_CMD {
    /// Loads a camera parameter set from the user memory.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_PARAMETERSET_CMD_LOAD_EEPROM                         = 1,

    /// Loads a camera parameter set from a file.
    ///
    /// You must pass the path to the ini file as Unicode string.
    /// You can pass either a relative or an absolute path.
    ///
    /// _Windows only:_ If you pass [`NULL`] the "Open file" dialog opens.
    ///
    /// # Parameter type
    /// * _Windows only:_ [`NULL`] (_"Open file" dialog_)
    /// * [`wchar_t`]
    IS_PARAMETERSET_CMD_LOAD_FILE                           = 2,

    /// Saves a camera parameter set in the user memory.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_PARAMETERSET_CMD_SAVE_EEPROM                         = 3,

    /// Saves a camera parameter set in a file.
    ///
    /// You must pass the path to the ini file as Unicode string.
    /// You can pass either a relative or an absolute path.
    ///
    /// _Windows only:_ If you pass [`NULL`] the "Save as" dialog opens.
    ///
    /// # Parameter type
    /// * _Windows only:_ [`NULL`] (_"Save as" dialog_)
    /// * [`wchar_t`]
    IS_PARAMETERSET_CMD_SAVE_FILE                           = 4,

    /// Returns the number of supported parameter sets in the camera's user memory.
    ///
    /// At the moment this is `1` for all cameras.
    ///
    /// # Parameter type
    /// [`UINT`]
    IS_PARAMETERSET_CMD_GET_NUMBER_SUPPORTED                = 5,

    /// Returns if a camera parameter set in the user memory is supported.
    ///
    /// # Parameter type
    /// [`BOOL`]
    IS_PARAMETERSET_CMD_GET_HW_PARAMETERSET_AVAILABLE       = 6,

    /// Deletes the camera parameter set in the user memory.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_PARAMETERSET_CMD_ERASE_HW_PARAMETERSET               = 7
}

unsafe extern "C" {
    /// Saves the current camera parameters to a file or to the user memory of the camera and
    /// loads the parameter set from a file or the user memory.
    ///
    /// Note that the following settings are only saved in the parameter file and
    /// not in the user memory of the camera:
    /// * Long exposure settings
    /// * Color mode settings
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`PARAMETERSET_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INCOMPATIBLE_SETTING`]
    /// * [`IS_INVALID_CAMERA_TYPE`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_CameraStatus`]
    ///
    /// # Documentation
    /// [`is_ParameterSet`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_parameterset.html)
    pub fn is_ParameterSet(hCam: HIDS, nCommand: PARAMETERSET_CMD, pParam: *mut void, cbSizeOfParam: UINT) -> INT;
}
