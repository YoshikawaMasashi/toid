use super::super::super::data::music_info::{Beat, ChordProgression, Phrase, Pitch, PitchNote};

pub fn four_comp(prog: ChordProgression, min_pitch: Pitch, max_pitch: Pitch) -> Phrase<PitchNote> {
    let mut ph = Phrase::new();
    for i in 0..prog.length.to_f32() as usize {
        let pitchs = prog
            .get_chord(Beat::from(i as i32))
            .get_pitchs(min_pitch, max_pitch);
        for &pitch in pitchs.iter() {
            ph = ph.add_note(PitchNote {
                pitch,
                duration: Beat::from(1),
                start: Beat::from(i as i32),
            });
        }
    }
    ph = ph.set_length(prog.length);
    ph
}
