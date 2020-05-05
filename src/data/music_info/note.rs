use serde::{Deserialize, Serialize};

use super::beat::Beat;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Note {
    pub pitch: f32,
    pub duration: Beat,
    pub start: Beat,
}
