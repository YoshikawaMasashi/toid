extern crate portaudio;

use super::music_state_manager::MusicStateManager;
use portaudio as pa;
use std::option::Option;
use std::sync::Arc;
use std::sync::RwLock;

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 512;

pub struct PortAudioOutputter {
    sound_state_manager: Arc<RwLock<MusicStateManager>>,
    portaudio: pa::PortAudio,
    stream: Option<Arc<RwLock<pa::Stream<pa::NonBlocking, pa::Output<f32>>>>>,
}

impl PortAudioOutputter {
    pub fn new(sound_state_manager: Arc<RwLock<MusicStateManager>>) -> Self {
        let portaudio = pa::PortAudio::new().unwrap();

        PortAudioOutputter {
            sound_state_manager,
            portaudio,
            stream: None,
        }
    }

    pub fn run(&mut self) {
        let sound_state_manager = Arc::clone(&self.sound_state_manager);
        let callback = move |pa::OutputStreamCallbackArgs::<'static, f32> {
                                 buffer,
                                 frames,
                                 ..
                             }|
              -> pa::StreamCallbackResult {
            let waves = sound_state_manager.write().unwrap().get_wave();

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

        stream.start().unwrap();
        self.stream = Some(Arc::new(RwLock::new(stream)));
    }

    pub fn stop(&mut self) {
        Option::as_ref(&self.stream)
            .unwrap()
            .write()
            .unwrap()
            .stop()
            .unwrap();
        Option::as_ref(&self.stream)
            .unwrap()
            .write()
            .unwrap()
            .close()
            .unwrap();
        self.stream = None;
    }

    pub fn sleep(&mut self, millseconds: i32) {
        self.portaudio.sleep(millseconds);
    }
}
