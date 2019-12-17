use im::ordmap::OrdMap;
use std::sync::Arc;

pub enum CurrentMelodyState {
    On(f32, i64),
    Off,
}

pub enum MelodyEvent {
    On(f32),
    Off,
}

impl Clone for MelodyEvent {
    fn clone(&self) -> Self {
        match self {
            MelodyEvent::On(f) => MelodyEvent::On(*f),
            MelodyEvent::Off => MelodyEvent::Off,
        }
    }
}

impl Clone for CurrentMelodyState {
    fn clone(&self) -> Self {
        match self {
            CurrentMelodyState::On(f, i) => CurrentMelodyState::On(*f, *i),
            CurrentMelodyState::Off => CurrentMelodyState::Off,
        }
    }
}

pub struct MelodyState {
    pub event_seq: OrdMap<i64, MelodyEvent>,
    pub current_melody: CurrentMelodyState,
}

impl MelodyState {
    pub fn add_new_note_on_event(&self, pitch: f32, samples: i64) -> Self {
        MelodyState {
            event_seq: self.event_seq.update(samples, MelodyEvent::On(pitch)),
            current_melody: self.current_melody.clone(),
        }
    }

    pub fn add_new_note_off_event(&self, samples: i64) -> Self {
        MelodyState {
            event_seq: self.event_seq.update(samples, MelodyEvent::Off),
            current_melody: self.current_melody.clone(),
        }
    }

    pub fn change_current_melody_note_on(&self, pitch: f32, current_samples: i64) -> Self {
        MelodyState {
            event_seq: self.event_seq.clone(),
            current_melody: CurrentMelodyState::On(pitch, current_samples),
        }
    }

    pub fn change_current_melody_note_off(&self) -> Self {
        MelodyState {
            event_seq: self.event_seq.clone(),
            current_melody: CurrentMelodyState::Off,
        }
    }
}
