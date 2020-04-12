use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Rem, Sub};

use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

const BEAT_LENGTH: i64 = 960;

trait FromFraction<T> {
    fn from_fraction(numerator: T, denominator: T) -> Self;
}

#[derive(Clone, Copy)]
pub struct Beat {
    num: i64,
}

impl Serialize for Beat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.get_num().to_string())
    }
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("String only")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
        Ok(v.to_string())
    }
}

impl<'de> Deserialize<'de> for Beat {
    fn deserialize<D>(deserializer: D) -> Result<Beat, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = deserializer.deserialize_string(StringVisitor).unwrap();
        let num: i64 = string.parse().unwrap();
        Ok(Beat { num })
    }
}

impl Beat {
    pub fn to_f32(self) -> f32 {
        self.num as f32 / BEAT_LENGTH as f32
    }

    pub fn get_num(self) -> i64 {
        self.num
    }
}

impl FromFraction<i32> for Beat {
    fn from_fraction(numerator: i32, denominator: i32) -> Self {
        Beat {
            num: ((numerator * BEAT_LENGTH as i32) / denominator) as i64,
        }
    }
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

impl Rem<Self> for Beat {
    type Output = Self;

    fn rem(self, modulus: Self) -> Self {
        Self {
            num: self.num % modulus.num,
        }
    }
}
