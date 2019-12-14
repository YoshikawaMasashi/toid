extern crate portaudio;

use super::sound_output::SoundStateManager;
use portaudio as pa;
use std::sync::Arc;
use std::sync::RwLock;

const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 512;
const TABLE_SIZE: usize = 200;
const INTERLEAVED: bool = true;

/*
struct PortAudioOutputter {
    sound_state_manager: Arc<RwLock<SoundStateManager>>,
    portaudio: pa::PortAudio,
}

impl PortAudioOutputter {
    fn new(sound_state_manager: Arc<RwLock<SoundStateManager>>) -> Self {
        let portaudio = pa::PortAudio::new().unwrap();

        PortAudioOutputter {
            sound_state_manager,
            portaudio,
        }
    }

    fn run(&self) {
        // FnMut(<S::Flow as Flow>::CallbackArgs) -> ffi::PaStreamCallbackResult + 'static
        let callback: FnMut(pa::OutputStreamCallbackArgs<f32>) -> pa::StreamCallbackResult
            + 'static = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
            let waves = self.sound_state_manager.write().unwrap().get_wave();

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
            .default_output_stream_settings::<f32>(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)
            .unwrap();
        settings.flags = pa::stream_flags::CLIP_OFF;

        let mut stream = self
            .portaudio
            .open_non_blocking_stream(settings, callback)
            .unwrap();

        /*
        stream.start().unwrap();
            */
    }
}
*/
