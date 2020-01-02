use std::f64::consts::PI;
use std::sync::Arc;

use super::super::state_management::store_reader::StoreReader;
use super::melody_state::{CurrentMelodyState, MelodyEvent, MelodyStateEvent};
use super::new_music_store::NewMusicStore;
use super::scheduling_state::SchedulingStateEvent;

pub struct WaveReader {
    store: Arc<NewMusicStore>,
    wave_length: i64,
}

impl WaveReader {
    pub fn new(store: Arc<NewMusicStore>) -> Self {
        WaveReader {
            store: Arc::clone(&store),
            wave_length: 512,
        }
    }
}

fn get_hertz(pitch: f32) -> f32 {
    // A4 -> 69 440hz
    440. * (2.0 as f32).powf((pitch - 69.) / 12.)
}

impl StoreReader<NewMusicStore, Vec<i16>> for WaveReader {
    fn get_store(&self) -> Arc<NewMusicStore> {
        Arc::clone(&self.store)
    }

    fn read(&self) -> Vec<i16> {
        let mut ret: Vec<i16> = Vec::new();
        ret.resize(self.wave_length as usize, 0);

        let scheduling_state = self.store.scheduling.get_state();
        let sf2_state = self.store.sf2.get_state();

        let current_cumulative_samples = scheduling_state.cumulative_samples;
        let next_cumulative_samples = current_cumulative_samples + self.wave_length;

        for (melody_key, melody_store) in self.store.melody.read().unwrap().iter() {
            let melody_state = melody_store.get_state();
            let event_seq = melody_state.event_seq.clone();
            let mut current_melody = melody_state.current_melody.clone();
            for (wave_idx, current_samples) in
                (current_cumulative_samples..next_cumulative_samples).enumerate()
            {
                let current_samples = match melody_state.repeat_length {
                    Some(repeat_length) => {
                        (current_samples - melody_state.repeat_start) % repeat_length
                    }
                    None => current_samples,
                };
                current_melody = match event_seq.get(&current_samples) {
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

                let addition = match current_melody {
                    CurrentMelodyState::On(pitch, samples) => {
                        if let Some(sf2) = &sf2_state.sf2 {
                            sf2.get_sample(0, pitch as u8, samples as usize)
                        } else {
                            let x = (samples as f32) * get_hertz(pitch) / 44100.0;
                            let x = x * 2.0 * (PI as f32);
                            (x.sin() * 30000.0) as i16
                        }
                    }
                    _ => 0,
                };
                ret[wave_idx] = ret[wave_idx].saturating_add(addition);
            }

            match current_melody {
                CurrentMelodyState::On(pitch, samples) => melody_store
                    .update_state(MelodyStateEvent::ChangeCurrentMelodyNoteOn(pitch, samples)),
                CurrentMelodyState::Off => {
                    melody_store.update_state(MelodyStateEvent::ChangeCurrentMelodyNoteOff)
                }
            };
        }

        self.store
            .scheduling
            .update_state(SchedulingStateEvent::ChangeCumulativeSamples(
                next_cumulative_samples,
            ));

        ret
    }
}
