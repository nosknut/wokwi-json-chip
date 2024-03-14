use crate::uart_tx::UartTX;

/// This is a trait for handling Uarts
///
/// Although it could just be a function callback this is a more all encompassing solution.
/// This allows for both data persistance and ease of use
///
/// Other Uart types derive and build upon this
pub trait Uart: 'static {
    /// Runs on every RX
    ///
    /// Please note this runs on EVERY byte
    fn rx(&mut self, transmitter: &mut UartTX, byte: u8);
}
