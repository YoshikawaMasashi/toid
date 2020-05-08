use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::super::data::music_info::Track;
use super::super::state_management::serialize;
use super::super::state_management::state::State;
use super::scheduling_state::{SchedulingState, SchedulingStateEvent};
use super::sf2_state::{SF2State, SF2StateEvent};

#[derive(Serialize, Deserialize)]
pub struct MusicState {
    pub scheduling: Arc<SchedulingState>,
    pub track_map: HashMap<String, Track>,
    pub sf2: Arc<SF2State>,
}

impl MusicState {
    fn new_track(&self, key: String, track: Track) -> Self {
        let mut new_track_map = self.track_map.clone();
        new_track_map.insert(key, track);
        Self {
            scheduling: self.scheduling.clone(),
            track_map: new_track_map,
            sf2: self.sf2.clone(),
        }
    }

    fn scheduling_state_event(&self, e: SchedulingStateEvent) -> Self {
        let new_scheduling = Arc::new(self.scheduling.reduce(e));
        Self {
            scheduling: new_scheduling,
            track_map: self.track_map.clone(),
            sf2: self.sf2.clone(),
        }
    }

    fn sf2_state_event(&self, e: SF2StateEvent) -> Self {
        let new_sf2 = Arc::new(self.sf2.reduce(e));
        Self {
            scheduling: self.scheduling.clone(),
            track_map: self.track_map.clone(),
            sf2: new_sf2,
        }
    }
}

impl State<MusicStateEvent> for MusicState {
    fn new() -> Self {
        Self {
            scheduling: Arc::new(SchedulingState::new()),
            track_map: HashMap::new(),
            sf2: Arc::new(SF2State::new()),
        }
    }

    fn reduce(&self, event: MusicStateEvent) -> Self {
        match event {
            MusicStateEvent::NewTrack(key, track) => self.new_track(key, track),
            MusicStateEvent::SchedulingStateEvent(e) => self.scheduling_state_event(e),
            MusicStateEvent::SF2StateEvent(e) => self.sf2_state_event(e),
        }
    }
}

impl serialize::Serialize<MusicState> for MusicState {
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
pub enum MusicStateEvent {
    NewTrack(String, Track),
    SchedulingStateEvent(SchedulingStateEvent),
    SF2StateEvent(SF2StateEvent),
}

impl serialize::Serialize<MusicStateEvent> for MusicStateEvent {
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
