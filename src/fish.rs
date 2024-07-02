use std::{error::Error, fs::File, io::BufReader};

use mappy::lake_reader::depth_lines::DepthLines;
use rppal::uart::Parity;
use unda::core::{data::input::Input, network::Network};

use crate::{inp::reader::{self, DataReader}, out::motor_driver::MotorDriver};

pub struct FishRs {
    reader: DataReader,
    output: MotorDriver,
    model: Network,
    response_thresh: f32,
    depth_reader: DepthLines
}

impl FishRs {
    pub fn new(reader_args: ((u32, Parity, u8, u8), u8), model_file: &str, depth_file: &str, res_thresh: f32) -> Result<Self, Box<dyn Error>> {
        let fish_file = File::open(depth_file)?;

        let reader = BufReader::new(fish_file);
        Ok(Self {
            reader: DataReader::new(reader_args.0, reader_args.1)?,
            output: MotorDriver {},
            model: Network::load(model_file),
            depth_reader: serde_json::from_reader(reader)?,
            response_thresh: res_thresh
        })
    }

    pub fn beat(&mut self) {
        let ((lon, lat), strain) = self.reader.read();
        let depth = self.depth_reader.get_depth_at((lon, lat));

        let input = vec![depth as f32, strain as f32];
        let model_response = self.model.predict(&input);

        if model_response[0] >= self.response_thresh {
            todo!("Reel in with motor")
        }
    }
}
