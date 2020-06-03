use std::collections::BTreeSet;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::{PitchInOctave, PitchInterval};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scale {
    root: PitchInOctave,
    member: BTreeSet<PitchInterval>,
}

#[derive(Debug)]
enum ScaleType {
    MajPenta,
    MinPenta,
    Maj,
    Min,
}

impl Scale {
    fn parse_sclae_name(s: &str) -> IResult<&str, Self> {
        // parse root
        let (s, root) = one_of(&b"CDEFGAB"[..])(s)?;
        let root = root.to_string();
        let one_of_ret: IResult<&str, char> = one_of(&b"#b"[..])(s);
        let (s, root) = match one_of_ret {
            Ok((s, sf)) => (s, root + &sf.to_string()),
            Err(_) => (s, root),
        };

        // parse type
        let alt_ret: IResult<&str, &str> =
            alt((tag("MajPenta"), tag("MinPenta"), tag("Maj"), tag("Min")))(s);
        let (_s, scale_type) = match alt_ret {
            Ok((s, scale_type)) => match scale_type {
                "MajPenta" => (s, ScaleType::MajPenta),
                "MinPenta" => (s, ScaleType::MinPenta),
                "Maj" => (s, ScaleType::Maj),
                "Min" => (s, ScaleType::Min),
                _ => (s, ScaleType::Maj),
            },
            Err(_) => (s, ScaleType::Maj),
        };

        let root = PitchInOctave::from(root);
        let member = match scale_type {
            ScaleType::MajPenta => [
                PitchInterval::from(0.0),
                PitchInterval::from(2.0),
                PitchInterval::from(4.0),
                PitchInterval::from(7.0),
                PitchInterval::from(9.0),
            ]
            .iter()
            .cloned()
            .collect(),
            ScaleType::MinPenta => [
                PitchInterval::from(0.0),
                PitchInterval::from(3.0),
                PitchInterval::from(5.0),
                PitchInterval::from(7.0),
                PitchInterval::from(10.0),
            ]
            .iter()
            .cloned()
            .collect(),
            ScaleType::Maj => [
                PitchInterval::from(0.0),
                PitchInterval::from(2.0),
                PitchInterval::from(4.0),
                PitchInterval::from(5.0),
                PitchInterval::from(7.0),
                PitchInterval::from(9.0),
                PitchInterval::from(11.0),
            ]
            .iter()
            .cloned()
            .collect(),
            ScaleType::Min => [
                PitchInterval::from(0.0),
                PitchInterval::from(2.0),
                PitchInterval::from(3.0),
                PitchInterval::from(5.0),
                PitchInterval::from(7.0),
                PitchInterval::from(8.0),
                PitchInterval::from(10.0),
            ]
            .iter()
            .cloned()
            .collect(),
        };

        Ok(("", Scale { root, member }))
    }

    pub fn to_pitch_in_octave_vec(&self) -> Vec<PitchInOctave> {
        let mut ret_vec = vec![];
        for mem in self.member.iter() {
            ret_vec.push(PitchInOctave::from(self.root.pitch + mem.interval));
        }
        ret_vec
    }
}

impl From<String> for Scale {
    fn from(scale_name: String) -> Self {
        Self::parse_sclae_name(scale_name.as_str()).unwrap().1
    }
}

impl From<(PitchInOctave, BTreeSet<PitchInterval>)> for Scale {
    fn from(scale_info: (PitchInOctave, BTreeSet<PitchInterval>)) -> Self {
        Self {
            root: scale_info.0,
            member: scale_info.1,
        }
    }
}

impl From<Vec<f32>> for Scale {
    fn from(scale_vec: Vec<f32>) -> Self {
        let mut member = BTreeSet::new();
        for mem in scale_vec.iter() {
            member.insert(PitchInterval::from(mem - scale_vec[0]));
        }
        Self {
            root: PitchInOctave::from(scale_vec[0]),
            member,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{PitchInOctave, PitchInterval};

    #[test]
    fn test_parse_scale_name() {
        let scale = Scale::from("AbMajPenta".to_string());
        assert_eq!(scale.root, PitchInOctave::from(8.0));
        assert_eq!(
            scale.member.iter().cloned().collect::<Vec<PitchInterval>>(),
            vec![
                PitchInterval::from(0.0),
                PitchInterval::from(2.0),
                PitchInterval::from(4.0),
                PitchInterval::from(7.0),
                PitchInterval::from(9.0),
            ]
        );
    }

    #[test]
    fn test_to_pitch_in_octave_vec() {
        let scale = Scale::from("AbMajPenta".to_string());
        let pitch_in_octave_vec = scale.to_pitch_in_octave_vec();
        assert_eq!(
            pitch_in_octave_vec,
            vec![
                PitchInOctave::from(8.0),
                PitchInOctave::from(10.0),
                PitchInOctave::from(0.0),
                PitchInOctave::from(3.0),
                PitchInOctave::from(5.0),
            ]
        )
    }
}
