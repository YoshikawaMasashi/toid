extern crate portaudio;

use std::sync::Arc;
use std::sync::RwLock;

use toid::music_store::beat::Beat;
use toid::music_store::melody_state::MelodyStateEvent;
use toid::music_store::melody_state::NoteInfo;
use toid::music_store::music_store::MusicStore;
use toid::music_store::scheduling_state::SchedulingStateEvent;
use toid::music_store::sf2_state::SF2StateEvent;
use toid::music_store::wave_reader::WaveReader;
use toid::outputters::portaudio_outputter::PortAudioOutputter;

fn main() {
    let store = MusicStore::new();
    let store = Arc::new(store);

    let wave_reader = WaveReader::new(Arc::clone(&store));
    let wave_reader = Arc::new(RwLock::new(wave_reader));

    let mut portaudio_outputter = PortAudioOutputter::new(Arc::clone(&wave_reader));

    store.new_melody(String::from("main"));
    store.new_melody(String::from("sub"));

    store
        .sf2
        .update_state(SF2StateEvent::LoadAndSetSF2(String::from(
            "../resource/sf2/florestan-subset.sf2",
        )));

    {
        let melody_hash_map = store.melody.read().unwrap();
        let main_melody_store = melody_hash_map.get(&String::from("main")).unwrap();
        let sub_melody_store = melody_hash_map.get(&String::from("sub")).unwrap();
        store
            .scheduling
            .update_state(SchedulingStateEvent::ChangeBPM(Beat::from(0), 120.0));
        store
            .scheduling
            .update_state(SchedulingStateEvent::ChangeBPM(Beat::from(8), 180.0));
        store
            .scheduling
            .update_state(SchedulingStateEvent::ChangeBPM(Beat::from(16), 120.0));

        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 48.0,
            duration: Beat::from(0.5),
            start: Beat::from(0.0),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 50.0,
            duration: Beat::from(0.5),
            start: Beat::from(0.5),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 52.0,
            duration: Beat::from(0.5),
            start: Beat::from(1.0),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 53.0,
            duration: Beat::from(0.5),
            start: Beat::from(1.5),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 55.0,
            duration: Beat::from(1.0),
            start: Beat::from(2.0),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 57.0,
            duration: Beat::from(0.5),
            start: Beat::from(3.0),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 53.0,
            duration: Beat::from(0.5),
            start: Beat::from(3.5),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 52.0,
            duration: Beat::from(0.5),
            start: Beat::from(4.0),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 50.0,
            duration: Beat::from(0.5),
            start: Beat::from(5.0),
        }));
        main_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 48.0,
            duration: Beat::from(2.0),
            start: Beat::from(6.0),
        }));

        sub_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 36.0,
            duration: Beat::from(2.0),
            start: Beat::from(0.0),
        }));
        sub_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 41.0,
            duration: Beat::from(2.0),
            start: Beat::from(2.0),
        }));
        sub_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 43.0,
            duration: Beat::from(2.0),
            start: Beat::from(4.0),
        }));
        sub_melody_store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch: 36.0,
            duration: Beat::from(2.0),
            start: Beat::from(6.0),
        }));
    }

    portaudio_outputter.run();
    portaudio_outputter.sleep(12000);
    portaudio_outputter.stop();
}
