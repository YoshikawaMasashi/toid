use serde::{Deserialize, Serialize};

use super::SamplePhrase;

#[derive(Serialize, Deserialize, Clone)]
pub struct SampleTrack {
    pub phrase: SamplePhrase,
    pub sample_name: String,
    pub vol: f32, // 0.0 ~ 1.0
    pub pan: f32, // -1.0(L) ~ 1.0(R)
}
