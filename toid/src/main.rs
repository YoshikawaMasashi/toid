extern crate portaudio;

use std::sync::Arc;

use toid::music_store::melody_state::MelodyStateEvent;
use toid::music_store::new_music_store::NewMusicStore;
use toid::music_store::sf2_state::SF2StateEvent;
use toid::music_store::wave_reader::WaveReader;
use toid::outputters::portaudio_outputter::PortAudioOutputter;

fn main() {
    let store = NewMusicStore::new();
    let store = Arc::new(store);

    let wave_reader = WaveReader::new(Arc::clone(&store));
    let wave_reader = Arc::new(wave_reader);

    let mut portaudio_outputter = PortAudioOutputter::new(Arc::clone(&wave_reader));

    store
        .sf2
        .update_state(SF2StateEvent::LoadAndSetSF2(String::from(
            "../florestan-subset.sf2",
        )));

    store
        .melody
        .update_state(MelodyStateEvent::AddNewNoteOn(48.0, 0 * (44100 / 4)));
    store
        .melody
        .update_state(MelodyStateEvent::AddNewNoteOn(50.0, 1 * (44100 / 4)));
    store
        .melody
        .update_state(MelodyStateEvent::AddNewNoteOn(52.0, 2 * (44100 / 4)));
    store
        .melody
        .update_state(MelodyStateEvent::AddNewNoteOn(53.0, 3 * (44100 / 4)));
    store
        .melody
        .update_state(MelodyStateEvent::AddNewNoteOn(55.0, 4 * (44100 / 4)));
    store
        .melody
        .update_state(MelodyStateEvent::AddNewNoteOn(57.0, 6 * (44100 / 4)));
    store
        .melody
        .update_state(MelodyStateEvent::AddNewNoteOn(53.0, 7 * (44100 / 4)));
    store
        .melody
        .update_state(MelodyStateEvent::AddNewNoteOn(52.0, 8 * (44100 / 4)));
    store
        .melody
        .update_state(MelodyStateEvent::AddNewNoteOff(9 * (44100 / 4)));
    store
        .melody
        .update_state(MelodyStateEvent::AddNewNoteOn(50.0, 10 * (44100 / 4)));
    store
        .melody
        .update_state(MelodyStateEvent::AddNewNoteOff(11 * (44100 / 4)));
    store
        .melody
        .update_state(MelodyStateEvent::AddNewNoteOn(48.0, 12 * (44100 / 4)));

    portaudio_outputter.run();
    portaudio_outputter.sleep(4000);
    portaudio_outputter.stop();
}
