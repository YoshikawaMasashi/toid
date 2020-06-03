use std::collections::BTreeSet;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::{PitchInOctave, PitchInterval, Scale};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chord {
    root: PitchInOctave,
    onroot: PitchInOctave,
    member: BTreeSet<PitchInterval>,
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
        let (s, tension) = match alt_ret {
            Ok((s, tension_str)) => match tension_str {
                "7" => (s, Tension::Seventh),
                "M7" => (s, Tension::Maj7),
                "dim7" => (s, Tension::Dim7),
                "add9" => (s, Tension::Add9),
                _ => (s, Tension::None),
            },
            Err(_) => (s, Tension::None),
        };

        let on_ret: IResult<&str, &str> = tag("on")(s);
        let (_s, onroot) = match on_ret {
            Ok((s, _on_str)) => {
                let (s, onroot) = one_of(&b"CDEFGAB"[..])(s)?;
                let onroot = onroot.to_string();
                let one_of_ret: IResult<&str, char> = one_of(&b"#b"[..])(s);
                match one_of_ret {
                    Ok((s, sf)) => (s, onroot + &sf.to_string()),
                    Err(_) => (s, onroot),
                }
            }
            Err(_) => (s, root.clone()),
        };

        let root = PitchInOctave::from(root);
        let onroot = PitchInOctave::from(onroot);
        let mut member = BTreeSet::new();
        member.insert(PitchInterval::from(0.0));
        match tension {
            Tension::Seventh => {
                match second {
                    Second::Major => {
                        member.insert(PitchInterval::from(4.0));
                    }
                    Second::Minor => {
                        member.insert(PitchInterval::from(3.0));
                    }
                    Second::Sus4 => {
                        member.insert(PitchInterval::from(5.0));
                    }
                };
                member.insert(PitchInterval::from(7.0));
                member.insert(PitchInterval::from(10.0));
            }
            Tension::Maj7 => {
                match second {
                    Second::Major => {
                        member.insert(PitchInterval::from(4.0));
                    }
                    Second::Minor => {
                        member.insert(PitchInterval::from(3.0));
                    }
                    Second::Sus4 => {
                        member.insert(PitchInterval::from(5.0));
                    }
                };
                member.insert(PitchInterval::from(7.0));
                member.insert(PitchInterval::from(11.0));
            }
            Tension::Dim7 => {
                member.insert(PitchInterval::from(3.0));
                member.insert(PitchInterval::from(6.0));
                member.insert(PitchInterval::from(9.0));
            }
            Tension::Add9 => {
                match second {
                    Second::Major => {
                        member.insert(PitchInterval::from(4.0));
                    }
                    Second::Minor => {
                        member.insert(PitchInterval::from(3.0));
                    }
                    Second::Sus4 => {
                        member.insert(PitchInterval::from(5.0));
                    }
                };
                member.insert(PitchInterval::from(7.0));
                member.insert(PitchInterval::from(14.0));
            }
            Tension::None => {
                match second {
                    Second::Major => {
                        member.insert(PitchInterval::from(4.0));
                    }
                    Second::Minor => {
                        member.insert(PitchInterval::from(3.0));
                    }
                    Second::Sus4 => {
                        member.insert(PitchInterval::from(5.0));
                    }
                };
                member.insert(PitchInterval::from(7.0));
            }
        }

        Ok((
            "",
            Chord {
                root,
                onroot,
                member,
            },
        ))
    }

    pub fn to_scale(&self) -> Scale {
        Scale::from((self.root, self.member.clone()))
    }
}

impl From<String> for Chord {
    fn from(chord_name: String) -> Self {
        Self::parse_chord_name(chord_name.as_str()).unwrap().1
    }
}

impl From<Vec<f32>> for Chord {
    fn from(scale_vec: Vec<f32>) -> Self {
        let mut member = BTreeSet::new();
        for mem in scale_vec.iter() {
            member.insert(PitchInterval::from(mem - scale_vec[0]));
        }
        Chord {
            root: PitchInOctave::from(scale_vec[0]),
            onroot: PitchInOctave::from(scale_vec[0]),
            member
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{PitchInOctave, PitchInterval};

    #[test]
    fn test_parse_chord_name() {
        let chord = Chord::from("C#m7".to_string());

        assert_eq!(chord.root, PitchInOctave::from(1.0));
        assert_eq!(chord.onroot, PitchInOctave::from(1.0));
        assert_eq!(
            chord.member.iter().cloned().collect::<Vec<PitchInterval>>(),
            vec![
                PitchInterval::from(0.0),
                PitchInterval::from(3.0),
                PitchInterval::from(7.0),
                PitchInterval::from(10.0)
            ]
        );

        let chord = Chord::from("EM7".to_string());

        assert_eq!(chord.root, PitchInOctave::from(4.0));
        assert_eq!(chord.onroot, PitchInOctave::from(4.0));
        assert_eq!(
            chord.member.iter().cloned().collect::<Vec<PitchInterval>>(),
            vec![
                PitchInterval::from(0.0),
                PitchInterval::from(4.0),
                PitchInterval::from(7.0),
                PitchInterval::from(11.0)
            ]
        );

        let chord = Chord::from("Abadd9onC".to_string());

        assert_eq!(chord.root, PitchInOctave::from(8.0));
        assert_eq!(chord.onroot, PitchInOctave::from(0.0));
        assert_eq!(
            chord.member.iter().cloned().collect::<Vec<PitchInterval>>(),
            vec![
                PitchInterval::from(0.0),
                PitchInterval::from(4.0),
                PitchInterval::from(7.0),
                PitchInterval::from(14.0)
            ]
        );
    }
}
