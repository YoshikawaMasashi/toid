use std::sync::Arc;

use toid::data::music_info::{Beat, Instrument, Scale};
use toid::high_layer_trial::music_language::num_lang::parse_num_lang;
use toid::high_layer_trial::music_language::send_phrase::send_phrase;
use toid::high_layer_trial::num::{
    change_max_min, f32_vec_to_beat_vec, f32_vec_to_pitch_vec, linspace, parlin_noise_seq,
};
use toid::high_layer_trial::phrase_operation::condition::is_down_beat;
use toid::high_layer_trial::phrase_operation::{
    concat, delay, marge, round_line, shuffle_start, split_by_condition,
};
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

    let phrase1 = parse_num_lang("1234321 ".to_string(), 1.0, 0.0);
    let phrase2 = parse_num_lang("3456543 ".to_string(), 1.0, 0.0);
    let phrase3 = parse_num_lang("1 1 1 1 ".to_string(), 1.0, 0.0);
    let phrase4 = parse_num_lang("1234321 ".to_string(), 1.0, 0.0);

    let phrase5 = concat(concat(concat(phrase1, phrase2), phrase3), phrase4);
    let phrase6 = delay(phrase5.clone(), Beat::from(4));
    let phrase7 = marge(phrase5, phrase6);

    let (phrase8, phrase9) = split_by_condition(phrase7.clone(), is_down_beat(phrase7));
    let phrase10 = shuffle_start(phrase9);
    let phrase11 = marge(phrase8, phrase10);

    let parlin = parlin_noise_seq(121, 0.1, None);
    let parlin = change_max_min(&parlin, 84.0, 96.0);
    let parlin = f32_vec_to_pitch_vec(&parlin);

    let parlin_beat = linspace(0.0, 8.1, 121);
    let parlin_beat = f32_vec_to_beat_vec(&parlin_beat);

    let start = linspace(0.0, 7.5, 16);
    let start = f32_vec_to_beat_vec(&start);

    let duration: Vec<f32> = vec![0.5; 16];
    let duration = f32_vec_to_beat_vec(&duration);

    let phrase12 = round_line(
        parlin_beat,
        parlin,
        start,
        duration,
        Scale::from("CMajPenta".to_string()),
    );

    send_phrase(
        phrase11,
        Beat::from(0),
        "phrase11".to_string(),
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

    send_phrase(
        phrase12,
        Beat::from(0),
        "phrase12".to_string(),
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

    portaudio_outputter.run().unwrap();
    portaudio_outputter.sleep(12000);
    portaudio_outputter.stop().unwrap();
}
