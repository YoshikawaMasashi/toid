use std::collections::BTreeMap;
use std::f64::consts::PI;
use std::iter::Iterator;
use std::ops::Bound::{Excluded, Included};
use std::sync::Arc;

use log::error;

use super::super::data::music_info::{Beat, Note, Track};
use super::super::resource_management::resource_manager::ResourceManager;

pub struct TrackPlayer {
    wave_length: u64,
    played_notes: BTreeMap<u64, Vec<(u64, Note)>>,
}

fn get_hertz(pitch: f32) -> f32 {
    // A4 -> 69 440hz
    440. * (2.0 as f32).powf((pitch - 69.) / 12.)
}

impl TrackPlayer {
    pub fn new() -> Self {
        Self {
            wave_length: 512,
            played_notes: BTreeMap::new(),
        }
    }

    pub fn clean(&mut self) {
        self.played_notes = BTreeMap::new();
    }

    pub fn play(
        &mut self,
        track: &Track,
        resource_manager: Arc<ResourceManager>,
        cum_current_samples: &u64,
        cum_current_beats: &Beat,
        current_bpm: &f32,
    ) -> Vec<i16> {
        let mut ret: Vec<i16> = Vec::new();
        ret.resize(self.wave_length as usize, 0);

        let cum_next_samples = cum_current_samples + self.wave_length;
        let cum_next_beats =
            *cum_current_beats + Beat::from(self.wave_length as f32 * current_bpm / 44100.0 / 60.0);

        // 付け加えるnotesをリストアップする。
        // self.played_notesに加える。
        let rep_current_beats = *cum_current_beats % track.phrase.length;
        let rep_next_beats = cum_next_beats % track.phrase.length;

        if rep_current_beats < rep_next_beats {
            for (_, new_notes) in track
                .phrase
                .notes
                .range((Included(rep_current_beats), Excluded(rep_next_beats)))
            {
                self.register_notes(
                    new_notes,
                    &rep_current_beats,
                    &current_bpm,
                    &cum_current_samples,
                );
            }
        } else {
            for (_, new_notes) in track
                .phrase
                .notes
                .range((Included(rep_current_beats), Excluded(track.phrase.length)))
            {
                self.register_notes(
                    new_notes,
                    &rep_current_beats,
                    &current_bpm,
                    &cum_current_samples,
                );
            }
            for (_, new_notes) in track
                .phrase
                .notes
                .range((Included(Beat::from(0)), Excluded(rep_next_beats)))
            {
                self.register_notes(
                    new_notes,
                    &rep_current_beats,
                    &current_bpm,
                    &cum_current_samples,
                );
            }
        }

        // self.played_notesのを鳴らす
        match &track.sf2_name {
            None => {
                for (&cum_end_samples, notes) in self.played_notes.iter() {
                    for (cum_start_samples, note) in notes.iter() {
                        let herts_par_sample = get_hertz(note.pitch) / 44100.0;
                        let start_idx = if *cum_start_samples <= *cum_current_samples {
                            0
                        } else {
                            (cum_start_samples - cum_current_samples) as usize
                        };
                        let end_idx = if cum_end_samples >= cum_next_samples {
                            self.wave_length as usize
                        } else {
                            (cum_end_samples - cum_current_samples) as usize
                        };

                        for i in start_idx..end_idx {
                            let x = (cum_current_samples + i as u64 - cum_start_samples) as f32
                                * herts_par_sample;
                            let x = x * 2.0 * (PI as f32);
                            let addition = (x.sin() * 15000.0) as i16;
                            ret[i] = ret[i].saturating_add(addition);
                        }
                    }
                }
            }
            Some(sf2_name) => {
                let sf2 = resource_manager.get_sf2(sf2_name.to_string());
                match sf2 {
                    Ok(sf2) => {
                        for (&cum_end_samples, notes) in self.played_notes.iter() {
                            for (cum_start_samples, note) in notes.iter() {
                                let start_idx = if *cum_start_samples <= *cum_current_samples {
                                    0
                                } else {
                                    (cum_start_samples - cum_current_samples) as usize
                                };
                                let end_idx = if cum_end_samples >= cum_next_samples {
                                    self.wave_length as usize
                                } else {
                                    (cum_end_samples - cum_current_samples) as usize
                                };

                                let start_idx_for_sample = (cum_current_samples + start_idx as u64
                                    - cum_start_samples)
                                    as usize;
                                let end_idx_for_sample = (cum_current_samples + end_idx as u64
                                    - cum_start_samples)
                                    as usize;

                                let sample_data = sf2.get_samples(
                                    0,
                                    note.pitch as u8,
                                    start_idx_for_sample,
                                    end_idx_for_sample,
                                );
                                match sample_data {
                                    Ok(sample_data) => {
                                        for (i, j) in (start_idx..end_idx).enumerate() {
                                            ret[j] = ret[j].saturating_add(sample_data[i]);
                                        }
                                    }
                                    Err(e) => {
                                        // TODO:
                                        error!("error {}", e);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("sf2 error {}", e);
                    }
                }
            }
        };

        // 使ったself.played_notesのノートを消す
        for cum_note_samples in *cum_current_samples..cum_next_samples {
            self.played_notes.remove(&cum_note_samples);
        }

        ret
    }

    fn register_notes(
        &mut self,
        notes: &Vec<Note>,
        rep_current_beats: &Beat,
        current_bpm: &f32,
        cum_current_samples: &u64,
    ) {
        for &note in notes.iter() {
            let rep_note_beats = note.start;
            let cum_start_samples =
                ((rep_note_beats - *rep_current_beats).to_f32() * 44100.0 * 60.0 / current_bpm)
                    as u64
                    + cum_current_samples;
            let cum_end_samples =
                cum_start_samples + (note.duration.to_f32() * 44100.0 * 60.0 / current_bpm) as u64;

            if self.played_notes.contains_key(&cum_end_samples) {
                match self.played_notes.get_mut(&cum_end_samples) {
                    Some(notes_in_cum_end_samples) => {
                        notes_in_cum_end_samples.push((cum_start_samples, note));
                    }
                    None => {
                        error!("get_mut failed");
                    }
                };
            } else {
                self.played_notes
                    .insert(cum_end_samples, vec![(cum_start_samples, note)]);
            }
        }
    }
}
