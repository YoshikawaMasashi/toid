use std::boxed::Box;
use std::f64::consts::PI;
use std::sync::Arc;
use std::sync::RwLock;

use super::data::sf2::SF2;
use super::reducers::default_reducer::DefaultReducer;
use super::state_management::reducer::Reduce;
use super::state_management::reducer::Reducer;
use super::state_management::store::Store;
use super::states::music_state::melody_state::CurrentMelodyState;
use super::states::music_state::melody_state::MelodyEvent;
use super::states::music_state::MusicState;

pub enum MusicStateEvent {
    AddNewNoteOn(f32, i64),
    AddNewNoteOff(i64),
    ChangeCurrentMalodyNoteOn(f32, i64),
    ChangeCurrentMelodyNoteOff,
    ChangeCumulativeSamples(i64),
    SetSF2(Arc<SF2>),
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
            MusicStateEvent::SetSF2(sf2) => state.set_sf2(sf2),
        }
    }
}

pub struct MusicStateManager {
    store: Arc<RwLock<Box<dyn Store<MusicState>>>>,
    reducer: Arc<dyn Reducer<MusicState, MusicStateEvent>>,
    wave_length: i32,
}

fn get_hertz(pitch: f32) -> f32 {
    // A4 -> 69 440hz
    440. * (2.0 as f32).powf((pitch - 69.) / 12.)
}

impl MusicStateManager {
    pub fn new(store: Arc<RwLock<Box<dyn Store<MusicState>>>>) -> Self {
        let reduce = MusicStateReduce {};
        let reducer = Arc::new(DefaultReducer::new(Arc::clone(&store), Box::new(reduce)));
        MusicStateManager {
            store,
            reducer,
            wave_length: 512,
        }
    }

    pub fn get_reducer(&self) -> Arc<dyn Reducer<MusicState, MusicStateEvent>> {
        Arc::clone(&self.reducer)
    }

    pub fn get_wave(&self) -> Vec<i16> {
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
                    if let Some(sf2) = &state.sf2.sf2 {
                        sf2.get_sample(0, pitch as u8, samples as usize)
                    } else {
                        let x = (samples as f32) * get_hertz(pitch) / 44100.0;
                        let x = x * 2.0 * (PI as f32);
                        (x.sin() * 30000.0) as i16
                    }
                }
                CurrentMelodyState::Off => 0,
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
    use super::super::stores::default_store::DefaultStore;
    use super::*;

    #[test]
    fn state_works() {
        let initial_state = MusicState::new();
        let store: Box<dyn Store<MusicState>> = Box::new(DefaultStore::new(initial_state));
        let store = Arc::new(RwLock::new(store));

        let state = store.read().unwrap().get_state();
        assert_eq!(state.scheduling.bpm, 120.0);

        let manager = MusicStateManager::new(Arc::clone(&store));

        let reducer = DefaultReducer::new(Arc::clone(&store), Box::new(MusicStateReduce {}));
        reducer.reduce(MusicStateEvent::AddNewNoteOn(69.0, 0));
        let state = store.read().unwrap().get_state();
        let first_note_on_pitch = match state.melody.event_seq.get(&0).unwrap() {
            MelodyEvent::On(pitch) => *pitch,
            _ => panic!("ERROR"),
        };
        assert_eq!(first_note_on_pitch, 69.0);

        let wave = manager.get_wave();

        let true_wave = [0, 1879, 3751, 5608, 7444, 9250, 11019, 12746, 14422, 16042];
        for i in 0..512 {
            println!("{}", wave[i]);
        }
        for i in 0..10 {
            assert_eq!(wave[i], true_wave[i]);
        }

        let state = store.read().unwrap().get_state();
        assert_eq!(state.scheduling.cumulative_samples, 512);
    }
}
