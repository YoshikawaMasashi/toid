use std::sync::Arc;

use super::preset::Preset;

pub struct SF2 {
    presets: Vec<Arc<Preset>>,
}

impl SF2 {
    pub fn new() -> Self {
        SF2 {
            presets: Vec::new(),
        }
    }

    pub fn add_preset(&mut self, preset: Arc<Preset>) {
        self.presets.push(preset);
    }
}
