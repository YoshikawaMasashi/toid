use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::super::super::data::music_info::beat::Beat;
use super::super::super::state_management::serialize;
use super::super::super::state_management::state::State;

#[derive(Serialize, Deserialize)]
pub struct SchedulingState {
    pub bpm_schedule: BTreeMap<Beat, f32>,
}

impl SchedulingState {
    fn change_bpm(&self, change: Beat, bpm: f32) -> Self {
        let mut new_bpm_schedule = self.bpm_schedule.clone();
        new_bpm_schedule.insert(change, bpm);
        SchedulingState {
            bpm_schedule: new_bpm_schedule,
        }
    }

    pub fn get_bpm_schedule(&self) -> BTreeMap<Beat, f32> {
        self.bpm_schedule.clone()
    }
}

impl State<SchedulingStateEvent> for SchedulingState {
    fn new() -> Self {
        let mut bpm_schedule = BTreeMap::new();
        bpm_schedule.insert(Beat::from(0), 120.0);
        SchedulingState { bpm_schedule }
    }

    fn reduce(&self, event: SchedulingStateEvent) -> Self {
        match event {
            SchedulingStateEvent::ChangeBPM(beat, bpm) => self.change_bpm(beat, bpm),
        }
    }
}

impl serialize::Serialize<SchedulingState> for SchedulingState {
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
pub enum SchedulingStateEvent {
    ChangeBPM(Beat, f32),
}

impl serialize::Serialize<SchedulingStateEvent> for SchedulingStateEvent {
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
