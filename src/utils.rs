use std::ffi::CString;

use wokwi_chip_ll::{debugPrint, UARTConfig, UARTDevId};

/// Uart device ID
pub type UartID = u32;

extern "C" {
    /// Init the uart with a config
    ///
    /// Best practice is to use `init_uart` or a derivative in `uart_wrapper.rs`
    pub(crate) fn uartInit(config: *const UARTConfig) -> UARTDevId;
}

/// Prints a line to output / debug
///
/// Standard println! and dbg! don't work
pub fn debug_print_string(s: String) {
    unsafe {
        debugPrint(CString::new(s).unwrap().into_raw());
    }
}
