use std::cmp::Ordering;
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::pitch::Pitch;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chord {
    pitchs: BTreeSet<Pitch>,
}

impl Eq for Chord {}

impl PartialEq for Chord {
    fn eq(&self, other: &Self) -> bool {
        self.pitchs == other.pitchs
    }
}

impl Ord for Chord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pitchs.cmp(&other.pitchs)
    }
}

impl PartialOrd for Chord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pitchs.partial_cmp(&other.pitchs)
    }
}
