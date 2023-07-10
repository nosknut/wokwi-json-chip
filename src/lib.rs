// Wokwi Custom Chips with Rust
//
// Very rough prototype by Uri Shaked
//
// Look at chipInit() at the bottom, and open Chrome devtools console to see the debugPrint().

pub mod json_parser;
pub mod uart;

use json_parser::JsonParser;
use uart::{debug_print_string, Uart};

static mut PARSER: Option<JsonParser> = None;

#[no_mangle]
pub unsafe extern "C" fn chipInit() {
    PARSER = Some(JsonParser::new());

    Uart::init("TX", "RX", 115200, |uart, _c| {
        if let Ok(Some(json)) = PARSER.as_mut().unwrap().parse_uart(uart) {
            debug_print_string(format!("Received Json:\n{}\n", json));
            uart.write_string(format!("{}", json));
        }
    });
}
