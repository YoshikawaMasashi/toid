use std::sync::Arc;

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
    let connect_address = format!("ws://wait:password@{}:3012", ip).replace("\n", "");

    let mut player = WebSocketPlayer::new();

    player
        .get_resource_manager()
        .register(String::from("./toid-sample-resource/sf2/sf2.toml"))
        .unwrap();
    player.connect(connect_address);
    let player = Arc::new(player);

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

    portaudio_outputter.run().unwrap();
    portaudio_outputter.sleep(300000);
}
