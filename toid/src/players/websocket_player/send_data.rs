use serde::{Deserialize, Serialize};

use super::super::super::state_management::serialize;

#[derive(Serialize, Deserialize)]
pub enum SendData {
    StateUpdate(String),
    SyncState(String),
    ApplyReader(String),
}

impl serialize::Serialize<SendData> for SendData {
    fn serialize(&self) -> Result<String, String> {
        if let Ok(serialized) = serde_json::to_string(&self) {
            Ok(serialized)
        } else {
            Err(String::from("error in serizalization"))
        }
    }
    fn deserialize(serialized: String) -> Result<Self, String> {
        if let Ok(string) = serde_json::from_str(serialized.as_str()) {
            Ok(string)
        } else {
            Err(String::from("error in deserizalization"))
        }
    }
}
