use super::super::super::data::music_info::{Note, Phrase};

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
