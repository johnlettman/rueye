use crate::types::UINT;

pub const IS_SET_EVENT_ODD: UINT = 0;
pub const IS_SET_EVENT_EVEN: UINT = 1;

/// A new image is available.
pub const IS_SET_EVENT_FRAME: UINT = 2;

/// An image which was captured following the arrival of a trigger has been transferred completely.
///
/// This is the earliest possible moment for a new capturing process. The image must then be
/// post-processed by the driver and will be available after the [`IS_SET_EVENT_FRAME`]
/// processing event.
pub const IS_SET_EVENT_EXTTRIG: UINT = 3;
pub const IS_SET_EVENT_VSYNC: UINT = 4;

/// The sequence is completed (see [`is_AddToSequence`] /
/// [Event/message handling](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk_funktionsbloecke_event_handling.html#image-sequence)).
pub const IS_SET_EVENT_SEQ: UINT = 5;

/// An image extracted from the overlay is available (see [`is_DirectRenderer`]).
pub const IS_SET_EVENT_STEAL: UINT = 6;
pub const IS_SET_EVENT_VPRES: UINT = 7;

/// There is an information about image capturing available.
///
/// This information can be requested by [`is_CaptureStatus`].
/// Note that this event replaces the former [`IS_SET_EVENT_TRANSFER_FAILED`]
/// from previous versions.
pub const IS_SET_EVENT_CAPTURE_STATUS: UINT = 8;

/// Transfer failed. Replaced by [`IS_SET_EVENT_CAPTURE_STATUS`].
pub const IS_SET_EVENT_TRANSFER_FAILED: UINT = IS_SET_EVENT_CAPTURE_STATUS;

/// A camera initialized with [`is_InitCamera`] and disconnected afterwards was reconnected.
pub const IS_SET_EVENT_DEVICE_RECONNECTED: UINT = 9;
pub const IS_SET_EVENT_MEMORY_MODE_FINISH: UINT = 10;

/// This event is signaled when a new image has arrived in the API before the post-processing
/// is performed.
pub const IS_SET_EVENT_FRAME_RECEIVED: UINT = 11;

/// The automatic white balance control is completed (see [Automatic image control](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/hw_automatische_bildregelung.html#awb)).
pub const IS_SET_EVENT_WB_FINISHED: UINT = 12;

/// The automatic brightness control in the run-once mode is completed.
pub const IS_SET_EVENT_AUTOBRIGHTNESS_FINISHED: UINT = 13;

/// _Direct3D/OpenGL mode:_ Because of a re-programming the parameters of the overlay are invalid.
/// The overlay must be draw new.
pub const IS_SET_EVENT_OVERLAY_DATA_LOST: UINT = 16;

/// In the camera memory mode an image acquisition iteration is finished (see [`is_ImageBuffer`]).
pub const IS_SET_EVENT_CAMERA_MEMORY: UINT = 17;

/// The connection speed of a USB 3.x camera has been reduced to USB 2.0 speed or
/// has been increased.
pub const IS_SET_EVENT_CONNECTIONSPEED_CHANGED: UINT = 18;

/// Automatic focus control is finished (see [`is_Focus`]).
pub const IS_SET_EVENT_AUTOFOCUS_FINISHED: UINT = 19;

/// The first data packet of the image was transferred to the PC.
pub const IS_SET_EVENT_FIRST_PACKET_RECEIVED: UINT = 20;

/// _Multicast mode:_ The master has changed camera parameters which affect the data format like
/// image size or color format. The client must reconfigure the virtual camera.
pub const IS_SET_EVENT_PMC_IMAGE_PARAMS_CHANGED: UINT = 21;

/// An already paired GigE uEye camera was reconnected to the network.
///
/// Requirement: the camera is operated in trusted pairing mode (see [`is_Configuration`]).
pub const IS_SET_EVENT_DEVICE_PLUGGED_IN: UINT = 22;

/// A paired GigE uEye camera was disconnected from the network.
///
/// Requirement: the camera is operated in trusted pairing mode (see [`is_Configuration`]).
pub const IS_SET_EVENT_DEVICE_UNPLUGGED: UINT = 23;

