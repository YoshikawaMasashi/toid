use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::beat::Beat;
use super::sample_note::SampleNote;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SamplePhrase {
    pub notes: BTreeMap<Beat, BTreeSet<SampleNote>>,
    pub length: Beat,
}

impl SamplePhrase {
    pub fn new() -> Self {
        Self {
            notes: BTreeMap::new(),
            length: Beat::from(0),
        }
    }

    pub fn add_note(&self, note: SampleNote) -> Self {
        let mut new_notes = self.notes.clone();
        let mut new_note_set;
        if self.notes.contains_key(&note.start) {
            new_note_set = self.notes[&note.start].clone();
        } else {
            new_note_set = BTreeSet::new();
        }
        new_note_set.insert(note.clone());
        new_notes.insert(note.start, new_note_set);
        Self {
            notes: new_notes,
            length: self.length,
        }
    }

    pub fn set_length(&self, length: Beat) -> Self {
        Self {
            notes: self.notes.clone(),
            length: length,
        }
    }
}

impl PartialEq for SamplePhrase {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.notes == other.notes
    }
}

impl Eq for SamplePhrase {}

#[cfg(test)]
mod tests {
    use super::super::{Beat, SampleNote};
    use super::*;

    #[test]
    fn test_eq() {
        let phrase1 = SamplePhrase::new();

        let phrase1 = phrase1.add_note(SampleNote {
            sound: "x".to_string(),
            start: Beat::from(0.0),
        });
        let phrase1 = phrase1.add_note(SampleNote {
            sound: "o".to_string(),
            start: Beat::from(1.0),
        });
        let phrase2 = phrase1.clone();
        let phrase3 = phrase2.add_note(SampleNote {
            sound: "-".to_string(),
            start: Beat::from(2.0),
        });

        assert_eq!(phrase1, phrase2);
        assert_ne!(phrase1, phrase3);
    }
}
