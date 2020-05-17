use std::cmp::Ordering;
use std::collections::BTreeSet;

use nom::branch::alt;
use nom::character::complete::{char, digit1, multispace0, one_of};
use nom::error::ParseError;
use nom::{named, one_of};
use nom::{Err, IResult};
use serde::{Deserialize, Serialize};

use super::pitch::Pitch;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chord {
    pitchs: BTreeSet<Pitch>,
    root: Pitch,
}

impl Chord {
    fn parse_chord_name(s: &str) -> IResult<&str, &str> {
        let (s, root) = one_of(&b"CDEFGAB"[..])(s)?;
        let root = root.to_string();
        /*
        let (s, root) = match one_of::<&str, &u8, ParseError<&str>>(&b"#b"[..])(s) {
            Ok((s, sf)) => (s, root + &sf.to_string()),
            Err(_) => (s, root),
        };
        */
        Ok(("", s))
    }
}
impl Eq for Chord {}

impl PartialEq for Chord {
    fn eq(&self, other: &Self) -> bool {
        self.pitchs == other.pitchs && self.root == other.root
    }
}

impl Ord for Chord {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.root != other.root {
            self.root.cmp(&other.root)
        } else {
            self.pitchs.cmp(&other.pitchs)
        }
    }
}

impl PartialOrd for Chord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.root != other.root {
            self.root.partial_cmp(&other.root)
        } else {
            self.pitchs.partial_cmp(&other.pitchs)
        }
    }
}

impl From<String> for Chord {
    fn from(chord_name: String) -> Self {
        Chord {
            pitchs: BTreeSet::new(),
            root: Pitch::from("C4".to_string()),
        }
    }
}

// C, Cm, C7, CM7, CmM7, Cdim7, Cadd9, Csus4
// 最初の文字を取る
// 次のもじが#かbだったら取る
// 次のもじがmだったら取る

// 次にM7, dim7, add9, sus4がきたら取る

// 次にonがきたら取る
