#![allow(non_camel_case_types)]

use bitflags::bitflags;
use crate::types::{INT, UINT, double, HCAM, void, NULL};
use crate::constants::return_values::*;

/// Structure for flash delay and duration.
///
/// # Related commands
/// * [`IO_CMD::IS_IO_CMD_FLASH_GET_GLOBAL_PARAMS`]
/// * [`IO_CMD::IS_IO_CMD_FLASH_APPLY_GLOBAL_PARAMS`]
/// * [`IO_CMD::IS_IO_CMD_FLASH_GET_PARAMS`]
/// * [`IO_CMD::IS_IO_CMD_FLASH_SET_PARAMS`]
/// * [`IO_CMD::IS_IO_CMD_FLASH_GET_PARAMS_MIN`]
/// * [`IO_CMD::IS_IO_CMD_FLASH_GET_PARAMS_MAX`]
/// * [`IO_CMD::IS_IO_CMD_FLASH_GET_PARAMS_INC`]
/// * [`IO_CMD::IS_IO_CMD_FLASH_GET_GPIO_PARAMS_MIN`]
/// * [`IO_CMD::IS_IO_CMD_FLASH_SET_GPIO_PARAMS`]
///
/// # Documentation
/// [Using flash: Contents of the `IO_FLASH_PARAMS` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html#io_flash_params)
pub struct IO_FLASH_PARAMS {
    /// Flash delay (in μs).
    pub s32Delay: INT,

    /// Flash duration (in μs).
    ///
    /// If `0` is passed, the flash output will be active until the end of the exposure time.
    /// For sensors with Global Start Shutter this is the time until the end of exposure of the
    /// first sensor row.
    pub u32Duration: UINT
}

/// Enumeration of flash modes.
///
/// # Related commands
/// * [`IO_CMD::IS_IO_CMD_FLASH_SET_MODE`]
/// * [`IO_CMD::IS_IO_CMD_FLASH_GET_MODE`]
///
/// # Documentation
/// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IO_FLASH_MODE {
    /// Disables the digital output.
    IO_FLASH_MODE_OFF =                  0,

    /// Enables the flash strobe in trigger mode.
    /// The digital output is set to low level for the flash duration.
    IO_FLASH_MODE_TRIGGER_LO_ACTIVE   =  1,

    /// Enables the flash strobe in trigger mode.
    /// The digital output is set to high level for the flash duration.
    IO_FLASH_MODE_TRIGGER_HI_ACTIVE   =  2,

    /// Statically sets the digital output to high level (HIGH).
    IO_FLASH_MODE_CONSTANT_HIGH       =  3,

    /// Statically sets the digital output to low level (LOW).
    IO_FLASH_MODE_CONSTANT_LOW        =  4,

    /// Enables the flash strobe in freerun mode.
    /// The digital output is set to low level for the flash duration.
    IO_FLASH_MODE_FREERUN_LO_ACTIVE   =  5,

    /// Enables the flash strobe in freerun mode.
    /// The digital output is set to high level for the flash duration.
    IO_FLASH_MODE_FREERUN_HI_ACTIVE   =  6,
}

/// Enables PWM for the flash mode GPIO.
pub const IS_FLASH_MODE_PWM: UINT                  = 0x8000;

bitflags! {
    /// Enumeration of flash ports (_supports bitmask_).
    ///
    /// # Related commands
    /// * [`IO_CMD::IS_IO_CMD_FLASH_SET_MODE`]
    /// * [`IO_CMD::IS_IO_CMD_FLASH_GET_MODE`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct IO_FLASH_PORT: UINT {
        /// Enables additionally the flash mode via the 1st programmable input/output (GPIO 1)
        /// of some models.
     const IO_FLASH_MODE_GPIO_1               = 0x0010;

        /// Enables additionally the flash mode via the 2nd programmable input/output (GPIO 2)
        /// of some models.
     const IO_FLASH_MODE_GPIO_2               = 0x0020;

        /// Enables additionally the flash mode via the 3rd programmable input/output (GPIO 3)
        /// of some models.
     const IO_FLASH_MODE_GPIO_3               = 0x0040;

        /// Enables additionally the flash mode via the 4th programmable input/output (GPIO 4)
        /// of some models.
     const IO_FLASH_MODE_GPIO_4               = 0x0080;

        /// Enables additionally the flash mode via the 5th programmable input/output (GPIO 5)
        /// of some models.
     const IO_FLASH_MODE_GPIO_5               = 0x0100;

        /// Enables additionally the flash mode via the 6th programmable input/output (GPIO 6)
        /// of some models.
     const IO_FLASH_MODE_GPIO_6               = 0x0200;
    }
}

