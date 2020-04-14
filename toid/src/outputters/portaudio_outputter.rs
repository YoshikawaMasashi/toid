extern crate portaudio;

use super::super::music_state::music_state::{MusicState, MusicStateEvent};
use super::super::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use super::super::players::player::Player;
use super::super::state_management::store_reader::StoreReader;
use portaudio as pa;
use std::option::Option;
use std::sync::Arc;

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 512;

pub struct PortAudioOutputter {
    player: Arc<dyn Player<MusicState, MusicStateEvent, WaveReader, Vec<i16>, WaveReaderEvent>>,
    portaudio: pa::PortAudio,
    stream: Option<pa::Stream<pa::NonBlocking, pa::Output<i16>>>,
}

impl PortAudioOutputter {
    pub fn new(
        player: Arc<dyn Player<MusicState, MusicStateEvent, WaveReader, Vec<i16>, WaveReaderEvent>>,
    ) -> Self {
        let portaudio = pa::PortAudio::new().unwrap();

        PortAudioOutputter {
            player,
            portaudio,
            stream: None,
        }
    }

    pub fn run(&mut self) {
        let wave_reader = Arc::clone(&self.player.get_reader());
        let store = Arc::clone(&self.player.get_store());
        let resource_manager = Arc::clone(&self.player.get_resource_manager());
        let callback = move |pa::OutputStreamCallbackArgs::<'static, i16> {
                                 buffer,
                                 frames,
                                 ..
                             }|
              -> pa::StreamCallbackResult {
            let waves = wave_reader
                .write()
                .unwrap()
                .read(Arc::clone(&store), Arc::clone(&resource_manager));

            let mut idx = 0;
            for i in 0..frames {
                buffer[idx] = waves[i];
                buffer[idx + 1] = waves[i];
                idx += 2;
            }
            pa::Continue
        };

        let mut settings = self
            .portaudio
            .default_output_stream_settings::<i16>(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)
            .unwrap();
        settings.flags = pa::stream_flags::CLIP_OFF;

        let mut stream = self
            .portaudio
            .open_non_blocking_stream(settings, callback)
            .unwrap();

        stream.start().unwrap();
        self.stream = Some(stream);
    }

    pub fn stop(&mut self) {
        Option::as_mut(&mut self.stream).unwrap().stop().unwrap();
        Option::as_mut(&mut self.stream).unwrap().close().unwrap();
        self.stream = None;
    }

    pub fn sleep(&mut self, millseconds: i32) {
        self.portaudio.sleep(millseconds);
    }
}
