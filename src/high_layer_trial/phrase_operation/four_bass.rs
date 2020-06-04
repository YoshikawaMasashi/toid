use super::super::super::data::music_info::{Beat, ChordProgression, Note, Phrase, Pitch};

pub fn four_bass(prog: ChordProgression) -> Phrase {
    let mut ph = Phrase::new();
    for i in 0..prog.length.to_f32() as usize {
        let root = prog.get_chord(Beat::from(i as i32)).onroot;
        let pitch = Pitch::from(root.pitch + 36.0);
        ph = ph.add_note(Note {
            pitch,
            duration: Beat::from(1),
            start: Beat::from(i as i32),
        });
    }
    ph = ph.set_length(prog.length);
    ph
}
