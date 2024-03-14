use wokwi_chip_ll::uartWrite;

use crate::utils::UartId;

/// A handler for sending data via Uart
///
/// This simplifies data sending
pub struct UartTX {
    /// Device id given from init
    /// ## Warning!
    /// Default for moments will be u32::MAX before the id is assigned.
    ///
    /// Look at the init code for more info
    device_id: UartId,
    /// The out/TX data buffer
    out_buffer: Vec<u8>,
}

impl UartTX {
    #[allow(clippy::new_without_default)]
    /// Make a new empty UartTX
    ///
    /// ## Warning!
    /// Call to update_id needed
    pub(crate) fn new() -> Self {
        Self {
            device_id: u32::MAX,
            out_buffer: Vec::new(),
        }
    }

    /// Updates device id
    pub(crate) fn update_id(&mut self, device_id: UartId) {
        self.device_id = device_id
    }

    /// # Internal
    /// This is called with a external write or when a previous writing is complete
    ///
    /// Try writing to uart output
    pub(crate) fn try_write(&mut self) {
        // if buffer is empty end early
        if self.out_buffer.is_empty() {
            return;
        }

        // idk why clone is needed but it fails without it
        let data = self.out_buffer.clone();

        // Convert data to ptr for write
        let ptr = data.as_ptr();

        // write the data if possible
        let did_write = unsafe { uartWrite(self.device_id, ptr, data.len() as u32) };

        // if the write went through clear the out_buffer
        if did_write {
            self.out_buffer.clear();
        }
    }

    /// Write / Transmit a byte
    pub fn write_byte(&mut self, byte: u8) {
        self.out_buffer.push(byte);
        self.try_write();
    }

    /// Write / Transmit bytes (or a Vec of bytes)
    pub fn write_bytes(&mut self, bytes: Vec<u8>) {
        self.out_buffer.extend(bytes);
        self.try_write();
    }
}
