use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::ops::Bound::Included;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::super::super::data::music_info::Beat;
use super::super::super::state_management::serialize;
use super::super::super::state_management::state::State;
use super::scheduling::{SchedulingState, SchedulingStateEvent};
use super::section::{SectionState, SectionStateEvent};

#[derive(Serialize, Deserialize)]
pub struct MusicState {
    pub scheduling: Arc<SchedulingState>,
    pub section_map: BTreeMap<Beat, Arc<SectionState>>,
}

impl MusicState {
    fn section_state_event(&self, beat: Beat, e: SectionStateEvent) -> Self {
        let mut new_section_map = self.section_map.clone();

        match self
            .section_map
            .range((Included(&Beat::from(0)), Included(&beat)))
            .rev()
            .next()
        {
            Some((&change_beat, section_state)) => {
                new_section_map.insert(change_beat, Arc::new(section_state.reduce(e)));
            }
            None => {}
        }
        Self {
            scheduling: Arc::clone(&self.scheduling),
            section_map: new_section_map,
        }
    }

    fn scheduling_state_event(&self, e: SchedulingStateEvent) -> Self {
        let new_scheduling = Arc::new(self.scheduling.reduce(e));
        Self {
            scheduling: new_scheduling,
            section_map: self.section_map.clone(),
        }
    }

    fn new_section(&self, beat: Beat) -> Self {
        let mut new_section_map = self.section_map.clone();
        let section = Arc::new(SectionState::new());
        new_section_map.insert(beat, section);
        Self {
            scheduling: Arc::clone(&self.scheduling),
            section_map: new_section_map,
        }
    }

    pub fn get_section_state_by_beat(&self, beat: Beat) -> Arc<SectionState> {
        Arc::clone(
            self.section_map
                .range((Included(&Beat::from(0)), Included(&beat)))
                .rev()
                .next()
                .unwrap()
                .1,
        )
    }

    pub fn get_section_beats(&self) -> Vec<Beat> {
        Vec::from_iter(self.section_map.keys().cloned())
    }
}

impl State<MusicStateEvent> for MusicState {
    fn new() -> Self {
        let mut section_map = BTreeMap::new();
        section_map.insert(Beat::from(0), Arc::new(SectionState::new()));
        Self {
            scheduling: Arc::new(SchedulingState::new()),
            section_map,
        }
    }

    fn reduce(&self, event: MusicStateEvent) -> Self {
        match event {
            MusicStateEvent::SectionStateEvent(beat, e) => self.section_state_event(beat, e),
            MusicStateEvent::SchedulingStateEvent(e) => self.scheduling_state_event(e),
            MusicStateEvent::NewSection(beat) => self.new_section(beat),
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
    SectionStateEvent(Beat, SectionStateEvent),
    SchedulingStateEvent(SchedulingStateEvent),
    NewSection(Beat),
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
