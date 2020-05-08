use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::beat::Beat;
use super::note::Note;

#[derive(Serialize, Deserialize, Clone)]
pub struct Phrase {
    pub notes: BTreeMap<Beat, BTreeSet<Note>>,
    pub length: Beat,
}

impl Phrase {
    pub fn new() -> Self {
        Self {
            notes: BTreeMap::new(),
            length: Beat::from(0),
        }
    }

    pub fn add_note(&self, note: Note) -> Self {
        let mut new_notes = self.notes.clone();
        let mut new_note_set;
        if self.notes.contains_key(&note.start) {
            new_note_set = self.notes[&note.start].clone();
        } else {
            new_note_set = BTreeSet::new();
        }
        new_note_set.insert(note);
        new_notes.insert(note.start, new_note_set);
        Phrase {
            notes: new_notes,
            length: self.length,
        }
    }

    pub fn set_length(&self, length: Beat) -> Self {
        Phrase {
            notes: self.notes.clone(),
            length: length,
        }
    }
}

impl PartialEq for Phrase {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.notes == other.notes
    }
}

impl Eq for Phrase {}