pub const IO_FLASH_GPIO_PORT_MASK: UINT = IO_FLASH_PORT::all().bits();

/// Enumeration for auto flash settings in freerun mode.
///
/// # Related commands
/// * [`IO_CMD::IS_IO_CMD_FLASH_SET_AUTO_FREERUN`]
/// * [`IO_CMD::IS_IO_CMD_FLASH_GET_AUTO_FREERUN`]
/// * [`IO_CMD::IS_IO_CMD_FLASH_GET_AUTO_FREERUN_DEFAULT`]
///
/// # Documentation
/// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum FLASH_AUTO_FREERUN {
    /// Disables auto flash mode.
 IS_FLASH_AUTO_FREERUN_OFF =          0,
    /// Enables auto flash mode.
 IS_FLASH_AUTO_FREERUN_ON =            1
}

impl From<bool> for FLASH_AUTO_FREERUN {
    #[inline]
    fn from(b: bool) -> Self {
        if b { Self::IS_FLASH_AUTO_FREERUN_ON } else { Self::IS_FLASH_AUTO_FREERUN_OFF }
    }
}

/// Structure for the PWM params.
///
/// # Related commands
/// * [`IO_CMD::IS_IO_CMD_PWM_SET_PARAMS`]
/// * [`IO_CMD::IS_IO_CMD_PWM_GET_PARAMS`]
/// * [`IO_CMD::IS_IO_CMD_PWM_GET_PARAMS_MIN`]
/// * [`IO_CMD::IS_IO_CMD_PWM_GET_PARAMS_MAX`]
/// * [`IO_CMD::IS_IO_CMD_PWM_GET_PARAMS_INC`]
///
/// # Documentation
/// [Using pulse-width modulation: Contents of the `IO_PWM_PARAMS` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iopwm.html)
pub struct IO_PWM_PARAMS {
    /// Frequency of the pulse-width modulation (PWM).
    /// Valid range: `1.0`…`10000` Hz.
    dblFrequency_Hz: double,

    /// Duty cycle of the pulse-width modulation.
    /// Valid range: `0.0`…`1.0` Hz (`1.0` corresponds to 100%).
    dblDutyCycle: double
}

/// Structure for the configuration params of the GPIOs.
///
/// [Using GPIO: Contents of the `IO_GPIO_CONFIGURATION` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html#io_gpio_configuration)
pub struct IO_GPIO_CONFIGURATION {
    /// Sets the GPIO whose configuration is to be read or set ([`IO_GPIO_1`], [`IO_GPIO_2`]).
    ///
    /// This value must be initialized before the GPIO configuration is read or set.
    pub u32Gpio: UINT,

    /// When reading the configuration: "OR" bitmask of the supported GPIO modes
    /// (`IS_GPIO_INPUT | IS_GPIO_OUTPUT`…).
    pub u32Caps: GPIO_CAPS,

    /// When reading the configuration: returns the current set configuration.
    /// When setting the configuration: sets the configuration.
    pub u32Configuration: GPIO_CAPS,

    /// When reading the configuration: returns the current state of the GPIO
    /// (`0` = Low, `1` = High).
    /// When setting the configuration: sets the state of the GPIO
    /// (`0` = Low, `1` = High).
    pub u32State: GPIO_STATE,

    /// (**reserved**)
    u32Reserved: [UINT; 12],
}

bitflags! {
    /// GPIO ID (_supports bitmask_).
    ///
    /// # Related commands
    /// * [`IO_CMD::IS_IO_CMD_PWM_GET_MODE`]
    /// * [`IO_CMD::IS_IO_CMD_PWM_SET_MODE`]
    /// * [`IO_CMD::IS_IO_CMD_PWM_GET_SUPPORTED_GPIOS`]
    /// * [`IO_CMD::IS_IO_CMD_FLASH_GET_SUPPORTED_GPIOS`]
    ///
    /// # Documentation
    /// [Using GPIO](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html)
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct IO_GPIO: UINT {
        const IO_GPIO_1 =                          0x0001;
        const IO_GPIO_2 =                          0x0002;
        const IO_GPIO_3 =                          0x0004;
        const IO_GPIO_4 =                          0x0008;
        const IO_GPIO_5 =                          0x0010;
        const IO_GPIO_6 =                          0x0020;
    }
}

