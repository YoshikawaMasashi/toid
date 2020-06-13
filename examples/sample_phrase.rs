use std::sync::Arc;

use toid::data::music_info::{Beat, Instrument};
use toid::high_layer_trial::music_language::num_lang::send_num_lang;
use toid::high_layer_trial::music_language::sample_lang::send_sample_lang;
use toid::music_state::states::{MusicState, MusicStateEvent, SchedulingStateEvent};
use toid::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use toid::outputters::portaudio_outputter::PortAudioOutputter;
use toid::players::local_player::LocalPlayer;
use toid::players::player::Player;

fn main() {
    let player = LocalPlayer::new();
    let player = Arc::new(player);

    player
        .get_resource_manager()
        .register(String::from("./toid-sample-resource/sf2/sf2.toml"))
        .unwrap();
    player
        .get_resource_manager()
        .register(String::from("./toid-sample-resource/samples/samples.toml"))
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

    player
        .send_event(MusicStateEvent::NewSection(Beat::from(8.0)))
        .unwrap();

    send_num_lang(
        "12345 643 2 1   ".to_string(),
        0.0,
        0.0,
        Beat::from(0),
        "main".to_string(),
        Instrument::SF2(String::from("example_sf2"), 0),
        1.0,
        -0.5,
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

    send_num_lang(
        "1   4   5   1   ".to_string(),
        -2.0,
        0.0,
        Beat::from(0),
        "sub".to_string(),
        Instrument::SF2(String::from("example_sf2"), 0),
        0.7,
        0.5,
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

    send_sample_lang(
        "x ".to_string(),
        Beat::from(0),
        "samples".to_string(),
        "example_samples".to_string(),
        1.0,
        0.0,
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

    send_num_lang(
        "5 3 4 65        ".to_string(),
        0.0,
        0.0,
        Beat::from(8),
        "main".to_string(),
        Instrument::SF2(String::from("example_sf2"), 0),
        1.0,
        -0.5,
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

    send_num_lang(
        "3   5   4   1   ".to_string(),
        -2.0,
        0.0,
        Beat::from(8),
        "sub".to_string(),
        Instrument::SF2(String::from("example_sf2"), 0),
        0.7,
        0.5,
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

    send_sample_lang(
        "----".to_string(),
        Beat::from(8),
        "hat".to_string(),
        "example_samples".to_string(),
        1.0,
        0.0,
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

    send_sample_lang(
        "x x x  x".to_string(),
        Beat::from(8),
        "bass".to_string(),
        "example_samples".to_string(),
        1.0,
        0.0,
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

    send_sample_lang(
        "  o   o ".to_string(),
        Beat::from(8),
        "snare".to_string(),
        "example_samples".to_string(),
        1.0,
        0.0,
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
    portaudio_outputter.sleep(2250);
    player
        .send_reader_event(WaveReaderEvent::MoveStart)
        .unwrap();
    portaudio_outputter.sleep(12000);
    portaudio_outputter.stop().unwrap();
}
