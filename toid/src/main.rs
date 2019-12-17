extern crate portaudio;

use std::boxed::Box;
use std::sync::Arc;
use std::sync::RwLock;

use toid::music_state_manager::{MusicStateEvent, MusicStateManager, MusicStateReduce};
use toid::portaudio_outputter::PortAudioOutputter;
use toid::state_management::reducer::Reducer;
use toid::state_management::store::Store;
use toid::states::music_state::MusicState;

fn main() {
    let store = Arc::new(RwLock::new(Store::new(MusicState::new())));

    let sound_state_manager = MusicStateManager::new(Arc::clone(&store));
    let sound_state_manager = Arc::new(RwLock::new(sound_state_manager));

    let mut portaudio_outputter = PortAudioOutputter::new(Arc::clone(&sound_state_manager));
    let reducer = Reducer::new(Arc::clone(&store), Box::new(MusicStateReduce {}));

    portaudio_outputter.run();

    reducer.reduce(MusicStateEvent::AddNewNoteOn(60.0, 0 * (44100 / 4)));
    reducer.reduce(MusicStateEvent::AddNewNoteOn(62.0, 1 * (44100 / 4)));
    reducer.reduce(MusicStateEvent::AddNewNoteOn(64.0, 2 * (44100 / 4)));
    reducer.reduce(MusicStateEvent::AddNewNoteOn(65.0, 3 * (44100 / 4)));
    reducer.reduce(MusicStateEvent::AddNewNoteOn(67.0, 4 * (44100 / 4)));
    reducer.reduce(MusicStateEvent::AddNewNoteOn(69.0, 6 * (44100 / 4)));
    reducer.reduce(MusicStateEvent::AddNewNoteOn(65.0, 7 * (44100 / 4)));
    reducer.reduce(MusicStateEvent::AddNewNoteOn(64.0, 8 * (44100 / 4)));
    reducer.reduce(MusicStateEvent::AddNewNoteOff(9 * (44100 / 4)));
    reducer.reduce(MusicStateEvent::AddNewNoteOn(62.0, 10 * (44100 / 4)));
    reducer.reduce(MusicStateEvent::AddNewNoteOff(11 * (44100 / 4)));
    reducer.reduce(MusicStateEvent::AddNewNoteOn(60.0, 12 * (44100 / 4)));

    portaudio_outputter.sleep(4000);
    portaudio_outputter.stop();
}
