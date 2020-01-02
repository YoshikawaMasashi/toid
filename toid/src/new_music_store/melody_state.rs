use std::sync::Arc;

use im::ordmap::OrdMap;
use serde::{Deserialize, Serialize};

use super::super::new_state_management::reducer::Reducer;
use super::super::new_state_management::serialize;

pub enum CurrentMelodyState {
    On(f32, i64),
    Off,
}

impl Clone for CurrentMelodyState {
    fn clone(&self) -> Self {
        match self {
            CurrentMelodyState::On(f, i) => CurrentMelodyState::On(*f, *i),
            CurrentMelodyState::Off => CurrentMelodyState::Off,
        }
    }
}

pub enum MelodyEvent {
    On(f32),
    Off,
}

impl Clone for MelodyEvent {
    fn clone(&self) -> Self {
        match self {
            MelodyEvent::On(f) => MelodyEvent::On(*f),
            MelodyEvent::Off => MelodyEvent::Off,
        }
    }
}

pub struct MelodyState {
    pub event_seq: OrdMap<i64, MelodyEvent>,
    pub current_melody: CurrentMelodyState,
}

impl MelodyState {
    pub fn new() -> Self {
        MelodyState {
            event_seq: OrdMap::new(),
            current_melody: CurrentMelodyState::Off,
        }
    }

    pub fn add_new_note_on_event(&self, pitch: f32, samples: i64) -> Self {
        MelodyState {
            event_seq: self.event_seq.update(samples, MelodyEvent::On(pitch)),
            current_melody: self.current_melody.clone(),
        }
    }

    pub fn add_new_note_off_event(&self, samples: i64) -> Self {
        MelodyState {
            event_seq: self.event_seq.update(samples, MelodyEvent::Off),
            current_melody: self.current_melody.clone(),
        }
    }

    pub fn change_current_melody_note_on(&self, pitch: f32, current_samples: i64) -> Self {
        MelodyState {
            event_seq: self.event_seq.clone(),
            current_melody: CurrentMelodyState::On(pitch, current_samples),
        }
    }

    pub fn change_current_melody_note_off(&self) -> Self {
        MelodyState {
            event_seq: self.event_seq.clone(),
            current_melody: CurrentMelodyState::Off,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum MelodyStateEvent {
    AddNewNoteOn(f32, i64),
    AddNewNoteOff(i64),
    ChangeCurrentMelodyNoteOn(f32, i64),
    ChangeCurrentMelodyNoteOff,
}

impl serialize::Serialize<MelodyStateEvent> for MelodyStateEvent {
    fn serialize(&self) -> Result<String, String> {
        if let Ok(serialized) = serde_json::to_string(&self) {
            Ok(serialized)
        } else {
            Err(String::from("error in serizalization"))
        }
    }
    fn deserialize(serialized: String) -> Result<MelodyStateEvent, String> {
        if let Ok(string) = serde_json::from_str(serialized.as_str()) {
            Ok(string)
        } else {
            Err(String::from("error in deserizalization"))
        }
    }
}

pub struct MelodyStateReducer {}

impl Reducer<MelodyState, MelodyStateEvent> for MelodyStateReducer {
    fn reduce(&self, state: Arc<MelodyState>, event: MelodyStateEvent) -> MelodyState {
        match event {
            MelodyStateEvent::AddNewNoteOn(pitch, samples) => {
                state.add_new_note_on_event(pitch, samples)
            }
            MelodyStateEvent::AddNewNoteOff(samples) => state.add_new_note_off_event(samples),
            MelodyStateEvent::ChangeCurrentMelodyNoteOn(pitch, current_samples) => {
                state.change_current_melody_note_on(pitch, current_samples)
            }
            MelodyStateEvent::ChangeCurrentMelodyNoteOff => state.change_current_melody_note_off(),
        }
    }
}
