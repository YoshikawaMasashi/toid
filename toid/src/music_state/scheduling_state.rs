use std::collections::BTreeMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::super::state_management::reducer::Reducer;
use super::super::state_management::serialize;
use super::beat::Beat;

pub struct SchedulingState {
    pub bpm_schedule: BTreeMap<Beat, f32>,
}

impl SchedulingState {
    pub fn new() -> Self {
        let mut bpm_schedule = BTreeMap::new();
        bpm_schedule.insert(Beat::from(0), 120.0);
        SchedulingState { bpm_schedule }
    }

    fn change_bpm(&self, change: Beat, bpm: f32) -> Self {
        let mut new_bpm_schedule = self.bpm_schedule.clone();
        new_bpm_schedule.insert(change, bpm);
        SchedulingState {
            bpm_schedule: new_bpm_schedule,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SchedulingStateEvent {
    ChangeBPM(Beat, f32),
}

impl serialize::Serialize<SchedulingStateEvent> for SchedulingStateEvent {
    fn serialize(&self) -> Result<String, String> {
        if let Ok(serialized) = serde_json::to_string(&self) {
            Ok(serialized)
        } else {
            Err(String::from("error in serizalization"))
        }
    }
    fn deserialize(serialized: String) -> Result<SchedulingStateEvent, String> {
        if let Ok(string) = serde_json::from_str(serialized.as_str()) {
            Ok(string)
        } else {
            Err(String::from("error in deserizalization"))
        }
    }
}

pub struct SchedulingStateReducer {}

impl Reducer<SchedulingState, SchedulingStateEvent> for SchedulingStateReducer {
    fn reduce(&self, state: Arc<SchedulingState>, event: SchedulingStateEvent) -> SchedulingState {
        match event {
            SchedulingStateEvent::ChangeBPM(beat, bpm) => state.change_bpm(beat, bpm),
        }
    }
}
