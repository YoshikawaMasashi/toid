use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::super::state_management::serialize;
use super::super::state_management::state::State;
use super::beat::Beat;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NoteInfo {
    pub pitch: f32,
    pub duration: Beat,
    pub start: Beat,
}

#[derive(Serialize, Deserialize)]
pub struct MelodyState {
    pub notes: BTreeMap<Beat, Vec<NoteInfo>>,
    pub repeat_length: Beat,
}

impl MelodyState {
    pub fn new() -> Self {
        MelodyState {
            notes: BTreeMap::new(),
            repeat_length: Beat::from(8),
        }
    }

    pub fn add_note(&self, note: NoteInfo) -> Self {
        let mut new_notes = self.notes.clone();
        let mut new_note_vec;
        if self.notes.contains_key(&note.start) {
            new_note_vec = self.notes[&note.start].clone();
        } else {
            new_note_vec = Vec::new();
        }
        new_note_vec.push(note);
        new_notes.insert(note.start, new_note_vec);
        MelodyState {
            notes: new_notes,
            repeat_length: self.repeat_length,
        }
    }
}

impl State<MelodyStateEvent> for MelodyState {
    fn reduce(&self, event: MelodyStateEvent) -> Self {
        match event {
            MelodyStateEvent::AddNote(note) => self.add_note(note),
        }
    }
}

impl serialize::Serialize<MelodyState> for MelodyState {
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
pub enum MelodyStateEvent {
    AddNote(NoteInfo),
}

impl serialize::Serialize<MelodyStateEvent> for MelodyStateEvent {
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
