use std::collections::BTreeMap;
use std::f64::consts::PI;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::sync::Arc;
use std::sync::RwLock;

use super::super::state_management::store_reader::StoreReader;
use super::melody_state::{MelodyStateEvent, NoteInfo};
use super::new_music_store::NewMusicStore;
use super::scheduling_state::SchedulingStateEvent;

pub struct WaveReader {
    store: Arc<NewMusicStore>,
    wave_length: u64,
    played_notes: BTreeMap<u64, Vec<(u64, NoteInfo)>>,
    current_cumulative_samples: u64,
}

impl WaveReader {
    pub fn new(store: Arc<NewMusicStore>) -> Self {
        WaveReader {
            store: Arc::clone(&store),
            wave_length: 512,
            played_notes: BTreeMap::new(),
            current_cumulative_samples: 0,
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

    fn read(&mut self) -> Vec<i16> {
        let mut ret: Vec<i16> = Vec::new();
        ret.resize(self.wave_length as usize, 0);

        // let scheduling_state = self.store.scheduling.get_state();
        // let sf2_state = self.store.sf2.get_state();

        let next_cumulative_samples = self.current_cumulative_samples + self.wave_length;

        // TODO: samplesがどっちなのかちゃんと確かめる
        for (_, melody_store) in self.store.melody.read().unwrap().iter() {
            // 付け加えるnotesをリストアップする。
            // self.played_notesに加える。
            let melody_state = melody_store.get_state();

            let current_cumulative_samples_in_repeat =
                self.current_cumulative_samples % melody_state.repeat_length;
            let next_cumulative_samples_in_repeat =
                next_cumulative_samples & melody_state.repeat_length;

            if current_cumulative_samples_in_repeat < next_cumulative_samples_in_repeat {
                for (samples, new_notes) in melody_state.notes.range((
                    Included(current_cumulative_samples_in_repeat),
                    Excluded(next_cumulative_samples_in_repeat),
                )) {
                    for new_note in new_notes.iter() {
                        let start_samples = samples - current_cumulative_samples_in_repeat
                            + self.current_cumulative_samples;
                        let end_samples = start_samples + new_note.duration;

                        if self.played_notes.contains_key(&end_samples) {
                            self.played_notes
                                .get_mut(&end_samples)
                                .unwrap()
                                .push((start_samples, *new_note));
                        } else {
                            self.played_notes
                                .insert(end_samples, vec![(start_samples, *new_note)]);
                        }
                    }
                }
            } else {
                for (samples, new_notes) in melody_state.notes.range((
                    Included(current_cumulative_samples_in_repeat),
                    Excluded(melody_state.repeat_length),
                )) {
                    for new_note in new_notes.iter() {
                        let start_samples = samples - current_cumulative_samples_in_repeat
                            + self.current_cumulative_samples;
                        let end_samples = start_samples + new_note.duration;

                        if self.played_notes.contains_key(&end_samples) {
                            self.played_notes
                                .get_mut(&end_samples)
                                .unwrap()
                                .push((start_samples, *new_note));
                        } else {
                            self.played_notes
                                .insert(end_samples, vec![(start_samples, *new_note)]);
                        }
                    }
                }
                for (samples, new_notes) in melody_state
                    .notes
                    .range((Included(0), Excluded(next_cumulative_samples_in_repeat)))
                {
                    for new_note in new_notes.iter() {
                        let start_samples = samples - current_cumulative_samples_in_repeat
                            + self.current_cumulative_samples;
                        let end_samples = start_samples + new_note.duration;

                        if self.played_notes.contains_key(&end_samples) {
                            self.played_notes
                                .get_mut(&end_samples)
                                .unwrap()
                                .push((start_samples, *new_note));
                        } else {
                            self.played_notes
                                .insert(end_samples, vec![(start_samples, *new_note)]);
                        }
                    }
                }
            }
        }

        // TODO: self.played_notesのを鳴らす
        for (&end_samples, notes) in self.played_notes.iter() {
            for (start_samples, note) in notes.iter() {
                let freq_samples = get_hertz(note.pitch) / 44100.0;
                if *start_samples <= self.current_cumulative_samples {
                    if end_samples >= next_cumulative_samples {
                        for i in 0..self.wave_length as usize {
                            let x = (self.current_cumulative_samples - start_samples + i as u64)
                                as f32
                                * freq_samples;
                            let x = x * 2.0 * (PI as f32);
                            let addition = (x.sin() * 30000.0) as i16;
                            ret[i] = ret[i].saturating_add(addition);
                        }
                    } else {
                        for i in 0..(end_samples - self.current_cumulative_samples) as usize {
                            let x = (self.current_cumulative_samples - start_samples + i as u64)
                                as f32
                                * freq_samples;
                            let x = x * 2.0 * (PI as f32);
                            let addition = (x.sin() * 30000.0) as i16;
                            ret[i] = ret[i].saturating_add(addition);
                        }
                    }
                } else {
                    if end_samples >= next_cumulative_samples {
                    } else {
                    }
                }
            }
        }

        // 使ったself.played_notesのノートを消す
        for samples in self.current_cumulative_samples..next_cumulative_samples {
            self.played_notes.remove(&samples);
        }

        self.current_cumulative_samples = next_cumulative_samples;

        ret
    }
}
