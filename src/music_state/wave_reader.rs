use std::collections::BTreeMap;
use std::f64::consts::PI;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::sync::Arc;

use log::error;
use serde::{Deserialize, Serialize};

use super::super::data::music_info::beat::Beat;
use super::super::data::music_info::note::Note;
use super::super::resource_management::resource_manager::ResourceManager;
use super::super::state_management::serialize;
use super::super::state_management::store::Store;
use super::super::state_management::store_reader::StoreReader;
use super::music_state::{MusicState, MusicStateEvent};

pub struct WaveReader {
    wave_length: u64,
    played_notes: BTreeMap<u64, Vec<(u64, Note)>>,
    cum_current_samples: u64,
    current_bpm: f32,
    bpm_change_samples: u64,
    bpm_change_beats: Beat,
    cum_current_beats: Beat,
}

fn get_hertz(pitch: f32) -> f32 {
    // A4 -> 69 440hz
    440. * (2.0 as f32).powf((pitch - 69.) / 12.)
}

impl StoreReader<Vec<i16>, WaveReaderEvent, MusicState, MusicStateEvent> for WaveReader {
    fn new() -> Self {
        WaveReader {
            wave_length: 512,
            played_notes: BTreeMap::new(),
            cum_current_samples: 0,
            current_bpm: 120.0,
            bpm_change_samples: 0,
            bpm_change_beats: Beat::from(0),
            cum_current_beats: Beat::from(0),
        }
    }

