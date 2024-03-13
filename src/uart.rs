use std::{
    collections::VecDeque,
    ffi::{c_void, CString},
};

use wokwi_chip_ll::{debugPrint, pinInit, uartWrite, UARTConfig, UARTDevId, INPUT, INPUT_PULLUP};

extern "C" {
    pub fn uartInit(config: *const UARTConfig) -> UARTDevId;
}

pub fn debug_print_string(s: String) {
    unsafe {
        debugPrint(CString::new(s).unwrap().into_raw());
    }
}

type Byte = u8;
pub type UartId = u32;
pub type UartOnReadHandler = fn(uart: &mut Uart, byte: Byte);

#[allow(dead_code)]
pub struct Uart {
    device_id: UartId,
    in_buffer: VecDeque<Byte>,
    out_buffer: Vec<Byte>,
    on_read: UartOnReadHandler,
}

struct UartManager;

impl UartManager {
    fn on_uart_rx_data(ptr: *mut Uart, byte: u8) {
        let Some(uart) = (unsafe { ptr.as_mut() }) else {
            debug_print_string("Missing uart detected".to_string());
            return;
        };

        uart.in_buffer.push_back(byte);
        (uart.on_read)(uart, byte);
    }

    fn on_uart_write_done(ptr: *mut Uart) {
        let Some(uart) = (unsafe { ptr.as_mut() }) else {
            debug_print_string("Missing uart detected".to_string());
            return;
        };

        uart.update_out_buffer();
    }
}

impl Uart {
    fn update_out_buffer(&mut self) {
        if !self.out_buffer.is_empty() {
            let data = self.out_buffer.clone();

            debug_print_string(format!("{}", self.device_id));
            let did_write = unsafe { uartWrite(self.device_id, data.as_ptr(), data.len() as u32) };

            if did_write {
                self.out_buffer.clear();
            }
        }
    }
}

impl Uart {
    pub fn init(pin_tx: &str, pin_rx: &str, baud_rate: u32, on_read: UartOnReadHandler) {
        debug_print_string("Initializing ...".to_string());

        let uart = Uart {
            device_id: u32::MAX,
            in_buffer: VecDeque::new(),
            out_buffer: Vec::new(),
            on_read,
        };

        let ptr = make_ptr(uart);

        let config = UARTConfig {
            rx: unsafe { pinInit(CString::new(pin_rx).unwrap().into_raw(), INPUT_PULLUP) },
            tx: unsafe { pinInit(CString::new(pin_tx).unwrap().into_raw(), INPUT) },
            user_data: ptr as *const c_void,
            baud_rate,
            rx_data: UartManager::on_uart_rx_data as *const c_void,
            write_done: UartManager::on_uart_write_done as *const c_void,
        };

        let uart = unsafe { ptr.as_mut().unwrap() };

        let device_id = unsafe { uartInit(&config) };

        uart.device_id = device_id;

        debug_print_string(format!("Initialized on uart port: {}!", device_id));
    }

    pub fn available(&self) -> u32 {
        self.in_buffer.len() as u32
    }
}

impl Uart {
    pub fn read(&mut self) -> Option<Byte> {
        self.in_buffer.pop_front()
    }

    pub fn read_bytes(&mut self, length: u32) -> Vec<Byte> {
        let mut result = Vec::new();
        for _ in 0..length {
            if let Some(byte) = self.read() {
                result.push(byte);
            } else {
                break;
            }
        }
        result
    }

    pub fn write(&mut self, data: Byte) {
        self.out_buffer.push(data);
        self.update_out_buffer();
    }

    pub fn write_bytes(&mut self, data: Vec<Byte>) {
        self.out_buffer.extend(data);
        self.update_out_buffer();
    }
}

impl Uart {
    pub fn read_char(&mut self) -> Option<char> {
        self.read().map(|byte| byte as char)
    }

    pub fn read_string(&mut self, length: u32) -> String {
        self.read_bytes(length)
            .into_iter()
            .map(|byte| byte as char)
            .collect()
    }
    pub fn write_char(&mut self, data: char) {
        self.write(data as Byte);
    }

    pub fn write_string(&mut self, data: String) {
        self.write_bytes(data.into_bytes());
    }
}

/// Makes a pointer from any data
///
/// Uses box leak
fn make_ptr<T>(data: T) -> *mut T {
    Box::leak(Box::new(data))
}
