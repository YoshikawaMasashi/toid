use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::{PitchInOctave, PitchInterval};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chord {
    root: PitchInOctave,
    onroot: PitchInOctave,
    member: Vec<PitchInterval>,
}

#[derive(Debug)]
enum Second {
    Major,
    Minor,
    Sus4,
}

#[derive(Debug)]
enum Tension {
    Seventh,
    Maj7,
    Dim7,
    Add9,
    None,
}

impl Chord {
    fn parse_chord_name(s: &str) -> IResult<&str, Chord> {
        // parse root
        let (s, root) = one_of(&b"CDEFGAB"[..])(s)?;
        let root = root.to_string();
        let one_of_ret: IResult<&str, char> = one_of(&b"#b"[..])(s);
        let (s, root) = match one_of_ret {
            Ok((s, sf)) => (s, root + &sf.to_string()),
            Err(_) => (s, root),
        };

        // parse second
        let alt_ret: IResult<&str, &str> = alt((tag("m"), tag("sus4")))(s);
        let (s, second) = match alt_ret {
            Ok((s, second_str)) => match second_str {
                "m" => (s, Second::Minor),
                "sus4" => (s, Second::Sus4),
                _ => (s, Second::Major),
            },
            Err(_) => (s, Second::Major),
        };

        // parse M7, dim7, add9, sus4
        let alt_ret: IResult<&str, &str> =
            alt((tag("7"), tag("M7"), tag("dim7"), tag("add9"), tag("sus4")))(s);
        let (_, tension) = match alt_ret {
            Ok((s, tension_str)) => match tension_str {
                "7" => (s, Tension::Seventh),
                "M7" => (s, Tension::Maj7),
                "dim7" => (s, Tension::Dim7),
                "add9" => (s, Tension::Add9),
                _ => (s, Tension::None),
            },
            Err(_) => (s, Tension::None),
        };

        let root = PitchInOctave::from(root);
        let mut member = vec![];
        member.push(PitchInterval::from(0.0));
        match tension {
            Tension::Seventh => {
                match second {
                    Second::Major => {
                        member.push(PitchInterval::from(4.0));
                    }
                    Second::Minor => {
                        member.push(PitchInterval::from(3.0));
                    }
                    Second::Sus4 => {
                        member.push(PitchInterval::from(5.0));
                    }
                };
                member.push(PitchInterval::from(7.0));
                member.push(PitchInterval::from(10.0));
            }
            Tension::Maj7 => {
                match second {
                    Second::Major => {
                        member.push(PitchInterval::from(4.0));
                    }
                    Second::Minor => {
                        member.push(PitchInterval::from(3.0));
                    }
                    Second::Sus4 => {
                        member.push(PitchInterval::from(5.0));
                    }
                };
                member.push(PitchInterval::from(7.0));
                member.push(PitchInterval::from(11.0));
            }
            Tension::Dim7 => {
                member.push(PitchInterval::from(3.0));
                member.push(PitchInterval::from(6.0));
                member.push(PitchInterval::from(9.0));
            }
            Tension::Add9 => {
                match second {
                    Second::Major => {
                        member.push(PitchInterval::from(4.0));
                    }
                    Second::Minor => {
                        member.push(PitchInterval::from(3.0));
                    }
                    Second::Sus4 => {
                        member.push(PitchInterval::from(5.0));
                    }
                };
                member.push(PitchInterval::from(7.0));
                member.push(PitchInterval::from(14.0));
            }
            Tension::None => {
                match second {
                    Second::Major => {
                        member.push(PitchInterval::from(4.0));
                    }
                    Second::Minor => {
                        member.push(PitchInterval::from(3.0));
                    }
                    Second::Sus4 => {
                        member.push(PitchInterval::from(5.0));
                    }
                };
                member.push(PitchInterval::from(7.0));
            }
        }

        Ok(("", Chord { root, onroot: root, member }))
    }

    pub fn to_scale(&self) -> Vec<PitchInOctave> {
        let mut scale = vec![];
        for &p in self.member.iter() {
            scale.push(PitchInOctave::from(self.root.pitch + p.interval));
        }
        scale
    }
}

impl From<String> for Chord {
    fn from(chord_name: String) -> Self {
        Self::parse_chord_name(chord_name.as_str()).unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{PitchInOctave, PitchInterval};

    #[test]
    fn test_parse_chord_name() {
        let chord = Chord::parse_chord_name("C#m7").unwrap().1;

        assert_eq!(chord.root, PitchInOctave::from(1.0));
        assert_eq!(
            chord.member,
            vec![
                PitchInterval::from(0.0),
                PitchInterval::from(3.0),
                PitchInterval::from(7.0),
                PitchInterval::from(10.0)
            ]
        );

        let chord = Chord::parse_chord_name("EM7").unwrap().1;

        assert_eq!(chord.root, PitchInOctave::from(4.0));
        assert_eq!(
            chord.member,
            vec![
                PitchInterval::from(0.0),
                PitchInterval::from(4.0),
                PitchInterval::from(7.0),
                PitchInterval::from(11.0)
            ]
        );
    }
}
