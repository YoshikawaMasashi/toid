use serde::{Deserialize, Serialize};

use super::{Beat, Chord};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChordProgression {
    chords: Vec<(Chord, Beat)>,
}

impl ChordProgression {
    pub fn new() -> Self {
        ChordProgression { chords: Vec::new() }
    }

    pub fn add_chord(&self, chord: Chord, duration: Beat) -> Self {
        let mut new_chords = self.chords.clone();
        new_chords.push((chord, duration));
        ChordProgression { chords: new_chords }
    }
}
