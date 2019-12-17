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
    pub fn change_cumulative_samples(&self, cumulative_samples: i64) -> Self {
        SchedulingState {
            bpm: self.bpm,
            cumulative_samples: cumulative_samples,
        }
    }
}
