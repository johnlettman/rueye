//! Common function return values.

use crate::types::INT;

/// General error message.
pub const IS_NO_SUCCESS: INT = -1;

/// Function executed successfully.
pub const IS_SUCCESS: INT = 0;

/// Invalid camera handle.
///
/// Most of the uEye SDK functions expect the camera handle as the first parameter.
pub const IS_INVALID_CAMERA_HANDLE: INT = 1;

/// Invalid camera handle. See [`IS_INVALID_CAMERA_HANDLE`].
pub const IS_INVALID_HANDLE: INT = 1;

/// An IO request from the uEye driver failed.
///
/// Possibly the versions of the `ueye_api.dll` (API) and the driver file
/// (`ueye_usb.sys` or `ueye_eth.sys`) do not match.
pub const IS_IO_REQUEST_FAILED: INT = 2;

/// An attempt to initialize or select the camera failed
/// (no camera connected or initialization error).
pub const IS_CANT_OPEN_DEVICE: INT = 3;

#[deprecated]
pub const IS_CANT_CLOSE_DEVICE: INT = 4;

#[deprecated]
pub const IS_CANT_SETUP_MEMORY: INT = 5;

#[deprecated]
pub const IS_NO_HWND_FOR_ERROR_REPORT: INT = 6;

#[deprecated]
pub const IS_ERROR_MESSAGE_NOT_CREATED: INT = 7;

#[deprecated]
pub const IS_ERROR_STRING_NOT_FOUND: INT = 8;

#[deprecated]
pub const IS_HOOK_NOT_CREATED: INT = 9;

#[deprecated]
pub const IS_TIMER_NOT_CREATED: INT = 10;

/// Error opening a Windows registry key.
pub const IS_CANT_OPEN_REGISTRY: INT = 11;

/// Error reading settings from the Windows registry.
pub const IS_CANT_READ_REGISTRY: INT = 12;

#[deprecated]
pub const IS_CANT_VALIDATE_BOARD: INT = 13;

#[deprecated]
pub const IS_CANT_GIVE_BOARD_ACCESS: INT = 14;

/// The driver could not allocate memory.
pub const IS_NO_IMAGE_MEM_ALLOCATED: INT = 15;

/// The driver could not release the allocated memory.
pub const IS_CANT_CLEANUP_MEMORY: INT = 16;

/// Communication with the driver failed because no driver has been loaded.
pub const IS_CANT_COMMUNICATE_WITH_DRIVER: INT = 17;

/// The function is not supported yet.
pub const IS_FUNCTION_NOT_SUPPORTED_YET: INT = 18;

/// Operating system not supported.
pub const IS_OPERATING_SYSTEM_NOT_SUPPORTED: INT = 19;

pub const IS_INVALID_VIDEO_IN: INT = 20;
pub const IS_INVALID_IMG_SIZE: INT = 21;
pub const IS_INVALID_ADDRESS: INT = 22;
pub const IS_INVALID_VIDEO_MODE: INT = 23;
pub const IS_INVALID_AGC_MODE: INT = 24;
pub const IS_INVALID_GAMMA_MODE: INT = 25;
pub const IS_INVALID_SYNC_LEVEL: INT = 26;
pub const IS_INVALID_CBARS_MODE: INT = 27;
pub const IS_INVALID_COLOR_MODE: INT = 28;
pub const IS_INVALID_SCALE_FACTOR: INT = 29;

/// Invalid image size.
pub const IS_INVALID_IMAGE_SIZE: INT = 30;
pub const IS_INVALID_IMAGE_POS: INT = 31;

