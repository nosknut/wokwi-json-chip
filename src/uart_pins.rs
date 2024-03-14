use std::ffi::CString;

use wokwi_chip_ll::{pinInit, PinId};

/// The pin configuration on a Uart
///
/// `UartPins::new` is the preferred method of construction
pub struct UartPins {
    pub rx: PinId,
    pub tx: PinId,
    pub baud_rate: u32,
}

impl UartPins {
    /// Make a new UartPins config from rx Pin tx Pin and baud_rate
    pub fn new(rx: Pin, tx: Pin, baud_rate: u32) -> Self {
        Self::new_raw(rx.build(), tx.build(), baud_rate)
    }

    /// Make a new UartPins config from rx Pin tx Pin and baud_rate
    ///
    /// New_raw assumes you called pinInit yourself
    pub fn new_raw(rx: PinId, tx: PinId, baud_rate: u32) -> Self {
        Self { rx, tx, baud_rate }
    }
}

/// A reference to a pin
pub struct Pin {
    name: Vec<u8>,
    mode: PinMode,
}

impl Pin {
    /// Make a new pin reference from a String and PinMode
    pub fn new<StringLike: Into<Vec<u8>>>(name: StringLike, mode: PinMode) -> Self {
        Self {
            name: name.into(),
            mode,
        }
    }

    /// Runs pinInit to get the actual PinId
    fn build(self) -> PinId {
        unsafe {
            pinInit(
                CString::new(self.name).unwrap().into_raw(),
                self.mode as u32,
            )
        }
    }
}

/// A re-export of the PinModes in a enum instead of constants
pub enum PinMode {
    Input = 0,
    Output = 1,
    InputPullUp = 2,
    InputPullDown = 3,
    Analog = 4,
    /// Equivalent to OutputLow or OutputHigh
    OutputEnd = 16,
}
