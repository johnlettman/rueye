//! Opens the camera at the system start and allows a faster access to the camera in the running
//! application (see [boot boost](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/cm_addfunc_boot-boost.html)).
//!
//! With version 4.50 new commands are added to the boot boost function so that the function can be
//! synchronously enabled or disabled. This means that the function only returns when a change was
//! performed for all affected cameras
//! (IS_BOOTBOOST_CMD_ENABLE_AND_WAIT and IS_BOOTBOOST_CMD_DISABLE_AND_WAIT).
//! Using the IS_BOOTBOOST_CMD_WAIT command allows to synchronize changes of the boot boost
//! configuration to an application.
//!
//! Note: The change of the camera ID only has an effect on the boot boost mode after reconnecting
//! the camera. If you add a camera to the boot boost list by changing the camera ID, the camera
//! must not be open.
//!
//! # Documentation
//! [is_BootBoost](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_bootboost.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{void, BYTE, DWORD, HIDS, INT, NULL, UINT};

pub type IS_BOOTBOOST_ID = BYTE;

/// Minimum valid ID.
pub const IS_BOOTBOOST_ID_MIN: IS_BOOTBOOST_ID = 1;

/// Maximum valid ID.
pub const IS_BOOTBOOST_ID_MAX: IS_BOOTBOOST_ID = 254;

/// Special value: no IDs.
pub const IS_BOOTBOOST_NONE: IS_BOOTBOOST_ID = 0;

/// Special value: all IDs.
pub const IS_BOOTBOOST_ID_ALL: IS_BOOTBOOST_ID = 255;

/// Default wait timeout for the boot boost 'wait operations':
/// * [`BOOTBOOST_CMD::IS_BOOTBOOST_CMD_ENABLE_AND_WAIT`]
/// * [`BOOTBOOST_CMD::IS_BOOTBOOST_CMD_DISABLE_AND_WAIT`]
/// * [`BOOTBOOST_CMD::IS_BOOTBOOST_CMD_WAIT`]
pub const IS_BOOTBOOST_DEFAULT_WAIT_TIMEOUT_SEC: UINT = 60;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IS_BOOTBOOST_IDLIST {
    /// The number of listed elements.
    pub u32NumberOfEntries: DWORD,

    /// The list.
    pub aList: [IS_BOOTBOOST_ID; 1],
}

/// Size of [`IS_BOOTBOOST_IDLIST`] header.
pub const IS_BOOTBOOST_IDLIST_HEADERSIZE: usize = size_of::<DWORD>();

/// Size of [`IS_BOOTBOOST_IDLIST`] element.
pub const IS_BOOTBOOST_IDLIST_ELEMENTSIZE: usize = size_of::<IS_BOOTBOOST_ID>();

/// Enumeration of commands for [`is_BootBoost`].
///
/// # Documentation
/// [is_BootBoost](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_bootboost.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum BOOTBOOST_CMD {
    /// Enable BootBoost.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_BOOTBOOST_CMD_ENABLE = 0x00010001,

    /// Enable BootBoost and wait until all relevant devices are boot-boosted.
    ///
    /// If the caller does not specify a timeout, the function will default to
    /// [`IS_BOOTBOOST_DEFAULT_WAIT_TIMEOUT_SEC`][BOOTBOOST_CMD::IS_BOOTBOOST_DEFAULT_WAIT_TIMEOUT_SEC].
    ///
    /// # Parameter type
    /// _Optional:_ [`UINT`] = timeout value in seconds
    IS_BOOTBOOST_CMD_ENABLE_AND_WAIT = 0x00010101,

    /// Disable BootBoost.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_BOOTBOOST_CMD_DISABLE = 0x00010011,

    /// Disable BootBoost and wait until all relevant devices are non-boot-boosted.
    ///
    /// If the caller does not specify a timeout, the function will default to
    /// [`IS_BOOTBOOST_DEFAULT_WAIT_TIMEOUT_SEC`][BOOTBOOST_CMD::IS_BOOTBOOST_DEFAULT_WAIT_TIMEOUT_SEC].
    ///
    /// # Parameter type
    /// _Optional:_ [`UINT`] = timeout value in seconds
    IS_BOOTBOOST_CMD_DISABLE_AND_WAIT = 0x00010111,

    /// Wait for all relevant devices to apply the configured boot-boost mode (enabled or disabled).
    ///
    /// If the caller does not specify a timeout, the function will default to
    /// [`IS_BOOTBOOST_DEFAULT_WAIT_TIMEOUT_SEC`][BOOTBOOST_CMD::IS_BOOTBOOST_DEFAULT_WAIT_TIMEOUT_SEC].
    ///
    /// # Parameter type
    /// _Optional:_ [`UINT`] = timeout value in seconds
    IS_BOOTBOOST_CMD_WAIT = 0x00010100,

    /// Query enabled state of BootBoost.
    ///
    /// # Parameter type
    /// [`DWORD`]
    IS_BOOTBOOST_CMD_GET_ENABLED = 0x20010021,

    /// Add an ID to the ID list.
    ///
    /// # Parameter type
    /// [`IS_BOOTBOOST_ID`]
    IS_BOOTBOOST_CMD_ADD_ID = 0x10100001,

    /// Replace ID list.
    ///
    /// # Parameter type
    /// [`IS_BOOTBOOST_IDLIST`]
    IS_BOOTBOOST_CMD_SET_IDLIST = 0x10100005,

    /// Remove an ID from the ID list.
    ///
    /// # Parameter type
    /// [`IS_BOOTBOOST_ID`]
    IS_BOOTBOOST_CMD_REMOVE_ID = 0x10100011,

    /// Clear ID list.
    ///
    /// # Parameter type
    /// [`NULL`]
    IS_BOOTBOOST_CMD_CLEAR_IDLIST = 0x00100015,

    /// Query ID list.
    ///
    /// # Parameter type
    /// [`IS_BOOTBOOST_IDLIST`]
    IS_BOOTBOOST_CMD_GET_IDLIST = 0x30100021,

    /// Query the number of listed IDs.
    ///
    /// # Parameter type
    /// [`DWORD`]
    IS_BOOTBOOST_CMD_GET_IDLIST_SIZE = 0x20100022,
}

unsafe extern "C" {
    /// Opens the camera at the system start and allows a faster access to the camera in the running
    /// application (see [boot boost](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/cm_addfunc_boot-boost.html)).
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`BOOTBOOST_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `nSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    /// * [`IS_TIMED_OUT`]
    ///
    /// # Documentation
    /// [is_BootBoost](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_bootboost.html)
    pub fn is_BootBoost(
        hCam: HIDS,
        nCommand: BOOTBOOST_CMD,
        pParam: *mut void,
        cbSizeOfParam: UINT,
    ) -> INT;
}
