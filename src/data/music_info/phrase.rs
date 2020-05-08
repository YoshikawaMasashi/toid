use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::beat::Beat;
use super::note::Note;

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[cfg(test)]
mod tests {
    use super::super::{Beat, Note, Pitch};
    use super::*;

    #[test]
    fn test_eq() {
        let phrase1 = Phrase::new();

        let phrase1 = phrase1.add_note(Note {
            pitch: Pitch::from(60),
            start: Beat::from(0.0),
            duration: Beat::from(1.0),
        });
        let phrase1 = phrase1.add_note(Note {
            pitch: Pitch::from(62),
            start: Beat::from(1.0),
            duration: Beat::from(1.0),
        });
        let phrase2 = phrase1.clone();
        let phrase3 = phrase2.add_note(Note {
            pitch: Pitch::from(64),
            start: Beat::from(2.0),
            duration: Beat::from(1.0),
        });

        assert_eq!(phrase1, phrase2);
        assert_ne!(phrase1, phrase3);
    }
}
