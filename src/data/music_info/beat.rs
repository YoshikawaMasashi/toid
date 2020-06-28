use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

const BEAT_LENGTH: i64 = 960;

trait FromFraction<T> {
    fn from_fraction(numerator: T, denominator: T) -> Self;
}

#[derive(Clone, Copy, Debug)]
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
        let string = deserializer.deserialize_string(StringVisitor)?;
        match string.parse().map_err(Error::custom) {
            Ok(num) => Ok(Beat { num }),
            Err(e) => Err(e),
        }
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

impl Mul<f32> for Beat {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            num: (self.num as f32 * other) as i64,
        }
    }
}

impl Mul<usize> for Beat {
    type Output = Self;

    fn mul(self, other: usize) -> Self {
        Self {
            num: self.num * other as i64,
        }
    }
}

impl Div<f32> for Beat {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Self {
            num: (self.num as f32 / other) as i64,
        }
    }
}

impl Div<usize> for Beat {
    type Output = Self;

    fn div(self, other: usize) -> Self::Output {
        Self {
            num: self.num / other as i64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        assert_eq!(Beat::from(100), Beat::from(100));
        assert_ne!(Beat::from(100), Beat::from(101));
    }

    #[test]
    fn test_from_fraction() {
        assert_eq!(Beat::from_fraction(2, 3), Beat { num: 640 });
    }

    #[test]
    fn test_serialize_deserialize() {
        let serialized = serde_json::to_string(&Beat::from(100)).unwrap();
        let deserialized: Beat = serde_json::from_str(&serialized.to_string()).unwrap();
        assert_eq!(deserialized, Beat::from(100));
    }

    #[test]
    fn test_to_f32() {
        assert_eq!(Beat::from(1.5 as f32).to_f32(), 1.5 as f32);
    }

    #[test]
    fn test_get_num() {
        assert_eq!(Beat::from(1.5).get_num(), 1440);
    }

    #[test]
    fn test_from_f32() {
        assert_eq!(Beat::from(1.5 as f32), Beat { num: 1440 });
    }

    #[test]
    fn test_from_f64() {
        assert_eq!(Beat::from(1.5 as f64), Beat { num: 1440 });
    }

    #[test]
    fn test_from_u32() {
        assert_eq!(Beat::from(2 as u32), Beat { num: 1920 });
    }

    #[test]
    fn test_from_u64() {
        assert_eq!(Beat::from(2 as u64), Beat { num: 1920 });
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(Beat::from(2 as i32), Beat { num: 1920 });
    }

    #[test]
    fn test_from_i64() {
        assert_eq!(Beat::from(2 as i64), Beat { num: 1920 });
    }

    #[test]
    fn test_cmp() {
        assert_eq!(Beat::from(1.5) > Beat::from(1), true);
    }

    #[test]
    fn test_add() {
        assert_eq!(Beat::from(1.5) + Beat::from(1), Beat::from(2.5));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Beat::from(1.5) - Beat::from(1), Beat::from(0.5));
    }

    #[test]
    fn test_rem() {
        assert_eq!(Beat::from(1.5) % Beat::from(1), Beat::from(0.5));
    }
}
