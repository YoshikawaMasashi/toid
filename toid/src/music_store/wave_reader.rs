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
    cum_current_samples: u64,
}

impl WaveReader {
    pub fn new(store: Arc<NewMusicStore>) -> Self {
        WaveReader {
            store: Arc::clone(&store),
            wave_length: 512,
            played_notes: BTreeMap::new(),
            cum_current_samples: 0,
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

        let cum_next_samples = self.cum_current_samples + self.wave_length;

        // TODO: samplesがどっちなのかちゃんと確かめる
        // TODO: 命名規則、cum_ -> cumurative, rep_ -> repeat, frm_ -> frame
        for (_, melody_store) in self.store.melody.read().unwrap().iter() {
            // 付け加えるnotesをリストアップする。
            // self.played_notesに加える。
            let melody_state = melody_store.get_state();

            let rep_current_samples = self.cum_current_samples % melody_state.repeat_length;
            let rep_next_samples = cum_next_samples % melody_state.repeat_length;

            if rep_current_samples < rep_next_samples {
                for (rep_note_samples, new_notes) in melody_state
                    .notes
                    .range((Included(rep_current_samples), Excluded(rep_next_samples)))
                {
                    for new_note in new_notes.iter() {
                        let cum_start_samples =
                            rep_note_samples - rep_current_samples + self.cum_current_samples;
                        let cum_end_samples = cum_start_samples + new_note.duration;

                        if self.played_notes.contains_key(&cum_end_samples) {
                            self.played_notes
                                .get_mut(&cum_end_samples)
                                .unwrap()
                                .push((cum_start_samples, *new_note));
                        } else {
                            self.played_notes
                                .insert(cum_end_samples, vec![(cum_start_samples, *new_note)]);
                        }
                    }
                }
            } else {
                for (rep_note_samples, new_notes) in melody_state.notes.range((
                    Included(rep_current_samples),
                    Excluded(melody_state.repeat_length),
                )) {
                    for new_note in new_notes.iter() {
                        let cum_start_samples =
                            rep_note_samples + self.cum_current_samples - rep_current_samples;
                        let cum_end_samples = cum_start_samples + new_note.duration;

                        if self.played_notes.contains_key(&cum_end_samples) {
                            self.played_notes
                                .get_mut(&cum_end_samples)
                                .unwrap()
                                .push((cum_start_samples, *new_note));
                        } else {
                            self.played_notes
                                .insert(cum_end_samples, vec![(cum_start_samples, *new_note)]);
                        }
                    }
                }
                for (rep_note_samples, new_notes) in melody_state
                    .notes
                    .range((Included(0), Excluded(rep_next_samples)))
                {
                    for new_note in new_notes.iter() {
                        let cum_start_samples = rep_note_samples + self.cum_current_samples
                            - rep_next_samples
                            + self.wave_length;
                        let cum_end_samples = cum_start_samples + new_note.duration;

                        if self.played_notes.contains_key(&cum_end_samples) {
                            self.played_notes
                                .get_mut(&cum_end_samples)
                                .unwrap()
                                .push((cum_start_samples, *new_note));
                        } else {
                            self.played_notes
                                .insert(cum_end_samples, vec![(cum_start_samples, *new_note)]);
                        }
                    }
                }
            }
        }

        // self.played_notesのを鳴らす
        for (&cum_end_samples, notes) in self.played_notes.iter() {
            for (cum_start_samples, note) in notes.iter() {
                let herts_par_sample = get_hertz(note.pitch) / 44100.0;
                let start_idx = if *cum_start_samples <= self.cum_current_samples {
                    0
                } else {
                    (cum_start_samples - self.cum_current_samples) as usize
                };
                let end_idx = if cum_end_samples >= cum_next_samples {
                    self.wave_length as usize
                } else {
                    (cum_end_samples - self.cum_current_samples) as usize
                };

                for i in start_idx..end_idx {
                    let x = (self.cum_current_samples + i as u64 - cum_start_samples) as f32
                        * herts_par_sample;
                    let x = x * 2.0 * (PI as f32);
                    let addition = (x.sin() * 15000.0) as i16;
                    ret[i] = ret[i].saturating_add(addition);
                }
            }
        }

        // 使ったself.played_notesのノートを消す
        for cum_note_samples in self.cum_current_samples..cum_next_samples {
            self.played_notes.remove(&cum_note_samples);
        }

        self.cum_current_samples = cum_next_samples;

        ret
    }
}
