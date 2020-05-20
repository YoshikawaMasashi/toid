use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{Beat, Chord};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChordProgression {
    chords_map: BTreeMap<Beat, Chord>,
}

impl ChordProgression {
    pub fn new() -> Self {
        ChordProgression {
            chords_map: BTreeMap::new(),
        }
    }

    pub fn add_chord(&self, start: Beat, chord: Chord) -> Self {
        let mut new_chords_map = self.chords_map.clone();
        new_chords_map.insert(start, chord);
        ChordProgression {
            chords_map: new_chords_map,
        }
    }
}
