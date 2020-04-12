extern crate portaudio;

use std::sync::Arc;
use std::sync::RwLock;

use toid::music_state::beat::Beat;
use toid::music_state::melody_state::MelodyStateEvent;
use toid::music_state::melody_state::NoteInfo;
use toid::music_state::music_state::{MusicState, MusicStateEvent};
use toid::music_state::scheduling_state::SchedulingStateEvent;
use toid::music_state::sf2_state::SF2StateEvent;
use toid::music_state::wave_reader::WaveReader;
use toid::outputters::portaudio_outputter::PortAudioOutputter;
use toid::players::local_player::LocalPlayer;
use toid::players::player::Player;
use toid::resource_management::resource_manager::ResourceManager;
use toid::state_management::serialize::Serialize;
use toid::state_management::store::Store;

fn main() {
    let store = Store::new(MusicState::new());
    let store = Arc::new(store);

    let resource_manager = ResourceManager::new();
    resource_manager.register(String::from("../resource/sf2/sf2.toml"));
    resource_manager.load_sf2(String::from("sf2.test"));
    let resource_manager = Arc::new(resource_manager);

    let wave_reader = WaveReader::new(Arc::clone(&store), Arc::clone(&resource_manager));
    let wave_reader = Arc::new(RwLock::new(wave_reader));

    let player = LocalPlayer::new(Arc::clone(&store), Arc::clone(&resource_manager));
    let player = Arc::new(player);

    let mut portaudio_outputter = PortAudioOutputter::new(Arc::clone(&wave_reader));

    player.send_event(MusicStateEvent::NewMelody(String::from("main")));
    player.send_event(MusicStateEvent::NewMelody(String::from("sub")));

    println!("{}", player.get_store().get_state().serialize().unwrap());
    let desirialized =
        MusicState::deserialize(player.get_store().get_state().serialize().unwrap()).unwrap();
    /*
    player.send_event(MusicStateEvent::SF2StateEvent(SF2StateEvent::SetSF2Name(
        String::from("sf2.test"),
    )));

    player.send_event(MusicStateEvent::SchedulingStateEvent(
        SchedulingStateEvent::ChangeBPM(Beat::from(0), 120.0),
    ));
    player.send_event(MusicStateEvent::SchedulingStateEvent(
        SchedulingStateEvent::ChangeBPM(Beat::from(8), 180.0),
    ));
    player.send_event(MusicStateEvent::SchedulingStateEvent(
        SchedulingStateEvent::ChangeBPM(Beat::from(16), 120.0),
    ));

    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("main"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 48.0,
            duration: Beat::from(0.5),
            start: Beat::from(0.0),
        }),
    ));
    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("main"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 50.0,
            duration: Beat::from(0.5),
            start: Beat::from(0.5),
        }),
    ));
    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("main"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 52.0,
            duration: Beat::from(0.5),
            start: Beat::from(1.0),
        }),
    ));
    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("main"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 53.0,
            duration: Beat::from(0.5),
            start: Beat::from(1.5),
        }),
    ));
    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("main"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 55.0,
            duration: Beat::from(1.0),
            start: Beat::from(2.0),
        }),
    ));
    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("main"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 57.0,
            duration: Beat::from(0.5),
            start: Beat::from(3.0),
        }),
    ));
    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("main"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 53.0,
            duration: Beat::from(0.5),
            start: Beat::from(3.5),
        }),
    ));
    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("main"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 52.0,
            duration: Beat::from(0.5),
            start: Beat::from(4.0),
        }),
    ));
    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("main"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 50.0,
            duration: Beat::from(0.5),
            start: Beat::from(5.0),
        }),
    ));
    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("main"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 48.0,
            duration: Beat::from(1.5),
            start: Beat::from(6.0),
        }),
    ));

    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("sub"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 36.0,
            duration: Beat::from(2.0),
            start: Beat::from(0.0),
        }),
    ));
    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("sub"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 41.0,
            duration: Beat::from(2.0),
            start: Beat::from(2.0),
        }),
    ));
    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("sub"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 43.0,
            duration: Beat::from(2.0),
            start: Beat::from(4.0),
        }),
    ));
    player.send_event(MusicStateEvent::MelodyStateEvent(
        String::from("sub"),
        MelodyStateEvent::AddNote(NoteInfo {
            pitch: 36.0,
            duration: Beat::from(2.0),
            start: Beat::from(6.0),
        }),
    ));

    portaudio_outputter.run();
    portaudio_outputter.sleep(12000);
    portaudio_outputter.stop();
    */
}
