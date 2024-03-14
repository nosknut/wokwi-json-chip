use serde_json::Value;

use crate::{uart_tx::UartTX, utils::debug_print_string};

use super::Uart;

/// This is a trait for handling Uarts using JSON
///
/// Read traits/uart.rs for more info
pub trait UartJson: 'static {
    /// Runs on good json parse
    fn rx(&mut self, transmitter: &mut UartTX, json: Value);

    /// Runs when there is a json parse error
    ///
    /// Not needed unless you want to handle errors
    fn err(&mut self, transmitter: &mut UartTX, string: String) {
        // This is just here because clippy gets mad
        let _ = transmitter;
        debug_print_string(format!("JSON serialize error: {}", string));
    }
}

pub(crate) struct UartJsonInner<T: UartJson> {
    inner: T,
    json: String,
    indent: i32,
}

impl<T: UartJson> UartJsonInner<T> {
    pub fn new(uart: T) -> Self {
        Self {
            inner: uart,
            indent: 0,
            json: String::default(),
        }
    }
}

impl<T: UartJson> Uart for UartJsonInner<T> {
    fn rx(&mut self, transmitter: &mut UartTX, byte: u8) {
        let c = byte as char;
        self.json.push(c);

        match c {
            '{' => {
                self.indent += 1;
            }
            '}' => {
                self.indent -= 1;
                //NOTE: code below should be here but would be less than readable
            }
            _ => {}
        }

        if self.indent == 0 {
            let Ok(json) = serde_json::from_str::<Value>(&self.json) else {
                // To prevent future errors
                let fail = self.json.drain(..).collect::<String>();
                self.inner.err(transmitter, fail);
                return;
            };

            self.inner.rx(transmitter, json);

            self.json.clear();
        }
    }
}
