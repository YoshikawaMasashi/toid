use std::cmp;
use std::collections::BTreeMap;

use rand::seq::SliceRandom;
use rand::thread_rng;

use super::super::data::music_info::{Beat, Note, Phrase};

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

pub fn marge(phrase1: Phrase, phrase2: Phrase) -> Phrase {
    let mut new_phrase = Phrase::new();
    new_phrase = new_phrase.set_length(cmp::max(phrase1.length, phrase2.length));
    for (_, note_vec) in phrase1.notes.iter() {
        for &note in note_vec.iter() {
            new_phrase = new_phrase.add_note(note);
        }
    }
    for (_, note_vec) in phrase2.notes.iter() {
        for &note in note_vec.iter() {
            new_phrase = new_phrase.add_note(note);
        }
    }
    new_phrase
}

pub fn concat(phrase1: Phrase, phrase2: Phrase) -> Phrase {
    let mut new_phrase = Phrase::new();
    new_phrase = new_phrase.set_length(phrase1.length + phrase2.length);

    for (_, note_vec) in phrase1.notes.iter() {
        for &note in note_vec.iter() {
            new_phrase = new_phrase.add_note(note);
        }
    }

    for (_, note_vec) in phrase2.notes.iter() {
        for &note in note_vec.iter() {
            let new_note = Note {
                pitch: note.pitch,
                duration: note.duration,
                start: note.start + phrase1.length,
            };
            new_phrase = new_phrase.add_note(new_note);
        }
    }

    new_phrase
}

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

pub fn shuffle_start(phrase: Phrase) -> Phrase {
    let mut new_phrase = Phrase::new();
    new_phrase = new_phrase.set_length(phrase.length);

    let mut new_starts: Box<[Beat]> = phrase.notes.keys().cloned().collect();
    let mut rng = thread_rng();
    new_starts.shuffle(&mut rng);
    for ((_, note_vec), &new_start) in phrase.notes.iter().zip(new_starts.iter()) {
        for note in note_vec.iter() {
            new_phrase.add_note(Note {
                pitch: note.pitch,
                duration: note.duration,
                start: new_start,
            });
        }
    }

    new_phrase
}

pub fn invert_start_order(phrase: Phrase) -> Phrase {
    let mut new_phrase = Phrase::new();
    new_phrase = new_phrase.set_length(phrase.length);

    let new_starts = phrase.notes.keys().rev();
    for ((_, note_vec), &new_start) in phrase.notes.iter().zip(new_starts) {
        for note in note_vec.iter() {
            new_phrase.add_note(Note {
                pitch: note.pitch,
                duration: note.duration,
                start: new_start,
            });
        }
    }

    new_phrase
}

pub fn invert_pitch(phrase: Phrase, center: f32) -> Phrase {
    let mut new_phrase = Phrase::new();
    new_phrase = new_phrase.set_length(phrase.length);

    for (_, note_vec) in phrase.notes.iter() {
        for note in note_vec.iter() {
            new_phrase.add_note(Note {
                pitch: center - (note.pitch - center),
                duration: note.duration,
                start: note.start,
            });
        }
    }

    new_phrase
}
