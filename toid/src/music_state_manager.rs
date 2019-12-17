use std::sync::Arc;
use std::sync::RwLock;

use super::state_management::reducer::Reduce;
use super::state_management::reducer::Reducer;
use super::state_management::store::Store;
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
}

fn get_hertz(pitch: f32) -> f32 {
    // A4 -> 69 440hz
    440. * (2.0 as f32).powf((pitch - 69.) / 12.)
}

impl MusicStateManager {
    pub fn new(store: Arc<RwLock<Store<MusicState>>>) -> Self {
        let reduce = MusicStateReduce {};
        let reducer = Reducer::new(Arc::clone(&store), Box::new(reduce));
        MusicStateManager { store, reducer }
    }

    pub fn get_wave(&self) -> Vec<f32> {
        let mut ret = Vec::new();
        ret
    }
}
