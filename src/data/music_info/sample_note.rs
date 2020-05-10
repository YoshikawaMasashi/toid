use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use super::beat::Beat;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SampleNote {
    pub sound: String,
    pub start: Beat,
}

impl Eq for SampleNote {}

impl PartialEq for SampleNote {
    fn eq(&self, other: &Self) -> bool {
        self.sound == other.sound && self.start == other.start
    }
}

impl Ord for SampleNote {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.sound != other.sound {
            self.sound.cmp(&other.sound)
        } else {
            self.start.cmp(&other.start)
        }
    }
}

impl PartialOrd for SampleNote {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.sound != other.sound {
            self.sound.partial_cmp(&other.sound)
        } else {
            self.start.partial_cmp(&other.start)
        }
    }
}
