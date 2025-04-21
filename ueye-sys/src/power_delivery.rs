//! Query which USB Power Delivery profiles are supported by the camera in combination with the
//! host PC and set the appropriate profile.
//!
//! USB Power Delivery (PD) expands the USB system with a very flexible power supply management
//! which is implemented in addition to the data connection via the same cable. The
//! distribution of roles in power supplier and consumer with limited voltage and current thus
//! no longer exists. PD-capable devices negotiate their capabilities and requirements
//! concerning supply voltage over a standardized protocol.
//!
//! _IDS Imaging Development Systems GmbH_ introduces USB Power Delivery for the first time
//! with the camera families with USB Type-C connectors. The PD-capable camera requests as a PD
//! consumer more power than it needs for itself. The additional voltages are used, for example
//! to power an LED light via the camera's I/O connector.
//!
//! The prerequisite is that the host PC also supports USB Power Delivery and correspondingly
//! provides more power. Note that power transmission via USB Power Delivery is only possible
//! with "Full Featured" USB Type-C to Type-C cables.
//!
//! Note that the profile has to be set again after a reconnect. Switching between power
//! delivery profiles should be performed before image acquisition starts. If image acquisition
//! is already started, it is stopped briefly when switching.
//!
//! Make sure that the connected peripheral devices does not receive any power when switching
//! between different power delivery profiles. If the power consumption of the peripheral
//! device is too high during switching, the host may turn off the cameras. This will restart
//! the camera with the fallback profile.
//!
//! # Documentation
//! [is_PowerDelivery](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_powerdelivery.html)

#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{void, BOOL, HIDS, INT, UINT};
use bitflags::bitflags;

/// Enumeration of commands for [`is_PowerDelivery`].
///
/// # Documentation
/// [is_PowerDelivery](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_powerdelivery.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum POWER_DELIVERY_CMD {
    /// Returns if power delivery is supported by the camera.
    ///
    /// # Parameter type
    /// [`BOOL`]
    IS_POWER_DELIVERY_CMD_GET_SUPPORTED = 1,

    /// Returns the current set power delivery profile.
    ///
    /// # Parameter type
    /// [`IS_POWER_DELIVERY_PROFILES`]
    IS_POWER_DELIVERY_CMD_GET_PROFILE = 2,

    /// Returns all power delivery profiles which are supported by the host in combination
    /// with the camera.
    ///
    /// # Parameter type
    /// [`IS_POWER_DELIVERY_PROFILES`], _bitmask_
    IS_POWER_DELIVERY_CMD_GET_SUPPORTED_PROFILES = 3,

    /// Sets a power delivery profile.
    ///
    /// # Parameter type
    /// [`IS_POWER_DELIVERY_PROFILES`]
    IS_POWER_DELIVERY_CMD_SET_PROFILE = 4,
}

bitflags! {
    /// Supported voltages for power delivery (_supports bitmask_).
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct IS_POWER_DELIVERY_PROFILES: UINT {
        /// No valid power delivery profile.
        const IS_POWER_DELIVERY_PROFILE_INVALID       = 0x00000000;

        /// 5V without additional power supply for peripheral devices (fallback profile).
        const IS_POWER_DELIVERY_PROFILE_5V_LOW_POWER  = 0x00000001;

        /// 5V with 1A for peripheral devices.
        const IS_POWER_DELIVERY_PROFILE_5V_HIGH_POWER = 0x00000002;

        /// 9V with 1A for peripheral devices.
        const IS_POWER_DELIVERY_PROFILE_9V            = 0x00000004;

        /// 12V with 1A for peripheral devices.
        const IS_POWER_DELIVERY_PROFILE_12V           = 0x00000008;

        /// 14.8V with 1A for peripheral devices.
        const IS_POWER_DELIVERY_PROFILE_14V8          = 0x00000010;

        /// 15V with 1A for peripheral devices.
        const IS_POWER_DELIVERY_PROFILE_15V           = 0x00000020;

        const _ = !0;
    }
}

impl Default for IS_POWER_DELIVERY_PROFILES {
    #[inline]
    fn default() -> Self {
        Self::IS_POWER_DELIVERY_PROFILE_5V_LOW_POWER
    }
}

impl IS_POWER_DELIVERY_PROFILES {
    pub const fn voltage(&self) -> f32 {
        match *self {
            IS_POWER_DELIVERY_PROFILES::IS_POWER_DELIVERY_PROFILE_INVALID | _ => 0.0,
            IS_POWER_DELIVERY_PROFILES::IS_POWER_DELIVERY_PROFILE_5V_LOW_POWER
            | IS_POWER_DELIVERY_PROFILES::IS_POWER_DELIVERY_PROFILE_5V_HIGH_POWER => 5.0,
            IS_POWER_DELIVERY_PROFILES::IS_POWER_DELIVERY_PROFILE_9V => 9.0,
            IS_POWER_DELIVERY_PROFILES::IS_POWER_DELIVERY_PROFILE_12V => 12.0,
            IS_POWER_DELIVERY_PROFILES::IS_POWER_DELIVERY_PROFILE_14V8 => 14.8,
            IS_POWER_DELIVERY_PROFILES::IS_POWER_DELIVERY_PROFILE_15V => 15.0,
        }
    }
}

unsafe extern "C" {
    /// Query which USB Power Delivery profiles are supported by the camera in combination with the
    /// host PC and set the appropriate profile.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`POWER_DELIVERY_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParams` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Documentation
    /// [is_PowerDelivery](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_powerdelivery.html)
    pub fn is_PowerDelivery(
        hCam: HIDS,
        nCommand: POWER_DELIVERY_CMD,
        pParam: *mut void,
        cbSizeOfParams: UINT,
    ) -> INT;
}
