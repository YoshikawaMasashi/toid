use pyo3::prelude::{
    pyclass, pymethods, pymodule, PyModule, PyObject, PyRawObject, PyResult, Python,
};
use std::sync::Arc;
use std::sync::RwLock;

use toid::high_layer_trial::num_lang::send_num_lang;
use toid::music_state::music_state::{MusicState, MusicStateEvent};
use toid::music_state::sf2_state::SF2StateEvent;
use toid::music_state::wave_reader;
use toid::outputters::portaudio_outputter;
use toid::players::local_player;
use toid::players::player::Player;
use toid::resource_management::resource_manager;
use toid::state_management::store::Store;

pub mod sf2;

#[pyclass(module = "toid")]
struct MusicStore {
    store: Arc<Store<MusicState, MusicStateEvent>>,
}

#[pymethods]
impl MusicStore {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(MusicStore {
            store: Arc::new(Store::new(MusicState::new())),
        });
    }
}

#[pyclass(module = "toid")]
struct ResourceManager {
    manager: Arc<resource_manager::ResourceManager>,
}

#[pymethods]
impl ResourceManager {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(ResourceManager {
            manager: Arc::new(resource_manager::ResourceManager::new()),
        });
    }

    fn register(&self, path: String) {
        self.manager.register(path);
    }

    fn load_sf2(&self, name: String) {
        self.manager.load_sf2(name);
    }
}

#[pyclass]
struct WaveReader {
    reader: Arc<RwLock<wave_reader::WaveReader>>,
}

#[pymethods]
impl WaveReader {
    #[new]
    fn new(obj: &PyRawObject, store: &MusicStore, resource_manager: &ResourceManager) {
        obj.init(WaveReader {
            reader: Arc::new(RwLock::new(wave_reader::WaveReader::new(
                Arc::clone(&store.store),
                Arc::clone(&resource_manager.manager),
            ))),
        })
    }
}

#[pyclass]
struct LocalPlayer {
    player: Arc<local_player::LocalPlayer<MusicState, MusicStateEvent>>,
}

#[pymethods]
impl LocalPlayer {
    #[new]
    fn new(obj: &PyRawObject, store: &MusicStore, resource_manager: &ResourceManager) {
        obj.init(LocalPlayer {
            player: Arc::new(local_player::LocalPlayer::new(
                Arc::clone(&store.store),
                Arc::clone(&resource_manager.manager),
            )),
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
            Arc::clone(&self.player) as Arc<dyn Player<MusicState, MusicStateEvent>>,
        );
    }
}

#[pyclass(module = "toid")]
struct PortAudioOutputter {
    outputter: Arc<RwLock<portaudio_outputter::PortAudioOutputter>>,
}

#[pymethods]
impl PortAudioOutputter {
    #[new]
    fn new(obj: &PyRawObject, reader: &WaveReader) {
        obj.init(PortAudioOutputter {
            outputter: Arc::new(RwLock::new(portaudio_outputter::PortAudioOutputter::new(
                Arc::clone(&reader.reader),
            ))),
        });
    }

    fn run(&self) {
        self.outputter.write().unwrap().run();
    }

    fn stop(&self) {
        self.outputter.write().unwrap().stop();
    }
}

#[pymodule]
fn toid(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<MusicStore>()?;
    m.add_class::<ResourceManager>()?;
    m.add_class::<WaveReader>()?;
    m.add_class::<LocalPlayer>()?;
    m.add_class::<PortAudioOutputter>()?;

    Ok(())
}
