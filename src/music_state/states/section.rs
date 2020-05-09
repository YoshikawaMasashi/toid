use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::super::super::data::music_info::Track;
use super::super::super::state_management::serialize;
use super::super::super::state_management::state::State;

#[derive(Serialize, Deserialize)]
pub struct SectionState {
    pub track_map: HashMap<String, Track>,
}

impl SectionState {
    fn new_track(&self, key: String, track: Track) -> Self {
        let mut new_track_map = self.track_map.clone();
        new_track_map.insert(key, track);
        Self {
            track_map: new_track_map,
        }
    }
}

impl State<SectionStateEvent> for SectionState {
    fn new() -> Self {
        Self {
            track_map: HashMap::new(),
        }
    }

    fn reduce(&self, event: SectionStateEvent) -> Self {
        match event {
            SectionStateEvent::NewTrack(key, track) => self.new_track(key, track),
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
