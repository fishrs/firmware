use std::{error::Error, fs::File, io::BufReader};

use datafmt::LakeData;
use mappy::lake_reader::depth_lines::DepthLines;
use unda::core::network::Network;

pub struct FishRs {
    model: Network,
    response_thresh: f32,
    depth_reader: DepthLines
}

impl FishRs {
    pub fn new(model_file: &str, depth_file: &str, res_thresh: f32) -> Result<Self, Box<dyn Error>> {
        let fish_file = File::open(depth_file)?;

        let reader = BufReader::new(fish_file);
        Ok(Self {
            model: Network::load(model_file),
            depth_reader: serde_json::from_reader(reader)?,
            response_thresh: res_thresh
        })
    }

    pub fn beat(&mut self, state: LakeData) -> bool {
        let depth = self.depth_reader.get_depth_at(state.coords.export());

        let input = vec![depth as f32, state.pull as f32];
        let model_response = self.model.predict(&input);

        model_response[0] >= self.response_thresh 
    }
}
