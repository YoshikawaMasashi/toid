use std::collections::BTreeMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::super::state_management::reducer::Reducer;
use super::super::state_management::serialize;
use super::beat::Beat;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NoteInfo {
    pub pitch: f32,
    pub duration: Beat,
    pub start: Beat,
}

pub struct MelodyState {
    pub notes: BTreeMap<Beat, Vec<NoteInfo>>,
    pub repeat_length: Beat,
}

impl MelodyState {
    pub fn new() -> Self {
        MelodyState {
            notes: BTreeMap::new(),
            repeat_length: Beat::from(4),
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
