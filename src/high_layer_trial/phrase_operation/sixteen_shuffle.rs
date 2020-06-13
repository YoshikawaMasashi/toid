use super::super::super::data::music_info::{Beat, Note, Phrase};

pub fn sixteen_shuffle<N: Note + Ord + Eq + Clone>(phrase: Phrase<N>) -> Phrase<N> {
    let mut new_phrase = Phrase::new();

    for (&start, note_vec) in phrase.notes.iter() {
        let start_in_8 = start % Beat::from(0.5);
        let start_res = start - start_in_8;
        let start_in_8 = if start_in_8 < Beat::from(0.25) {
            Beat::from(start_in_8.to_f32() * 2.0 / 3.0)
        } else {
            Beat::from(0.5 - (0.5 - start_in_8.to_f32()) * 2.0 / 3.0)
        };
        let new_start = start_in_8 + start_res;

        for note in note_vec.iter() {
            new_phrase = new_phrase.add_note(note.set_start(new_start));
        }
    }
    new_phrase = new_phrase.set_length(phrase.length);
    new_phrase
}
