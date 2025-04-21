//! **Obsolete:** Common EEPROM functions.
use crate::types::{char, HIDS, INT};

unsafe extern "C" {

    /// **Obsolete:** Read from the EEPROM.
    ///
    /// There is a rewritable EEPROM on the camera which serves as a small memory.
    /// Additionally, to the information which is stored in the EEPROM, 64 extra bytes can be
    /// written. With the [`is_ReadEEPROM`] command the contents of these 64 bytes can be read.
    /// Also see [`is_WriteEEPROM`].
    ///
    /// <div class="warning">
    /// The reading of the EEPROM is only allowed if the live image acquisition has stopped!
    /// </div>
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `Adr` - Begin address from where data is to be read (_value range: `0`…`63`_).
    /// * `pcString` - Buffer for data to be read (_minimum size: `Count`_).
    /// * `Count` - Number of characters to be read.
    ///
    /// # Return values
    /// * [`IS_SUCCESS`]
    /// * [`IS_NO_SUCCESS`]
    #[deprecated]
    pub fn is_ReadEEPROM(hCam: HIDS, Adr: INT, pcString: *mut char, Count: INT) -> INT;

    /// **Obsolete:** Write to the EEPROM.
    ///
    /// On the frame grabber there is a rewritable EEPROM, where 64 bytes of information can be
    /// written. With the [`is_ReadEEPROM`] function the contents of this 64 byte block can be read.
    ///
    /// <div class="warning">
    /// Writing to EEPROM is only allowed if the live image acquisition has stopped!
    /// </div>
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `Adr` - Start address to where data will be written (_value range: `0`…`63`_).
    /// * `pcString` - Buffer for data to be written.
    /// * `Count` - Number of writable characters (`1`…`64`).
    ///
    /// # Return values
    /// * [`IS_SUCCESS`]
    /// * [`IS_NO_SUCCESS`]
    #[deprecated]
    pub fn is_WriteEEPROM(hCam: HIDS, Adr: INT, pcString: *mut char, Count: INT) -> INT;
}
