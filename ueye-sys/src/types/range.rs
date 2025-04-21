//! Range types.

#![allow(non_camel_case_types)]

use crate::types::{double, INT, UINT};

/// Range with increments, [`UINT`].
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct IS_RANGE_U32 {
    /// Minimum.
    pub u32Min: UINT,

    /// Maximum.
    pub u32Max: UINT,

    /// Increment.
    pub u32Inc: UINT,
}

impl IS_RANGE_U32 {
    #[inline]
    pub const fn size(&self) -> UINT {
        self.u32Max - self.u32Min
    }
}

impl PartialOrd for IS_RANGE_U32 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size().partial_cmp(&other.size())
    }
}

impl Ord for IS_RANGE_U32 {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size().cmp(&other.size())
    }
}

/// Range with increments, [`INT`].
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct IS_RANGE_S32 {
    /// Minimum.
    pub s32Min: INT,

    /// Maximum.
    pub s32Max: INT,

    /// Increment.
    pub s32Inc: INT,
}

impl IS_RANGE_S32 {
    #[inline]
    pub const fn size(&self) -> INT {
        self.s32Max - self.s32Min
    }
}

impl PartialOrd for IS_RANGE_S32 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size().partial_cmp(&other.size())
    }
}

impl Ord for IS_RANGE_S32 {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size().cmp(&other.size())
    }
}

/// Range with increments, [`double`].
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct IS_RANGE_F64 {
    pub f64Min: double,
    pub f64Max: double,
    pub f64Inc: double,
}

impl IS_RANGE_F64 {
    #[inline]
    pub fn size(&self) -> double {
        self.f64Max - self.f64Min
    }
}

impl PartialEq for IS_RANGE_F64 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.f64Min - other.f64Min).abs() <= double::EPSILON
            && (self.f64Max - other.f64Max).abs() <= double::EPSILON
            && (self.f64Inc - other.f64Inc).abs() <= double::EPSILON
    }
}

impl Eq for IS_RANGE_F64 {}

impl PartialOrd for IS_RANGE_F64 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size().partial_cmp(&other.size())
    }
}

impl Ord for IS_RANGE_F64 {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size().cmp(&other.size())
    }
}

/// Range of values (w/ _default_ and _infinite_), [`UINT`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RANGE_OF_VALUES_U32 {
    /// Minimum value.
    pub u32Minimum: UINT,

    /// Maximum value, not considered Infinite.
    pub u32Maximum: UINT,

    /// Increment value.
    pub u32Increment: UINT,

    /// Default value.
    pub u32Default: UINT,

    /// Infinite code, where applicable.
    pub u32Infinite: UINT,
}