/// State of a GPIO.
///
/// # Documentation
/// [Using GPIO](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
#[repr(u32)]
pub enum GPIO_STATE {
    LOW = 0,
    HIGH = 1,
}

bitflags! {
    /// Configuration of a GPIO (_supports bitmask_).
    ///
    /// # Documentation
    /// [Using GPIO](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html)
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct GPIO_CAPS: UINT {
        /// GPIO is used as input.
        const IS_GPIO_INPUT =                        0x0001;

        /// GPIO is used as output.
        const IS_GPIO_OUTPUT =                       0x0002;

        /// GPIO is used for flash.
        const IS_GPIO_FLASH =                        0x0004;

        /// GPIO is used for
        /// [pulse width modulation](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iopwm.html).
        const IS_GPIO_PWM =                          0x0008;

        /// Windows only: GPIO is used as
        /// [serial interface](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/hw_serielleschnittstelle_rs-232.html).
        const IS_GPIO_COMPORT_RX =                   0x0010;

        /// Windows only: GPIO is used as
        /// [serial interface](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/hw_serielleschnittstelle_rs-232.html).
        const IS_GPIO_COMPORT_TX =                   0x0020;

        /// GPIO is used for multi integration mode.
        const IS_GPIO_MULTI_INTEGRATION_MODE =       0x0040;

        /// GPIO is used for trigger.
        const IS_GPIO_TRIGGER =                      0x0080;

        /// GPIO is used as I<sup>2</sup>C interface.
        const IS_GPIO_I2C =                          0x0100;
    }
}

/// Enumeration of LED states.
///
/// # Documentation
/// [Configuring LED](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioled.html)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum IO_LED_STATE {
    /// Sets LED to red.
    IO_LED_STATE_1                     = 0,

    /// Sets LED to green.
    IO_LED_STATE_2                     = 1,

    /// Enables the LED (default setting when the camera starts).
    /// (_USB 3 uEye cameras only_).
    IO_LED_ENABLE                      = 2,

    /// Disables the LED. The LED only flashes if an error occurs.
    /// (_USB 3 uEye cameras only_).
    IO_LED_DISABLE                     = 3,

    /// Enables the LED with permanent flashing.
    /// The permanent flashing can be disabled by [`IO_LED_STATE::IO_LED_BLINK_DISABLE`] or
    /// if an error occurs.
    /// (_USB 3 uEye cameras only_).
    IO_LED_BLINK_ENABLE                = 4,

    /// Disables the permanent flashing of the LED. The LED only flashes if an error occurs.
    /// (_USB 3 uEye cameras only_).
    IO_LED_BLINK_DISABLE               = 5,

    /// The LED flashes five times and returns to its previous state (active/inactive).
    /// (_USB 3 uEye cameras only_).
    IO_LED_BLINK_5_TIMES               = 6
}

/// Enumeration of commands of function [`is_IO`].
pub enum IO_CMD {
    /// Returns the supported GPIO ports.
    ///
    /// # Parameter type
    /// [`IO_FLASH_PORT`], _bitmask_
    ///
    /// # Documentation
    /// [Using GPIO](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html)
    IS_IO_CMD_GPIOS_GET_SUPPORTED               = 1,

    /// Returns the supported GPIO inputs.
    ///
    /// # Parameter type
    /// [`IO_FLASH_PORT`], _bitmask_
    ///
    /// # Documentation
    /// [Using GPIO](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html)
    IS_IO_CMD_GPIOS_GET_SUPPORTED_INPUTS        = 2,

    /// Returns the supported GPIO outputs.
    ///
    /// # Parameter type
    /// [`IO_FLASH_PORT`], _bitmask_
    ///
    /// # Documentation
    /// [Using GPIO](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html)
    IS_IO_CMD_GPIOS_GET_SUPPORTED_OUTPUTS       = 3,

    /// Returns the input/output mask of the GPIOs.
    ///
    /// # Parameter type
    /// [`IO_FLASH_PORT`], _bitmask_ (`1` = output)
    ///
    /// # Documentation
    /// [Using GPIO](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html)
    IS_IO_CMD_GPIOS_GET_DIRECTION               = 4,

    /// Set the GPIO on input/output.
    ///
    /// # Parameter type
    /// [`IO_FLASH_PORT`], _bitmask_ (`1` = output)
    ///
    /// # Documentation
    /// [Using GPIO](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html)
    IS_IO_CMD_GPIOS_SET_DIRECTION               = 5,

