pub struct SchedulingState {
    bpm: f32,
    cumulative_samples: i64,
}

impl SchedulingState {
    fn new(bpm: f32) -> Self {
        SchedulingState {
            bpm,
            cumulative_samples: 0,
        }
    }
}

impl SchedulingState {
    pub fn change_cumulative_samples(&self, cumulative_samples: i64) -> Self {
        SchedulingState {
            bpm: self.bpm,
            cumulative_samples: cumulative_samples,
        }
    }
}
