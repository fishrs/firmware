use std::error::Error;

use rppal::gpio::{Gpio, InputPin};

pub struct StrainGauge {
    pin: InputPin
}

impl StrainGauge {
    pub fn new(pin: u8) -> Result<Self, Box<dyn Error>> {
        let gpio = Gpio::new()?;

        Ok(Self { pin: gpio.get(pin)?.into_input() })
    }

    pub fn get_strain(&self) -> f64 {
        todo!("Not implemented yet :(")
    }
}
