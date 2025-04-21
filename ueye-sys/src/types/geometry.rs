#![allow(non_camel_case_types)]

use crate::types::INT;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IS_POINT_2D {
    pub s32X: INT,
    pub s32Y: INT,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct IS_SIZE_2D {
    pub s32Width: INT,
    pub s32Height: INT,
}

impl IS_SIZE_2D {
    #[inline]
    pub fn area(&self) -> INT {
        self.s32Width * self.s32Height
    }
}

impl PartialOrd for IS_SIZE_2D {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

impl Ord for IS_SIZE_2D {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area().cmp(&other.area())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct IS_RECT {
    pub s32X: INT,
    pub s32Y: INT,
    pub s32Width: INT,
    pub s32Height: INT,
}

impl IS_RECT {
    #[inline]
    pub fn area(&self) -> INT {
        self.s32X * self.s32Y
    }
}

impl PartialOrd for IS_RECT {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

impl Ord for IS_RECT {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area().cmp(&other.area())
    }
}
