use std::sync::Arc;

use toid::data::music_info::{Beat, Instrument, Track};
use toid::high_layer_trial::music_language::num_lang::parse_num_lang;
use toid::high_layer_trial::music_language::send_phrase::send_pitch_track;
use toid::music_state::effects::EffectInfo;
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
        .register(String::from(
            "./toid-sample-resource/impulse_response/impulse_response.toml",
        ))
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

    let phrase = parse_num_lang("1234321 ".to_string(), 1.0, 0.0);
    let mut track = Track::new();
    track = track.set_phrase(phrase);
    track = track.set_inst(Instrument::SF2(String::from("example_sf2"), 0));
    track = track.set_vol(1.0);
    track = track.set_pan(0.0);
    track = track.add_effect(EffectInfo::ToLeftEffect);

    send_pitch_track(
        track,
        Beat::from(0),
        "track".to_string(),
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
