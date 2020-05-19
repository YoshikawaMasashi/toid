use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::IResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chord {
    root: f32,
    member: Vec<f32>,
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

        let root = match root.as_str() {
            "C" => 0.0,
            "C#" => 1.0,
            "Db" => 1.0,
            "D" => 2.0,
            "D#" => 3.0,
            "Eb" => 3.0,
            "E" => 4.0,
            "F" => 5.0,
            "F#" => 6.0,
            "Gb" => 6.0,
            "G" => 7.0,
            "G#" => 8.0,
            "Ab" => 8.0,
            "A" => 9.0,
            "A#" => 10.0,
            "Bb" => 10.0,
            "B" => 11.0,
            _ => std::f32::NAN,
        };
        let mut member = vec![];
        member.push(0.0);
        match tension {
            Tension::Seventh => {
                match second {
                    Second::Major => {
                        member.push(4.0);
                    }
                    Second::Minor => {
                        member.push(3.0);
                    }
                    Second::Sus4 => {
                        member.push(5.0);
                    }
                };
                member.push(7.0);
                member.push(10.0);
            }
            Tension::Maj7 => {
                match second {
                    Second::Major => {
                        member.push(4.0);
                    }
                    Second::Minor => {
                        member.push(3.0);
                    }
                    Second::Sus4 => {
                        member.push(5.0);
                    }
                };
                member.push(7.0);
                member.push(11.0);
            }
            Tension::Dim7 => {
                member.push(3.0);
                member.push(6.0);
                member.push(9.0);
            }
            Tension::Add9 => {
                match second {
                    Second::Major => {
                        member.push(4.0);
                    }
                    Second::Minor => {
                        member.push(3.0);
                    }
                    Second::Sus4 => {
                        member.push(5.0);
                    }
                };
                member.push(7.0);
                member.push(14.0);
            }
            Tension::None => {
                match second {
                    Second::Major => {
                        member.push(4.0);
                    }
                    Second::Minor => {
                        member.push(3.0);
                    }
                    Second::Sus4 => {
                        member.push(5.0);
                    }
                };
                member.push(7.0);
            }
        }

        Ok(("", Chord { root, member }))
    }
}

impl From<String> for Chord {
    fn from(chord_name: String) -> Self {
        Self::parse_chord_name(chord_name.as_str()).unwrap().1
    }
}

// C, Cm, C7, CM7, CmM7, Cdim7, Cadd9, Csus4
// 最初の文字を取る
// 次のもじが#かbだったら取る
// 次のもじがmだったら取る

// 次にM7, dim7, add9, sus4がきたら取る

// 次にonがきたら取る

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chord_name() {
        let chord = Chord::parse_chord_name("C#m7").unwrap().1;

        assert_eq!(chord.root, 1.0);
        assert_eq!(chord.member, vec![0.0, 3.0, 7.0, 10.0]);

        let chord = Chord::parse_chord_name("EM7").unwrap().1;

        assert_eq!(chord.root, 4.0);
        assert_eq!(chord.member, vec![0.0, 4.0, 7.0, 11.0]);
    }
}
