use serde::{Deserialize, Serialize};

use super::super::state_management::serialize;
use super::super::state_management::state::State;

#[derive(Serialize, Deserialize)]
pub struct SF2State {
    pub sf2_name: Option<String>,
}

impl SF2State {
    pub fn new() -> Self {
        SF2State { sf2_name: None }
    }

    pub fn set_sf2_name(&self, sf2_name: String) -> Self {
        SF2State {
            sf2_name: Some(sf2_name),
        }
    }
}

impl State<SF2StateEvent> for SF2State {
    fn reduce(&self, event: SF2StateEvent) -> Self {
        match event {
            SF2StateEvent::SetSF2Name(sf2_name) => self.set_sf2_name(sf2_name),
        }
    }
}

impl serialize::Serialize<SF2State> for SF2State {
    fn serialize(&self) -> Result<String, String> {
        match serde_json::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(err) => Err(format!("error in serizalization : {}", err)),
        }
    }
    fn deserialize(serialized: String) -> Result<Self, String> {
        match serde_json::from_str(serialized.as_str()) {
            Ok(deserialized) => Ok(deserialized),
            Err(err) => Err(format!("error in deserizalization : {}", err)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SF2StateEvent {
    SetSF2Name(String),
}

impl serialize::Serialize<SF2StateEvent> for SF2StateEvent {
    fn serialize(&self) -> Result<String, String> {
        match serde_json::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(err) => Err(format!("error in serizalization : {}", err)),
        }
    }
    fn deserialize(serialized: String) -> Result<Self, String> {
        match serde_json::from_str(serialized.as_str()) {
            Ok(deserialized) => Ok(deserialized),
            Err(err) => Err(format!("error in deserizalization : {}", err)),
        }
    }
}
