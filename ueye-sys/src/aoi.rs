#![allow(non_camel_case_types)]

use std::mem::MaybeUninit;
use crate::types::{double, BYTE, INT, UINT};
use bitflags::bitflags;

#[derive(Debug, Clone, Copy,  PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_AOI_CMD {
    IS_AOI_IMAGE_SET_AOI = 0x0001,
    IS_AOI_IMAGE_GET_AOI = 0x0002,
    IS_AOI_IMAGE_SET_POS = 0x0003,
    IS_AOI_IMAGE_GET_POS = 0x0004,
    IS_AOI_IMAGE_SET_SIZE = 0x0005,
    IS_AOI_IMAGE_GET_SIZE = 0x0006,
    IS_AOI_IMAGE_GET_POS_MIN = 0x0007,
    IS_AOI_IMAGE_GET_SIZE_MIN = 0x0008,
    IS_AOI_IMAGE_GET_POS_MAX = 0x0009,
    IS_AOI_IMAGE_GET_SIZE_MAX = 0x0010,
    IS_AOI_IMAGE_GET_POS_INC = 0x0011,
    IS_AOI_IMAGE_GET_SIZE_INC = 0x0012,
    IS_AOI_IMAGE_GET_POS_X_ABS = 0x0013,
    IS_AOI_IMAGE_GET_POS_Y_ABS = 0x0014,
    IS_AOI_IMAGE_GET_ORIGINAL_AOI = 0x0015,

    IS_AOI_MULTI_GET_SUPPORTED_MODES = 0x0100,
    IS_AOI_MULTI_SET_AOI = 0x0200,
    IS_AOI_MULTI_GET_AOI = 0x0400,
    IS_AOI_MULTI_DISABLE_AOI = 0x0800,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_AOI_CMD_MODIFIERS {
    IS_AOI_MULTI_MODE_X_Y_AXES = 0x0001,
    IS_AOI_MULTI_MODE_Y_AXES = 0x0002,
    IS_AOI_MULTI_MODE_GET_MAX_NUMBER = 0x0003,
    IS_AOI_MULTI_MODE_GET_DEFAULT = 0x0004,
    IS_AOI_MULTI_MODE_ONLY_VERIFY_AOIS = 0x0005,
    IS_AOI_MULTI_MODE_GET_MINIMUM_SIZE = 0x0006,
    IS_AOI_MULTI_MODE_GET_ENABLED = 0x0007,
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct IS_AOI_MULTI_STATUS: UINT {
        const IS_AOI_MULTI_STATUS_SETBYUSER = 0x00000001;
        const IS_AOI_MULTI_STATUS_COMPLEMENT = 0x00000002;
        const IS_AOI_MULTI_STATUS_VALID = 0x00000004;
        const IS_AOI_MULTI_STATUS_CONFLICT = 0x00000008;
        const IS_AOI_MULTI_STATUS_ERROR = 0x00000010;
        const IS_AOI_MULTI_STATUS_UNUSED = 0x00000020;
    }
}

///
///
/// # Documentation
/// [Using the multi AOI function: Content of the `IS_MULTI_AOI_DESCRIPTOR` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_aoi_multiaoi.html?q=IS_MULTI_AOI_DESCRIPTOR#is_multi_aoi_descriptor)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct IS_MULTI_AOI_DESCRIPTOR {
    /// Start position of the AOI in X direction.
    ///
    /// _UI-336x/UI-536x and UI-337x/UI-537x only:_
    /// [`nPosX`][IS_MULTI_AOI_DESCRIPTOR::nPosX] = `0`.
    pub nPosX: UINT,

    /// Start position of the AOI in Y direction.
    pub nPosY: UINT,

    /// AOI width.
    ///
    /// _UI-336x/UI-536x and UI-337x/UI-537x only:_
    /// [`nWidth`][IS_MULTI_AOI_DESCRIPTOR::nWidth] = minimum multi AOI width
    /// (see [`IS_AOI_MULTI_GET_AOI`][IS_AOI_CMD::IS_AOI_MULTI_GET_AOI] with
    /// [`IS_AOI_MULTI_MODE_GET_MINIMUM_SIZE`][IS_AOI_CMD_MODIFIERS::IS_AOI_MULTI_MODE_GET_MINIMUM_SIZE]).
    pub nWidth: UINT,

    /// AOI height.
    pub nHeight: UINT,

    pub nStatus: IS_AOI_MULTI_STATUS,
}

///
/// # Documentation
/// [Using the multi AOI function: Content of the `IS_MULTI_AOI_CONTAINER` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_aoi_multiaoi.html?q=IS_MULTI_AOI_CONTAINER)
pub struct IS_MULTI_AOI_CONTAINER {
    /// Number of entries in [`pMultiAOIList`][IS_MULTI_AOI_CONTAINER::pMultiAOIList].
    pub nNumberOfAOIs: UINT,

    /// Data of the single AOIs.
    pub pMultiAOIList: *mut IS_MULTI_AOI_DESCRIPTOR,
}

/// Parameters of an AOI used in the AOI sequence mode.
///
#[derive(Debug)]
#[repr(C)]
pub struct AOI_SEQUENCE_PARAMS {
    pub s32AOIIndex: INT,
    pub s32NumberOfCycleRepetitions: INT,
    pub s32X: INT,
    pub s32Y: INT,
    pub dblExposure: double,
    pub s32Gain: INT,
    pub s32BinningMode: INT,
    pub s32SubsamplingMode: INT,
    pub s32DetachImageParameters: INT,
    pub dblScalerFactor: double,
    pub s32InUse: INT,

    byReserved: [BYTE; 60]
}

impl Clone for AOI_SEQUENCE_PARAMS {
    fn clone(&self) -> Self {
        // Unsafe allocate clone to avoid zeroing `byReserved`.
        let mut other = unsafe { MaybeUninit::<Self>::uninit().assume_init() };

        other.s32AOIIndex = self.s32AOIIndex;
        other.s32NumberOfCycleRepetitions = self.s32NumberOfCycleRepetitions;
        other.s32X = self.s32X;
        other.s32Y = self.s32Y;
        other.dblExposure = self.dblExposure;
        other.s32Gain = self.s32Gain;
        other.s32BinningMode = self.s32BinningMode;
        other.s32SubsamplingMode = self.s32SubsamplingMode;
        other.s32DetachImageParameters = self.s32DetachImageParameters;
        other.dblScalerFactor = self.dblScalerFactor;
        other.s32InUse = self.s32InUse;

        other
    }
}
