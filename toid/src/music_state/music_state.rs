use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::super::state_management::serialize;
use super::super::state_management::state::State;
use super::melody_state::{MelodyState, MelodyStateEvent};
use super::scheduling_state::{SchedulingState, SchedulingStateEvent};
use super::sf2_state::{SF2State, SF2StateEvent};

pub struct MusicState {
    pub scheduling: Arc<SchedulingState>,
    pub melody_map: HashMap<String, Arc<MelodyState>>,
    pub sf2: Arc<SF2State>,
}

impl MusicState {
    pub fn new() -> Self {
        Self {
            scheduling: Arc::new(SchedulingState::new()),
            melody_map: HashMap::new(),
            sf2: Arc::new(SF2State::new()),
        }
    }

    fn new_melody(&self, key: String) -> Self {
        let mut new_melody_map = self.melody_map.clone();
        new_melody_map.insert(key, Arc::new(MelodyState::new()));
        Self {
            scheduling: self.scheduling.clone(),
            melody_map: new_melody_map,
            sf2: self.sf2.clone(),
        }
    }

    fn scheduling_state_event(&self, e: SchedulingStateEvent) -> Self {
        let new_scheduling = Arc::new(self.scheduling.reduce(e));
        Self {
            scheduling: new_scheduling,
            melody_map: self.melody_map.clone(),
            sf2: self.sf2.clone(),
        }
    }

    fn melody_state_event(&self, key: String, e: MelodyStateEvent) -> Self {
        let mut new_melody_map = self.melody_map.clone();
        let new_melody = Arc::new(self.melody_map[&key].reduce(e));
        new_melody_map.insert(key, new_melody);
        Self {
            scheduling: self.scheduling.clone(),
            melody_map: new_melody_map,
            sf2: self.sf2.clone(),
        }
    }

    fn sf2_state_event(&self, e: SF2StateEvent) -> Self {
        let new_sf2 = Arc::new(self.sf2.reduce(e));
        Self {
            scheduling: self.scheduling.clone(),
            melody_map: self.melody_map.clone(),
            sf2: new_sf2,
        }
    }
}

impl State<MusicStateEvent> for MusicState {
    fn reduce(&self, event: MusicStateEvent) -> Self {
        match event {
            MusicStateEvent::NewMelody(key) => self.new_melody(key),
            MusicStateEvent::SchedulingStateEvent(e) => self.scheduling_state_event(e),
            MusicStateEvent::MelodyStateEvent(key, e) => self.melody_state_event(key, e),
            MusicStateEvent::SF2StateEvent(e) => self.sf2_state_event(e),
        }
    }
}
#[derive(Serialize, Deserialize)]
pub enum MusicStateEvent {
    NewMelody(String),
    SchedulingStateEvent(SchedulingStateEvent),
    MelodyStateEvent(String, MelodyStateEvent),
    SF2StateEvent(SF2StateEvent),
}

impl serialize::Serialize<MusicStateEvent> for MusicStateEvent {
    fn serialize(&self) -> Result<String, String> {
        if let Ok(serialized) = serde_json::to_string(&self) {
            Ok(serialized)
        } else {
            Err(String::from("error in serizalization"))
        }
    }
    fn deserialize(serialized: String) -> Result<Self, String> {
        if let Ok(string) = serde_json::from_str(serialized.as_str()) {
            Ok(string)
        } else {
            Err(String::from("error in deserizalization"))
        }
    }
}
