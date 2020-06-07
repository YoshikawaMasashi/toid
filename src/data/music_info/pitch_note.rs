use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use super::beat::Beat;
use super::note::Note;
use super::pitch::Pitch;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct PitchNote {
    pub pitch: Pitch,
    pub duration: Beat,
    pub start: Beat,
}

impl Note for PitchNote {
    fn get_start(&self) -> Beat {
        self.start
    }
    fn set_start(&self, start: Beat) -> Self {
        PitchNote {
            pitch: self.pitch,
            duration: self.duration,
            start,
        }
    }
}

impl Eq for PitchNote {}

impl PartialEq for PitchNote {
    fn eq(&self, other: &Self) -> bool {
        self.pitch == other.pitch && self.duration == other.duration && self.start == other.start
    }
}

impl Ord for PitchNote {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.pitch != other.pitch {
            self.pitch.cmp(&other.pitch)
        } else if self.start != other.start {
            self.start.cmp(&other.start)
        } else {
            self.duration.cmp(&other.duration)
        }
    }
}

impl PartialOrd for PitchNote {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.pitch != other.pitch {
            self.pitch.partial_cmp(&other.pitch)
        } else if self.start != other.start {
            self.start.partial_cmp(&other.start)
        } else {
            self.duration.partial_cmp(&other.duration)
        }
    }
}
