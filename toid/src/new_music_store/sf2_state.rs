use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::super::data::sf2::SF2;
use super::super::new_state_management::reducer::Reducer;
use super::super::new_state_management::serialize;

pub struct SF2State {
    pub sf2: Option<Arc<SF2>>,
}

impl Clone for SF2State {
    fn clone(&self) -> Self {
        SF2State {
            sf2: match &self.sf2 {
                Some(sf2) => Some(Arc::clone(&sf2)),
                None => None,
            },
        }
    }
}

impl SF2State {
    pub fn new() -> Self {
        SF2State { sf2: None }
    }

    pub fn set_sf2(&self, sf2: Arc<SF2>) -> Self {
        SF2State {
            sf2: Some(Arc::clone(&sf2)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SF2StateEvent {
    LoadAndSetSF2(String),
}

impl serialize::Serialize<SF2StateEvent> for SF2StateEvent {
    fn serialize(&self) -> Result<String, String> {
        if let Ok(serialized) = serde_json::to_string(&self) {
            Ok(serialized)
        } else {
            Err(String::from("error in serizalization"))
        }
    }
    fn deserialize(serialized: String) -> Result<SF2StateEvent, String> {
        if let Ok(string) = serde_json::from_str(serialized.as_str()) {
            Ok(string)
        } else {
            Err(String::from("error in deserizalization"))
        }
    }
}

pub struct SF2StateReducer {}

impl Reducer<SF2State, SF2StateEvent> for SF2StateReducer {
    fn reduce(&self, state: Arc<SF2State>, event: SF2StateEvent) -> SF2State {
        match event {
            SF2StateEvent::LoadAndSetSF2(path) => {
                let mut f = File::open(path).unwrap();
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).unwrap();
                let buffer = buffer.as_slice();
                let sf2 = SF2::parse(buffer);
                let sf2 = Arc::new(sf2);
                state.set_sf2(sf2)
            }
        }
    }
}
