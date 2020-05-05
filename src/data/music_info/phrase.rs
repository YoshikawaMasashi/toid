use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::beat::Beat;
use super::note::Note;

#[derive(Serialize, Deserialize, Clone)]
pub struct Phrase {
    pub notes: BTreeMap<Beat, Vec<Note>>,
    pub repeat_length: Beat,
}

impl Phrase {
    pub fn add_note(&self, note: Note) -> Self {
        let mut new_notes = self.notes.clone();
        let mut new_note_vec;
        if self.notes.contains_key(&note.start) {
            new_note_vec = self.notes[&note.start].clone();
        } else {
            new_note_vec = Vec::new();
        }
        new_note_vec.push(note);
        new_notes.insert(note.start, new_note_vec);
        Phrase {
            notes: new_notes,
            repeat_length: self.repeat_length,
        }
    }

    pub fn set_repeat_length(&self, repeat_length: Beat) -> Self {
        Phrase {
            notes: self.notes.clone(),
            repeat_length: repeat_length,
        }
    }
}
