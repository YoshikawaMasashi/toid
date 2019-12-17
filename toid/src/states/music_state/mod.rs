use std::sync::Arc;

pub mod melody_state;
pub mod scheduling_state;

pub struct MusicState {
    pub scheduling: Arc<scheduling_state::SchedulingState>,
    pub melody: Arc<melody_state::MelodyState>,
}

impl Clone for MusicState {
    fn clone(&self) -> Self {
        MusicState {
            scheduling: Arc::clone(&self.scheduling),
            melody: Arc::clone(&self.melody),
        }
    }
}

impl MusicState {
    pub fn add_new_note_on_event(&self, pitch: f32, samples: i64) -> Self {
        MusicState {
            scheduling: Arc::clone(&self.scheduling),
            melody: Arc::new(self.melody.add_new_note_on_event(pitch, samples)),
        }
    }

    pub fn add_new_note_off_event(&self, samples: i64) -> Self {
        MusicState {
            scheduling: Arc::clone(&self.scheduling),
            melody: Arc::new(self.melody.add_new_note_off_event(samples)),
        }
    }

    pub fn change_current_melody_note_on(&self, pitch: f32, current_samples: i64) -> Self {
        MusicState {
            scheduling: Arc::clone(&self.scheduling),
            melody: Arc::new(
                self.melody
                    .change_current_melody_note_on(pitch, current_samples),
            ),
        }
    }

    pub fn change_current_melody_note_off(&self) -> Self {
        MusicState {
            scheduling: Arc::clone(&self.scheduling),
            melody: Arc::new(self.melody.change_current_melody_note_off()),
        }
    }

    pub fn change_cumulative_samples(&self, cumulative_samples: i64) -> Self {
        MusicState {
            scheduling: Arc::new(
                self.scheduling
                    .change_cumulative_samples(cumulative_samples),
            ),
            melody: Arc::clone(&self.melody),
        }
    }
}
