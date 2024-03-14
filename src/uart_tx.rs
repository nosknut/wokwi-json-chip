use wokwi_chip_ll::uartWrite;

use crate::utils::{debug_print_string, UartId};

pub struct UartTX {
    device_id: UartId,
    out_buffer: Vec<u8>,
}

impl UartTX {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            device_id: u32::MAX,
            out_buffer: Vec::new(),
        }
    }

    pub(crate) fn update_id(&mut self, device_id: UartId) {
        self.device_id = device_id
    }

    pub(crate) fn try_write(&mut self) {
        if !self.out_buffer.is_empty() {
            let data = self.out_buffer.clone();

            debug_print_string(format!("{}", self.device_id));
            let did_write = unsafe { uartWrite(self.device_id, data.as_ptr(), data.len() as u32) };

            if did_write {
                self.out_buffer.clear();
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.out_buffer.push(byte);
        self.try_write();
    }

    pub fn write_bytes(&mut self, bytes: Vec<u8>) {
        self.out_buffer.extend(bytes);
        self.try_write();
    }
}
