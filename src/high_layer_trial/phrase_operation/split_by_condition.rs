use super::super::super::data::music_info::{Note, Phrase};

pub fn split_by_condition<N: Note + Eq + Ord + Clone>(
    phrase: Phrase<N>,
    condition: Vec<bool>,
) -> (Phrase<N>, Phrase<N>) {
    let mut true_phrase = Phrase::new();
    let mut false_phrase = Phrase::new();
    true_phrase = true_phrase.set_length(phrase.length);
    false_phrase = false_phrase.set_length(phrase.length);

    for (note, &cond) in phrase.note_vec().iter().zip(condition.iter()) {
        if cond {
            true_phrase = true_phrase.add_note(note.clone());
        } else {
            false_phrase = false_phrase.add_note(note.clone());
        }
    }

    (true_phrase, false_phrase)
}