/// The function can not be executed in the current camera operating mode
/// (free run, trigger or standby).
pub const IS_INVALID_CAPTURE_MODE: INT = 32;
pub const IS_INVALID_RISC_PROGRAM: INT = 33;
pub const IS_INVALID_BRIGHTNESS: INT = 34;
pub const IS_INVALID_CONTRAST: INT = 35;
pub const IS_INVALID_SATURATION_U: INT = 36;
pub const IS_INVALID_SATURATION_V: INT = 37;
pub const IS_INVALID_HUE: INT = 38;
pub const IS_INVALID_HOR_FILTER_STEP: INT = 39;
pub const IS_INVALID_VERT_FILTER_STEP: INT = 40;
pub const IS_INVALID_EEPROM_READ_ADDRESS: INT = 41;
pub const IS_INVALID_EEPROM_WRITE_ADDRESS: INT = 42;
pub const IS_INVALID_EEPROM_READ_LENGTH: INT = 43;
pub const IS_INVALID_EEPROM_WRITE_LENGTH: INT = 44;
pub const IS_INVALID_BOARD_INFO_POINTER: INT = 45;
pub const IS_INVALID_DISPLAY_MODE: INT = 46;
pub const IS_INVALID_ERR_REP_MODE: INT = 47;
pub const IS_INVALID_BITS_PIXEL: INT = 48;

/// Invalid pointer or invalid memory ID.
pub const IS_INVALID_MEMORY_POINTER: INT = 49;

/// File cannot be opened for writing or reading.
pub const IS_FILE_WRITE_OPEN_ERROR: INT = 50;

/// The file cannot be opened.
pub const IS_FILE_READ_OPEN_ERROR: INT = 51;

/// The specified file is not a valid bitmap file.
pub const IS_FILE_READ_INVALID_BMP_ID: INT = 52;

/// The bitmap size is not correct (bitmap too large).
pub const IS_FILE_READ_INVALID_BMP_SIZE: INT = 53;
pub const IS_FILE_READ_INVALID_BIT_COUNT: INT = 54;
pub const IS_WRONG_KERNEL_VERSION: INT = 55;

pub const IS_RISC_INVALID_XLENGTH: INT = 60;
pub const IS_RISC_INVALID_YLENGTH: INT = 61;
pub const IS_RISC_EXCEED_IMG_SIZE: INT = 62;

// DirectDraw Mode errors
pub const IS_DD_MAIN_FAILED: INT = 70;
pub const IS_DD_PRIMSURFACE_FAILED: INT = 71;
pub const IS_DD_SCRN_SIZE_NOT_SUPPORTED: INT = 72;
pub const IS_DD_CLIPPER_FAILED: INT = 73;
pub const IS_DD_CLIPPER_HWND_FAILED: INT = 74;
pub const IS_DD_CLIPPER_CONNECT_FAILED: INT = 75;
pub const IS_DD_BACKSURFACE_FAILED: INT = 76;
pub const IS_DD_BACKSURFACE_IN_SYSMEM: INT = 77;
pub const IS_DD_MDL_MALLOC_ERR: INT = 78;
pub const IS_DD_MDL_SIZE_ERR: INT = 79;
pub const IS_DD_CLIP_NO_CHANGE: INT = 80;
pub const IS_DD_PRIMMEM_NULL: INT = 81;
pub const IS_DD_BACKMEM_NULL: INT = 82;
pub const IS_DD_BACKOVLMEM_NULL: INT = 83;
pub const IS_DD_OVERLAYSURFACE_FAILED: INT = 84;
pub const IS_DD_OVERLAYSURFACE_IN_SYSMEM: INT = 85;
pub const IS_DD_OVERLAY_NOT_ALLOWED: INT = 86;
pub const IS_DD_OVERLAY_COLKEY_ERR: INT = 87;
pub const IS_DD_OVERLAY_NOT_ENABLED: INT = 88;
pub const IS_DD_GET_DC_ERROR: INT = 89;
pub const IS_DD_DDRAW_DLL_NOT_LOADED: INT = 90;
pub const IS_DD_THREAD_NOT_CREATED: INT = 91;
pub const IS_DD_CANT_GET_CAPS: INT = 92;
pub const IS_DD_NO_OVERLAYSURFACE: INT = 93;
pub const IS_DD_NO_OVERLAYSTRETCH: INT = 94;
pub const IS_DD_CANT_CREATE_OVERLAYSURFACE: INT = 95;
pub const IS_DD_CANT_UPDATE_OVERLAYSURFACE: INT = 96;
pub const IS_DD_INVALID_STRETCH: INT = 97;

