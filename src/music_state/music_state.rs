use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::super::data::music_info::beat::Beat;
use super::super::state_management::serialize;
use super::super::state_management::state::State;
use super::phrase_state::{PhraseState, PhraseStateEvent};
use super::scheduling_state::{SchedulingState, SchedulingStateEvent};
use super::sf2_state::{SF2State, SF2StateEvent};

#[derive(Serialize, Deserialize)]
pub struct MusicState {
    pub scheduling: Arc<SchedulingState>,
    pub phrase_map: HashMap<String, Arc<PhraseState>>,
    pub sf2: Arc<SF2State>,
}

impl MusicState {
    fn new_phrase(&self, key: String, repeat_length: Beat) -> Self {
        let mut new_phrase_map = self.phrase_map.clone();
        new_phrase_map.insert(
            key,
            Arc::new(PhraseState::new().set_repeat_length(repeat_length)),
        );
        Self {
            scheduling: self.scheduling.clone(),
            phrase_map: new_phrase_map,
            sf2: self.sf2.clone(),
        }
    }

    fn scheduling_state_event(&self, e: SchedulingStateEvent) -> Self {
        let new_scheduling = Arc::new(self.scheduling.reduce(e));
        Self {
            scheduling: new_scheduling,
            phrase_map: self.phrase_map.clone(),
            sf2: self.sf2.clone(),
        }
    }

    fn phrase_state_event(&self, key: String, e: PhraseStateEvent) -> Self {
        let mut new_phrase_map = self.phrase_map.clone();
        let new_phrase = Arc::new(self.phrase_map[&key].reduce(e));
        new_phrase_map.insert(key, new_phrase);
        Self {
            scheduling: self.scheduling.clone(),
            phrase_map: new_phrase_map,
            sf2: self.sf2.clone(),
        }
    }

    fn sf2_state_event(&self, e: SF2StateEvent) -> Self {
        let new_sf2 = Arc::new(self.sf2.reduce(e));
        Self {
            scheduling: self.scheduling.clone(),
            phrase_map: self.phrase_map.clone(),
            sf2: new_sf2,
        }
    }
}

impl State<MusicStateEvent> for MusicState {
    fn new() -> Self {
        Self {
            scheduling: Arc::new(SchedulingState::new()),
            phrase_map: HashMap::new(),
            sf2: Arc::new(SF2State::new()),
        }
    }

    fn reduce(&self, event: MusicStateEvent) -> Self {
        match event {
            MusicStateEvent::NewPhrase(key, repeat_length) => self.new_phrase(key, repeat_length),
            MusicStateEvent::SchedulingStateEvent(e) => self.scheduling_state_event(e),
            MusicStateEvent::PhraseStateEvent(key, e) => self.phrase_state_event(key, e),
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
    NewPhrase(String, Beat),
    SchedulingStateEvent(SchedulingStateEvent),
    PhraseStateEvent(String, PhraseStateEvent),
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
