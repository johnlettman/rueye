//! Register, manage, and wait for event object signals.
//!
//! **Event handling for images:**
//! If you use the event handling to signal new images from the camera, it may occur under
//! high system load that the event-based application does not receive a signal for every image.
//! This is because the event concept does not provide that several events of the same type
//! are stored. If due to system load more than one image is signaled before the application
//! is ready (e.g. `WaitForSingleObject`), you have no option to capture these missed events.
//!
//! # Documentation
//! [is_Event](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_event.html)

use crate::constants::return_values::*;
use crate::types::{void, BOOL, FALSE, HIDS, INT, TRUE, UINT};

/// Enumeration of commands of function [`is_Event`]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IS_EVENT_CMD {
    /// Initializes and registers the specified event objects.
    ///
    /// After the event object is initialized, it can be
    /// activated ([`IS_EVENT_CMD_ENABLE`][IS_EVENT_CMD::IS_EVENT_CMD_ENABLE]),
    /// deactivated ([`IS_EVENT_CMD_DISABLE`][IS_EVENT_CMD::IS_EVENT_CMD_DISABLE]),
    /// set ([`IS_EVENT_CMD_SET`][IS_EVENT_CMD::IS_EVENT_CMD_SET]),
    /// reset ([`IS_EVENT_CMD_RESET`][IS_EVENT_CMD::IS_EVENT_CMD_RESET])
    /// or waited for ([`IS_EVENT_CMD_WAIT`][IS_EVENT_CMD::IS_EVENT_CMD_WAIT]).
    ///
    /// # Parameter type
    /// [`IS_INIT_EVENT`]
    IS_EVENT_CMD_INIT = 1,

    /// Deinitalizes and deregisters the specified event objects.
    ///
    /// After the event object has been deinitialized, it can no longer be used.
    ///
    /// # Parameter type
    /// _Array of:_ [`UINT`]
    IS_EVENT_CMD_EXIT = 2,

    /// Activates the specified event objects.
    ///
    /// After activation, event messages are permitted for the registered event object.
    ///
    /// # Parameter type
    /// _Array of:_ [`UINT`]
    IS_EVENT_CMD_ENABLE = 3,

    /// Deactivates the specified event objects.
    ///
    /// After deactivation, event messages for the registered event object are prevented.
    ///
    /// # Parameter type
    /// _Array of:_ [`UINT`]
    IS_EVENT_CMD_DISABLE = 4,

    /// Sets the specified event objects to the "signaled" state.
    ///
    /// # Parameter type
    /// _Array of:_ [`UINT`]
    IS_EVENT_CMD_SET = 5,

    /// Sets the specified event objects to the "not signaled" state.
    ///
    /// # Parameter type
    /// _Array of:_ [`UINT`]
    IS_EVENT_CMD_RESET = 6,

    /// Waits for the specified event object or event objects
    /// (see [`IS_WAIT_EVENT`] / [`IS_WAIT_EVENTS`])
    ///
    /// # Parameter type
    /// * [`IS_WAIT_EVENT`]
    /// * [`IS_WAIT_EVENTS`]
    IS_EVENT_CMD_WAIT = 7,
}

/// Event initialization structure.
///
/// # Documentation
/// [is_Event: Contents of the IS_INIT_EVENT structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_event.html#is_init_event)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IS_INIT_EVENT {
    /// ID of the event object that is being initialized.
    pub nEvent: UINT,

    /// Flag for the behaviour after signalling.
    ///
    /// # Possible values
    /// * [`TRUE`]: an event object is registered which must be manually reset to the
    ///     "not signalled" state by the user using the
    ///     [IS_EVENT_CMD_RESET][IS_EVENT_CMD::IS_EVENT_CMD_RESET] command.
    /// * [`FALSE`]: an event object is registered, which is automatically reset to the
    ///     "not signalled" state by the system after signalling.
    pub bManualReset: BOOL,

    /// Flag for the state of the event object after initialization and registration
    ///
    /// # Possible values
    /// * [`TRUE`]: an event object is registered that is in the "signaled" state.
    /// * [`FALSE`]: an event object is registered that is in the " not signaled" state.
    pub bInitialState: BOOL,
}

