use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

mod outputters;
mod players;

use outputters::portaudio_outputter::PortAudioOutputter;
use players::local_player::LocalPlayer;

#[pymodule]
fn toid(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<LocalPlayer>()?;
    m.add_class::<PortAudioOutputter>()?;

    Ok(())
}
