use std::cmp::Ordering;
use std::ops::{Add, Rem, Sub};

use serde::{Deserialize, Serialize};

const BEAT_LENGTH: i64 = 960;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Beat {
    num: i64,
}

impl From<f32> for Beat {
    fn from(beat: f32) -> Self {
        Beat {
            num: (beat * BEAT_LENGTH as f32) as i64,
        }
    }
}

impl From<f64> for Beat {
    fn from(beat: f64) -> Self {
        Beat {
            num: (beat * BEAT_LENGTH as f64) as i64,
        }
    }
}

impl From<u32> for Beat {
    fn from(beat: u32) -> Self {
        Beat {
            num: (beat * BEAT_LENGTH as u32) as i64,
        }
    }
}

impl From<u64> for Beat {
    fn from(beat: u64) -> Self {
        Beat {
            num: (beat * BEAT_LENGTH as u64) as i64,
        }
    }
}

impl From<i32> for Beat {
    fn from(beat: i32) -> Self {
        Beat {
            num: (beat * BEAT_LENGTH as i32) as i64,
        }
    }
}

impl From<i64> for Beat {
    fn from(beat: i64) -> Self {
        Beat {
            num: beat * BEAT_LENGTH,
        }
    }
}

impl Ord for Beat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num.cmp(&other.num)
    }
}

impl PartialOrd for Beat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.num.partial_cmp(&other.num)
    }
}

impl PartialEq for Beat {
    fn eq(&self, other: &Self) -> bool {
        self.num.eq(&other.num)
    }
}

impl Eq for Beat {}

impl Add for Beat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            num: self.num + other.num,
        }
    }
}

impl Sub for Beat {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            num: self.num - other.num,
        }
    }
}

impl Rem<i32> for Beat {
    type Output = Self;

    fn rem(self, modulus: i32) -> Self {
        Self {
            num: self.num % modulus,
        }
    }
}

impl Rem<i64> for Beat {
    type Output = Self;

    fn rem(self, modulus: i64) -> Self {
        Self {
            num: self.num % modulus,
        }
    }
}

impl Rem<u32> for Beat {
    type Output = Self;

    fn rem(self, modulus: u32) -> Self {
        Self {
            num: self.num % modulus,
        }
    }
}

impl Rem<u64> for Beat {
    type Output = Self;

    fn rem(self, modulus: u64) -> Self {
        Self {
            num: self.num % modulus,
        }
    }
}


impl Rem<usize> for Beat {
    type Output = Self;

    fn rem(self, modulus: usize) -> Self {
        Self {
            num: self.num % modulus,
        }
    }
}
