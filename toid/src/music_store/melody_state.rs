use std::sync::Arc;

use im::ordmap::OrdMap;
use serde::{Deserialize, Serialize};

use super::super::state_management::reducer::Reducer;
use super::super::state_management::serialize;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NoteInfo {
    pitch: f32,
    duration: i64,
    start: i64,
}

pub struct MelodyState {
    pub notes: OrdMap<i64, NoteInfo>,
    pub repeat_length: Option<i64>,
    pub repeat_start: i64,
}

impl MelodyState {
    pub fn new() -> Self {
        MelodyState {
            notes: OrdMap::new(),
            repeat_length: Some(4 * 44100),
            repeat_start: 0,
        }
    }

    pub fn add_note(&self, note: NoteInfo) -> Self {
        MelodyState {
            notes: self.notes.update(note.start, note),
            repeat_length: Some(4 * 44100),
            repeat_start: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum MelodyStateEvent {
    AddNote(NoteInfo),
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
            MelodyStateEvent::AddNote(note) => state.add_note(note),
        }
    }
}
