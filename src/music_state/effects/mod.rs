mod convolution;
pub mod fft;
pub mod ring_buffer;
mod to_left;

use serde::{Deserialize, Serialize};

use convolution::ConvolutionEffect;
use to_left::ToLeftEffect;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum EffectInfo {
    ToLeftEffect,
    ConvolutionEffect,
}

impl EffectInfo {
    pub fn get_effect(&self) -> Box<dyn Effect + Sync + Send> {
        match self {
            EffectInfo::ToLeftEffect => Box::new(ToLeftEffect {}) as Box<dyn Effect + Sync + Send>,
            EffectInfo::ConvolutionEffect => {
                Box::new(ConvolutionEffect::new(&vec![0.0; 512])) as Box<dyn Effect + Sync + Send>
            }
        }
    }
}

pub trait Effect {
    fn effect(&mut self, left_wave: &Vec<f32>, right_wave: &Vec<f32>) -> (Vec<f32>, Vec<f32>);
}
