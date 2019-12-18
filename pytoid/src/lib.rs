use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::sync::Arc;
use std::sync::RwLock;

use toid::music_state_manager;
use toid::portaudio_outputter;
use toid::state_management::reducer;
use toid::state_management::store;
use toid::states::music_state;

#[pyclass(module = "toid")]
struct MusicStateStore {
    store: Arc<RwLock<store::Store<music_state::MusicState>>>,
}

#[pymethods]
impl MusicStateStore {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(MusicStateStore {
            store: Arc::new(RwLock::new(store::Store::new(
                music_state::MusicState::new(),
            ))),
        });
    }
}

#[pyclass(module = "toid")]
struct MusicStateManager {
    manager: Arc<RwLock<music_state_manager::MusicStateManager>>,
}

#[pymethods]
impl MusicStateManager {
    #[new]
    fn new(obj: &PyRawObject, store: &MusicStateStore) {
        let store = Arc::clone(&store.store);
        obj.init(MusicStateManager {
            manager: Arc::new(RwLock::new(music_state_manager::MusicStateManager::new(
                store,
            ))),
        });
    }
}

#[pyclass(module = "toid")]
struct PortAudioOutputter {
    outputter: portaudio_outputter::PortAudioOutputter,
}

#[pymethods]
impl PortAudioOutputter {
    #[new]
    fn new(obj: &PyRawObject, manager: &MusicStateManager) {
        obj.init(PortAudioOutputter {
            outputter: portaudio_outputter::PortAudioOutputter::new(Arc::clone(&manager.manager)),
        });
    }

    fn run(&mut self) {
        self.outputter.run();
    }

    fn sleep(&mut self, millseconds: i32) {
        self.outputter.sleep(millseconds);
    }

    fn stop(&mut self) {
        self.outputter.stop();
    }
}

#[pyclass(module = "toid")]
struct Reducer {
    reducer: reducer::Reducer<music_state::MusicState, music_state_manager::MusicStateEvent>,
}

#[pymethods]
impl Reducer {
    #[new]
    fn new(obj: &PyRawObject, store: &MusicStateStore) {
        let store = Arc::clone(&store.store);
        let reduce = Box::new(music_state_manager::MusicStateReduce {});
        obj.init(Reducer {
            reducer: reducer::Reducer::new(store, reduce),
        });
    }
}

#[pyfunction]
fn add_new_note_on(reducer: &Reducer, pitch: f32, samples: i64) {
    reducer
        .reducer
        .reduce(music_state_manager::MusicStateEvent::AddNewNoteOn(
            pitch, samples,
        ));
}

#[pyfunction]
fn add_new_note_off(reducer: &Reducer, samples: i64) {
    reducer
        .reducer
        .reduce(music_state_manager::MusicStateEvent::AddNewNoteOff(samples));
}

#[pymodule]
fn toid(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<MusicStateStore>()?;
    m.add_class::<MusicStateManager>()?;
    m.add_class::<PortAudioOutputter>()?;
    m.add_class::<Reducer>()?;

    m.add_wrapped(wrap_pyfunction!(add_new_note_on))?;
    m.add_wrapped(wrap_pyfunction!(add_new_note_off))?;
    Ok(())
}
