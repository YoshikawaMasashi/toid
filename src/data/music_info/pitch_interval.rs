use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct PitchInterval {
    pub interval: f32,
}

impl PitchInterval {
    pub fn abs(&self) -> Self {
        Self {
            interval: self.interval.abs(),
        }
    }
}
impl PartialEq for PitchInterval {
    fn eq(&self, other: &Self) -> bool {
        self.interval == other.interval
    }
}

impl Eq for PitchInterval {}

impl PartialOrd for PitchInterval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.interval.partial_cmp(&other.interval)
    }
}

impl Ord for PitchInterval {
    fn cmp(&self, other: &Self) -> Ordering {
        // from https://docs.rs/crate/ordered-float/1.0.2/source/src/lib.rs
        match self.partial_cmp(&other) {
            Some(ordering) => ordering,
            None => {
                if self.interval.is_nan() {
                    if other.interval.is_nan() {
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

impl From<f32> for PitchInterval {
    fn from(interval: f32) -> Self {
        Self { interval }
    }
}

impl From<i32> for PitchInterval {
    fn from(interval: i32) -> Self {
        Self {
            interval: interval as f32,
        }
    }
}
