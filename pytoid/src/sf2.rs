use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::{
    pyclass, pyfunction, pymethods, pymodule, Py, PyModule, PyObject, PyRawObject, PyResult, Python,
};
use pyo3::wrap_pyfunction;

use toid::data::sf2;

#[pyfunction]
fn read_sf2(path: String) -> SF2 {
    let mut f = File::open(path).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    let buffer = buffer.as_slice();

    let sf2_data = sf2::SF2::parse(buffer);
    SF2 {
        sf2: Arc::new(sf2_data),
    }
}

#[pyclass(module = "sf2")]
struct SF2 {
    pub sf2: Arc<sf2::SF2>,
}

#[pymethods]
impl SF2 {
    #[getter]
    fn info(&self) -> SF2Info {
        SF2Info {
            info: Arc::clone(&self.sf2.info),
        }
    }

    #[getter]
    fn sdta(&self) -> SF2sdta {
        SF2sdta {
            sdta: Arc::clone(&self.sf2.sdta),
        }
    }

    #[getter]
    fn pdta(&self) -> SF2pdta {
        SF2pdta {
            pdta: Arc::clone(&self.sf2.pdta),
        }
    }
}

#[pyclass(module = "sf2")]
struct SF2Info {
    info: Arc<sf2::info::SF2Info>,
}

#[pymethods]
impl SF2Info {
    #[getter]
    fn ifil(&self) -> String {
        format!("{}.{}", self.info.ifil.major, self.info.ifil.minor)
    }

    #[getter]
    fn isng(&self) -> String {
        self.info.isng.clone()
    }

    #[getter]
    fn inam(&self) -> String {
        self.info.inam.clone()
    }

    #[getter]
    fn irom(&self) -> Option<String> {
        self.info.irom.clone()
    }

    #[getter]
    fn iver(&self) -> Option<String> {
        if let Some(iver) = self.info.iver.clone() {
            Some(format!("{}.{}", iver.major, iver.minor))
        } else {
            None
        }
    }

    #[getter]
    fn icrd(&self) -> Option<String> {
        self.info.icrd.clone()
    }

    #[getter]
    fn ieng(&self) -> Option<String> {
        self.info.ieng.clone()
    }

    #[getter]
    fn iprd(&self) -> Option<String> {
        self.info.iprd.clone()
    }

    #[getter]
    fn icop(&self) -> Option<String> {
        self.info.icop.clone()
    }

    #[getter]
    fn icmt(&self) -> Option<String> {
        self.info.icmt.clone()
    }

    #[getter]
    fn isft(&self) -> Option<String> {
        self.info.isft.clone()
    }
}

#[pyclass(module = "sf2")]
struct SF2sdta {
    sdta: Arc<sf2::sdta::SF2sdta>,
}

#[pymethods]
impl SF2sdta {
    #[getter]
    fn smpl(&self) -> PyResult<Py<PyArray1<i16>>> {
        let gil = pyo3::Python::acquire_gil();
        let smpl = self.sdta.smpl.to_vec().clone();
        Ok(PyArray1::from_vec(gil.python(), smpl).to_owned())
    }
}

#[pyclass(module = "sf2")]
struct SF2pdta {
    pdta: Arc<sf2::pdta::SF2pdta>,
}

#[pymodule]
pub fn sf2(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<SF2>()?;
    m.add_class::<SF2Info>()?;
    m.add_class::<SF2sdta>()?;
    m.add_class::<SF2pdta>()?;

    m.add_wrapped(wrap_pyfunction!(read_sf2))?;

    Ok(())
}
