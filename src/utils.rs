use std::ffi::CString;

use wokwi_chip_ll::{debugPrint, UARTConfig, UARTDevId};

pub type UartId = u32;

extern "C" {
    pub fn uartInit(config: *const UARTConfig) -> UARTDevId;
}

pub fn debug_print_string(s: String) {
    unsafe {
        debugPrint(CString::new(s).unwrap().into_raw());
    }
}
