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
