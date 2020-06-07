use super::super::super::data::music_info::{Note, Phrase};

pub fn invert_start_order<N: Note + Ord + Eq + Clone>(phrase: Phrase<N>) -> Phrase<N> {
    let mut new_phrase = Phrase::new();
    new_phrase = new_phrase.set_length(phrase.length);

    let new_starts = phrase.notes.keys().rev();
    for ((_, note_vec), &new_start) in phrase.notes.iter().zip(new_starts) {
        for note in note_vec.iter() {
            new_phrase = new_phrase.add_note(note.set_start(new_start));
        }
    }

    new_phrase
}
