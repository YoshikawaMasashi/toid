use std::collections::BTreeMap;

use super::super::super::data::music_info::{Note, Phrase};

pub fn change_key(phrase: Phrase, key: f32) -> Phrase {
    let mut new_notes = BTreeMap::new();
    for (&start, note_vec) in phrase.notes.iter() {
        let mut new_note_vec = vec![];
        for note in note_vec.iter() {
            new_note_vec.push(Note {
                pitch: note.pitch + key,
                duration: note.duration,
                start: note.start,
            });
        }
        new_notes.insert(start, new_note_vec);
    }

    let new_length = phrase.length;

    Phrase {
        notes: new_notes,
        length: new_length,
    }
}
