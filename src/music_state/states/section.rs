use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::super::super::data::music_info::{SampleTrack, Track};
use super::super::super::state_management::serialize;
use super::super::super::state_management::state::State;

#[derive(Serialize, Deserialize)]
pub struct SectionState {
    pub track_map: HashMap<String, Track>,
    pub sample_track_map: HashMap<String, SampleTrack>,
}

impl SectionState {
    fn new_track(&self, key: String, track: Track) -> Self {
        let mut new_track_map = self.track_map.clone();
        new_track_map.insert(key, track);
        Self {
            track_map: new_track_map,
            sample_track_map: self.sample_track_map.clone(),
        }
    }

    fn new_sample_track(&self, key: String, track: SampleTrack) -> Self {
        let mut new_sample_track_map = self.sample_track_map.clone();
        new_sample_track_map.insert(key, track);
        Self {
            track_map: self.track_map.clone(),
            sample_track_map: new_sample_track_map,
        }
    }

    pub fn get_track(&self, key: String) -> Option<Track> {
        self.track_map.get(&key).cloned()
    }
}

impl State<SectionStateEvent> for SectionState {
    fn new() -> Self {
        Self {
            track_map: HashMap::new(),
            sample_track_map: HashMap::new(),
        }
    }

    fn reduce(&self, event: SectionStateEvent) -> Self {
        match event {
            SectionStateEvent::NewTrack(key, track) => self.new_track(key, track),
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
    NewTrack(String, Track),
    NewSampleTrack(String, SampleTrack),
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
