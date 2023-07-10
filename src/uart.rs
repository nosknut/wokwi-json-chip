// Wokwi Custom Chips with Rust
//
// Very rough prototype by Uri Shaked
//
// Look at chipInit() at the bottom, and open Chrome devtools console to see the debugPrint().

use std::{
    collections::HashMap,
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
    id: UartId,
    config: UARTConfig,
    device_id: UARTDevId,
    in_buffer: VecDeque<Byte>,
    out_buffer: Vec<Byte>,
}

static mut INSTANCES: Option<HashMap<UartId, (Uart, UartOnReadHandler)>> = None;

struct UartManager {}
impl UartManager {
    fn get_instances() -> &'static mut HashMap<UartId, (Uart, UartOnReadHandler)> {
        unsafe {
            if INSTANCES.is_none() {
                INSTANCES = Some(HashMap::new());
            }

            INSTANCES.as_mut().unwrap()
        }
    }
}

impl UartManager {
    fn get_next_id() -> UartId {
        UartManager::get_instances().len() as UartId
    }

    fn register(uart: Uart, on_read: UartOnReadHandler) {
        UartManager::get_instances().insert(uart.id, (uart, on_read));
    }

    fn get(id: UartId) -> Option<(&'static mut Uart, &'static mut UartOnReadHandler)> {
        let Some((uart, on_read)) = UartManager::get_instances().get_mut(&id) else {
                debug_print_string(format!("UART with id {} no longer exists", id));
                return None;
            };

        Some((uart, on_read))
    }
}

impl UartManager {
    fn get_id(user_data: *const c_void) -> UartId {
        user_data as UartId
    }

    fn on_uart_rx_data(user_data: *const c_void, byte: u8) {
        if let Some((uart, on_read)) = UartManager::get(UartManager::get_id(user_data)) {
            uart.in_buffer.push_back(byte);
            on_read(uart, byte);
            uart.update_out_buffer();
        };
    }

    fn on_uart_write_done(user_data: *const c_void) {
        if let Some((uart, _)) = UartManager::get(UartManager::get_id(user_data)) {
            uart.update_out_buffer();
        }
    }
}

impl Uart {
    fn update_out_buffer(&mut self) {
        if !self.out_buffer.is_empty() {
            let data = self.out_buffer.clone();

            let did_write = unsafe {
                uartWrite(
                    self.device_id,
                    data.as_ptr() as *const u8,
                    data.len() as u32,
                )
            };

            if did_write {
                self.out_buffer.clear();
            }
        }
    }
}

impl Uart {
    pub fn init(pin_tx: &str, pin_rx: &str, baud_rate: u32, on_read: UartOnReadHandler) -> UartId {
        debug_print_string("Initializing ...".to_string());

        let id: UartId = UartManager::get_next_id();

        let config = UARTConfig {
            rx: unsafe { pinInit(CString::new(pin_rx).unwrap().into_raw(), INPUT_PULLUP) },
            tx: unsafe { pinInit(CString::new(pin_tx).unwrap().into_raw(), INPUT) },
            user_data: id as *const c_void,
            baud_rate,
            rx_data: UartManager::on_uart_rx_data as *const c_void,
            write_done: UartManager::on_uart_write_done as *const c_void,
        };

        let device_id = unsafe { uartInit(&config) };

        UartManager::register(
            Uart {
                id,
                config,
                device_id,
                in_buffer: VecDeque::new(),
                out_buffer: Vec::new(),
            },
            on_read,
        );

        debug_print_string(format!("Initialized on uart port: {}!", device_id));

        return id;
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
