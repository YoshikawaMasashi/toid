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
