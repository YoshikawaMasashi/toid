use std::sync::Arc;

use toid::data::music_info::{Beat, Instrument};
use toid::high_layer_trial::music_language::num_lang::send_num_lang;
use toid::music_state::states::{MusicState, MusicStateEvent};
use toid::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use toid::outputters::portaudio_outputter::PortAudioOutputter;
use toid::players::player::Player;
use toid::players::websocket_player::WebSocketPlayer;

fn main() {
    let mut ip = String::new();
    println!("please input ip (ex. 127.0.0.1):");
    std::io::stdin().read_line(&mut ip).unwrap();
    println!("ip: {}", ip);
    let ip = ip;
    let connect_address = format!("ws://play:password@{}:3012", ip).replace("\n", "");

    let mut player = WebSocketPlayer::new();
    player.connect(connect_address);
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

    send_num_lang(
        "12345 643 2 1   ".to_string(),
        0.0,
        0.0,
        Beat::from(0),
        "main".to_string(),
        Instrument::SF2(String::from("example_sf2"), 0),
        1.0,
        1.0,
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
        -1.0,
        0.0,
        Beat::from(0),
        "sub".to_string(),
        Instrument::SF2(String::from("example_sf2"), 0),
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

    loop {
        portaudio_outputter.sleep(5000);
        player.sync_state().unwrap();
    }
}
