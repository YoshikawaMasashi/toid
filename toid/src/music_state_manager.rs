use std::f64::consts::PI;
use std::sync::Arc;
use std::sync::RwLock;

use super::state_management::reducer::Reduce;
use super::state_management::reducer::Reducer;
use super::state_management::store::Store;
use super::states::music_state::melody_state::CurrentMelodyState;
use super::states::music_state::melody_state::MelodyEvent;
use super::states::music_state::melody_state::MelodyState;
use super::states::music_state::MusicState;

pub enum MusicStateEvent {
    AddNewNoteOn(f32, i64),
    AddNewNoteOff(i64),
    ChangeCurrentMalodyNoteOn(f32, i64),
    ChangeCurrentMelodyNoteOff,
    ChangeCumulativeSamples(i64),
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
        let current_cumulative_samples = self
            .store
            .read()
            .unwrap()
            .get_state()
            .scheduling
            .cumulative_samples;
        let next_cumulative_samples = current_cumulative_samples + (self.wave_length as i64);
        let event_seq = self
            .store
            .read()
            .unwrap()
            .get_state()
            .melody
            .event_seq
            .clone();
        let mut current_melody = self
            .store
            .read()
            .unwrap()
            .get_state()
            .melody
            .current_melody
            .clone();

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
                    let x = ((samples as f32) * 44100.0 / get_hertz(pitch));
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
