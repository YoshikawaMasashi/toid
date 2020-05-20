use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct PitchInOctave {
    pitch: f32,
}

impl PartialEq for PitchInOctave {
    fn eq(&self, other: &Self) -> bool {
        self.pitch == other.pitch
    }
}

impl Eq for PitchInOctave {}

impl PartialOrd for PitchInOctave {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pitch.partial_cmp(&other.pitch)
    }
}

impl Ord for PitchInOctave {
    fn cmp(&self, other: &Self) -> Ordering {
        // from https://docs.rs/crate/ordered-float/1.0.2/source/src/lib.rs
        match self.partial_cmp(&other) {
            Some(ordering) => ordering,
            None => {
                if self.pitch.is_nan() {
                    if other.pitch.is_nan() {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Less
                }
            }
        }
    }
}

impl From<f32> for PitchInOctave {
    fn from(pitch: f32) -> Self {
        Self {
            pitch: pitch % 12.0,
        }
    }
}

impl From<i32> for PitchInOctave {
    fn from(pitch: i32) -> Self {
        Self {
            pitch: (pitch % 12) as f32,
        }
    }
}
