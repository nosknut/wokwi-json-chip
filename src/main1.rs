/*
    File is called main1 as it need to remain lib for chipInit()
    main.rs would require a main however this runs off chipInit()
*/

use std::ffi::CString;

use serde_json::Value;
use wokwi_chip_ll::{pinInit, INPUT, INPUT_PULLUP};

use crate::{
    traits::UartJson,
    uart_tx::UartTX,
    uart_wrapper::{init_uart_json, UartSettings},
    utils::debug_print_string,
};

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn chipInit() {
    let settings = UartSettings {
        tx: unsafe { pinInit(CString::new("TX").unwrap().into_raw(), INPUT) },
        rx: unsafe { pinInit(CString::new("RX").unwrap().into_raw(), INPUT_PULLUP) },
        baud_rate: 115200,
    };

    init_uart_json(ServoUart, settings)
}

pub struct ServoUart;

impl UartJson for ServoUart {
    fn rx(&mut self, transmitter: &mut UartTX, json: Value) {
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
        transmitter.write_bytes(response.to_string().into_bytes());
    }
}
