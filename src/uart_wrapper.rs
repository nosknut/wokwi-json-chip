use std::ffi::c_void;

use wokwi_chip_ll::UARTConfig;

use crate::{
    traits::{Uart, UartJson, UartJsonInner},
    uart_pins::UartPins,
    uart_tx::UartTX,
    utils::{debug_print_string, uartInit},
};

pub fn init_uart<T: Uart>(uart: T, pins: UartPins) {
    UartWrapper::init(uart, pins)
}

pub fn init_uart_json<T: UartJson>(uart: T, pins: UartPins) {
    UartWrapper::init(UartJsonInner::new(uart), pins)
}

struct UartWrapper<T: Uart> {
    inner: T,

    tx: UartTX,
}

impl<T: Uart> UartWrapper<T> {
    fn from_uart(uart: T) -> Self {
        Self {
            inner: uart,

            tx: UartTX::new(),
        }
    }

    fn init(uart: T, pins: UartPins) {
        debug_print_string("Initializing ...".to_string());

        let wrapper = Self::from_uart(uart);

        let ptr = make_ptr(wrapper);

        let config = UARTConfig {
            rx: pins.rx,
            tx: pins.tx,
            baud_rate: pins.baud_rate,
            user_data: ptr as *const c_void,
            rx_data: Self::on_uart_rx_data as *const c_void,
            write_done: Self::on_uart_write_done as *const c_void,
        };

        let wrapper = unsafe { ptr.as_mut().unwrap() };

        let device_id = unsafe { uartInit(&config) };

        wrapper.tx.update_id(device_id);

        debug_print_string(format!("Initialized on uart port: {}!", device_id));
    }

    fn on_uart_rx_data(ptr: *mut Self, byte: u8) {
        let Some(uart) = (unsafe { ptr.as_mut() }) else {
            debug_print_string("Missing uart detected".to_string());
            return;
        };

        // uart.in_buffer.push_back(byte);
        uart.inner.rx(&mut uart.tx, byte);
    }

    fn on_uart_write_done(ptr: *mut Self) {
        let Some(uart) = (unsafe { ptr.as_mut() }) else {
            debug_print_string("Missing uart detected".to_string());
            return;
        };

        uart.tx.try_write();
    }
}

/// Makes a pointer from any data
///
/// Uses box leak
fn make_ptr<T>(data: T) -> *mut T {
    Box::leak(Box::new(data))
}
