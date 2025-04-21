

/// Image file types.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IMG {
    IS_IMG_BMP                         = 0,
    IS_IMG_JPG                         = 1,
    IS_IMG_PNG                         = 2,
    IS_IMG_RAW                         = 4,
    IS_IMG_TIF                         = 8
}