    /// Returns the state of the GPIO (High, Low).
    ///
    /// # Parameter type
    /// [`IO_FLASH_PORT`], _bitmask_ (`1` = high)
    ///
    /// # Documentation
    /// [Using GPIO](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html)
    IS_IO_CMD_GPIOS_GET_STATE                   = 6,

    /// Sets the state of the GPIOs if they are defined as output (High, Low).
    ///
    /// # Parameter type
    /// [`IO_FLASH_PORT`], _bitmask_ (`1` = high)
    ///
    /// # Documentation
    /// [Using GPIO](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html)
    IS_IO_CMD_GPIOS_SET_STATE                   = 7,

    /// Returns the state of the LED.
    ///
    /// # Parameter type
    /// [`IO_LED_STATE`]
    ///
    /// # Documentation
    /// [Configuring LED](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioled.html)
    IS_IO_CMD_LED_GET_STATE                     = 8,

    /// Sets the state of the LED.
    ///
    /// # Parameter type
    /// [`IO_LED_STATE`]
    ///
    /// # Documentation
    /// [Configuring LED](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioled.html)
    IS_IO_CMD_LED_SET_STATE                     = 9,

    /// Toggles between the LED states.
    ///
    /// # Parameter type
    /// [`NULL`]
    ///
    /// # Documentation
    /// [Configuring LED](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioled.html)
    IS_IO_CMD_LED_TOGGLE_STATE                  = 10,

    /// Returns the parameters for the global exposure window.
    ///
    /// # Parameter type
    /// [`IO_FLASH_PARAMS`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_GET_GLOBAL_PARAMS           = 11,

    /// Returns the parameters for the global exposure window and sets them as flash parameters.
    ///
    /// # Parameter type
    /// [`IO_FLASH_PARAMS`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_APPLY_GLOBAL_PARAMS         = 12,

    /// Returns the GPIOs which can be used for flash output.
    ///
    /// # Parameter type
    /// [`IO_GPIO`], _bitmask_
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_GET_SUPPORTED_GPIOS         = 13,


    /// Returns the minimum possible values for flash delay and duration.
    ///
    /// # Parameter type
    /// [`IO_FLASH_PARAMS`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_GET_PARAMS_MIN              = 14,

    /// Returns the maximum possible values for flash delay and duration.
    ///
    /// # Parameter type
    /// [`IO_FLASH_PARAMS`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_GET_PARAMS_MAX              = 15,

    /// Returns the increments for flash delay and duration.
    ///
    /// # Parameter type
    /// [`IO_FLASH_PARAMS`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_GET_PARAMS_INC              = 16,

    /// Returns the current values for flash delay and duration.
    ///
    /// # Parameter type
    /// [`IO_FLASH_PARAMS`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_GET_PARAMS                  = 17,

    /// Sets the current values for flash delay and duration.
    ///
    /// # Parameter type
    /// [`IO_FLASH_PARAMS`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_SET_PARAMS                  = 18,

    /// Returns the current flash mode.
    ///
    /// # Parameter type
    /// [`UINT`], _combination of:_
    /// * [`IO_FLASH_MODE`]
    /// * [`IO_FLASH_PORT`]
    /// * [`IS_FLASH_MODE_PWM`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_GET_MODE                    = 19,

    /// Sets the flash mode.
    ///
    /// # Parameter type
    /// [`UINT`], _combination of:_
    /// * [`IO_FLASH_MODE`]
    /// * [`IO_FLASH_PORT`]
    /// * [`IS_FLASH_MODE_PWM`] (_sets the flash output as output for PWM mode_)
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_SET_MODE                    = 20,

    /// Returns the GPIOs which can be used for pulse-width modulation (PWM).
    ///
    /// # Parameter type
    /// [`IO_GPIO`], _bitmask_
    ///
    /// # Documentation
    /// [Using pulse-width modulation](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iopwm.html)
    IS_IO_CMD_PWM_GET_SUPPORTED_GPIOS           = 21,

    /// Returns the minimum possible values for PWM parameters.
    ///
    /// # Parameter type
    /// [`IO_PWM_PARAMS`]
    ///
    /// # Documentation
    /// [Using pulse-width modulation](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iopwm.html)
    IS_IO_CMD_PWM_GET_PARAMS_MIN                = 22,

    /// Returns the maximum possible values for PWM parameters.
    ///
    /// # Parameter type
    /// [`IO_PWM_PARAMS`]
    ///
    /// # Documentation
    /// [Using pulse-width modulation](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iopwm.html)
    IS_IO_CMD_PWM_GET_PARAMS_MAX                = 23,

