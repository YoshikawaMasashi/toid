extern crate portaudio;

use std::sync::Arc;

use toid::high_layer_trial::num_lang::send_num_lang;
use toid::music_state::beat::Beat;
use toid::music_state::music_state::{MusicState, MusicStateEvent};
use toid::music_state::scheduling_state::SchedulingStateEvent;
use toid::music_state::sf2_state::SF2StateEvent;
use toid::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use toid::outputters::portaudio_outputter::PortAudioOutputter;
use toid::players::local_player::LocalPlayer;
use toid::players::player::Player;
use toid::resource_management::resource_manager::ResourceManagerEvent;

fn main() {
    let player = LocalPlayer::new();
    let player = Arc::new(player);

    player
        .get_resource_manager()
        .register(String::from("../resource/sf2/sf2.toml"))
        .unwrap();

    player
        .send_resource_event(ResourceManagerEvent::LoadSF2(String::from("sf2.test")))
        .unwrap();

    let mut portaudio_outputter = PortAudioOutputter::new(Arc::clone(&player)
        as Arc<dyn Player<MusicState, MusicStateEvent, WaveReader, Vec<i16>, WaveReaderEvent>>)
    .unwrap();

    player
        .send_event(MusicStateEvent::SF2StateEvent(SF2StateEvent::SetSF2Name(
            String::from("sf2.test"),
        )))
        .unwrap();

    player
        .send_event(MusicStateEvent::SchedulingStateEvent(
            SchedulingStateEvent::ChangeBPM(Beat::from(0), 120.0),
        ))
        .unwrap();
    player
        .send_event(MusicStateEvent::SchedulingStateEvent(
            SchedulingStateEvent::ChangeBPM(Beat::from(8), 180.0),
        ))
        .unwrap();
    player
        .send_event(MusicStateEvent::SchedulingStateEvent(
            SchedulingStateEvent::ChangeBPM(Beat::from(16), 120.0),
        ))
        .unwrap();

    send_num_lang(
        "12345 643 2 1".to_string(),
        0.0,
        "main".to_string(),
        Arc::clone(&player)
            as Arc<dyn Player<MusicState, MusicStateEvent, WaveReader, Vec<i16>, WaveReaderEvent>>,
    )
    .unwrap();

    send_num_lang(
        "1   4   5   1".to_string(),
        -1.0,
        "sub".to_string(),
        Arc::clone(&player)
            as Arc<dyn Player<MusicState, MusicStateEvent, WaveReader, Vec<i16>, WaveReaderEvent>>,
    )
    .unwrap();

    portaudio_outputter.run().unwrap();
    portaudio_outputter.sleep(2250);
    player
        .send_reader_event(WaveReaderEvent::MoveStart)
        .unwrap();
    portaudio_outputter.sleep(12000);
    portaudio_outputter.stop().unwrap();
}
