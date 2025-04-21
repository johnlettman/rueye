#[allow(non_camel_case_types, non_snake_case)]
// Standard C types
pub use std::ffi::{
    c_char as char, c_double as double, c_float as float, c_int as INT, c_long as long, c_longlong,
    c_uchar as BYTE, c_uint as UINT, c_ulong as ULONG, c_void as void,
};

pub const INFINITE_UINT: UINT = UINT::MAX;
pub const INFINITE_INT: INT = -1;

macro_rules! HIBYTE {
    ($x:expr) => {
        (($x >> 8) & 0xFF) as BYTE
    };
}

macro_rules! LOBYTE {
    ($x:expr) => {
        ($x & 0xFF) as BYTE
    };
}

macro_rules! HIWORD {
    ($x:expr) => {
        ($x >> 16) as WORD
    };
}

macro_rules! LOWORD {
    ($x:expr) => {
        ($x & 0xFFFF) as WORD
    };
}

// Extensions to C types
pub type DWORD = u32;
pub type WORD = u16;

pub type CHAR = char;
pub type STRING = *mut char;

pub type BOOL = u32;
pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;

pub type HWND = *mut void;

pub type HDC = *mut void;

#[cfg(target_os = "windows")]
pub type wchar_t = u16;

#[cfg(not(target_os = "windows"))]
pub type wchar_t = u32;

pub type LPCWSTR = *const wchar_t;
pub type WSTRING = *const wchar_t;

// Constants to C types
pub const NULL: *mut void = std::ptr::null_mut();

// uEye-declared discrete types
pub type HIDS = DWORD;
pub type HCAM = DWORD;
pub type HFALC = DWORD;

pub const IS_INVALID_HIDS: HIDS = 0;
pub const IS_INVALID_HCAM: HCAM = 0;
pub const IS_INVALID_HFALC: HFALC = 0;

// Helper functions for C types
/// Converts a Rust `&str` to a `Vec<wchar_t>` suitable for passing to FFI (null-terminated).
pub fn to_wide(s: &str) -> Vec<wchar_t> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::ffi::OsStrExt;
        std::ffi::OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
    }
    #[cfg(not(target_os = "windows"))]
    {
        s.chars().map(|c| c as wchar_t).chain(std::iter::once(0)).collect()
    }
}