pub const IS_EV_INVALID_EVENT_NUMBER: INT = 100;
pub const IS_INVALID_MODE: INT = 101;
pub const IS_CANT_FIND_FALCHOOK: INT = 102;
pub const IS_CANT_FIND_HOOK: INT = 102;
pub const IS_CANT_GET_HOOK_PROC_ADDR: INT = 103;
pub const IS_CANT_CHAIN_HOOK_PROC: INT = 104;
pub const IS_CANT_SETUP_WND_PROC: INT = 105;
pub const IS_HWND_NULL: INT = 106;
pub const IS_INVALID_UPDATE_MODE: INT = 107;

/// No active image memory available.
///
/// You must set the memory to active using the [`is_SetImageMem`] function or create a sequence
/// using the [`is_AddToSequence`] function.
pub const IS_NO_ACTIVE_IMG_MEM: INT = 108;
pub const IS_CANT_INIT_EVENT: INT = 109;
pub const IS_FUNC_NOT_AVAIL_IN_OS: INT = 110;
pub const IS_CAMERA_NOT_CONNECTED: INT = 111;

/// The sequence list is empty and cannot be deleted.
pub const IS_SEQUENCE_LIST_EMPTY: INT = 112;

/// The image memory is already included in the sequence and cannot be added again.
pub const IS_CANT_ADD_TO_SEQUENCE: INT = 113;
pub const IS_LOW_OF_SEQUENCE_RISC_MEM: INT = 114;
pub const IS_IMGMEM2FREE_USED_IN_SEQ: INT = 115;
pub const IS_IMGMEM_NOT_IN_SEQUENCE_LIST: INT = 116;

/// The memory could not be locked. The pointer to the buffer is invalid.
pub const IS_SEQUENCE_BUF_ALREADY_LOCKED: INT = 117;

/// The device ID is invalid.
///
/// Valid IDs start from 1 for USB cameras, and from 1001 for GigE cameras.
pub const IS_INVALID_DEVICE_ID: INT = 118;

/// The board ID is invalid.
///
/// Valid IDs range from 1 through 255.
pub const IS_INVALID_BOARD_ID: INT = 119;

/// All cameras are in use.
pub const IS_ALL_DEVICES_BUSY: INT = 120;
pub const IS_HOOK_BUSY: INT = 121;

/// A timeout occurred. An image capturing process could not be terminated within the
/// allowable period.
pub const IS_TIMED_OUT: INT = 122;

/// Invalid array.
pub const IS_NULL_POINTER: INT = 123;
pub const IS_WRONG_HOOK_VERSION: INT = 124;

/// One of the submitted parameters is outside the valid range or is not supported for this sensor
/// or is not available in this mode.
pub const IS_INVALID_PARAMETER: INT = 125;
pub const IS_NOT_ALLOWED: INT = 126;

/// No memory could be allocated.
pub const IS_OUT_OF_MEMORY: INT = 127;
pub const IS_INVALID_WHILE_LIVE: INT = 128;

/// An access violation has occurred.
pub const IS_ACCESS_VIOLATION: INT = 129;
pub const IS_UNKNOWN_ROP_EFFECT: INT = 130;
pub const IS_INVALID_RENDER_MODE: INT = 131;
pub const IS_INVALID_THREAD_CONTEXT: INT = 132;
pub const IS_NO_HARDWARE_INSTALLED: INT = 133;
pub const IS_INVALID_WATCHDOG_TIME: INT = 134;
pub const IS_INVALID_WATCHDOG_MODE: INT = 135;
pub const IS_INVALID_PASSTHROUGH_IN: INT = 136;
pub const IS_ERROR_SETTING_PASSTHROUGH_IN: INT = 137;
pub const IS_FAILURE_ON_SETTING_WATCHDOG: INT = 138;

/// The camera is connected to a port which does not support the USB 2.0 high-speed standard.
///
/// Cameras without a memory board cannot be operated on a USB 1.1 port.
pub const IS_NO_USB20: INT = 139;

/// A capturing operation is in progress and must be terminated first.
pub const IS_CAPTURE_RUNNING: INT = 140;

/// Operation could not execute while `mboard` is enabled.
pub const IS_MEMORY_BOARD_ACTIVATED: INT = 141;

/// Operation could not execute while `mboard` is disabled.
pub const IS_MEMORY_BOARD_DEACTIVATED: INT = 142;

/// No memory board connected.
pub const IS_NO_MEMORY_BOARD_CONNECTED: INT = 143;

