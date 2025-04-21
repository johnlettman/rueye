
#![allow(non_camel_case_types)]

use std::cmp::Ordering;
use bitflags::bitflags;
use crate::types::{INT, IS_RECT, UINT, void};

bitflags! {
    /// Focus capability flags (_supports bitmask_).
    ///
    /// # Documentation
    /// [is_Focus: Status flags from FOCUS_CAPABILITY_FLAGS](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_focus.html#focus_capability_flags)
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct FOCUS_CAPABILITY_FLAGS: UINT {
        /// The camera does not support focus settings.
        const FOC_CAP_INVALID = 0;

        /// The camera supports autofocus.
        const FOC_CAP_AUTOFOCUS_SUPPORTED = 0x00000001;

        /// The camera supports manual focus.
        const FOC_CAP_MANUAL_SUPPORTED = 0x00000002;

        /// The camera supports querying the focal distance.
        const FOC_CAP_GET_DISTANCE = 0x00000004;

        /// The camera supports the selection of the autofocus range.
        const FOC_CAP_SET_AUTOFOCUS_RANGE = 0x00000008;


        const FOC_CAP_AUTOFOCUS_FDT_AOI = 0x00000010;

        /// The camera supports the use of the focus measure window.
        const FOC_CAP_AUTOFOCUS_ZONE = 0x00000020;

        /// The camera supports use of the sharpness algorithm for autofocus.
        const FOC_CAP_AUTOFOCUS_SHARPNESS_CALCULATION_ALGORITHM = 0x00000040;

        /// The camera supports use of the peak search algorithm for triggered autofocus.
        const FOC_CAP_AUTOFOCUS_ONCE_PEAK_SEARCH_ALGORITHM = 0x00000080;

        /// The camera supports the use of focus AOI.
        const FOC_CAP_AUTOFOCUS_AOI = 0x00000100;

        /// The camera supports the use of the minimum and maximum limits for the focus range of
        /// the peak search algorithm.
        const FOC_CAP_AUTOFOCUS_LIMIT = 0x00000200;

        /// The camera supports use of lens response time (positioning time).
        const FOC_CAP_AUTOFOCUS_LENS_RESPONSE_TIME = 0x00000400;

        /// The camera supports the use of hysteresis.
        const FOC_CAP_AUTOFOCUS_HYSTERESIS = 0x00000800;

        /// The camera supports the use of a callback function
        const FOC_CAP_AUTOFOCUS_CALLBACK = 0x00001000;
    }
}

bitflags! {
    /// Focus range (_supports bitmask_).
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct FOCUS_RANGE: UINT {
        /// Normal focus range (without macro).
        const FOC_RANGE_NORMAL = 0x00000001;

        /// All-range (macro to infinity).
        const FOC_RANGE_ALLRANGE = 0x00000002;

        /// Macro (only macro).
        const FOC_RANGE_MACRO = 0x00000004;
    }
}

