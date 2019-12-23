use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::sync::Arc;
use std::sync::RwLock;

use super::state_management::reducer::Reduce;
use super::state_management::reducer::Reducer;
use super::state_management::serialize;
use super::state_management::store::Store;
use super::states::music_state::melody_state::CurrentMelodyState;
use super::states::music_state::melody_state::MelodyEvent;
use super::states::music_state::MusicState;

#[derive(Serialize, Deserialize)]
pub enum MusicStateEvent {
    AddNewNoteOn(f32, i64),
    AddNewNoteOff(i64),
    ChangeCurrentMalodyNoteOn(f32, i64),
    ChangeCurrentMelodyNoteOff,
    ChangeCumulativeSamples(i64),
}

impl serialize::Serialize for MusicStateEvent {
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn deserialize(&self, serialized: String) -> Self {
        serde_json::from_str(serialized.as_str()).unwrap()
    }
}

pub struct MusicStateReduce {}

impl Reduce<MusicState, MusicStateEvent> for MusicStateReduce {
    fn reduce(&self, state: MusicState, event: MusicStateEvent) -> MusicState {
        match event {
            MusicStateEvent::AddNewNoteOn(pitch, samples) => {
                state.add_new_note_on_event(pitch, samples)
            }
            MusicStateEvent::AddNewNoteOff(samples) => state.add_new_note_off_event(samples),
            MusicStateEvent::ChangeCurrentMalodyNoteOn(pitch, samples) => {
                state.change_current_melody_note_on(pitch, samples)
            }
            MusicStateEvent::ChangeCurrentMelodyNoteOff => state.change_current_melody_note_off(),
            MusicStateEvent::ChangeCumulativeSamples(samples) => {
                state.change_cumulative_samples(samples)
            }
        }
    }
}

pub struct MusicStateManager {
    store: Arc<RwLock<Store<MusicState>>>,
    reducer: Reducer<MusicState, MusicStateEvent>,
    wave_length: i32,
}

fn get_hertz(pitch: f32) -> f32 {
    // A4 -> 69 440hz
    440. * (2.0 as f32).powf((pitch - 69.) / 12.)
}

impl MusicStateManager {
    pub fn new(store: Arc<RwLock<Store<MusicState>>>) -> Self {
        let reduce = MusicStateReduce {};
        let reducer = Reducer::new(Arc::clone(&store), Box::new(reduce));
        MusicStateManager {
            store,
            reducer,
            wave_length: 512,
        }
    }

    pub fn get_wave(&self) -> Vec<f32> {
        let mut ret = Vec::new();

        let state = self.store.read().unwrap().get_state();
        let current_cumulative_samples = state.scheduling.cumulative_samples;
        let next_cumulative_samples = current_cumulative_samples + (self.wave_length as i64);
        let event_seq = state.melody.event_seq.clone();
        let mut current_melody = state.melody.current_melody.clone();

        for i in current_cumulative_samples..next_cumulative_samples {
            current_melody = match event_seq.get(&i) {
                Some(melody_event) => match melody_event {
                    MelodyEvent::On(pitch) => CurrentMelodyState::On(*pitch, 0),
                    MelodyEvent::Off => CurrentMelodyState::Off,
                },
                None => match current_melody {
                    CurrentMelodyState::On(pitch, samples) => {
                        CurrentMelodyState::On(pitch, samples + 1)
                    }
                    CurrentMelodyState::Off => CurrentMelodyState::Off,
                },
            };

            let ret_ = match current_melody {
                CurrentMelodyState::On(pitch, samples) => {
                    let x = (samples as f32) * get_hertz(pitch) / 44100.0;
                    let x = x * 2.0 * (PI as f32);
                    x.sin()
                }
                CurrentMelodyState::Off => 0.0,
            };
            ret.push(ret_);
        }

        match current_melody {
            CurrentMelodyState::On(pitch, samples) => self
                .reducer
                .reduce(MusicStateEvent::ChangeCurrentMalodyNoteOn(pitch, samples)),
            CurrentMelodyState::Off => self
                .reducer
                .reduce(MusicStateEvent::ChangeCurrentMelodyNoteOff),
        };
        self.reducer
            .reduce(MusicStateEvent::ChangeCumulativeSamples(
                next_cumulative_samples,
            ));

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_is_close(a: f32, b: f32, delta: f32) {
        if (a - b).abs() > delta {
            panic!("is not close: {} {}", a, b)
        }
    }

    #[test]
    fn state_works() {
        let initial_state = MusicState::new();
        let store = Arc::new(RwLock::new(Store::new(initial_state)));

        let state = store.read().unwrap().get_state();
        assert_eq!(state.scheduling.bpm, 120.0);

        let manager = MusicStateManager::new(Arc::clone(&store));

        let reducer = Reducer::new(Arc::clone(&store), Box::new(MusicStateReduce {}));
        reducer.reduce(MusicStateEvent::AddNewNoteOn(69.0, 0));
        let state = store.read().unwrap().get_state();
        let first_note_on_pitch = match state.melody.event_seq.get(&0).unwrap() {
            MelodyEvent::On(pitch) => *pitch,
            _ => panic!("ERROR"),
        };
        assert_eq!(first_note_on_pitch, 69.0);

        let wave = manager.get_wave();

        let true_wave = [
            0., 0.06268834, 0.12537667, 0.188065, 0.25075334, 0.3134417, 0.37613, 0.43881837,
            0.5015067, 0.56419504,
        ];
        for i in 0..512 {
            println!("{}", wave[i]);
        }
        for i in 0..10 {
            assert_is_close(wave[i], true_wave[i], 0.03);
        }

        let state = store.read().unwrap().get_state();
        assert_eq!(state.scheduling.cumulative_samples, 512);
    }
}
