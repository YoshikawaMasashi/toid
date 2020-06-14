use std::sync::Arc;

use super::super::data::wave::{Data, Wave};
use super::super::music_state::states::{MusicState, MusicStateEvent};
use super::super::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use super::super::players::player::Player;
use super::super::state_management::store_reader::StoreReader;

const SAMPLE_RATE: f32 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 512;

pub struct WaveFileOutputter {
    player: Arc<
        dyn Player<MusicState, MusicStateEvent, WaveReader, (Vec<i16>, Vec<i16>), WaveReaderEvent>,
    >,
}

impl WaveFileOutputter {
    pub fn new(
        player: Arc<
            dyn Player<
                MusicState,
                MusicStateEvent,
                WaveReader,
                (Vec<i16>, Vec<i16>),
                WaveReaderEvent,
            >,
        >,
    ) -> Result<Self, String> {
        Ok(WaveFileOutputter { player })
    }

    pub fn save(&mut self, path: String, sec: f32) {
        let mut all_left_wave: Vec<f32> = vec![];
        let mut all_right_wave: Vec<f32> = vec![];

        let wave_reader = self.player.get_reader();
        let store = Arc::clone(&self.player.get_store());
        let resource_manager = Arc::clone(&self.player.get_resource_manager());

        for _ in 0..((sec * SAMPLE_RATE) / FRAMES_PER_BUFFER as f32) as usize {
            let (left_waves, right_waves) = match wave_reader.write() {
                Ok(mut wave_reader) => {
                    wave_reader.read(Arc::clone(&store), Arc::clone(&resource_manager))
                }
                Err(_) => {
                    panic!("save rrror");
                }
            };

            for &v in left_waves.iter() {
                all_left_wave.push(v as f32 / std::i16::MAX as f32);
            }
            for &v in right_waves.iter() {
                all_right_wave.push(v as f32 / std::i16::MAX as f32);
            }
        }

        let sample_num = all_left_wave.len();
        let wave = Wave {
            data: Data::Stereo((all_left_wave, all_right_wave)),
            sample_num,
            sample_rate: SAMPLE_RATE,
        };

        wave.save(path);
    }
}
