use std::ffi::CString;

use wokwi_chip_ll::{pinInit, PinId};

pub struct UartPins {
    pub rx: PinId,
    pub tx: PinId,
    pub baud_rate: u32,
}

impl UartPins {
    pub fn new(rx: Pin, tx: Pin, baud_rate: u32) -> Self {
        Self::new_raw(rx.build(), tx.build(), baud_rate)
    }

    pub fn new_raw(rx: PinId, tx: PinId, baud_rate: u32) -> Self {
        Self { rx, tx, baud_rate }
    }
}

pub struct Pin {
    name: Vec<u8>,
    mode: PinMode,
}

impl Pin {
    pub fn new<StringLike: Into<Vec<u8>>>(name: StringLike, mode: PinMode) -> Self {
        Self {
            name: name.into(),
            mode,
        }
    }

    fn build(self) -> PinId {
        unsafe {
            pinInit(
                CString::new(self.name).unwrap().into_raw(),
                self.mode as u32,
            )
        }
    }
}

pub enum PinMode {
    Input = 0,
    Output = 1,
    InputPullUp = 2,
    InputPullDown = 3,
    Analog = 4,
    /// Equivalent to OutputLow or OutputHigh
    OutputEnd = 16,
}
