use std::sync::Arc;

use toid::data::music_info::beat::Beat;
use toid::high_layer_trial::music_language::num_lang::parse_num_lang;
use toid::high_layer_trial::music_language::send_phrase::send_phrase;
use toid::high_layer_trial::phrase_operation::condition::IsDownBeat;
use toid::high_layer_trial::phrase_operation::{
    concat, delay, marge, shuffle_start, split_by_condition,
};
use toid::music_state::states::{MusicState, MusicStateEvent, SchedulingStateEvent};
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
        .register(String::from("./toid-sample-resource/sf2/sf2.toml"))
        .unwrap();

    player
        .send_resource_event(ResourceManagerEvent::LoadSF2(String::from("sf2.test")))
        .unwrap();

    let mut portaudio_outputter = PortAudioOutputter::new(Arc::clone(&player)
        as Arc<
            dyn Player<
                MusicState,
                MusicStateEvent,
                WaveReader,
                (Vec<i16>, Vec<i16>),
                WaveReaderEvent,
            >,
        >)
    .unwrap();
    portaudio_outputter.set_volume(0.3);

    player
        .send_event(MusicStateEvent::SchedulingStateEvent(
            SchedulingStateEvent::ChangeBPM(Beat::from(0), 120.0),
        ))
        .unwrap();

    let phrase1 = parse_num_lang("1234321 ".to_string(), 0.0, 0.0);
    let phrase2 = parse_num_lang("3456543 ".to_string(), 0.0, 0.0);
    let phrase3 = parse_num_lang("1 1 1 1 ".to_string(), 0.0, 0.0);
    let phrase4 = parse_num_lang("1234321 ".to_string(), 0.0, 0.0);

    let phrase5 = concat(concat(concat(phrase1, phrase2), phrase3), phrase4);
    let phrase6 = delay(phrase5.clone(), Beat::from(4));
    let phrase7 = marge(phrase5, phrase6);

    let (phrase8, phrase9) = split_by_condition(phrase7.clone(), Box::new(IsDownBeat::new()));
    let phrase10 = shuffle_start(phrase9);
    let phrase11 = marge(phrase8, phrase10);

    send_phrase(
        phrase11,
        Beat::from(0),
        "phrase11".to_string(),
        Some(String::from("sf2.test")),
        1.0,
        -1.0,
        Arc::clone(&player)
            as Arc<
                dyn Player<
                    MusicState,
                    MusicStateEvent,
                    WaveReader,
                    (Vec<i16>, Vec<i16>),
                    WaveReaderEvent,
                >,
            >,
    )
    .unwrap();

    portaudio_outputter.run().unwrap();
    portaudio_outputter.sleep(12000);
    portaudio_outputter.stop().unwrap();
}
