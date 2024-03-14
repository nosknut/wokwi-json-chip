use wokwi_chip_ll::PinId;

pub struct UartPins {
    pub rx: PinId,
    pub tx: PinId,
    pub baud_rate: u32,
}
