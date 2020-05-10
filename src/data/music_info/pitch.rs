use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Sub;

use serde::{Deserialize, Serialize};

lazy_static! {
    static ref STRING_TO_F32_PITCH: HashMap<String, f32> = {
        let mut map = HashMap::new();
        for octave_idx in 0..9 {
            map.insert(format!("C{}", octave_idx), octave_idx as f32 * 12.0 + 12.0);
            map.insert(format!("C#{}", octave_idx), octave_idx as f32 * 12.0 + 13.0);
            map.insert(format!("Db{}", octave_idx), octave_idx as f32 * 12.0 + 13.0);
            map.insert(format!("D{}", octave_idx), octave_idx as f32 * 12.0 + 14.0);
            map.insert(format!("D#{}", octave_idx), octave_idx as f32 * 12.0 + 15.0);
            map.insert(format!("Eb{}", octave_idx), octave_idx as f32 * 12.0 + 15.0);
            map.insert(format!("E{}", octave_idx), octave_idx as f32 * 12.0 + 16.0);
            map.insert(format!("F{}", octave_idx), octave_idx as f32 * 12.0 + 17.0);
            map.insert(format!("F#{}", octave_idx), octave_idx as f32 * 12.0 + 18.0);
            map.insert(format!("Gb{}", octave_idx), octave_idx as f32 * 12.0 + 18.0);
            map.insert(format!("G{}", octave_idx), octave_idx as f32 * 12.0 + 19.0);
            map.insert(format!("G#{}", octave_idx), octave_idx as f32 * 12.0 + 20.0);
            map.insert(format!("Ab{}", octave_idx), octave_idx as f32 * 12.0 + 20.0);
            map.insert(format!("A{}", octave_idx), octave_idx as f32 * 12.0 + 21.0);
            map.insert(format!("A#{}", octave_idx), octave_idx as f32 * 12.0 + 22.0);
            map.insert(format!("Bb{}", octave_idx), octave_idx as f32 * 12.0 + 22.0);
            map.insert(format!("B{}", octave_idx), octave_idx as f32 * 12.0 + 23.0);
        }
        map
    };

    static ref I32_PITCH_TO_PITCH_NAME: HashMap<i32, String> = {
        let mut map = HashMap::new();
        for octave_idx in 0..9 {
            map.insert(octave_idx as i32 * 12 + 12, format!("C{}", octave_idx));
            // map.insert(octave_idx as i32 * 12 + 13, format!("C#{}", octave_idx));
            map.insert(octave_idx as i32 * 12 + 13, format!("Db{}", octave_idx));
            map.insert(octave_idx as i32 * 12 + 14, format!("D{}", octave_idx));
            // map.insert(octave_idx as i32 * 12 + 15, format!("D#{}", octave_idx));
            map.insert(octave_idx as i32 * 12 + 15, format!("Eb{}", octave_idx));
            map.insert(octave_idx as i32 * 12 + 16, format!("E{}", octave_idx));
            map.insert(octave_idx as i32 * 12 + 17, format!("F{}", octave_idx));
            // map.insert(octave_idx as i32 * 12 + 18, format!("F#{}", octave_idx));
            map.insert(octave_idx as i32 * 12 + 18, format!("Gb{}", octave_idx));
            map.insert(octave_idx as i32 * 12 + 19, format!("G{}", octave_idx));
            // map.insert(octave_idx as i32 * 12 + 20, format!("G#{}", octave_idx));
            map.insert(octave_idx as i32 * 12 + 20, format!("Ab{}", octave_idx));
            map.insert(octave_idx as i32 * 12 + 21, format!("A{}", octave_idx));
            // map.insert(octave_idx as i32 * 12 + 22, format!("A#{}", octave_idx));
            map.insert(octave_idx as i32 * 12 + 22, format!("Bb{}", octave_idx));
            map.insert(octave_idx as i32 * 12 + 23, format!("B{}", octave_idx));
        }
        map
    };
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Pitch {
    pitch: f32,
}

impl Pitch {
    pub fn add_f32(&self, offset: f32) -> Self {
        Self {
            pitch: self.pitch + offset,
        }
    }

    pub fn sub_f32(&self, offset: f32) -> Self {
        Self {
            pitch: self.pitch - offset,
        }
    }

    pub fn rem_f32(&self, offset: f32) -> f32 {
        self.pitch % offset
    }

    pub fn get_hertz(&self) -> f32 {
        // A4 -> 69 440hz
        440. * (2.0 as f32).powf((self.pitch - 69.) / 12.)
    }

    pub fn get_u8_pitch(&self) -> u8 {
        self.pitch as u8
    }

    pub fn get_f32_pitch(&self) -> f32 {
        self.pitch
    }
}

impl PartialEq for Pitch {
    fn eq(&self, other: &Self) -> bool {
        self.pitch == other.pitch
    }
}

impl Eq for Pitch {}

impl PartialOrd for Pitch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pitch.partial_cmp(&other.pitch)
    }
}

impl Ord for Pitch {
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

impl From<f32> for Pitch {
    fn from(pitch: f32) -> Self {
        Pitch { pitch }
    }
}

impl From<i32> for Pitch {
    fn from(pitch: i32) -> Self {
        Pitch {
            pitch: pitch as f32,
        }
    }
}

impl From<String> for Pitch {
    fn from(pitch_name: String) -> Self {
        Pitch {
            pitch: STRING_TO_F32_PITCH[&pitch_name],
        }
    }
}

impl Into<String> for Pitch {
    fn into(self) -> String {
        I32_PITCH_TO_PITCH_NAME[&(self.pitch as i32)].clone()
    }
}

impl Sub for Pitch {
    type Output = f32;

    fn sub(self, other: Self) -> f32 {
        self.pitch - other.pitch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_f32() {
        let pitch = Pitch::from(60);
        let pitch = pitch.add_f32(0.5);
        assert_eq!(pitch, Pitch::from(60.5));
    }
}
