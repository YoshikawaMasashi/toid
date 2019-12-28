use std::collections::HashMap;
use std::sync::Arc;

use super::super::sf2;
use super::preset::Preset;
use super::sample::{Sample, SampleType};

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

fn parsed_sf2_to_own_sf2(parsed_sf2: sf2::SF2) -> SF2 {
    let own_sf2 = SF2::new();
    let sample_access = Arc::clone(&parsed_sf2.sdta.smpl);

    let mut sample_id_to_sample = HashMap::new();
    for (sample_i, sample_header) in parsed_sf2.pdta.shdr.iter().enumerate() {
        let sample = Sample {
            sample_access: Arc::clone(&sample_access),
            name: sample_header.name.clone(),
            start: sample_header.start,
            end: sample_header.end,
            loopstart: sample_header.loopstart,
            loopend: sample_header.end,
            sample_rate: sample_header.sample_rate,
            original_key: sample_header.original_key,
            correction: sample_header.correction,
            sample_link: None,
            typee: SampleType::from_flg(sample_header.typee).unwrap(),
        };
        let sample = Arc::new(sample);
        sample_id_to_sample.insert(sample_i, sample);
    }

    own_sf2
}
