use super::super::super::data::music_info::Phrase;

pub fn split_by_condition(phrase: Phrase, condition: Vec<bool>) -> (Phrase, Phrase) {
    let mut true_phrase = Phrase::new();
    let mut false_phrase = Phrase::new();
    true_phrase = true_phrase.set_length(phrase.length);
    false_phrase = false_phrase.set_length(phrase.length);

    for (&note, &cond) in phrase.note_vec().iter().zip(condition.iter()) {
        if cond {
            true_phrase = true_phrase.add_note(note);
        } else {
            false_phrase = false_phrase.add_note(note);
        }
    }

    (true_phrase, false_phrase)
}
