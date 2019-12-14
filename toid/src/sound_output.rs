//! MVP的にmonophonicなメロディをライブコーディングするモジュールです。
//! sound_outputモジュールでは、stateモジュールのユースケースとなります。

use std::boxed::Box;
use std::option::Option;
use std::sync::Arc;
use std::sync::RwLock;
use std::vec::Vec;

use super::state::Reduce;
use super::state::Reducer;
use super::state::Store;

/// state::Storeで使う用のStateです。
/// SoundStateから、audioのAPIのコールバックで使う用の波形が取得できます。
pub struct SoundState {
    phase: f32,
    pitch: i32,
    sound_on: bool,
    wave_length: usize,
}

impl SoundState {
    pub fn new(wave_length: usize) -> Self {
        SoundState {
            phase: 0.0,
            pitch: 60,
            sound_on: true,
            wave_length,
        }
    }
}

struct SoundStateManager {
    store: Arc<RwLock<Store<SoundState>>>,
    reducer: Reducer<SoundState, SoundStateEvent>,
}

impl SoundStateManager {
    pub fn new(
        store: Arc<RwLock<Store<SoundState>>>,
        reducer: Reducer<SoundState, SoundStateEvent>,
    ) -> Self {
        SoundStateManager { store, reducer }
    }
    pub fn get_wave(&self) -> Vec<f32> {
        let mut ret = Vec::new();
        let state = self.store.read().unwrap().get_state();
        let hertz = self.get_hertz(state.pitch);

        if state.sound_on {
            for wave_idx in 0..state.wave_length {
                ret.push(((state.phase + wave_idx as f32) * hertz / (44100 as f32)).sin());
            }
            let next_phase =
                (state.phase + (state.wave_length as f32) * hertz / (44100 as f32)) % 1.0;
            println!("{}", next_phase);
            self.reducer
                .reduce(&SoundStateEvent::ChangePhase(next_phase));
        } else {
            for wave_idx in 0..state.wave_length {
                ret.push(0.0);
            }
        }
        ret
    }

    fn get_hertz(&self, pitch: i32) -> f32 {
        // A4 -> 69 440hz
        (440 as f32) * (2.0 as f32).powf(((pitch - 69) as f32) / 12 as f32)
    }
}

enum SoundStateEvent {
    ChangePitch(i32),
    SoundOn,
    SoundOff,
    ChangePhase(f32),
}

struct SoundStateReduce {}

impl Reduce<SoundState, SoundStateEvent> for SoundStateReduce {
    fn reduce(&self, state: Arc<SoundState>, event: &SoundStateEvent) -> SoundState {
        match event {
            SoundStateEvent::ChangePitch(pitch) => SoundState {
                phase: state.phase,
                pitch: *pitch,
                sound_on: state.sound_on,
                wave_length: state.wave_length,
            },
            SoundStateEvent::SoundOn => SoundState {
                phase: state.phase,
                pitch: state.pitch,
                sound_on: true,
                wave_length: state.wave_length,
            },
            SoundStateEvent::SoundOff => SoundState {
                phase: state.phase,
                pitch: state.pitch,
                sound_on: false,
                wave_length: state.wave_length,
            },
            SoundStateEvent::ChangePhase(phase) => SoundState {
                phase: *phase,
                pitch: state.pitch,
                sound_on: false,
                wave_length: state.wave_length,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_is_close(a: f32, b: f32, delta: f32) {
        if (a - b).abs() > delta {
            panic!("is not close: {} {}")
        }
    }

    #[test]
    fn state_works() {
        let initial_state: SoundState = SoundState::new(512);
        let store = Arc::new(RwLock::new(Store::new(initial_state)));

        assert_eq!(store.read().unwrap().get_state().phase, 0.0);
        assert_eq!(store.read().unwrap().get_state().pitch, 60);
        assert_eq!(store.read().unwrap().get_state().sound_on, true);
        assert_eq!(store.read().unwrap().get_state().wave_length, 512);

        let reduce = Box::new(SoundStateReduce {});
        let reducer = Reducer::new(Arc::clone(&store), reduce);
        let manager = SoundStateManager::new(Arc::clone(&store), reducer);

        let reduce = Box::new(SoundStateReduce {});
        let reducer = Reducer::new(Arc::clone(&store), reduce);

        reducer.reduce(&SoundStateEvent::ChangePitch(69));
        assert_eq!(store.read().unwrap().get_state().pitch, 69);

        let wave = manager.get_wave();

        let true_wave = [
            0., 0.00997716, 0.01995432, 0.02993148, 0.03990864, 0.04988579, 0.05986295, 0.06984011,
            0.07981727, 0.08979443,
        ];

        for i in 0..10 {
            assert_is_close(wave[i], true_wave[i], 0.01);
        }

        assert_is_close(store.read().unwrap().get_state().phase, 0.108390026, 0.01);
    }
}
