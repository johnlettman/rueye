//! Common functions for metadata about the system.

#![allow(non_camel_case_types)]

use crate::types::INT;

/// Enumeration of Operating Systems returned by [`is_GetOsVersion`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum IS_OS {
    /// Undetermined Operating System.
    IS_OS_UNDETERMINED = 0,

    /// Windows 95.
    IS_OS_WIN95 = 1,

    /// Windows NT 4.0.
    IS_OS_WINNT40 = 2,

    /// Windows 98.
    IS_OS_WIN98 = 3,

    /// Windows 2000.
    IS_OS_WIN2000 = 4,

    /// Windows XP.
    IS_OS_WINXP = 5,

    /// Windows Millennium Edition.
    IS_OS_WINME = 6,
    IS_OS_WINNET = 7,

    /// Windows Server 2003.
    IS_OS_WINSERVER2003 = 8,

    /// Windows Vista.
    IS_OS_WINVISTA = 9,

    /// Linux 2.4.
    IS_OS_LINUX24 = 10,

    /// Linux 2.6.
    IS_OS_LINUX26 = 11,

    /// Windows 7.
    IS_OS_WIN7 = 12,

    /// Windows 8.
    IS_OS_WIN8 = 13,

    /// Windows 8 Server.
    IS_OS_WIN8SERVER = 14,

    /// Greater than Windows 8.
    IS_OS_GREATER_THAN_WIN8 = 15,
}

unsafe extern "C" {

    /// Returns the version of the `ueye_api.dll`.
    ///
    /// # Return values
    /// The return value contains the version number which is coded as follows:
    /// * Bits `31`…`24`: Major version
    /// * Bits `23`…`16`: Minor version
    /// * Bits `15`…`0`: Build version
    ///
    /// # Example
    /// ```rust
    /// use crate::ueye_sys::types::INT;
    ///
    /// const VERSION: INT = unsafe { crate::ueye_sys::meta::is_GetDLLVersion() };
    /// const BUILD: INT = VERSION & 0xFFFF;
    /// const MINOR: INT = (VERSION >> 16) & 0xFF;
    /// const MAJOR: INT = (VERSION >> 24) & 0xFF;
    /// println!("API version {MAJOR}.{MINOR}.{BUILD}");
    /// ```
    ///
    /// # Documentation
    /// [is_GetDLLVersion](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_getdllversion.html)
    pub fn is_GetDLLVersion() -> INT;

    /// **Obsolete:** Returns the type of operating system, which is currently running on the
    /// machine in which the camera is installed.
    ///
    /// # Return values
    /// * [`IS_OS`]
    #[deprecated]
    pub fn is_GetOsVersion() -> IS_OS;

}

/// Split the version number from [`is_GetDLLVersion`] into its components.
///
/// # Examples
/// ```rust
/// use ueye_sys::meta::split_version;
/// use ueye_sys::types::INT;
///
/// // Example version number from `is_GetDLLVersion`.
/// let version: INT = 73404305;
///
/// // Split it!
/// let (major, minor, build) = split_version(version);
///
/// assert_eq!(major, 4);
/// assert_eq!(minor, 96);
/// assert_eq!(build, 3985);
/// ```
///
/// # Input parameters
/// * `version` - Version number from [`is_GetDLLVersion`].
///
/// # Return values
/// _Tuple of:_
/// * Major version
/// * Minor version
/// * Build version
#[inline]
pub const fn split_version(version: INT) -> (INT, INT, INT) {
    ((version >> 24) & 0xFF, (version >> 16) & 0xFF, version & 0xFFFF)
}

/// Stringify the version number from [`is_GetDLLVersion`].
///
/// # Examples
/// ```rust
/// use ueye_sys::meta::get_version_string;
/// use ueye_sys::types::INT;
///
/// // Example version number from `is_GetDLLVersion`.
/// let version: INT = 73404305;
///
/// // Split it!
/// let version_string = get_version_string(version);
///
/// assert_eq!(version_string, "4.96.3985");
/// ```
///
/// # Input parameters
/// * `version` - Version number from [`is_GetDLLVersion`].
///
/// # Return values
/// Version [`String`]
pub const fn get_version_string(version: INT) -> String {
    let (major, minor, build) = split_version(version);
    format!("{major}.{minor}.{build}")
}
