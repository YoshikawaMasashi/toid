use std::sync::Arc;

use im::ordmap::OrdMap;
use im::vector::Vector;
use serde::{Deserialize, Serialize};

use super::super::state_management::reducer::Reducer;
use super::super::state_management::serialize;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NoteInfo {
    pub pitch: f32,
    pub duration: u64,
    pub start: u64,
}

pub struct MelodyState {
    pub notes: OrdMap<u64, Vector<NoteInfo>>,
    pub repeat_length: u64,
}

impl MelodyState {
    pub fn new() -> Self {
        MelodyState {
            notes: OrdMap::new(),
            repeat_length: 4 * 44100,
        }
    }

    pub fn add_note(&self, note: NoteInfo) -> Self {
        if self.notes.contains_key(&note.start) {
            MelodyState {
                notes: self.notes.update(
                    note.start,
                    self.notes[&note.start].update(self.notes[&note.start].len(), note),
                ),
                repeat_length: self.repeat_length,
            }
        } else {
            MelodyState {
                notes: self.notes.update(note.start, Vector::new().update(0, note)),
                repeat_length: self.repeat_length,
            }
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
