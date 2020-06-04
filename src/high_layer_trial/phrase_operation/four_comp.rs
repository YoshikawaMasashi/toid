use super::super::super::data::music_info::{Beat, ChordProgression, Note, Phrase, Pitch};

pub fn four_comp(prog: ChordProgression, min_pitch: Pitch, max_pitch: Pitch) -> Phrase {
    let mut ph = Phrase::new();
    for i in 0..prog.length.to_f32() as usize {
        let pitchs = prog
            .get_chord(Beat::from(i as i32))
            .get_pitchs(min_pitch, max_pitch);
        for &pitch in pitchs.iter() {
            ph = ph.add_note(Note {
                pitch,
                duration: Beat::from(1),
                start: Beat::from(i as i32),
            });
        }
    }
    ph
}