bitflags! {
    /// Focus range (_supports bitmask_).
    ///
    /// # Documentation
    /// [is_Focus](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_focus.html)
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct FOCUS_STATUS: UINT {
        /// Initial state when auto focus is active.
        const FOC_STATUS_UNDEFINED = 0x00000000;

        /// Focus error has occurred.
        ///
        /// _For uEye LE USB 3.1 Gen 1 AF:_ returned if no "sharpness peak" can be found within a
        /// certain number of iterations.
        const FOC_STATUS_ERROR = 0x00000001;

        /// Lens is focused.
        const FOC_STATUS_FOCUSED = 0x00000002;

        /// Lens is currently focused.
        const FOC_STATUS_FOCUSING = 0x00000004;

        /// A timeout has occurred.
        const FOC_STATUS_TIMEOUT = 0x00000008;

        /// Focusing was terminated.
        ///
        /// _For uEye LE USB 3.1 Gen 1 AF:_ Returned if the automatic autofocus control is
        /// terminated using
        /// [`FOC_CMD_SET_DISABLE_AUTOFOCUS`][IS_FOC_CMD::FOC_CMD_SET_DISABLE_AUTOFOCUS].
        const FOC_STATUS_CANCEL = 0x00000010;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum FOCUS_ZONE_WEIGHT {
    FOC_ZONE_WEIGHT_DISABLE     = 0,
    FOC_ZONE_WEIGHT_WEAK        = 0x0021,
    FOC_ZONE_WEIGHT_MIDDLE      = 0x0032,
    FOC_ZONE_WEIGHT_STRONG      = 0x0042
}

/// Enumeration of presets for the focus measurement window (_supports bitmask_).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum FOCUS_ZONE_AOI_PRESET {
    FOC_ZONE_AOI_PRESET_CENTER          = 0,
    FOC_ZONE_AOI_PRESET_UPPER_LEFT      = 0x0001,
    FOC_ZONE_AOI_PRESET_BOTTOM_LEFT     = 0x0002,
    FOC_ZONE_AOI_PRESET_UPPER_RIGHT     = 0x0004,
    FOC_ZONE_AOI_PRESET_BOTTOM_RIGHT    = 0x0008,
    FOC_ZONE_AOI_PRESET_UPPER_CENTER    = 0x0010,
    FOC_ZONE_AOI_PRESET_BOTTOM_CENTER   = 0x0020,
    FOC_ZONE_AOI_PRESET_CENTER_LEFT     = 0x0040,
    FOC_ZONE_AOI_PRESET_CENTER_RIGHT    = 0x0080
}

bitflags! {
    /// Autofocus sharpness calculation algorithms (_supports bitmask_).
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct AUTOFOCUS_SHARPNESS_CALCULATION_ALGORITHM: UINT {
        const AUTOFOCUS_SHARPNESS_CALCULATION_ALGORITHM_TENENGRAD = 0x01;
        const AUTOFOCUS_SHARPNESS_CALCULATION_ALGORITHM_MEAN_SCORE = 0x02;
        const AUTOFOCUS_SHARPNESS_CALCULATION_ALGORITHM_HISTOGRAM_VARIANCE = 0x04;
    }
}

bitflags! {
    /// Autofocus once-peak search algorithm (_supports bitmask_).
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct AUTOFOCUS_ONCE_PEAK_SEARCH_ALGORITHM: UINT {
        const AUTOFOCUS_ONCE_PEAK_SEARCH_ALGORITHM_GOLDEN_RATIO_SEARCH = 0x01;
        const AUTOFOCUS_ONCE_PEAK_SEARCH_ALGORITHM_HILL_CLIMBING_SEARCH = 0x02;
        const AUTOFOCUS_ONCE_PEAK_SEARCH_ALGORITHM_GLOBAL_SEARCH = 0x04;
        const AUTOFOCUS_ONCE_PEAK_SEARCH_ALGORITHM_FULL_SCAN = 0x08;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum AUTOFOCUS_AOI_WEIGHT {
    AUTOFOCUS_AOI_WEIGHT_MIDDLE = 0x0042
}

/// Autofocus area of interest.
///
/// # Documentation
/// [Focus settings for uEye LE USB 3.1 Gen 1 AF: Contents of the AUTOFOCUS_AOI structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_focus_le_af.html#autofocus_aoi)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct AUTOFOCUS_AOI {
    /// Number of the focus measure window.
    pub uNumberAOI: UINT,

    /// Defines the upper left corner ([`s32X`][IS_RECT::s32X], [`s32Y`][IS_RECT::s32Y]) as well as
    /// the width ([`s32Width`][IS_RECT::s32Width]) and height ([`s32Height`][IS_RECT::s32Height])
    /// of the focus measure window.
    pub rcAOI: IS_RECT,

    /// Defines the weighting of the zone.
    pub eWeight: AUTOFOCUS_AOI_WEIGHT
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum AUTOFOCUS_AOI_PRESET {
    AUTOFOCUS_AOI_PRESET_CENTER = 0x01
}

/// Autofocus limit structure.
///
/// [Focus settings for uEye LE USB 3.1 Gen 1 AF: Contents of the AUTOFOCUS_LIMIT structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_focus_le_af.html#autofocus_limit)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct AUTOFOCUS_LIMIT {
    /// Defines the minimum limit of the focus search range for the peak search algorithm.
    sMin: INT,

    /// Defines the maximum limit of the focus search range for the peak search algorithm.
    sMax: INT
}

impl AUTOFOCUS_LIMIT {
    #[inline]
    pub const fn size(&self) -> INT {
        (self.sMax - self.sMin).abs()
    }
}

impl PartialOrd for AUTOFOCUS_LIMIT {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.size().partial_cmp(&other.size())
    }
}

/// Autofocus callback function type.
///
/// # Documentation
/// [Focus settings for uEye LE USB 3.1 Gen 1 AF: Contents of the AUTOFOCUS_CALLBACK structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_focus_le_af.html#autofocus_callback)
pub type IS_AUTOFOCUS_CALLBACK_FUNC = Option<unsafe extern "C" fn(UINT, INT, *mut void)>;

/// Example debug line-printing autofocus callback function.
unsafe extern "C" fn print_autofocus_callback(focus: UINT, sharpness: INT, context: *mut void) {
    println!("Autofocus callback triggered: focus={}, sharpness={}, context={:?}",
             focus, sharpness, context);
}

/// Autofocus callback structure.
///
/// # Documentation
/// [Focus settings for uEye LE USB 3.1 Gen 1 AF: Contents of the AUTOFOCUS_CALLBACK structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_focus_le_af.html#autofocus_callback)
pub struct AUTOFOCUS_CALLBACK {
    /// Callback function.
    pub pfFunc: IS_AUTOFOCUS_CALLBACK_FUNC,

    /// Context.
    pub pContext: *mut void
}

pub enum FOCUS_CMD {
    /// Returns the focus functions supported by the camera.
    ///
    /// # Parameter type
    /// [`FOCUS_CAPABILITY_FLAGS`], _bitmask_
    ///
    /// # Documentation
    /// [is_Focus](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_focus.html)
    FOC_CMD_GET_CAPABILITIES                                        = 0,

    /// Disables autofocus.
    ///
    /// # Parameter type
    /// [`NULL`]
    ///
    /// # Documentation
    /// [is_Focus](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_focus.html)
    FOC_CMD_SET_DISABLE_AUTOFOCUS                                   = 1,

    /// Enables autofocus.
    ///
    /// # Parameter type
    /// [`NULL`]
    ///
    /// # Documentation
    /// [is_Focus](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_focus.html)
    FOC_CMD_SET_ENABLE_AUTOFOCUS                                    = 2,

    /// Returns if the autofocus is enabled.
    ///
    /// # Parameter type
    /// [`BOOL`]
    ///
    /// # Possible values
    /// * [`FALSE`] = autofocus disabled.
    /// * [`TRUE`] = autofocus enabled.
    ///
    /// # Documentation
    /// [is_Focus](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_focus.html)
    FOC_CMD_GET_AUTOFOCUS_ENABLE                                    = 3,    /* Autofocus enabled?.                                                                  */
    FOC_CMD_SET_AUTOFOCUS_RANGE                                     = 4,    /* Preset autofocus range.                                                              */
    FOC_CMD_GET_AUTOFOCUS_RANGE                                     = 5,    /* Get preset of autofocus range.                                                       */
    FOC_CMD_GET_DISTANCE                                            = 6,    /* Get distance to focused object.                                                      */
    FOC_CMD_SET_MANUAL_FOCUS                                        = 7,    /* Set manual focus.                                                                    */
    FOC_CMD_GET_MANUAL_FOCUS                                        = 8,    /* Get the value for manual focus.                                                      */
    FOC_CMD_GET_MANUAL_FOCUS_MIN                                    = 9,    /* Get the minimum manual focus value.                                                  */
    FOC_CMD_GET_MANUAL_FOCUS_MAX                                    = 10,   /* Get the maximum manual focus value.                                                  */
    FOC_CMD_GET_MANUAL_FOCUS_INC                                    = 11,   /* Get the increment of the manual focus value.                                         */
    FOC_CMD_SET_ENABLE_AF_FDT_AOI                                   = 12,   /* Enable face detection AOI use for autofocus.                                         */
    FOC_CMD_SET_DISABLE_AF_FDT_AOI                                  = 13,   /* Disable face detection AOI use for autofocus                                         */
    FOC_CMD_GET_AF_FDT_AOI_ENABLE                                   = 14,   /* Use autofocus FDT AOI?                                                               */

    /// If the triggered autofocus/manual focus is active, it is automatically triggered once and
    /// then the event [`IS_SET_EVENT_AUTOFOCUS_FINISHED`] is set.
    ///
    /// # Parameter type
    /// [`NULL`]
    ///
    /// # Documentation
    /// [is_Focus](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_focus.html)
    FOC_CMD_SET_ENABLE_AUTOFOCUS_ONCE                               = 15,
    FOC_CMD_GET_AUTOFOCUS_STATUS                                    = 16,   /* Get the autofocus status                                                             */
    FOC_CMD_SET_AUTOFOCUS_ZONE_AOI                                  = 17,   /* Set the focus measurement window                                                     */
    FOC_CMD_GET_AUTOFOCUS_ZONE_AOI                                  = 18,   /* Get the focus measurement window                                                     */
    FOC_CMD_GET_AUTOFOCUS_ZONE_AOI_DEFAULT                          = 19,   /* Get the default focus measurement window                                             */
    FOC_CMD_GET_AUTOFOCUS_ZONE_POS_MIN                              = 20,   /* Get the minimal position of the measurement window                                   */
    FOC_CMD_GET_AUTOFOCUS_ZONE_POS_MAX                              = 21,   /* Get the maximal position of the measurement window                                   */
    FOC_CMD_GET_AUTOFOCUS_ZONE_POS_INC                              = 22,   /* Get the incrementation for the positions of the measurement window                   */
    FOC_CMD_GET_AUTOFOCUS_ZONE_SIZE_MIN                             = 23,   /* Get the minimal size of the measurement window                                       */
    FOC_CMD_GET_AUTOFOCUS_ZONE_SIZE_MAX                             = 24,   /* Get the maxiaml size of the measurement window                                       */
    FOC_CMD_GET_AUTOFOCUS_ZONE_SIZE_INC                             = 25,   /* Get the incrementation for the size of the measurement window                        */
    FOC_CMD_SET_AUTOFOCUS_ZONE_WEIGHT                               = 26,   /* Set the weight for the different zones                                               */
    FOC_CMD_GET_AUTOFOCUS_ZONE_WEIGHT                               = 27,   /* Get the weight for the different zones                                               */
    FOC_CMD_GET_AUTOFOCUS_ZONE_WEIGHT_COUNT                         = 28,   /* Get the zone count                                                                   */
    FOC_CMD_GET_AUTOFOCUS_ZONE_WEIGHT_DEFAULT                       = 29,   /* Get the default weight for the different zones                                       */
    FOC_CMD_SET_AUTOFOCUS_ZONE_AOI_PRESET                           = 30,   /* Set the focus measurement window specified by a preset /see FOCUS_ZONE_AOI_PRESET    */
    FOC_CMD_GET_AUTOFOCUS_ZONE_AOI_PRESET                           = 31,   /* Get the focus measurement window specified by a preset                               */
    FOC_CMD_GET_AUTOFOCUS_ZONE_AOI_PRESET_DEFAULT                   = 32,   /* Get the default focus measurement window                                             */
    FOC_CMD_GET_AUTOFOCUS_ZONE_ARBITRARY_AOI_SUPPORTED              = 33,   /* Returns if an arbritrary focus measurement window is supported                       */
    FOC_CMD_SET_MANUAL_FOCUS_RELATIVE                               = 34,   /* Set manual focus relative.                                                           */
    FOC_CMD_GET_AUTOFOCUS_SUPPORTED_SHARPNESS_CALCULATION_ALGORITHM = 35,   /* Get autofocus supported sharpness calculation algorithm                              */
    FOC_CMD_SET_AUTOFOCUS_SHARPNESS_CALCULATION_ALGORITHM           = 36,   /* Set autofocus sharpness calculation algorithm                                        */
    FOC_CMD_GET_AUTOFOCUS_SHARPNESS_CALCULATION_ALGORITHM           = 37,   /* Get autofocus sharpness calculation algorithm                                        */
    FOC_CMD_GET_AUTOFOCUS_SHARPNESS_CALCULATION_ALGORITHM_DEFAULT   = 38,   /* Get autofocus default sharpness calculation algorithm                                */
    FOC_CMD_GET_AUTOFOCUS_ONCE_SUPPORTED_PEAK_SEARCH_ALGORITHM      = 39,   /* Get autofocus once supported peak search algorithm                                   */
    FOC_CMD_SET_AUTOFOCUS_ONCE_PEAK_SEARCH_ALGORITHM                = 40,   /* Set autofocus once peak search algorithm                                             */
    FOC_CMD_GET_AUTOFOCUS_ONCE_PEAK_SEARCH_ALGORITHM                = 41,   /* Get autofocus once peak search algorithm                                             */
    FOC_CMD_GET_AUTOFOCUS_ONCE_PEAK_SEARCH_ALGORITHM_DEFAULT        = 42,   /* Get autofocus once default peak search algorithm                                     */
    FOC_CMD_GET_AUTOFOCUS_NUMBER_OF_SUPPORTED_AOIS                  = 43,   /* Get autofocus number of supported measurement windows                                */
    FOC_CMD_SET_AUTOFOCUS_AOI                                       = 44,   /* Set autofocus measurement window                                                     */
    FOC_CMD_GET_AUTOFOCUS_AOI                                       = 45,   /* Get autofocus measurement window                                                     */
    FOC_CMD_GET_AUTOFOCUS_AOI_SIZE_MIN                              = 47,   /* Get the minimal size of the measurement window                                       */
    FOC_CMD_SET_AUTOFOCUS_AOI_PRESET                                = 48,   /* Set the autofocus measurement window specified by a preset                           */
    FOC_CMD_SET_AUTOFOCUS_LIMIT                                     = 49,   /* Set autofocus limit.                                                                 */
    FOC_CMD_GET_AUTOFOCUS_LIMIT                                     = 50,   /* Get autofocus limit.                                                                 */
    FOC_CMD_GET_AUTOFOCUS_LIMIT_DEFAULT                             = 51,   /* Get autofocus default                                                                */
    FOC_CMD_SET_AUTOFOCUS_LENS_RESPONSE_TIME                        = 52,   /* Set autofocus lens response time                                                     */
    FOC_CMD_GET_AUTOFOCUS_LENS_RESPONSE_TIME                        = 53,   /* Get autofocus lens reponse time                                                      */
    FOC_CMD_GET_AUTOFOCUS_LENS_RESPONSE_TIME_DEFAULT                = 54,   /* Get autofocus default lens reponse time                                              */
    FOC_CMD_SET_AUTOFOCUS_HYSTERESIS                                = 55,   /* Set autofocus hysteresis                                                             */
    FOC_CMD_GET_AUTOFOCUS_HYSTERESIS                                = 56,   /* Get autofocus hysteresis                                                             */
    FOC_CMD_GET_AUTOFOCUS_HYSTERESIS_DEFAULT                        = 57,   /* Get autofocus default hysteresis                                                     */
    FOC_CMD_SET_AUTOFOCUS_CALLBACK                                  = 58    /* Set autofocus callback                                                               */



}
