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
    ///
    /// Default implementation will print JSON error and the string
    fn err(&mut self, transmitter: &mut UartTX, string: String) {
        // This is just here because clippy gets mad
        let _ = transmitter;
        debug_print_string(format!("JSON serialize error: {}", string));
    }
}

/// # Internal
///
/// This holds UartJson and converts standard Uart output to UartJson
///
/// This is not a trait or external usable it only wraps a external UartJson
///
/// It does this but itself being a struct that implements
pub(crate) struct UartJsonInner<T: UartJson> {
    /// Storage of the UartJson
    ///
    /// Used to retain it for the UartJson::RX aka output
    inner: T,
    /// Json string built from Uart::RX
    json: String,
    /// Indent counter to count how many brackets deep the json is
    indent: i32,
}

impl<T: UartJson> UartJsonInner<T> {
    /// Makes UartJsonInner from a UartJson
    ///
    /// Read UartJsonInner comment for more info
    pub fn new(uart: T) -> Self {
        Self {
            inner: uart,
            indent: 0,
            json: String::default(),
        }
    }
}

impl<T: UartJson> Uart for UartJsonInner<T> {
    /// On rx process the new byte aka a char and check if it forms proper json
    fn rx(&mut self, transmitter: &mut UartTX, byte: u8) {
        // Converts the byte to its char
        let c = byte as char;
        // Push the char into the json string
        self.json.push(c);

        match c {
            // if the char is '{' then the indent increased by one
            '{' => self.indent += 1,
            // if the char is '{' then the indent decreased by by one
            '}' => self.indent -= 1,
            _ => {}
        }

        // Check if indent was reset to zero aka the last {} was closed
        if self.indent == 0 {
            // Check if the json parsed properly
            let Ok(json) = serde_json::from_str::<Value>(&self.json) else {
                // If it doesn't
                // clear the json string to prevent future errors
                let fail = self.json.drain(..).collect::<String>();
                // use error callback on string that errored and return
                self.inner.err(transmitter, fail);
                return;
            };

            // If json is parsed properly use rx callback
            self.inner.rx(transmitter, json);

            // Clear json to prevent double reads
            self.json.clear();
        }
    }
}
