use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use super::PitchInterval;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct PitchInOctave {
    pub pitch: f32,
}

impl PitchInOctave {
    pub fn to_pitch_interval(&self) -> PitchInterval {
        PitchInterval {
            interval: self.pitch,
        }
    }
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
impl From<String> for PitchInOctave {
    fn from(pitch_name: String) -> Self {
        match pitch_name.as_str() {
            "C" => PitchInOctave::from(0.0),
            "C#" => PitchInOctave::from(1.0),
            "Db" => PitchInOctave::from(1.0),
            "D" => PitchInOctave::from(2.0),
            "D#" => PitchInOctave::from(3.0),
            "Eb" => PitchInOctave::from(3.0),
            "E" => PitchInOctave::from(4.0),
            "F" => PitchInOctave::from(5.0),
            "F#" => PitchInOctave::from(6.0),
            "Gb" => PitchInOctave::from(6.0),
            "G" => PitchInOctave::from(7.0),
            "G#" => PitchInOctave::from(8.0),
            "Ab" => PitchInOctave::from(8.0),
            "A" => PitchInOctave::from(9.0),
            "A#" => PitchInOctave::from(10.0),
            "Bb" => PitchInOctave::from(10.0),
            "B" => PitchInOctave::from(11.0),
            _ => PitchInOctave::from(std::f32::NAN),
        }
    }
}
