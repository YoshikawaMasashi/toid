use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Instrument {
    SF2(String, usize),
    Sin,
    Tri,
    Saw,
    Sample(String),
}
