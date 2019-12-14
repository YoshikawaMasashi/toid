extern crate portaudio;

use std::boxed::Box;
use std::sync::Arc;
use std::sync::RwLock;

use toid::portaudio_outputter::PortAudioOutputter;
use toid::sound_output::SoundState;
use toid::sound_output::SoundStateEvent;
use toid::sound_output::SoundStateManager;
use toid::sound_output::SoundStateReduce;
use toid::state::Reducer;
use toid::state::Store;

fn main() {
    let initial_state: SoundState = SoundState::new(512);
    let store = Arc::new(RwLock::new(Store::new(initial_state)));

    let reduce = Box::new(SoundStateReduce {});
    let reducer = Reducer::new(Arc::clone(&store), reduce);

    let sound_state_manager = SoundStateManager::new(Arc::clone(&store), reducer);
    let sound_state_manager = Arc::new(RwLock::new(sound_state_manager));

    let mut portaudio_outputter = PortAudioOutputter::new(Arc::clone(&sound_state_manager));
    let reduce = Box::new(SoundStateReduce {});
    let reducer = Reducer::new(Arc::clone(&store), reduce);

    portaudio_outputter.run();

    reducer.reduce(&SoundStateEvent::SoundOff);
    portaudio_outputter.sleep(500);
    reducer.reduce(&SoundStateEvent::SoundOn);
    reducer.reduce(&SoundStateEvent::ChangePitch(60));
    portaudio_outputter.sleep(500);
    reducer.reduce(&SoundStateEvent::ChangePitch(62));
    portaudio_outputter.sleep(500);
    reducer.reduce(&SoundStateEvent::ChangePitch(64));
    portaudio_outputter.sleep(500);
    reducer.reduce(&SoundStateEvent::ChangePitch(65));
    portaudio_outputter.sleep(500);
    reducer.reduce(&SoundStateEvent::ChangePitch(67));
    portaudio_outputter.sleep(1000);
    reducer.reduce(&SoundStateEvent::ChangePitch(69));
    portaudio_outputter.sleep(500);
    reducer.reduce(&SoundStateEvent::ChangePitch(65));
    portaudio_outputter.sleep(500);
    reducer.reduce(&SoundStateEvent::ChangePitch(64));
    portaudio_outputter.sleep(500);
    reducer.reduce(&SoundStateEvent::SoundOff);
    portaudio_outputter.sleep(500);
    reducer.reduce(&SoundStateEvent::SoundOn);
    reducer.reduce(&SoundStateEvent::ChangePitch(62));
    portaudio_outputter.sleep(500);
    reducer.reduce(&SoundStateEvent::SoundOff);
    portaudio_outputter.sleep(500);
    reducer.reduce(&SoundStateEvent::SoundOn);
    reducer.reduce(&SoundStateEvent::ChangePitch(60));
    portaudio_outputter.sleep(1500);

    portaudio_outputter.stop();
}
