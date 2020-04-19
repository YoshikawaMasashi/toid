use pyo3::prelude::{pyclass, pymethods, PyObject, PyRawObject};
use std::sync::Arc;

use toid::high_layer_trial::num_lang::send_num_lang;
use toid::music_state::music_state::{MusicState, MusicStateEvent};
use toid::music_state::sf2_state::SF2StateEvent;
use toid::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use toid::players::player::Player;
use toid::players::websocket_player;
use toid::resource_management::resource_manager::ResourceManagerEvent;

use super::toid_player_holder::ToidPlayerHolder;

#[pyclass]
pub struct WebSocketPlayer {
    player: Arc<
        websocket_player::WebSocketPlayer<
            MusicState,
            MusicStateEvent,
            WaveReader,
            Vec<i16>,
            WaveReaderEvent,
        >,
    >,
}

#[pymethods]
impl WebSocketPlayer {
    #[new]
    fn new(obj: &PyRawObject, connect_address: String) {
        let mut player = websocket_player::WebSocketPlayer::new();
        player.connect(connect_address);
        obj.init(WebSocketPlayer {
            player: Arc::new(player),
        })
    }

    fn set_sf2_name(&self, name: String) {
        self.player
            .send_event(MusicStateEvent::SF2StateEvent(SF2StateEvent::SetSF2Name(
                name,
            )));
    }

    fn send_num_lang(&self, melody_string: String, octave: f32, name: String) {
        send_num_lang(
            melody_string,
            octave,
            name,
            Arc::clone(&self.player)
                as Arc<
                    dyn Player<MusicState, MusicStateEvent, WaveReader, Vec<i16>, WaveReaderEvent>,
                >,
        );
    }

    fn resource_register(&self, path: String) {
        self.player.get_resource_manager().register(path).unwrap();
    }

    fn load_sf2(&self, name: String) {
        self.player
            .send_resource_event(ResourceManagerEvent::LoadSF2(name))
            .unwrap();
    }

    fn get_toid_player(&self) -> ToidPlayerHolder {
        ToidPlayerHolder {
            player: (Arc::clone(&self.player)
                as Arc<
                    dyn Player<MusicState, MusicStateEvent, WaveReader, Vec<i16>, WaveReaderEvent>,
                >),
        }
    }
}

#[pyclass]
pub struct WebSocketPlayerServer {
    server: Arc<websocket_player::WebSocketPlayerServer>,
}

#[pymethods]
impl WebSocketPlayerServer {
    #[new]
    fn new(obj: &PyRawObject, connect_address: String) {
        let mut server = websocket_player::WebSocketPlayerServer::new();
        server.listen(connect_address);
        obj.init(Self {
            server: Arc::new(server),
        })
    }
}