/// _USB 3 uEye CP Rev. 2/GigE uEye CP Rev. 2/GigE uEye FA/GigE uEye SE Rev. 4 only:_
/// The temperature state of the camera has changed, see
/// [Querying the temperature state](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_devicefeature-status-temperature.html)
pub const IS_SET_EVENT_TEMPERATURE_STATUS: UINT = 24;

/// The exposure is finished.
///
/// This event is triggered after the sensor exposure time has expired. The time between the
/// exposure end and the software event signal is typically < 1 ms.
///
/// The event is triggered before the [`IS_SET_EVENT_FIRST_PACKET_RECEIVED`] event, as image
/// transfer to the PC is not yet complete when the [`IS_SET_EVENT_END_OF_EXPOSURE`] event is
/// transmitted.
///
/// The event has the restrictions that it is only supported in trigger mode and only by global
/// shutter models of the following camera families (see [`is_DeviceFeature`]):
/// * _GigE uEye CP Rev. 2_
/// * _GigE uEye FA_
/// * _GigE uEye SE Rev. 4_
/// * _USB 3 uEye CP Rev. 2_ (not for models with Sony sensors / _UI-3590CP Rev. 2_)
/// * _USB 3 uEye CP_
/// * _USB 3 uEye LE_
/// * _uEye LE USB 3.1 Gen 1_ (not for _UI-359xLE Rev. 2_)
/// * _uEye SE USB 3.1 Gen 1_ (not for models with Sony sensors)
pub const IS_SET_EVENT_END_OF_EXPOSURE: UINT = 25;

/// The event is signaled when an image was discarded (see [`is_CaptureConfiguration`]).
pub const IS_SET_EVENT_FRAME_SKIPPED: UINT = 26;

/// A camera initialized with [`is_InitCamera`] was removed.
pub const IS_SET_EVENT_REMOVE: UINT = 128;

/// A camera was removed.
///
/// This is independent of the device handle (`hCam` is ignored).
pub const IS_SET_EVENT_REMOVAL: UINT = 129;

/// A camera was newly connected.
///
/// This is independent of the device handle (`hCam` is ignored).
pub const IS_SET_EVENT_NEW_DEVICE: UINT = 130;

/// _Linux only:_ The availability of a camera has changed,
/// e.g. an available camera was opened.
///
/// This is independent of the device handle (`hCam` is ignored).
#[cfg(target_os = "linux")]
pub const IS_SET_EVENT_STATUS_CHANGED: UINT = 131;

/// A USB camera was removed.
///
/// This is independent of the device handle (`hCam` is ignored).
pub const IS_SET_EVENT_REMOVAL_USB: UINT = 133;

/// An USB camera was newly connected.
///
/// This is independent of the device handle (`hCam` is ignored).
pub const IS_SET_EVENT_NEW_DEVICE_USB: UINT = 134;

/// _Linux only:_ The availability of a USB camera has changed,
/// e.g. an available camera was opened.
///
/// This is independent of the device handle (`hCam` is ignored).
pub const IS_SET_EVENT_STATUS_CHANGED_USB: UINT = 135;

/// A GigE camera was removed.
///
/// This is independent of the device handle (`hCam` is ignored).
pub const IS_SET_EVENT_REMOVAL_ETH: UINT = 137;

/// A GigE camera was newly connected.
///
/// This is independent of the device handle (`hCam` is ignored).
pub const IS_SET_EVENT_NEW_DEVICE_ETH: UINT = 138;

/// _Linux only:_ The availability of a GigE camera has changed,
/// e.g. an available camera was opened.
///
/// This is independent of the device handle (`hCam` is ignored).
pub const IS_SET_EVENT_STATUS_CHANGED_ETH: UINT = 139;

/// Total number of user-defined events available.
pub const NUMBER_OF_USER_DEFINED_EVENTS: UINT = 200;

/// Start of user-defined events: these events are at the free disposal of the user
pub const IS_SET_EVENT_USER_DEFINED_BEGIN: UINT = 10000;

/// End of user-defined events: these events are at the free disposal of the user
pub const IS_SET_EVENT_USER_DEFINED_END: UINT = IS_SET_EVENT_USER_DEFINED_BEGIN + NUMBER_OF_USER_DEFINED_EVENTS;
