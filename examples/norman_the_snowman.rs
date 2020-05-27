use std::sync::Arc;

use toid::data::music_info::{Beat, Instrument, PitchInOctave};
use toid::high_layer_trial::music_language::num_lang::{parse_num_lang, send_num_lang};
use toid::high_layer_trial::music_language::sample_lang::send_sample_lang;
use toid::high_layer_trial::music_language::send_phrase::send_phrase;
use toid::high_layer_trial::num::{
    change_max_min, f32_vec_to_beat_vec, f32_vec_to_pitch_vec, linspace, parlin_noise_seq,
};
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

    player
        .get_resource_manager()
        .get_sf2(String::from("example_sf2"))
        .unwrap()
        .print_preset_names();

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
        Instrument::SF2(String::from("example_sf2"), 2),
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
        "3121".to_string(),
        1.0,
        -4.0,
        Beat::from(0),
        "b".to_string(),
        Instrument::SF2(String::from("example_sf2"), 0),
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
        Instrument::SF2(String::from("example_sf2"), 0),
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
        Beat::from(6),
        "d".to_string(),
        Instrument::SF2(String::from("example_sf2"), 0),
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
        Instrument::SF2(String::from("example_sf2"), 1),
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
        Instrument::SF2(String::from("example_sf2"), 1),
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
        Instrument::SF2(String::from("example_sf2"), 1),
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
    let ph4 = phrase_operation::change_pitch_in_key(ph3, PitchInOctave { pitch: -4.0 }, 4);
    send_phrase(
        ph4,
        Beat::from(0),
        "h".to_string(),
        Instrument::SF2(String::from("example_sf2"), 1),
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

    let ph5 = parse_num_lang("12356".to_string().repeat(8), 1.0, -4.0);
    let ph6 = phrase_operation::shuffle_start(ph5);
    send_phrase(
        ph6,
        Beat::from(0),
        "i".to_string(),
        Instrument::SF2(String::from("example_sf2"), 10),
        1.0,
        0.7,
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

    let parlin = parlin_noise_seq(121, 0.1, None);
    let parlin = change_max_min(&parlin, 72.0, 88.0);
    let parlin = f32_vec_to_pitch_vec(&parlin);

    let parlin_beat = linspace(0.0, 8.1, 121);
    let parlin_beat = f32_vec_to_beat_vec(&parlin_beat);

    let start = linspace(0.0, 7.75, 32);
    let start = f32_vec_to_beat_vec(&start);

    let scale = vec![
        PitchInOctave::from(0.0 - 4.0),
        PitchInOctave::from(2.0 - 4.0),
        PitchInOctave::from(4.0 - 4.0),
        PitchInOctave::from(7.0 - 4.0),
        PitchInOctave::from(9.0 - 4.0),
    ];

    let duration: Vec<f32> = vec![0.25; 32];
    let duration = f32_vec_to_beat_vec(&duration);

    let ph7 = phrase_operation::round_line((parlin_beat, parlin), start, duration, scale);
    let ph7 = phrase_operation::sixteen_shuffle(ph7);

    send_phrase(
        ph7,
        Beat::from(0),
        "j".to_string(),
        Instrument::SF2(String::from("example_sf2"), 13),
        0.3,
        -0.7,
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
