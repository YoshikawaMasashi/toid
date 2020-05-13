use std::sync::Arc;

use toid::data::music_info::Beat;
use toid::high_layer_trial::music_language::num_lang::{parse_num_lang, send_num_lang};
use toid::high_layer_trial::music_language::send_phrase::send_phrase;
use toid::high_layer_trial::music_language::sample_lang::send_sample_lang;
use toid::high_layer_trial::phrase_operation;
use toid::music_state::states::{MusicState, MusicStateEvent};
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
        .register(String::from("./toid-sample-resource/drums/drums.toml"))
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

    let ph1 = parse_num_lang("53".to_string().repeat(32), 2.0, -4.0);
    let ph2 = parse_num_lang(
        format!(
            "{}{}",
            "97".to_string().repeat(16),
            "86".to_string().repeat(16)
        ),
        1.0,
        -4.0,
    );

    send_phrase(
        phrase_operation::marge(ph1, ph2),
        Beat::from(0),
        "a".to_string(),
        Some("example_sf2".to_string()),
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
    ).unwrap();

    send_num_lang(
        "3121".to_string(),
        1.0,
        -4.0,
        Beat::from(0),
        "b".to_string(),
        Some(String::from("example_sf2")),
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
        "1     5 3       ".to_string(),
        3.0,
        -4.0,
        Beat::from(0),
        "c".to_string(),
        Some(String::from("example_sf2")),
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
        format!(
            "{}{}{}",
            "3".to_string().repeat(16),
            "5".to_string().repeat(16),
            "4".to_string().repeat(32),
        ),
        -2.0,
        -4.0,
        Beat::from(0),
        "d".to_string(),
        Some(String::from("example_sf2")),
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
        "2  1          1 5           5 432  1          1 3       4 3 2 1 ".to_string(),
        -1.0,
        -4.0,
        Beat::from(0),
        "e".to_string(),
        Some(String::from("example_sf2")),
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
        "2  1          1 5           5 432  1          1 3       4 3 2 1 ".to_string(),
        0.0,
        -4.0,
        Beat::from(0),
        "f".to_string(),
        Some(String::from("example_sf2")),
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
        "2  1          1 5           5 432  1          1 3       4 3 2 1 ".to_string(),
        1.0,
        -4.0,
        Beat::from(0),
        "g".to_string(),
        Some(String::from("example_sf2")),
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

    let ph3 = parse_num_lang(
        "2  1          1 5           5 432  1          1 3       4 3 2 1 ".to_string(),
        0.0,
        -4.0,
    );
    let ph4 = phrase_operation::change_pitch_in_key(ph3, -4.0, 4);
    send_phrase(
        ph4,
        Beat::from(0),
        "h".to_string(),
        Some("example_sf2".to_string()),
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
    ).unwrap();

    let ph5 = parse_num_lang(
        "12356".to_string().repeat(8),
        1.0,
        -4.0,
    );
    let ph6 = phrase_operation::shuffle_start(ph5);
    send_phrase(
        ph6,
        Beat::from(0),
        "i".to_string(),
        Some("example_sf2".to_string()),
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
    ).unwrap();

    send_sample_lang(
        "x x x x ".to_string(),
        Beat::from(8),
        "kick".to_string(),
        "example_drums".to_string(),
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
        "- - - - ".to_string(),
        Beat::from(8),
        "hat".to_string(),
        "example_drums".to_string(),
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
        "   ooo  ".to_string(),
        Beat::from(8),
        "snare".to_string(),
        "example_drums".to_string(),
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
    portaudio_outputter.sleep(60000);
    portaudio_outputter.stop().unwrap();
}
