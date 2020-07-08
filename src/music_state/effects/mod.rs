mod convolution;
pub mod fft;
pub mod ring_buffer;
mod to_left;
mod schroeder_reverb;

use std::sync::Arc;

use log::error;
use serde::{Deserialize, Serialize};

use super::super::resource_management::resource_manager::ResourceManager;
use convolution::ConvolutionEffect;
use to_left::ToLeftEffect;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EffectInfo {
    ToLeftEffect,
    SamplingReverb(String, String, f32, f32),
}

impl EffectInfo {
    pub fn get_effect(
        &self,
        resource_manager: Arc<ResourceManager>,
    ) -> Box<dyn Effect + Sync + Send> {
        match self {
            EffectInfo::ToLeftEffect => Box::new(ToLeftEffect {}) as Box<dyn Effect + Sync + Send>,
            EffectInfo::SamplingReverb(sample_name, sound, dry, wet) => {
                let wave = resource_manager.get_sample_wave(sample_name.to_string(), sound.clone());
                match wave {
                    Ok(wave) => {
                        let sample_data = wave.get_samples(0, wave.sample_num);
                        match sample_data {
                            Ok((left_sample, _right_sample)) => {
                                // TODO: fix
                                let left_sample = left_sample.split_at(44100).0;
                                let left_sample = Vec::from(left_sample);
                                Box::new(ConvolutionEffect::new(&left_sample, *dry, *wet))
                                    as Box<dyn Effect + Sync + Send>
                            }
                            Err(e) => {
                                // TODO:
                                error!("error {}", e);
                                Box::new(ConvolutionEffect::new(&vec![0.0; 512], 1.0, 0.0))
                                    as Box<dyn Effect + Sync + Send>
                            }
                        }
                    }
                    Err(e) => {
                        // TODO:
                        error!("error {}", e);
                        Box::new(ConvolutionEffect::new(&vec![0.0; 512], 1.0, 0.0))
                            as Box<dyn Effect + Sync + Send>
                    }
                }
            }
        }
    }
}

pub trait Effect {
    fn effect(&mut self, left_wave: &Vec<f32>, right_wave: &Vec<f32>) -> (Vec<f32>, Vec<f32>);
}
