mod convolution;
mod fft;
mod to_left;

use serde::{Deserialize, Serialize};

use to_left::ToLeftEffect;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum EffectInfo {
    ToLeftEffect,
}

impl EffectInfo {
    pub fn get_effect(&self) -> Box<dyn Effect + Sync + Send> {
        match self {
            EffectInfo::ToLeftEffect => Box::new(ToLeftEffect {}) as Box<dyn Effect + Sync + Send>,
        }
    }
}

pub trait Effect {
    fn effect(&mut self, left_wave: &Vec<f32>, right_wave: &Vec<f32>) -> (Vec<f32>, Vec<f32>);
}
