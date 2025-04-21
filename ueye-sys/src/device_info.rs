#![allow(non_camel_case_types)]

use crate::types::{BYTE, DWORD, HCAM, INT, UINT, WORD, void};

/// Definition of the uEye device info / heartbeat.
///
/// This data is periodically received from the device.
///
/// # Documentation
/// [Contents of the `IS_DEVICE_INFO::IS_DEVICE_INFO_HEARTBEAT` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_deviceinfo.html#is_device_info_heartbeat)
#[repr(C, packed(1))]
pub struct IS_DEVICE_INFO_HEARTBEAT {
    /// (**reserved**)
    reserved_1: [BYTE; 24],

    /// Runtime firmware version.
    pub dwRuntimeFirmwareVersion: DWORD,

    /// (**reserved**)
    reserved_2: [BYTE; 8],

    /// Camera temperature in °Celsius.
    ///
    /// If the value "-127.9 °C" is returned, the camera does not have a temperature sensor.
    ///
    /// # Layout
    /// * **Bit 15**: algebraic sign
    /// * **Bits 14…11**: filled according to algebraic sign
    /// * **Bits 10…4**: temperature (_places before the decimal point_)
    /// * **Bits 3…0**: temperature (_places after the decimal point_)
    pub wTemperature: WORD,

    /// Current link speed in Mbit/s.
    ///
    /// # See also
    /// * [`IS_USB_HIGH_SPEED`]
    /// * [`IS_USB_SUPER_SPEED`]
    pub wLinkSpeed_Mb: WORD,

    /// (**reserved**)
    reserved_3: [BYTE; 6],

    /// COM port offset from 100. Valid range: `-99`…`+156`.
    pub wComportOffset: WORD,
}

/// Definition of the uEye device info / control.
///
/// This data is provided by the uEye driver.
///
/// # Documentation
/// [Contents of the `IS_DEVICE_INFO::IS_DEVICE_INFO_CONTROL` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_deviceinfo.html#is_device_info_control)
#[repr(C, packed(1))]
pub struct IS_DEVICE_INFO_CONTROL {
    /// Device ID of the camera.
    dwDeviceId: DWORD,

    /// (**reserved**)
    reserved: [BYTE; 148],
}

/// Definition of the uEye device info.
///
/// This data is provided by the uEye driver.
///
/// # Related commands
/// * [`IS_DEVICE_INFO_CMD::IS_DEVICE_INFO_CMD_GET_DEVICE_INFO`]
///
/// # Documentation
/// [Contents of the `IS_DEVICE_INFO` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_deviceinfo.html#is_device_info)
#[repr(C, packed(1))]
pub struct IS_DEVICE_INFO {
    /// Camera-related data retrieved from the camera (from the heartbeat telegram).
    pub infoDevHeartbeat: IS_DEVICE_INFO_HEARTBEAT,

    /// Camera-related driver data.
    pub infoDevControl: IS_DEVICE_INFO_CONTROL,

    /// (**reserved**)
    reserved: [BYTE; 240],
}

/// Enumeration of commands supported by the device info access function [`is_DeviceInfo`].
///
/// # Documentation
/// [`is_DeviceInfo`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_deviceinfo.html)
pub enum IS_DEVICE_INFO_CMD {
    /// Returns an information structure about the specified device.
    ///
    /// # Parameter type
    /// [`IS_DEVICE_INFO`]
    IS_DEVICE_INFO_CMD_GET_DEVICE_INFO  = 0x02010001
}

unsafe extern "C" {
    pub fn is_DeviceInfo(hcam: HCAM, nCommand: IS_DEVICE_INFO_CMD, pParam: *mut void, cbSizeOfParam: UINT) -> INT;
}
