use std::cmp;

use super::super::super::data::music_info::Phrase;

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
