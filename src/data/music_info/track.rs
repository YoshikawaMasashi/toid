use serde::{Deserialize, Serialize};

use super::Instrument;
use super::Note;
use super::Phrase;
use super::super::super::music_state::effects::EffectInfo;

#[derive(Serialize, Deserialize, Clone)]
pub struct Track<N: Note + Ord + Eq + Clone> {
    pub phrase: Phrase<N>,
    pub instrument: Instrument,
    pub effects: Vec<EffectInfo>,
    pub vol: f32, // 0.0 ~ 1.0
    pub pan: f32, // -1.0(L) ~ 1.0(R)
}

impl<N: Note + Ord + Eq + Clone> Track<N> {
    pub fn new() -> Self {
        Self {
            phrase: Phrase::new(),
            instrument: Instrument::Sin,
            effects: vec![],
            vol: 1.0,
            pan: 0.0,
        }
    }

    pub fn set_phrase(&self, phrase: Phrase<N>) -> Self {
        Self {
            phrase,
            instrument: self.instrument.clone(),
            effects: self.effects.clone(),
            vol: self.vol,
            pan: self.pan,
        }
    }

    pub fn set_inst(&self, instrument: Instrument) -> Self {
        Self {
            phrase: self.phrase.clone(),
            instrument,
            effects: self.effects.clone(),
            vol: self.vol,
            pan: self.pan,
        }
    }

    pub fn set_vol(&self, vol: f32) -> Self {
        Self {
            phrase: self.phrase.clone(),
            instrument: self.instrument.clone(),
            effects: self.effects.clone(),
            vol,
            pan: self.pan,
        }
    }

    pub fn set_pan(&self, pan: f32) -> Self {
        Self {
            phrase: self.phrase.clone(),
            instrument: self.instrument.clone(),
            effects: self.effects.clone(),
            vol: self.vol,
            pan,
        }
    }
}