/// Image size is above memory capacity.
pub const IS_TOO_LESS_MEMORY: INT = 144;

/// The requested image is not available in the camera memory or is no longer valid.
pub const IS_IMAGE_NOT_PRESENT: INT = 145;
pub const IS_MEMORY_MODE_RUNNING: INT = 146;
pub const IS_MEMORYBOARD_DISABLED: INT = 147;

/// The function cannot be used because the camera is waiting for a trigger signal.
pub const IS_TRIGGER_ACTIVATED: INT = 148;
pub const IS_WRONG_KEY: INT = 150;

/// A CRC error-correction problem occurred while reading the settings.
pub const IS_CRC_ERROR: INT = 151;

/// This function has not been enabled yet in this version.
pub const IS_NOT_YET_RELEASED: INT = 152;

/// The camera does not contain any calibration data.
pub const IS_NOT_CALIBRATED: INT = 153;

/// The system is waiting for the kernel driver to respond.
pub const IS_WAITING_FOR_KERNEL: INT = 154;

/// The camera model used here does not support this function or setting.
pub const IS_NOT_SUPPORTED: INT = 155;

/// The function is not possible as trigger is disabled.
pub const IS_TRIGGER_NOT_ACTIVATED: INT = 156;

/// The operation was cancelled.
pub const IS_OPERATION_ABORTED: INT = 157;

/// An internal structure has an incorrect size.
pub const IS_BAD_STRUCTURE_SIZE: INT = 158;

/// The image memory has an inappropriate size to store the image in the desired format.
pub const IS_INVALID_BUFFER_SIZE: INT = 159;

/// This setting is not available for the currently set pixel clock frequency.
pub const IS_INVALID_PIXEL_CLOCK: INT = 160;

/// This setting is not available for the currently set exposure time.
pub const IS_INVALID_EXPOSURE_TIME: INT = 161;

/// This setting cannot be changed while automatic exposure time control is enabled.
pub const IS_AUTO_EXPOSURE_RUNNING: INT = 162;

/// The BackBuffer surface cannot be created.
pub const IS_CANNOT_CREATE_BB_SURF: INT = 163;

/// The BackBuffer mix surface cannot be created.
pub const IS_CANNOT_CREATE_BB_MIX: INT = 164;

/// The BackBuffer overlay memory cannot be locked.
pub const IS_BB_OVLMEM_NULL: INT = 165;

/// The BackBuffer overlay memory cannot be created.
pub const IS_CANNOT_CREATE_BB_OVL: INT = 166;

/// Not supported in BackBuffer Overlay mode.
pub const IS_NOT_SUPP_IN_OVL_SURF_MODE: INT = 167;

/// Back buffer surface invalid.
pub const IS_INVALID_SURFACE: INT = 168;

/// Back buffer surface not found.
pub const IS_SURFACE_LOST: INT = 169;

/// Error releasing the overlay device context.
pub const IS_RELEASE_BB_OVL_DC: INT = 170;

/// The back buffer timer could not be created.
pub const IS_BB_TIMER_NOT_CREATED: INT = 171;

/// The back buffer overlay was not enabled.
pub const IS_BB_OVL_NOT_EN: INT = 172;

/// Only possible in BackBuffer mode.
pub const IS_ONLY_IN_BB_MODE: INT = 173;

/// Invalid color format.
pub const IS_INVALID_COLOR_FORMAT: INT = 174;

/// Mono binning/mono sub-sampling do not support automatic white balance.
pub const IS_INVALID_WB_BINNING_MODE: INT = 175;

/// Invalid I2C device address.
pub const IS_INVALID_I2C_DEVICE_ADDRESS: INT = 176;

/// The current image could not be processed.
pub const IS_COULD_NOT_CONVERT: INT = 177;

/// Transfer error.
///
/// Frequent transfer errors can mostly be avoided by reducing the pixel rate.
pub const IS_TRANSFER_ERROR: INT = 178;

/// Parameter set is not present.
pub const IS_PARAMETER_SET_NOT_PRESENT: INT = 179;

/// The camera type defined in the `.ini` file does not match the current camera model.
pub const IS_INVALID_CAMERA_TYPE: INT = 180;

