use super::super::super::data::music_info::{Note, Phrase, PitchInOctave, PitchInterval};

pub fn change_pitch_in_key(phrase: Phrase, key: PitchInOctave, degree: usize) -> Phrase {
    const OFFSET_KEY: [[f32; 7]; 7] = [
        [0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0],
        [0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0],
        [0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0],
        [0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 11.0],
        [0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0],
        [0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0],
        [0.0, 1.0, 3.0, 5.0, 6.0, 8.0, 10.0],
    ];

    let mut new_phrase = Phrase::new();
    new_phrase = new_phrase.set_length(phrase.length);

    for (_, note_vec) in phrase.notes.iter() {
        for note in note_vec.iter() {
            let pitch_in_key: PitchInOctave =
                (note.pitch.sub_interval(PitchInterval::from(key.pitch))).to_pitch_in_octave();
            let degree_in_key: usize = if pitch_in_key.pitch < 1.0 || pitch_in_key.pitch >= 11.5 {
                0
            } else if pitch_in_key.pitch >= 1.0 && pitch_in_key.pitch < 3.0 {
                1
            } else if pitch_in_key.pitch >= 3.0 && pitch_in_key.pitch < 4.5 {
                2
            } else if pitch_in_key.pitch >= 4.5 && pitch_in_key.pitch < 6.0 {
                3
            } else if pitch_in_key.pitch >= 6.0 && pitch_in_key.pitch < 8.0 {
                4
            } else if pitch_in_key.pitch >= 8.0 && pitch_in_key.pitch < 10.0 {
                5
            } else {
                6
            };
            let offset = OFFSET_KEY[degree_in_key][degree % 7] + 12.0 * (degree / 7) as f32;
            new_phrase = new_phrase.add_note(Note {
                pitch: note.pitch.add_interval(PitchInterval { interval: offset }),
                duration: note.duration,
                start: note.start,
            });
        }
    }

    new_phrase
}