/// Structure for waiting on a single event.
///
/// # Documentation
/// [is_Event: Contents of the IS_WAIT_EVENT structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_event.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IS_WAIT_EVENT {
    /// ID of the event object to be waited for.
    nEvent: UINT,

    /// Timeout in milliseconds.
    ///
    /// # Possible values
    /// * Timeout ≠ `0`: The function waits until the specified event object is signalled or until
    ///     the specified timeout period has expired.
    /// * Timeout = `0`: The function returns immediately.
    /// * Timeout = [`INFINITE_UINT`]: The function does not return until the event object
    ///     is signalled.
    nTimeoutMilliseconds: UINT,

    /// If the function returns with [`IS_SUCCESS`], the parameter contains the ID of the
    /// signaled event object.
    nSignaled: UINT,

    /// If the function returns with [`IS_SUCCESS`], the parameter contains the number of
    /// signalings of the event object defined by [`nSignaled`][IS_WAIT_EVENT::nSignaled] since
    /// the last query with the [`IS_EVENT_CMD_WAIT`][IS_EVENT_CMD::IS_EVENT_CMD_WAIT] command.
    nSetCount: UINT,
}

/// Structure for waiting on multiple events.
///
/// # Documentation
/// [is_Event: Contents of the IS_WAIT_EVENTS structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_event.html)
#[derive(Debug, Clone)]
#[repr(C)]
pub struct IS_WAIT_EVENTS {
    /// Pointer (array) to the IDs of the event objects to be waited for.
    pub pEvents: *mut UINT,

    /// Number of event objects to which the [`pEvents`][IS_WAIT_EVENTS::pEvents] pointer refers.
    pub nCount: UINT,

    /// Flag for the behavior of the function return when signaling.
    ///
    /// # Possible values
    /// * [`TRUE`]: The function returns when all specified event objects are signalled or until
    ///     the specified timeout
    ///     ([`nTimeoutMilliseconds`][IS_WAIT_EVENTS::nTimeoutMilliseconds]) has expired.
    /// * [`FALSE`]: The function returns when at least one of the specified event objects signals
    ///     or when the specified timeout
    ///     ([`nTimeoutMilliseconds`][IS_WAIT_EVENTS::nTimeoutMilliseconds]) has expired.
    pub bWaitAll: BOOL,

    /// Timeout in milliseconds.
    ///
    /// # Possible values
    /// * Timeout ≠ `0`: The function waits until an event object is signaled or all event objects
    ///     are signaled (see [`bWaitAll`][IS_WAIT_EVENTS::bWaitAll]) or until the specified
    ///     timeout period has expired.
    /// * Timeout = `0`: The function returns immediately.
    /// * Timeout = [`INFINITE_UINT`]: The function returns if an event object is signaled or all
    ///     event objects are signaled (see [`bWaitAll`][IS_WAIT_EVENTS::bWaitAll]).
    pub nTimeoutMilliseconds: UINT,

    /// If the function returns with [`IS_SUCCESS`], the parameter contains depending on the
    /// [`bWaitAll`][IS_WAIT_EVENTS::bWaitAll] parameter.
    ///
    /// # Possible values
    /// * [`bWaitAll`][IS_WAIT_EVENTS::bWaitAll] = [`TRUE`]: an undefined value
    /// * [`bWaitAll`][IS_WAIT_EVENTS::bWaitAll] = [`FALSE`]: the ID of the signaled event object
    pub nSignaled: UINT,

    /// If the function returns with [`IS_SUCCESS`], the parameter contains the number of
    /// signalings of the event object defined by [`nSignaled`][IS_WAIT_EVENTS::nSignaled] since
    /// the last query with the [`IS_EVENT_CMD_WAIT`][IS_EVENT_CMD::IS_EVENT_CMD_WAIT] command.
    pub nSetCount: UINT,
}

unsafe extern "C" {
    /// Register, manage, and wait for event object signals.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`IS_EVENT_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_ACCESS_VIOLATION`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_OUT_OF_MEMORY`]
    /// * [`IS_SUCCESS`]
    /// * [`IS_TIMED_OUT`]
    ///
    /// # Related functions
    /// * [`is_EnableMessage`]
    pub fn is_Event(
        hCam: HIDS,
        nCommand: IS_EVENT_CMD,
        pParam: *mut void,
        cbSizeOfParam: UINT,
    ) -> INT;
}
