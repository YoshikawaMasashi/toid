use serde::{Deserialize, Serialize};

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

pub struct ToLeftEffect {}

impl Effect for ToLeftEffect {
    fn effect(&mut self, left_wave: &Vec<f32>, right_wave: &Vec<f32>) -> (Vec<f32>, Vec<f32>) {
        let mut new_left_wave = vec![];
        let mut new_right_wave = vec![];
        for (l, r) in left_wave.iter().zip(right_wave.iter()) {
            new_left_wave.push(l + r);
            new_right_wave.push(0.0);
        }
        (new_left_wave, new_right_wave)
    }
}
