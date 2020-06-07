use std::collections::HashMap;
use std::iter::FromIterator;

use serde::{Deserialize, Serialize};

use super::super::super::data::music_info::{PitchNote, SampleNote, Track};
use super::super::super::state_management::serialize;
use super::super::super::state_management::state::State;

#[derive(Serialize, Deserialize)]
pub struct SectionState {
    pub pitch_track_map: HashMap<String, Track<PitchNote>>,
    pub sample_track_map: HashMap<String, Track<SampleNote>>,
}

impl SectionState {
    fn new_pitch_track(&self, key: String, track: Track<PitchNote>) -> Self {
        let mut new_pitch_track_map = self.pitch_track_map.clone();
        new_pitch_track_map.insert(key, track);
        Self {
            pitch_track_map: new_pitch_track_map,
            sample_track_map: self.sample_track_map.clone(),
        }
    }

    fn new_sample_track(&self, key: String, track: Track<SampleNote>) -> Self {
        let mut new_sample_track_map = self.sample_track_map.clone();
        new_sample_track_map.insert(key, track);
        Self {
            pitch_track_map: self.pitch_track_map.clone(),
            sample_track_map: new_sample_track_map,
        }
    }

    pub fn get_pitch_track(&self, key: String) -> Option<Track<PitchNote>> {
        self.pitch_track_map.get(&key).cloned()
    }

    pub fn get_sample_track(&self, key: String) -> Option<Track<SampleNote>> {
        self.sample_track_map.get(&key).cloned()
    }

    pub fn get_pitch_track_names(&self) -> Vec<String> {
        Vec::from_iter(self.pitch_track_map.keys().cloned())
    }

    pub fn get_sample_track_names(&self) -> Vec<String> {
        Vec::from_iter(self.sample_track_map.keys().cloned())
    }
}

impl State<SectionStateEvent> for SectionState {
    fn new() -> Self {
        Self {
            pitch_track_map: HashMap::new(),
            sample_track_map: HashMap::new(),
        }
    }

    fn reduce(&self, event: SectionStateEvent) -> Self {
        match event {
            SectionStateEvent::NewPitchTrack(key, track) => self.new_pitch_track(key, track),
            SectionStateEvent::NewSampleTrack(key, track) => self.new_sample_track(key, track),
        }
    }
}

impl serialize::Serialize<SectionState> for SectionState {
    fn serialize(&self) -> Result<String, String> {
        match serde_json::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(err) => Err(format!("error in serizalization : {}", err)),
        }
    }
    fn deserialize(serialized: String) -> Result<Self, String> {
        match serde_json::from_str(serialized.as_str()) {
            Ok(deserialized) => Ok(deserialized),
            Err(err) => Err(format!("error in deserizalization : {}", err)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SectionStateEvent {
    NewPitchTrack(String, Track<PitchNote>),
    NewSampleTrack(String, Track<SampleNote>),
}

impl serialize::Serialize<SectionStateEvent> for SectionStateEvent {
    fn serialize(&self) -> Result<String, String> {
        match serde_json::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(err) => Err(format!("error in serizalization : {}", err)),
        }
    }
    fn deserialize(serialized: String) -> Result<Self, String> {
        match serde_json::from_str(serialized.as_str()) {
            Ok(deserialized) => Ok(deserialized),
            Err(err) => Err(format!("error in deserizalization : {}", err)),
        }
    }
}
