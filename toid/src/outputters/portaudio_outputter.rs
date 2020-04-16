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
    ) -> Result<Self, String> {
        let portaudio = pa::PortAudio::new().map_err(|e| e.to_string())?;

        Ok(PortAudioOutputter {
            player,
            portaudio,
            stream: None,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let wave_reader = Arc::clone(&self.player.get_reader());
        let store = Arc::clone(&self.player.get_store());
        let resource_manager = Arc::clone(&self.player.get_resource_manager());
        let callback = move |pa::OutputStreamCallbackArgs::<'static, i16> {
                                 buffer,
                                 frames,
                                 ..
                             }|
              -> pa::StreamCallbackResult {
            let waves = match wave_reader.write() {
                Ok(mut wave_reader) => {
                    wave_reader.read(Arc::clone(&store), Arc::clone(&resource_manager))
                }
                Err(_) => {
                    // TODO: rethinking
                    return pa::StreamCallbackResult::Abort;
                }
            };

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
            .map_err(|e| e.to_string())?;
        settings.flags = pa::stream_flags::CLIP_OFF;

        let mut stream = self
            .portaudio
            .open_non_blocking_stream(settings, callback)
            .map_err(|e| e.to_string())?;

        stream.start().map_err(|e| e.to_string())?;
        self.stream = Some(stream);

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        Option::as_mut(&mut self.stream)
            .ok_or("as_mut failed")?
            .stop()
            .map_err(|e| e.to_string())?;
        Option::as_mut(&mut self.stream)
            .ok_or("as_mut failed")?
            .close()
            .map_err(|e| e.to_string())?;
        self.stream = None;
        Ok(())
    }

    pub fn sleep(&mut self, millseconds: i32) {
        self.portaudio.sleep(millseconds);
    }
}
