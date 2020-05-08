use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::Bound::{Included, Unbounded};
use std::sync::Arc;

use log::error;
use serde::{Deserialize, Serialize};

use super::super::data::music_info::Beat;
use super::super::resource_management::resource_manager::ResourceManager;
use super::super::state_management::serialize;
use super::super::state_management::store::Store;
use super::super::state_management::store_reader::StoreReader;
use super::music_state::{MusicState, MusicStateEvent};
use super::track_player::TrackPlayer;

pub struct WaveReader {
    wave_length: u64,
    cum_current_samples: u64,
    current_bpm: f32,
    bpm_change_samples: u64,
    bpm_change_beats: Beat,
    cum_current_beats: Beat,
    track_players: HashMap<String, TrackPlayer>,
}

impl StoreReader<(Vec<i16>, Vec<i16>), WaveReaderEvent, MusicState, MusicStateEvent>
    for WaveReader
{
    fn new() -> Self {
        WaveReader {
            wave_length: 512,
            cum_current_samples: 0,
            current_bpm: 120.0,
            bpm_change_samples: 0,
            bpm_change_beats: Beat::from(0),
            cum_current_beats: Beat::from(0),
            track_players: HashMap::new(),
        }
    }

    fn read(
        &mut self,
        store: Arc<Store<MusicState, MusicStateEvent>>,
        resource_manager: Arc<ResourceManager>,
    ) -> (Vec<i16>, Vec<i16>) {
        let mut left_wave: Vec<i16> = Vec::new();
        left_wave.resize(self.wave_length as usize, 0);
        let mut right_wave: Vec<i16> = Vec::new();
        right_wave.resize(self.wave_length as usize, 0);

        let music_state = match store.get_state() {
            Ok(music_state) => music_state,
            Err(e) => {
                error!("get_state Error {}", e);
                return (left_wave, right_wave);
            }
        };
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

        let state_track_keys: HashSet<String> =
            HashSet::from_iter(music_state.track_map.keys().cloned());
        let track_player_keys: HashSet<String> =
            HashSet::from_iter(self.track_players.keys().cloned());

        for key in track_player_keys.difference(&state_track_keys) {
            self.track_players.remove(key);
        }
        for key in state_track_keys.difference(&track_player_keys) {
            self.track_players.insert(key.clone(), TrackPlayer::new());
        }
        for (key, track) in music_state.track_map.iter() {
            let (left_wave_of_track, right_wave_of_track) =
                self.track_players.get_mut(key).unwrap().play(
                    &track,
                    Arc::clone(&resource_manager),
                    &self.cum_current_samples,
                    &self.cum_current_beats,
                    &self.current_bpm,
                );
            for i in 0..self.wave_length as usize {
                left_wave[i] = left_wave[i].saturating_add(left_wave_of_track[i]);
                right_wave[i] = right_wave[i].saturating_add(right_wave_of_track[i]);
            }
        }

        self.cum_current_samples = cum_next_samples;
        self.cum_current_beats = cum_next_beats;

        (left_wave, right_wave)
    }

    fn apply(&mut self, event: WaveReaderEvent) {
        match event {
            WaveReaderEvent::MoveStart => {
                self.cum_current_samples = 0;
                self.current_bpm = 120.0;
                self.bpm_change_samples = 0;
                self.bpm_change_beats = Beat::from(0);
                self.cum_current_beats = Beat::from(0);
                self.track_players = HashMap::new();
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
