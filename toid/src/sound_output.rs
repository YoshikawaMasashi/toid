//! MVP的にmonophonicなメロディをライブコーディングするモジュールです。
//! sound_outputモジュールでは、stateモジュールのユースケースとなります。

use std::boxed::Box;
use std::option::Option;
use std::rc::Rc;
use std::vec::Vec;

use super::state::Reduce;

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

    pub fn get_wave(&self) -> Vec<f32> {
        let hertz = self.get_hertz();
        let mut ret = Vec::new();
        for wave_idx in 0..self.wave_length {
            ret.push(((self.phase + wave_idx as f32) * hertz).sin());
        }
        let next_phase = (self.phase + (self.wave_length as f32)) % 1.0;
        ret
    }

    fn get_hertz(&self) -> f32 {
        // A4 -> 69 440hz
        (440 as f32) * (2.0 as f32).powf(((self.pitch - 69) as f32) / 12 as f32)
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
    fn reduce(&self, state: Rc<SoundState>, event: &SoundStateEvent) -> SoundState {
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
