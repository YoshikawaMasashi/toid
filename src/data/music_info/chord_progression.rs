use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{Beat, Chord};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChordProgression {
    chords: BTreeMap<Beat, Chord>,
    length: Beat,
}

impl ChordProgression {
    pub fn new() -> Self {
        ChordProgression {
            chords: BTreeMap::new(),
            length: Beat::from(0),
        }
    }

    pub fn add_chord(&self, start: Beat, chord: Chord) -> Self {
        let mut new_chords = self.chords.clone();
        new_chords.insert(start, chord);
        ChordProgression {
            chords: new_chords,
            length: self.length,
        }
    }

    pub fn set_length(&self, length: Beat) -> Self {
        ChordProgression {
            chords: self.chords.clone(),
            length,
        }
    }
}

impl From<(Beat, Vec<Chord>)> for ChordProgression {
    fn from(info: (Beat, Vec<Chord>)) -> Self {
        let step = info.0;
        let chord_vec = info.1;

        let mut prog = ChordProgression::new();

        for (i, chord) in chord_vec.iter().enumerate() {
            prog = prog.add_chord(step * i, chord.clone());
        }
        prog = prog.set_length(step * chord_vec.len());

        prog
    }
}
