use std::collections::BTreeMap;

use super::super::super::data::music_info::{Beat, Note, Phrase};

pub fn delay(phrase: Phrase, delay: Beat) -> Phrase {
    let mut new_notes = BTreeMap::new();
    for (&start, note_vec) in phrase.notes.iter() {
        let mut new_note_vec = vec![];
        for note in note_vec.iter() {
            new_note_vec.push(Note {
                pitch: note.pitch,
                duration: note.duration,
                start: note.start + delay,
            });
        }
        new_notes.insert(start + delay, new_note_vec);
    }

    let new_length = phrase.length + delay;

    Phrase {
        notes: new_notes,
        length: new_length,
    }
}
