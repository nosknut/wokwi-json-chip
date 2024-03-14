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

/// # Internal
/// Used to wrap Uarts into a standard struct to allow the C pointers to work
///
/// This is needed as otherwise the size of the pointers will be unknown and lead to weird issues
struct UartWrapper<T: Uart> {
    /// The inner Uart externally provided
    inner: T,

    /// The transmit handler
    tx: UartTX,
}

impl<T: Uart> UartWrapper<T> {
    /// Make a warper from a external Uart
    fn from_uart(uart: T) -> Self {
        Self {
            inner: uart,

            // TX is empty by default but needs to be init later
            tx: UartTX::new(),
        }
    }

    /// Initiate a wrapper and the underlying Uart with a given uart and pin config
    fn init(uart: T, pins: UartPins) {
        debug_print_string("Initializing ...".to_string());

        // Wraps the external uart in the wrapper
        let wrapper = Self::from_uart(uart);

        // Leaks it to a pointer
        // Note: this consumes the wrapper and leaves only the pointer
        let ptr = make_ptr(wrapper);

        // Defines the C compatible UARTConfig
        let config = UARTConfig {
            rx: pins.rx,
            tx: pins.tx,
            baud_rate: pins.baud_rate,
            // converts the pointer to a C pointer to allow use later
            user_data: ptr as *const c_void,
            // converts the fns to a pointers to be called from C
            rx_data: Self::on_uart_rx_data as *const c_void,
            write_done: Self::on_uart_write_done as *const c_void,
        };

        // Re grab the wrapper from the pointer
        let wrapper = unsafe { ptr.as_mut().unwrap() };

        // Get the device id and init the Uart
        let device_id = unsafe { uartInit(&config) };

        // update the TX device id
        wrapper.tx.update_id(device_id);

        debug_print_string(format!("Initialized on uart port: {}!", device_id));
    }

    /// This the the fn the UARTConfig points to for rx_data
    ///
    /// ptr is from user_data
    fn on_uart_rx_data(ptr: *mut Self, byte: u8) {
        // Grab the wrapper from the pointer otherwise error out
        let Some(uart) = (unsafe { ptr.as_mut() }) else {
            debug_print_string("Missing uart detected".to_string());
            return;
        };

        // call the upstream rx fn with the byte
        uart.inner.rx(&mut uart.tx, byte);
    }

    /// This the the fn the UARTConfig points to for write_done
    fn on_uart_write_done(ptr: *mut Self) {
        // Grab the wrapper from the pointer otherwise error out
        let Some(uart) = (unsafe { ptr.as_mut() }) else {
            debug_print_string("Missing uart detected".to_string());
            return;
        };

        // call try_write again in case a write was queued
        uart.tx.try_write();
    }
}

/// Makes a pointer from any data
fn make_ptr<T>(data: T) -> *mut T {
    // Box::new moves the data onto the heap
    // Box::leak removes the box but keeps the pointer to the heap
    // This causes a technically memory leak but as the pointer is retained it can be dropped later
    Box::leak(Box::new(data))
}