/// Invalid `HIBYTE` of host address.
pub const IS_INVALID_HOST_IP_HIBYTE: INT = 181;

/// The color mode is not supported in the current display mode.
pub const IS_CM_NOT_SUPP_IN_CURR_DISPLAYMODE: INT = 182;

/// No IR filter available.
pub const IS_NO_IR_FILTER: INT = 183;

/// The camera's starter firmware is not compatible with the driver and needs to be updated.
pub const IS_STARTER_FW_UPLOAD_NEEDED: INT = 184;

/// The DirectRenderer library could not be found.
pub const IS_DR_LIBRARY_NOT_FOUND: INT = 185;

/// Not enough graphics memory available.
pub const IS_DR_DEVICE_OUT_OF_MEMORY: INT = 186;

/// The image surface or overlay surface could not be created.
pub const IS_DR_CANNOT_CREATE_SURFACE: INT = 187;

/// The vertex buffer could not be created.
pub const IS_DR_CANNOT_CREATE_VERTEX_BUFFER: INT = 188;

/// The texture could not be created.
pub const IS_DR_CANNOT_CREATE_TEXTURE: INT = 189;

/// The overlay surface could not be locked.
pub const IS_DR_CANNOT_LOCK_OVERLAY_SURFACE: INT = 190;

/// The overlay surface could not be unlocked.
pub const IS_DR_CANNOT_UNLOCK_OVERLAY_SURFACE: INT = 191;

/// Could not get the device context handle for the overlay.
pub const IS_DR_CANNOT_GET_OVERLAY_DC: INT = 192;

/// Could not release the device context handle for the overlay.
pub const IS_DR_CANNOT_RELEASE_OVERLAY_DC: INT = 193;

/// Function is not supported by the graphics hardware.
pub const IS_DR_DEVICE_CAPS_INSUFFICIENT: INT = 194;

/// Because of other incompatible settings the function is not possible.
pub const IS_INCOMPATIBLE_SETTING: INT = 195;

/// A device context handle is still open in the application.
pub const IS_DR_NOT_ALLOWED_WHILE_DC_IS_ACTIVE: INT = 196;

/// The device is already in use by the system or is being used by another system.
/// (Camera was opened and paired to a device).
pub const IS_DEVICE_ALREADY_PAIRED: INT = 197;

/// The subnet mask of the camera and PC network card are different.
pub const IS_SUBNETMASK_MISMATCH: INT = 198;

/// The subnet of the camera and PC network card are different.
pub const IS_SUBNET_MISMATCH: INT = 199;

/// The configuration of the IP address is invalid.
pub const IS_INVALID_IP_CONFIGURATION: INT = 200;

/// The device is not compatible to the drivers.
pub const IS_DEVICE_NOT_COMPATIBLE: INT = 201;

/// The settings for the image size of the camera are not compatible to the PC network card.
pub const IS_NETWORK_FRAME_SIZE_INCOMPATIBLE: INT = 202;

/// The configuration of the network card is invalid.
pub const IS_NETWORK_CONFIGURATION_INVALID: INT = 203;

/// The configuration of the CPU idle has failed.
pub const IS_ERROR_CPU_IDLE_STATES_CONFIGURATION: INT = 204;

/// The camera is busy and cannot transfer the requested image.
pub const IS_DEVICE_BUSY: INT = 205;

/// The initialization of the sensor failed.
pub const IS_SENSOR_INITIALIZATION_FAILED: INT = 206;

/// The image buffer is not [`c_dword`] aligned.
pub const IS_IMAGE_BUFFER_NOT_DWORD_ALIGNED: INT = 207;

/// The image memory is locked.
pub const IS_SEQ_BUFFER_IS_LOCKED: INT = 208;

/// The file path does not exist.
pub const IS_FILE_PATH_DOES_NOT_EXIST: INT = 209;

/// Invalid Window handle.
pub const IS_INVALID_WINDOW_HANDLE: INT = 210;

/// Invalid image parameter (position or size).
pub const IS_INVALID_IMAGE_PARAMETER: INT = 211;
pub const IS_NO_SUCH_DEVICE: INT = 212;
pub const IS_DEVICE_IN_USE: INT = 213;
