//! _Obsolete:_ Generic function to access memory.
// TODO: Locate documentation for `is_Memory`.

#![allow(non_camel_case_types, deprecated)]

use crate::types::{char, void, HIDS, INT, UINT};

#[deprecated]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum MEMORY_CMD {
    IS_MEMORY_GET_SIZE = 1,
    IS_MEMORY_READ = 2,
    IS_MEMORY_WRITE = 3,
}

#[deprecated]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum IS_MEMORY_DESCRIPTION {
    IS_MEMORY_USER_1 = 1,
    IS_MEMORY_USER_2 = 2,
}

#[deprecated]
#[derive(Debug, Clone)]
#[repr(C)]
pub struct IS_MEMORY_ACCESS {
    pub u32Description: IS_MEMORY_DESCRIPTION,
    pub u32Offset: UINT,
    pub pu8Data: *mut char,
    pub u32SizeOfData: UINT,
}

#[deprecated]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct IS_MEMORY_SIZE {
    pub u32Description: IS_MEMORY_DESCRIPTION,
    pub u32SizeBytes: UINT,
}

unsafe extern "C" {

    /// _Obsolete:_ Generic function to access memory.
    ///
    /// # Input parameters
    /// * `hf` - Camera handle.
    /// * `nCommand` - Command. See [`MEMORY_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    #[deprecated]
    pub fn is_Memory(hf: HIDS, nCommand: MEMORY_CMD, pParam: *mut void, cbSizeOfParam: UINT)
        -> INT;
}
