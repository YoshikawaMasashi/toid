use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

mod outputters;
mod players;
mod sf2;

use outputters::portaudio_outputter::PortAudioOutputter;
use players::local_player::LocalPlayer;
use players::websocket_player::{WebSocketPlayer, WebSocketPlayerServer};

#[pymodule]
fn toid(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PortAudioOutputter>()?;
    m.add_class::<LocalPlayer>()?;
    m.add_class::<WebSocketPlayer>()?;
    m.add_class::<WebSocketPlayerServer>()?;

    Ok(())
}
