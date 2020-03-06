use pyo3::prelude::{
    pyclass, pymethods, pymodule, PyModule, PyObject, PyRawObject, PyResult, Python,
};
use std::sync::Arc;
use std::sync::RwLock;

use toid::music_store::melody_state::NoteInfo;
use toid::music_store::melody_state::{MelodyState, MelodyStateEvent, MelodyStateReducer};
use toid::music_store::music_store;
use toid::music_store::sf2_state::SF2StateEvent;
use toid::music_store::wave_reader;
use toid::outputters::portaudio_outputter;
use toid::state_management::store::Store;

pub mod sf2;

#[pyclass(module = "toid")]
struct MusicStore {
    store: Arc<music_store::MusicStore>,
}

#[pymethods]
impl MusicStore {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(MusicStore {
            store: Arc::new(music_store::MusicStore::new()),
        });
    }

    fn new_melody(&self, key: String) {
        self.store.new_melody(key);
    }

    fn load_and_set_sf2(&self, path: String) {
        self.store
            .sf2
            .update_state(SF2StateEvent::LoadAndSetSF2(path));
    }

    fn get_melody(&self, key: String) -> MelodyStore {
        MelodyStore {
            store: Arc::clone(&self.store.melody.read().unwrap().get(&key).unwrap()),
        }
    }
}

#[pyclass(module = "toid")]
struct MelodyStore {
    store: Arc<Store<MelodyState, MelodyStateEvent, MelodyStateReducer>>,
}

#[pymethods]
impl MelodyStore {
    fn add_note(&self, pitch: f32, duration: u64, start: u64) {
        self.store.update_state(MelodyStateEvent::AddNote(NoteInfo {
            pitch,
            duration,
            start,
        }));
    }
}

#[pyclass(module = "toid")]
struct WaveReader {
    reader: Arc<RwLock<wave_reader::WaveReader>>,
}

#[pymethods]
impl WaveReader {
    #[new]
    fn new(obj: &PyRawObject, store: &MusicStore) {
        obj.init(WaveReader {
            reader: Arc::new(RwLock::new(wave_reader::WaveReader::new(Arc::clone(
                &store.store,
            )))),
        });
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
    m.add_class::<MelodyStore>()?;
    m.add_class::<WaveReader>()?;
    m.add_class::<PortAudioOutputter>()?;

    Ok(())
}
