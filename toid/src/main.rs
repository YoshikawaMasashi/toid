extern crate portaudio;

use std::sync::Arc;
use std::sync::RwLock;

use toid::music_store::melody_state::MelodyStateEvent;
use toid::music_store::melody_state::NoteInfo;
use toid::music_store::new_music_store::NewMusicStore;
use toid::music_store::sf2_state::SF2StateEvent;
use toid::music_store::wave_reader::WaveReader;
use toid::outputters::portaudio_outputter::PortAudioOutputter;

fn main() {
    let store = NewMusicStore::new();
    let store = Arc::new(store);

    let wave_reader = WaveReader::new(Arc::clone(&store));
    let wave_reader = Arc::new(RwLock::new(wave_reader));

    let mut portaudio_outputter = PortAudioOutputter::new(Arc::clone(&wave_reader));

    store.new_melody(String::from("main"));
    store.new_melody(String::from("sub"));

    store
        .sf2
        .update_state(SF2StateEvent::LoadAndSetSF2(String::from(
            "../florestan-subset.sf2",
        )));

    {
        let melody_hash_map = store.melody.read().unwrap();
        let main_melody_store = melody_hash_map.get(&String::from("main")).unwrap();
        let sub_melody_store = melody_hash_map.get(&String::from("sub")).unwrap();

        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 48.0,
            duration: 1 * (44100 / 4),
            start: 0 * (44100 / 4),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 50.0,
            duration: 1 * (44100 / 4),
            start: 1 * (44100 / 4),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 52.0,
            duration: 1 * (44100 / 4),
            start: 2 * (44100 / 4),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 53.0,
            duration: 1 * (44100 / 4),
            start: 3 * (44100 / 4),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 55.0,
            duration: 2 * (44100 / 4),
            start: 4 * (44100 / 4),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 57.0,
            duration: 1 * (44100 / 4),
            start: 6 * (44100 / 4),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 53.0,
            duration: 1 * (44100 / 4),
            start: 7 * (44100 / 4),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 52.0,
            duration: 1 * (44100 / 4),
            start: 8 * (44100 / 4),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 50.0,
            duration: 1 * (44100 / 4),
            start: 10 * (44100 / 4),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 48.0,
            duration: 4 * (44100 / 4) - 1,
            start: 12 * (44100 / 4),
        }));

        /*
        sub_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 24.0,
            duration: 4 * (44100 / 4),
            start: 0 * (44100 / 4),
        }));
        sub_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 29.0,
            duration: 4 * (44100 / 4),
            start: 4 * (44100 / 4),
        }));
        sub_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 31.0,
            duration: 4 * (44100 / 4),
            start: 8 * (44100 / 4),
        }));
        sub_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 24.0,
            duration: 4 * (44100 / 4),
            start: 12 * (44100 / 4),
        }));
        */
    }

    portaudio_outputter.run();
    portaudio_outputter.sleep(8000);
    portaudio_outputter.stop();
}
