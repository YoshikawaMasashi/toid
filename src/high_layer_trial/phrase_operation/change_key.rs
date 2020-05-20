use std::collections::{BTreeMap, BTreeSet};

use super::super::super::data::music_info::{Note, Phrase, PitchInterval};

pub fn change_key(phrase: Phrase, interval: PitchInterval) -> Phrase {
    let mut new_notes = BTreeMap::new();
    for (&start, note_set) in phrase.notes.iter() {
        let mut new_note_set = BTreeSet::new();
        for note in note_set.iter() {
            new_note_set.insert(Note {
                pitch: note.pitch.add_interval(interval),
                duration: note.duration,
                start: note.start,
            });
        }
        new_notes.insert(start, new_note_set);
    }

    let new_length = phrase.length;

    Phrase {
        notes: new_notes,
        length: new_length,
    }
}
