extern crate portaudio;

use portaudio as pa;
use std::sync::Arc;
use std::sync::RwLock;

use toid::sound_output::SoundState;
use toid::sound_output::SoundStateEvent;
use toid::sound_output::SoundStateManager;
use toid::sound_output::SoundStateReduce;
use toid::state::Reducer;
use toid::state::Store;

const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 512;
const TABLE_SIZE: usize = 200;
const INTERLEAVED: bool = true;

fn main() {
    let initial_state: SoundState = SoundState::new(512);
    let store = Arc::new(RwLock::new(Store::new(initial_state)));

    let reduce = Box::new(SoundStateReduce {});
    let reducer = Reducer::new(Arc::clone(&store), reduce);
    let sound_state_manager = SoundStateManager::new(Arc::clone(&store), reducer);

    let reduce = Box::new(SoundStateReduce {});
    let reducer = Reducer::new(Arc::clone(&store), reduce);

    let portaudio = pa::PortAudio::new().unwrap();
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let waves = sound_state_manager.get_wave();

        let mut idx = 0;
        for i in 0..frames {
            buffer[idx] = waves[i];
            buffer[idx + 1] = waves[i];
            idx += 2;
        }
        pa::Continue
    };
    let mut settings = portaudio
        .default_output_stream_settings::<f32>(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)
        .unwrap();
    settings.flags = pa::stream_flags::CLIP_OFF;
    let mut stream = portaudio
        .open_non_blocking_stream(settings, callback)
        .unwrap();

    stream.start().unwrap();

    reducer.reduce(&SoundStateEvent::SoundOff);
    portaudio.sleep(500);
    reducer.reduce(&SoundStateEvent::SoundOn);
    reducer.reduce(&SoundStateEvent::ChangePitch(60));
    portaudio.sleep(500);
    reducer.reduce(&SoundStateEvent::ChangePitch(62));
    portaudio.sleep(500);
    reducer.reduce(&SoundStateEvent::ChangePitch(64));
    portaudio.sleep(500);
    reducer.reduce(&SoundStateEvent::ChangePitch(65));
    portaudio.sleep(500);
    reducer.reduce(&SoundStateEvent::ChangePitch(67));
    portaudio.sleep(1000);
    reducer.reduce(&SoundStateEvent::ChangePitch(69));
    portaudio.sleep(500);
    reducer.reduce(&SoundStateEvent::ChangePitch(65));
    portaudio.sleep(500);
    reducer.reduce(&SoundStateEvent::ChangePitch(64));
    portaudio.sleep(500);
    reducer.reduce(&SoundStateEvent::SoundOff);
    portaudio.sleep(500);
    reducer.reduce(&SoundStateEvent::SoundOn);
    reducer.reduce(&SoundStateEvent::ChangePitch(62));
    portaudio.sleep(500);
    reducer.reduce(&SoundStateEvent::SoundOff);
    portaudio.sleep(500);
    reducer.reduce(&SoundStateEvent::SoundOn);
    reducer.reduce(&SoundStateEvent::ChangePitch(60));
    portaudio.sleep(1500);

    stream.stop().unwrap();
    stream.close().unwrap();
}
