extern crate portaudio;

use std::sync::Arc;
use std::sync::RwLock;

use toid::high_layer_trial::num_lang::send_num_lang;
use toid::music_state::beat::Beat;
use toid::music_state::music_state::{MusicState, MusicStateEvent};
use toid::music_state::scheduling_state::SchedulingStateEvent;
use toid::music_state::sf2_state::SF2StateEvent;
use toid::music_state::wave_reader::WaveReader;
use toid::outputters::portaudio_outputter::PortAudioOutputter;
use toid::players::local_player::LocalPlayer;
use toid::players::player::Player;
use toid::resource_management::resource_manager::ResourceManager;
use toid::state_management::store::Store;

fn main() {
    let store = Store::new(MusicState::new());
    let store = Arc::new(store);

    let resource_manager = ResourceManager::new();
    let resource_manager = Arc::new(resource_manager);
    resource_manager.register(String::from("../resource/sf2/sf2.toml"));
    resource_manager.load_sf2(String::from("sf2.test"));

    let wave_reader = WaveReader::new();
    let wave_reader = Arc::new(RwLock::new(wave_reader));

    let player = LocalPlayer::new(Arc::clone(&store), Arc::clone(&resource_manager));
    let player = Arc::new(player);

    let mut portaudio_outputter = PortAudioOutputter::new(
        Arc::clone(&wave_reader),
        Arc::clone(&player) as Arc<dyn Player<MusicState, MusicStateEvent>>,
    );

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

    send_num_lang(
        "12345 643 2 1".to_string(),
        0.0,
        "main".to_string(),
        Arc::clone(&player) as Arc<dyn Player<MusicState, MusicStateEvent>>,
    );

    send_num_lang(
        "1   4   5   1".to_string(),
        -1.0,
        "sub".to_string(),
        Arc::clone(&player) as Arc<dyn Player<MusicState, MusicStateEvent>>,
    );

    portaudio_outputter.run();
    portaudio_outputter.sleep(12000);
    portaudio_outputter.stop();
}
