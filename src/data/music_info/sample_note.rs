use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use super::beat::Beat;
use super::note::Note;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SampleNote {
    pub sound: String,
    pub start: Beat,
}

impl Note for SampleNote {
    fn get_start(&self) -> Beat {
        self.start
    }
    fn set_start(&self, start: Beat) -> Self {
        SampleNote {
            sound: self.sound.clone(),
            start,
        }
    }
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
