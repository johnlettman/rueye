use crate::types::UINT;

pub const IS_GET_LIVE: UINT = 0x8000;

/// The function waits.
pub const IS_WAIT: UINT = 0x0001;

/// The function returns immediately.
pub const IS_DONT_WAIT: UINT = 0x0000;

/// Digitizing is stopped immediately.
pub const IS_FORCE_VIDEO_STOP: UINT = 0x4000;
pub const IS_FORCE_VIDEO_START: UINT = 0x4000;
pub const IS_USE_NEXT_MEM: UINT = 0x8000;
