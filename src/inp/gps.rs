use std::error::Error;

use rppal::uart::{Parity, Uart};

pub struct Gps {
    uart: Uart
}

impl Default for Gps {
    fn default() -> Self {
        Gps::new((9600, Parity::None, 8, 1)).expect("Default gps")
    }
}

impl Gps {
    pub fn new(uart: (u32, Parity, u8, u8)) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            uart: Uart::new(uart.0, uart.1, uart.2, uart.3)?
        })
    }

    pub fn get_lon_lat(&self) -> (f64, f64) {
        todo!("No implemented yet :(")
    }
}
