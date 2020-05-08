use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use super::beat::Beat;
use super::pitch::Pitch;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Note {
    pub pitch: Pitch,
    pub duration: Beat,
    pub start: Beat,
}

impl Eq for Note {}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.pitch == other.pitch && self.duration == other.duration && self.start == other.start
    }
}

impl Ord for Note {
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

impl PartialOrd for Note {
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
