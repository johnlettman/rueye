//! **Obsolete:** Lens shading models.

#![allow(non_camel_case_types)]

use crate::types::UINT;
use bitflags::bitflags;

bitflags! {
    /// Lens shading models (_supports bitmask_).
    pub struct LENS_SHADING_MODELS: UINT {
        const LSC_MODEL_AGL = 0x0001;
        const LSC_MODEL_TL84 = 0x0002;
        const LSC_MODEL_D50 = 0x0004;
        const LSC_MODEL_D65 = 0x0008;
    }
}
