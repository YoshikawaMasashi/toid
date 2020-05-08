use std::cmp::Ordering;
use std::ops::Sub;

use serde::{Deserialize, Serialize};

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
