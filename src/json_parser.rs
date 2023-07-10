use std::borrow::Borrow;

use serde_json::{Error, Value};

use crate::uart::Uart;

pub struct JsonParser {
    json: String,
    num_curly_braces: i32,
}

impl JsonParser {
    pub fn new() -> JsonParser {
        JsonParser {
            json: String::from(""),
            num_curly_braces: 0,
        }
    }

    pub fn parse_uart(&mut self, uart: &mut Uart) -> Result<Option<Value>, Error> {
        while uart.available() > 0 {
            if let Some(c) = uart.read_char() {
                return self.parse(c);
            }
        }
        Ok(None)
    }

    pub fn parse(&mut self, c: char) -> Result<Option<Value>, Error> {
        if self.num_curly_braces < 0 {
            self.json.clear();
            self.num_curly_braces = 0;
            return Ok(None);
        }

        if self.num_curly_braces == 0 {
            if c != '{' {
                if !self.json.is_empty() {
                    self.json.clear();
                }
                return Ok(None);
            }
        }

        if c == '{' {
            self.num_curly_braces += 1;
        }

        if c == '}' {
            self.num_curly_braces -= 1;
        }

        self.json.push(c);

        if self.num_curly_braces == 0 {
            let content: Value = serde_json::from_str(self.json.borrow())?;
            self.json.clear();
            return Ok(Some(content));
        }

        Ok(None)
    }
}
