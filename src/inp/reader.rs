use std::error::Error;

use rppal::uart::Parity;

use super::{gps::Gps, strain::StrainGauge};

pub struct DataReader {
    gps: Gps,
    strain: StrainGauge
}

impl DataReader {
    pub fn new(gps_args: (u32, Parity, u8, u8), strain_pin: u8) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            gps: Gps::new(gps_args)?,
            strain: StrainGauge::new(strain_pin)?
        })
    }

    pub fn read(&self) -> ((f64, f64), f64) {
        let coords = self.gps.get_lon_lat();
        let strain = self.strain.get_strain();

        (coords, strain)
    }
}
