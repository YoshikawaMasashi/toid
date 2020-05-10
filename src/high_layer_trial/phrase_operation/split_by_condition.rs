use super::super::super::data::music_info::{Note, Phrase};

pub trait Condition {
    fn judge(&self, note: Note) -> bool;
}

pub fn split_by_condition(phrase: Phrase, condition: Box<dyn Condition>) -> (Phrase, Phrase) {
    let mut true_phrase = Phrase::new();
    let mut false_phrase = Phrase::new();
    true_phrase = true_phrase.set_length(phrase.length);
    false_phrase = false_phrase.set_length(phrase.length);

    for (_, note_vec) in phrase.notes.iter() {
        for &note in note_vec.iter() {
            if condition.judge(note) {
                true_phrase = true_phrase.add_note(note.clone());
            } else {
                false_phrase = false_phrase.add_note(note.clone());
            }
        }
    }

    (true_phrase, false_phrase)
}
