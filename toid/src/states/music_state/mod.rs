use std::sync::Arc;

use super::super::data::sf2::SF2;

pub mod melody_state;
pub mod scheduling_state;
pub mod sf2_state;

pub struct MusicState {
    pub scheduling: Arc<scheduling_state::SchedulingState>,
    pub melody: Arc<melody_state::MelodyState>,
    pub sf2: Arc<sf2_state::SF2State>,
}

impl Clone for MusicState {
    fn clone(&self) -> Self {
        MusicState {
            scheduling: Arc::clone(&self.scheduling),
            melody: Arc::clone(&self.melody),
            sf2: Arc::clone(&self.sf2),
        }
    }
}

impl MusicState {
    pub fn new() -> Self {
        MusicState {
            scheduling: Arc::new(scheduling_state::SchedulingState::new()),
            melody: Arc::new(melody_state::MelodyState::new()),
            sf2: Arc::new(sf2_state::SF2State::new()),
        }
    }

    pub fn add_new_note_on_event(&self, pitch: f32, samples: i64) -> Self {
        MusicState {
            scheduling: Arc::clone(&self.scheduling),
            melody: Arc::new(self.melody.add_new_note_on_event(pitch, samples)),
            sf2: Arc::clone(&self.sf2),
        }
    }

    pub fn add_new_note_off_event(&self, samples: i64) -> Self {
        MusicState {
            scheduling: Arc::clone(&self.scheduling),
            melody: Arc::new(self.melody.add_new_note_off_event(samples)),
            sf2: Arc::clone(&self.sf2),
        }
    }

    pub fn change_current_melody_note_on(&self, pitch: f32, current_samples: i64) -> Self {
        MusicState {
            scheduling: Arc::clone(&self.scheduling),
            melody: Arc::new(
                self.melody
                    .change_current_melody_note_on(pitch, current_samples),
            ),
            sf2: Arc::clone(&self.sf2),
        }
    }

    pub fn change_current_melody_note_off(&self) -> Self {
        MusicState {
            scheduling: Arc::clone(&self.scheduling),
            melody: Arc::new(self.melody.change_current_melody_note_off()),
            sf2: Arc::clone(&self.sf2),
        }
    }

    pub fn change_cumulative_samples(&self, cumulative_samples: i64) -> Self {
        MusicState {
            scheduling: Arc::new(
                self.scheduling
                    .change_cumulative_samples(cumulative_samples),
            ),
            melody: Arc::clone(&self.melody),
            sf2: Arc::clone(&self.sf2),
        }
    }

    pub fn set_sf2(&self, sf2: Arc<SF2>) -> Self {
        MusicState {
            scheduling: Arc::clone(&self.scheduling),
            melody: Arc::clone(&self.melody),
            sf2: Arc::new(self.sf2.set_sf2(sf2)),
        }
    }
}
