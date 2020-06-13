use std::collections::{BTreeMap, BTreeSet};
use std::iter::Iterator;
use std::ops::Bound::{Excluded, Included};
use std::sync::Arc;

use log::error;

use super::super::data::music_info::{Beat, Instrument, SampleNote, Track};
use super::super::music_state::effects::{Effect, EffectInfo};
use super::super::resource_management::resource_manager::ResourceManager;

pub struct SampleTrackPlayer {
    wave_length: u64,
    played_notes: BTreeMap<u64, Vec<(u64, SampleNote)>>,
    effect_infos: Vec<EffectInfo>,
    effects: Vec<Box<dyn Effect + Sync + Send>>,
}

impl SampleTrackPlayer {
    pub fn new() -> Self {
        Self {
            wave_length: 512,
            played_notes: BTreeMap::new(),
            effect_infos: vec![],
            effects: vec![],
        }
    }

    pub fn clean(&mut self) {
        self.played_notes = BTreeMap::new();
    }

    pub fn play(
        &mut self,
        track: &Track<SampleNote>,
        resource_manager: Arc<ResourceManager>,
        cum_current_samples: &u64,
        cum_current_beats: &Beat,
        current_bpm: &f32,
    ) -> (Vec<f32>, Vec<f32>) {
        let mut left_wave: Vec<f32> = Vec::new();
        let mut right_wave: Vec<f32> = Vec::new();
        left_wave.resize(self.wave_length as usize, 0.0);
        right_wave.resize(self.wave_length as usize, 0.0);

        let cum_next_samples = cum_current_samples + self.wave_length;
        let cum_next_beats =
            *cum_current_beats + Beat::from(self.wave_length as f32 * current_bpm / 44100.0 / 60.0);

        // 付け加えるnotesをリストアップする。
        // self.played_notesに加える。
        if track.phrase.length > Beat::from(0) {
            let rep_current_beats = *cum_current_beats % track.phrase.length;
            let rep_next_beats = cum_next_beats % track.phrase.length;

            if rep_current_beats < rep_next_beats {
                for (&start, new_notes) in track
                    .phrase
                    .notes
                    .range((Included(rep_current_beats), Excluded(rep_next_beats)))
                {
                    let cum_start_samples = ((start - rep_current_beats).to_f32() * 44100.0 * 60.0
                        / current_bpm) as u64
                        + cum_current_samples;
                    self.register_notes(new_notes, &current_bpm, &cum_start_samples);
                }
            } else {
                for (&start, new_notes) in track
                    .phrase
                    .notes
                    .range((Included(rep_current_beats), Excluded(track.phrase.length)))
                {
                    let cum_start_samples = ((start - rep_current_beats).to_f32() * 44100.0 * 60.0
                        / current_bpm) as u64
                        + cum_current_samples;
                    self.register_notes(new_notes, &current_bpm, &cum_start_samples);
                }
                for (&start, new_notes) in track
                    .phrase
                    .notes
                    .range((Included(Beat::from(0)), Excluded(rep_next_beats)))
                {
                    let cum_start_samples = ((track.phrase.length + start - rep_current_beats)
                        .to_f32()
                        * 44100.0
                        * 60.0
                        / current_bpm) as u64
                        + cum_current_samples;

                    self.register_notes(new_notes, &current_bpm, &cum_start_samples);
                }
            }
        }

        // self.played_notesのを鳴らす
        for (&cum_end_samples, notes) in self.played_notes.iter() {
            for (cum_start_samples, note) in notes.iter() {
                if let Instrument::Sample(sample_name) = &track.instrument {
                    let wave = resource_manager
                        .get_sample_wave(sample_name.to_string(), note.sound.clone());
                    match wave {
                        Ok(wave) => {
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
                            let end_idx_for_sample =
                                (cum_current_samples + end_idx as u64 - cum_start_samples) as usize;

                            let sample_data =
                                wave.get_samples(start_idx_for_sample, end_idx_for_sample);
                            match sample_data {
                                Ok((left_sample, right_sample)) => {
                                    for (i, j) in (start_idx..end_idx).enumerate() {
                                        let left_addition = left_sample[i] * 0.5 * track.vol;
                                        let right_addition = right_sample[i] * 0.5 * track.vol;
                                        if track.pan > 0.0 {
                                            left_wave[j] =
                                                left_wave[j] + (1.0 - track.pan) * left_addition;
                                            right_wave[j] = right_wave[j] + right_addition;
                                            right_wave[j] =
                                                right_wave[j] + track.pan * left_addition;
                                        } else {
                                            left_wave[j] = left_wave[j] + left_addition;
                                            left_wave[j] =
                                                left_wave[j] + (-track.pan) * right_addition;
                                            right_wave[j] = right_wave[j]
                                                + (1.0 - (-track.pan)) * right_addition;
                                        }
                                    }
                                }
                                Err(e) => {
                                    // TODO:
                                    error!("error {}", e);
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
        }

        // Effect更新
        if self.effect_infos != track.effects {
            self.effect_infos = track.effects.clone();
            let mut effects = vec![];
            for efi in self.effect_infos.iter() {
                effects.push(efi.get_effect(Arc::clone(&resource_manager)));
            }
            self.effects = effects;
        }

        // Effect
        for effect in self.effects.iter_mut() {
            let (l, r) = effect.effect(&left_wave, &right_wave);
            left_wave = l;
            right_wave = r;
        }

        // 使ったself.played_notesのノートを消す
        for cum_note_samples in *cum_current_samples..cum_next_samples {
            self.played_notes.remove(&cum_note_samples);
        }

        (left_wave, right_wave)
    }

    fn register_notes(
        &mut self,
        notes: &BTreeSet<SampleNote>,
        current_bpm: &f32,
        cum_start_samples: &u64,
    ) {
        for note in notes.iter() {
            let note = note.clone();
            let cum_end_samples = cum_start_samples + (1.0 * 44100.0 * 60.0 / current_bpm) as u64; // TODO: accurate wave length

            if self.played_notes.contains_key(&cum_end_samples) {
                match self.played_notes.get_mut(&cum_end_samples) {
                    Some(notes_in_cum_end_samples) => {
                        notes_in_cum_end_samples.push((*cum_start_samples, note));
                    }
                    None => {
                        error!("get_mut failed");
                    }
                };
            } else {
                self.played_notes
                    .insert(cum_end_samples, vec![(*cum_start_samples, note)]);
            }
        }
    }
}
