use pyo3::prelude::{pyclass, pymethods, PyObject, PyRawObject};
use std::sync::Arc;
use std::sync::RwLock;

use toid::outputters::portaudio_outputter;

use super::super::players::toid_player_holder::ToidPlayerHolder;

#[pyclass(module = "toid")]
pub struct PortAudioOutputter {
    outputter: Arc<RwLock<portaudio_outputter::PortAudioOutputter>>,
}

#[pymethods]
impl PortAudioOutputter {
    #[new]
    fn new(obj: &PyRawObject, player: &ToidPlayerHolder) {
        obj.init(PortAudioOutputter {
            outputter: Arc::new(RwLock::new(
                portaudio_outputter::PortAudioOutputter::new(Arc::clone(&player.player)).unwrap(),
            )),
        });
    }

    fn run(&self) {
        self.outputter.write().unwrap().run().unwrap();
    }

    fn stop(&self) {
        self.outputter.write().unwrap().stop().unwrap();
    }
}