    /// Returns the increments of the PWM parameters.
    ///
    /// # Parameter type
    /// [`IO_PWM_PARAMS`]
    ///
    /// # Documentation
    /// [Using pulse-width modulation](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iopwm.html)
    IS_IO_CMD_PWM_GET_PARAMS_INC                = 24,

    /// Returns the current values of the PWM parameters.
    ///
    /// # Parameter type
    /// [`IO_PWM_PARAMS`]
    ///
    /// # Documentation
    /// [Using pulse-width modulation](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iopwm.html)
    IS_IO_CMD_PWM_GET_PARAMS                    = 25,

    /// Sets the current values of the PWM parameters.
    ///
    /// # Parameter type
    /// [`IO_PWM_PARAMS`]
    ///
    /// # Documentation
    /// [Using pulse-width modulation](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iopwm.html)
    IS_IO_CMD_PWM_SET_PARAMS                    = 26,

    /// Returns the GPIOs which can be used for pulse-width modulation (PWM).
    ///
    /// # Parameter type
    /// [`IO_GPIO`], _bitmask_
    ///
    /// # Documentation
    /// [Using pulse-width modulation](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iopwm.html)
    IS_IO_CMD_PWM_GET_MODE                      = 27,

    /// Sets the current PWM mode.
    ///
    /// # Parameter type
    /// [`UINT`], _combination of:_
    /// * [`IO_GPIO`], _bitmask_
    /// * [`IS_FLASH_MODE_PWM`] (_sets the flash output as output for PWM mode_)
    ///
    /// # Documentation
    /// [Using pulse-width modulation](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iopwm.html)
    IS_IO_CMD_PWM_SET_MODE                      = 28,

    /// Returns the configuration of a GPIO port.
    ///
    /// # Parameter type
    /// [`IO_GPIO_CONFIGURATION`]
    ///
    /// # Documentation
    /// [Using GPIO](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html)
    IS_IO_CMD_GPIOS_GET_CONFIGURATION           = 29,

    /// Sets the configuration of a GPIO port.
    ///
    /// # Parameter type
    /// [`IO_GPIO_CONFIGURATION`]
    ///
    /// # Documentation
    /// [Using GPIO](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html)
    IS_IO_CMD_GPIOS_SET_CONFIGURATION           = 30,

    /// Returns the minimum possible values for flash delay and flash duration.
    ///
    /// # Parameter type
    /// [`IO_FLASH_PARAMS`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_GET_GPIO_PARAMS_MIN         = 31,

    /// Sets the flash delay and flash duration and allows the minimum values for GPIOs.
    ///
    /// <div class="warning">
    /// For values below 20 μs an unpredictable behavior can occur when flashing is done via
    /// the normal flash pin
    /// </div>
    ///
    /// # Parameter type
    /// [`IO_FLASH_PARAMS`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_SET_GPIO_PARAMS             = 32,

    /// Returns the default auto flash setting in freerun mode.
    ///
    /// # Parameter type
    /// [`FLASH_AUTO_FREERUN`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_GET_AUTO_FREERUN_DEFAULT    = 33,

    /// Returns the current auto flash setting in freerun mode.
    ///
    /// # Parameter type
    /// [`FLASH_AUTO_FREERUN`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_GET_AUTO_FREERUN            = 34,

    /// Enables/disables the auto flash in freerun mode.
    ///
    /// It may take a few images until the flash timing is adjusted.
    ///
    /// # Parameter type
    /// [`FLASH_AUTO_FREERUN`]
    ///
    /// # Documentation
    /// [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    IS_IO_CMD_FLASH_SET_AUTO_FREERUN            = 35
}

unsafe extern "C" {
    /// Control all flash functions and the additional digital outputs (GPIOs) of some uEye models.
    ///
    /// Additionally, you can toggle the color of the LED on the back of the
    /// USB uEye SE/RE camera housing.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `nCommand` - Command. See [`IO_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    /// * [`IS_TRIGGER_ACTIVATED`]
    /// * [`IS_TRIGGER_NOT_ACTIVATED`]
    ///
    /// # Documentation
    /// * [`is_IO`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_io.html)
    /// * [Using flash](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioflash.html)
    /// * [Using GPIO](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iogpio.html)
    /// * [Using pulse-width modulation](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_iopwm.html)
    /// * [Configuring LED](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ioled.html)
    pub fn is_IO(hCam: HCAM, nCommand: IO_CMD, pParam: *mut void, cbSizeOfParam: UINT) -> INT;

}
