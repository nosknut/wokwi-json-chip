# Wokwi Json Chip in Rust example

## Building

To build:

```
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```
Find the resulting binary in `target/wasm32-unknown-unknown/release/chip_uart_json.wasm`

To run the demo project:

Select the wokwi config in the demo directory, use the Arduino Extension to compile the firmware and start the simulation.


## Simulating

To simulate this project, install [Wokwi for VS Code](https://marketplace.visualstudio.com/items?itemName=wokwi.wokwi-vscode). Open the project directory in Visual Studio Code, press **F1** and select "Wokwi: Start Simulator".
