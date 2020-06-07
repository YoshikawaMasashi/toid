use super::super::super::data::music_info::{Note, Phrase};

pub fn concat<N: Note + Ord + Eq + Clone>(phrase1: Phrase<N>, phrase2: Phrase<N>) -> Phrase<N> {
    let mut new_phrase = Phrase::new();
    new_phrase = new_phrase.set_length(phrase1.length + phrase2.length);

    for (_, note_vec) in phrase1.notes.iter() {
        for note in note_vec.iter() {
            new_phrase = new_phrase.add_note(note.clone());
        }
    }

    for (_, note_vec) in phrase2.notes.iter() {
        for note in note_vec.iter() {
            let new_note = note.set_start(note.get_start() + phrase1.length);
            new_phrase = new_phrase.add_note(new_note);
        }
    }

    new_phrase
}
