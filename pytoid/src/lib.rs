use pyo3::prelude::{
    pyclass, pymethods, pymodule, PyModule, PyObject, PyRawObject, PyRef, PyResult, Python,
};
use std::sync::Arc;
use std::sync::RwLock;

use toid::high_layer_trial::num_lang::send_num_lang;
use toid::music_state::music_state::{MusicState, MusicStateEvent};
use toid::music_state::sf2_state::SF2StateEvent;
use toid::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use toid::outputters::portaudio_outputter;
use toid::players::local_player;
use toid::players::player::Player;
use toid::resource_management::resource_manager::ResourceManagerEvent;
use toid::state_management::store::Store;

pub mod sf2;

#[pyclass(module = "toid")]
struct ToidPlayerHolder {
    player: Arc<dyn Player<MusicState, MusicStateEvent, WaveReader, Vec<i16>, WaveReaderEvent>>,
}

#[pyclass]
struct LocalPlayer {
    player: Arc<
        local_player::LocalPlayer<
            MusicState,
            MusicStateEvent,
            WaveReader,
            Vec<i16>,
            WaveReaderEvent,
        >,
    >,
}

#[pymethods]
impl LocalPlayer {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(LocalPlayer {
            player: Arc::new(local_player::LocalPlayer::new()),
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

#[pyclass(module = "toid")]
struct PortAudioOutputter {
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

#[pymodule]
fn toid(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<LocalPlayer>()?;
    m.add_class::<PortAudioOutputter>()?;

    Ok(())
}
