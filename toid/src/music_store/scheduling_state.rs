use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::super::state_management::reducer::Reducer;
use super::super::state_management::serialize;

pub struct SchedulingState {
    pub bpm: f32,
    pub cumulative_samples: i64,
}

impl SchedulingState {
    pub fn new() -> Self {
        SchedulingState {
            bpm: 120.0,
            cumulative_samples: 0,
        }
    }
    fn change_bpm(&self, bpm: f32) -> Self {
        SchedulingState {
            bpm,
            cumulative_samples: self.cumulative_samples,
        }
    }

    fn change_cumulative_samples(&self, cumulative_samples: i64) -> Self {
        SchedulingState {
            bpm: self.bpm,
            cumulative_samples,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SchedulingStateEvent {
    ChangeBPM(f32),
    ChangeCumulativeSamples(i64),
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
            SchedulingStateEvent::ChangeBPM(bpm) => state.change_bpm(bpm),
            SchedulingStateEvent::ChangeCumulativeSamples(cumulative_samples) => {
                state.change_cumulative_samples(cumulative_samples)
            }
        }
    }
}
