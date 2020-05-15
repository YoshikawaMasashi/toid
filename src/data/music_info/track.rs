use serde::{Deserialize, Serialize};

use super::Phrase;

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    pub phrase: Phrase,
    pub sf2_name: Option<String>,
    pub vol: f32, // 0.0 ~ 1.0
    pub pan: f32, // -1.0(L) ~ 1.0(R)
}

impl Track {
    pub fn new() -> Self {
        Self {
            phrase: Phrase::new(),
            sf2_name: None,
            vol: 1.0,
            pan: 0.0,
        }
    }

    pub fn set_phrase(&self, phrase: Phrase) -> Self {
        Self {
            phrase,
            sf2_name: self.sf2_name.clone(),
            vol: self.vol,
            pan: self.pan,
        }
    }

    pub fn set_sf2_name(&self, sf2_name: Option<String>) -> Self {
        Self {
            phrase: self.phrase.clone(),
            sf2_name,
            vol: self.vol,
            pan: self.pan,
        }
    }

    pub fn set_vol(&self, vol: f32) -> Self {
        Self {
            phrase: self.phrase.clone(),
            sf2_name: self.sf2_name.clone(),
            vol,
            pan: self.pan,
        }
    }

    pub fn set_pan(&self, pan: f32) -> Self {
        Self {
            phrase: self.phrase.clone(),
            sf2_name: self.sf2_name.clone(),
            vol: self.vol,
            pan,
        }
    }
}
