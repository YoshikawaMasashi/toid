use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::super::state_management::reducer::Reducer;
use super::super::state_management::serialize;
use super::melody_state::{MelodyState, MelodyStateEvent, MelodyStateReducer};
use super::scheduling_state::{SchedulingState, SchedulingStateEvent, SchedulingStateReducer};
use super::sf2_state::{SF2State, SF2StateEvent, SF2StateReducer};

pub struct MusicState {
    pub scheduling: Arc<SchedulingState>,
    pub melody_map: HashMap<String, Arc<MelodyState>>,
    pub sf2: Arc<SF2State>,
    scheduling_reducer: SchedulingStateReducer,
    melody_reducer: MelodyStateReducer,
    sf2_reducer: SF2StateReducer,
}

impl MusicState {
    pub fn new() -> Self {
        Self {
            scheduling: Arc::new(SchedulingState::new()),
            melody_map: HashMap::new(),
            sf2: Arc::new(SF2State::new()),
            scheduling_reducer: SchedulingStateReducer {},
            melody_reducer: MelodyStateReducer {},
            sf2_reducer: SF2StateReducer {},
        }
    }

    fn new_melody(&self, key: String) -> Self {
        let mut new_melody_map = self.melody_map.clone();
        new_melody_map.insert(key, Arc::new(MelodyState::new()));
        Self {
            scheduling: self.scheduling.clone(),
            melody_map: new_melody_map,
            sf2: self.sf2.clone(),
            scheduling_reducer: SchedulingStateReducer {},
            melody_reducer: MelodyStateReducer {},
            sf2_reducer: SF2StateReducer {},
        }
    }

    fn scheduling_state_event(&self, e: SchedulingStateEvent) -> Self {
        let new_scheduling = Arc::new(
            self.scheduling_reducer
                .reduce(Arc::clone(&self.scheduling), e),
        );
        Self {
            scheduling: new_scheduling,
            melody_map: self.melody_map.clone(),
            sf2: self.sf2.clone(),
            scheduling_reducer: SchedulingStateReducer {},
            melody_reducer: MelodyStateReducer {},
            sf2_reducer: SF2StateReducer {},
        }
    }

    fn melody_state_event(&self, key: String, e: MelodyStateEvent) -> Self {
        let mut new_melody_map = self.melody_map.clone();
        let new_melody = Arc::new(
            self.melody_reducer
                .reduce(Arc::clone(&self.melody_map[&key]), e),
        );
        new_melody_map.insert(key, new_melody);
        Self {
            scheduling: self.scheduling.clone(),
            melody_map: new_melody_map,
            sf2: self.sf2.clone(),
            scheduling_reducer: SchedulingStateReducer {},
            melody_reducer: MelodyStateReducer {},
            sf2_reducer: SF2StateReducer {},
        }
    }

    fn sf2_state_event(&self, e: SF2StateEvent) -> Self {
        let new_sf2 = Arc::new(self.sf2_reducer.reduce(Arc::clone(&self.sf2), e));
        Self {
            scheduling: self.scheduling.clone(),
            melody_map: self.melody_map.clone(),
            sf2: new_sf2,
            scheduling_reducer: SchedulingStateReducer {},
            melody_reducer: MelodyStateReducer {},
            sf2_reducer: SF2StateReducer {},
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

pub struct MusicStateReducer {}

impl Reducer<MusicState, MusicStateEvent> for MusicStateReducer {
    fn reduce(&self, state: Arc<MusicState>, event: MusicStateEvent) -> MusicState {
        match event {
            MusicStateEvent::NewMelody(key) => state.new_melody(key),
            MusicStateEvent::SchedulingStateEvent(e) => state.scheduling_state_event(e),
            MusicStateEvent::MelodyStateEvent(key, e) => state.melody_state_event(key, e),
            MusicStateEvent::SF2StateEvent(e) => state.sf2_state_event(e),
        }
    }
}
