pub mod json_parser;
pub mod uart;

use json_parser::JsonParser;

use uart::{debug_print_string, Uart};

static mut PARSER: Option<JsonParser> = None;

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn chipInit() {
    PARSER = Some(JsonParser::new());

    Uart::init("TX", "RX", 115200, |uart, _c| {
        if let Ok(Some(json)) = PARSER.as_mut().unwrap().parse_uart(uart) {
            debug_print_string(format!("Received: {:?}", json));

            let response = match json["topic"].as_str().unwrap() {
                "servo/init" => {
                    serde_json::json!({
                        "topic": "servo/status",
                        "position": 50
                    })
                }
                "servo/target-position" => {
                    serde_json::json!({
                        "topic": "servo/status",
                        "position": json["position"]
                    })
                }
                _ => {
                    serde_json::json!({
                        "topic": "servo/error",
                        "message": format!("Unknown topic: {}", json["topic"].as_str().unwrap_or("None").to_owned())
                    })
                }
            };

            debug_print_string(format!("Sent: {}", response));
            uart.write_string(response.to_string());
        }
    });
}
