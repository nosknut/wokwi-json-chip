use std::ffi::c_void;

use wokwi_chip_ll::{PinId, UARTConfig};

use crate::{
    traits::{Uart, UartJson, UartJsonInner},
    uart_tx::UartTX,
    utils::{debug_print_string, uartInit},
};

//TODO: Rename to UARTConfig in a crate
pub struct UartSettings {
    pub rx: PinId,
    pub tx: PinId,
    pub baud_rate: u32,
}

pub fn init_uart<T: Uart>(uart: T, settings: UartSettings) {
    UartWrapper::init(uart, settings)
}

pub fn init_uart_json<T: UartJson>(uart: T, settings: UartSettings) {
    UartWrapper::init(UartJsonInner::new(uart), settings)
}

struct UartWrapper<T: Uart> {
    inner: T,
    rx_fn: fn(&mut T, &mut UartTX, u8),

    tx: UartTX,
}

impl<T: Uart> UartWrapper<T> {
    fn from_uart(uart: T) -> Self {
        let rx_fn = T::rx;

        Self {
            inner: uart,
            rx_fn,

            tx: UartTX::new(),
        }
    }

    fn init(uart: T, settings: UartSettings) {
        debug_print_string("Initializing ...".to_string());

        let wrapper = Self::from_uart(uart);

        let ptr = make_ptr(wrapper);

        let config = UARTConfig {
            rx: settings.rx,
            tx: settings.tx,
            user_data: ptr as *const c_void,
            baud_rate: settings.baud_rate,
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
        (uart.rx_fn)(&mut uart.inner, &mut uart.tx, byte);
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