    fn read(
        &mut self,
        store: Arc<Store<MusicState, MusicStateEvent>>,
        resource_manager: Arc<ResourceManager>,
    ) -> Vec<i16> {
        let mut ret: Vec<i16> = Vec::new();
        ret.resize(self.wave_length as usize, 0);

        let music_state = match store.get_state() {
            Ok(music_state) => music_state,
            Err(e) => {
                error!("get_state Error {}", e);
                return ret;
            }
        };
        let sf2_state = &music_state.sf2;
        let scheduling_state = &music_state.scheduling;

        let cum_next_samples = self.cum_current_samples + self.wave_length;

        if let Some((&_, &new_bpm)) = scheduling_state
            .bpm_schedule
            .range((Unbounded, Included(self.cum_current_beats)))
            .rev()
            .next()
        {
            if new_bpm != self.current_bpm {
                self.current_bpm = new_bpm;
                self.bpm_change_samples = self.cum_current_samples;
                self.bpm_change_beats = self.cum_current_beats;
            }
        }
        let cum_next_beats = self.cum_current_beats
            + Beat::from(self.wave_length as f32 * self.current_bpm / 44100.0 / 60.0);

        for (_, phrase) in music_state.phrase_map.iter() {
            // 付け加えるnotesをリストアップする。
            // self.played_notesに加える。
            let rep_current_beats = self.cum_current_beats % phrase.repeat_length;
            let rep_next_beats = cum_next_beats % phrase.repeat_length;

            if rep_current_beats < rep_next_beats {
                for (&rep_note_beats, new_notes) in phrase
                    .notes
                    .range((Included(rep_current_beats), Excluded(rep_next_beats)))
                {
                    for new_note in new_notes.iter() {
                        let cum_start_samples =
                            ((rep_note_beats - rep_current_beats).to_f32() * 44100.0 * 60.0
                                / self.current_bpm) as u64
                                + self.cum_current_samples;
                        let cum_end_samples = cum_start_samples
                            + (new_note.duration.to_f32() * 44100.0 * 60.0 / self.current_bpm)
                                as u64;

                        if self.played_notes.contains_key(&cum_end_samples) {
                            match self.played_notes.get_mut(&cum_end_samples) {
                                Some(notes_in_cum_end_samples) => {
                                    notes_in_cum_end_samples.push((cum_start_samples, *new_note));
                                }
                                None => {
                                    error!("get_mut failed");
                                }
                            };
                        } else {
                            self.played_notes
                                .insert(cum_end_samples, vec![(cum_start_samples, *new_note)]);
                        }
                    }
                }
            } else {
                for (&rep_note_beats, new_notes) in phrase
                    .notes
                    .range((Included(rep_current_beats), Excluded(phrase.repeat_length)))
                {
                    for new_note in new_notes.iter() {
                        let cum_start_samples =
                            ((rep_note_beats - rep_current_beats).to_f32() * 44100.0 * 60.0
                                / self.current_bpm) as u64
                                + self.cum_current_samples;
                        let cum_end_samples = cum_start_samples
                            + (new_note.duration.to_f32() * 44100.0 * 60.0 / self.current_bpm)
                                as u64;

                        if self.played_notes.contains_key(&cum_end_samples) {
                            match self.played_notes.get_mut(&cum_end_samples) {
                                Some(notes_in_cum_end_samples) => {
                                    notes_in_cum_end_samples.push((cum_start_samples, *new_note));
                                }
                                None => {
                                    error!("get_mut failed");
                                }
                            };
                        } else {
                            self.played_notes
                                .insert(cum_end_samples, vec![(cum_start_samples, *new_note)]);
                        }
                    }
                }
                for (&rep_note_beats, new_notes) in phrase
                    .notes
                    .range((Included(Beat::from(0)), Excluded(rep_next_beats)))
                {
                    for new_note in new_notes.iter() {
                        let cum_start_samples =
                            ((phrase.repeat_length + rep_note_beats - rep_current_beats).to_f32()
                                * 44100.0
                                * 60.0
                                / self.current_bpm) as u64
                                + self.cum_current_samples;
                        let cum_end_samples = cum_start_samples
                            + (new_note.duration.to_f32() * 44100.0 * 60.0 / self.current_bpm)
                                as u64;

                        if self.played_notes.contains_key(&cum_end_samples) {
                            match self.played_notes.get_mut(&cum_end_samples) {
                                Some(notes_in_cum_end_samples) => {
                                    notes_in_cum_end_samples.push((cum_start_samples, *new_note));
                                }
                                None => {
                                    error!("get_mut failed");
                                }
                            };
                        } else {
                            self.played_notes
                                .insert(cum_end_samples, vec![(cum_start_samples, *new_note)]);
                        }
                    }
                }
            }
        }

        // self.played_notesのを鳴らす
        match &sf2_state.sf2_name {
            None => {
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
                            let x = (self.cum_current_samples + i as u64 - cum_start_samples)
                                as f32
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

                                let start_idx_for_sample = (self.cum_current_samples
                                    + start_idx as u64
                                    - cum_start_samples)
                                    as usize;
                                let end_idx_for_sample = (self.cum_current_samples + end_idx as u64
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
        for cum_note_samples in self.cum_current_samples..cum_next_samples {
            self.played_notes.remove(&cum_note_samples);
        }

        self.cum_current_samples = cum_next_samples;
        self.cum_current_beats = cum_next_beats;

        ret
    }

    fn apply(&mut self, event: WaveReaderEvent) {
        match event {
            WaveReaderEvent::MoveStart => {
                self.played_notes = BTreeMap::new();
                self.cum_current_samples = 0;
                self.current_bpm = 120.0;
                self.bpm_change_samples = 0;
                self.bpm_change_beats = Beat::from(0);
                self.cum_current_beats = Beat::from(0);
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum WaveReaderEvent {
    MoveStart,
}

impl serialize::Serialize<WaveReaderEvent> for WaveReaderEvent {
    fn serialize(&self) -> Result<String, String> {
        match serde_json::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(err) => Err(format!("error in serizalization : {}", err)),
        }
    }
    fn deserialize(serialized: String) -> Result<Self, String> {
        match serde_json::from_str(serialized.as_str()) {
            Ok(deserialized) => Ok(deserialized),
            Err(err) => Err(format!("error in deserizalization : {}", err)),
        }
    }
}
