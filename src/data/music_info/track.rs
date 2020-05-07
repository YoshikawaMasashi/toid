use serde::{Deserialize, Serialize};

use super::Phrase;

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    pub phrase: Phrase,
    pub sf2_name: Option<String>,
}
