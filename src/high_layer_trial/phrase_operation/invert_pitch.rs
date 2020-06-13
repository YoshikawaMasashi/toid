use super::super::super::data::music_info::{Phrase, Pitch, PitchNote};

pub fn invert_pitch(phrase: Phrase<PitchNote>, center: Pitch) -> Phrase<PitchNote> {
    let mut new_phrase = Phrase::new();
    new_phrase = new_phrase.set_length(phrase.length);

    for (_, note_vec) in phrase.notes.iter() {
        for note in note_vec.iter() {
            new_phrase = new_phrase.add_note(PitchNote {
                pitch: center.sub_interval(note.pitch - center),
                duration: note.duration,
                start: note.start,
            });
        }
    }

    new_phrase
}
