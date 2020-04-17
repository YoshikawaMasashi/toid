use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use numpy::PyArray1;
use pyo3::prelude::{
    pyclass, pyfunction, pymethods, pymodule, Py, PyModule, PyObject, PyResult, Python,
};
use pyo3::wrap_pyfunction;

use toid::data::sf2::own;
use toid::data::sf2::parsed;

#[pyfunction]
pub fn read_sf2(path: String) -> SF2 {
    let mut f = File::open(path).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    let buffer = buffer.as_slice();

    let sf2_data = own::SF2::parse(buffer).unwrap();
    SF2 {
        sf2: Arc::new(sf2_data),
    }
}

#[pyclass(module = "sf2")]
pub struct SF2 {
    pub sf2: Arc<own::SF2>,
}

#[pymethods]
impl SF2 {
    fn get_samples(
        &self,
        preset_idx: usize,
        key: u8,
        start: usize,
        end: usize,
    ) -> PyResult<Py<PyArray1<i16>>> {
        let gil = pyo3::Python::acquire_gil();
        let sample = self.sf2.get_samples(preset_idx, key, start, end).unwrap();
        Ok(PyArray1::from_vec(gil.python(), sample).to_owned())
    }
}

#[pyclass(module = "sf2")]
pub struct SF2Preset {
    pub sf2_preset: Arc<own::preset::Preset>,
}

#[pyclass(module = "sf2")]
pub struct SF2Generator {
    pub sf2_generator: Arc<own::generator::Generator>,
}

#[pyclass(module = "sf2")]
pub struct SF2Instrument {
    pub sf2_instrument: Arc<own::instrument::Instrument>,
}

#[pyclass(module = "sf2")]
pub struct SF2Sample {
    pub sf2_sample: Arc<own::sample::Sample>,
}

#[pyfunction]
pub fn read_parsed_sf2(path: String) -> ParsedSF2 {
    let mut f = File::open(path).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    let buffer = buffer.as_slice();

    let parsed_sf2_data = parsed::SF2::parse(buffer).unwrap();
    ParsedSF2 {
        parsed_sf2: Arc::new(parsed_sf2_data),
    }
}

#[pyclass(module = "sf2")]
pub struct ParsedSF2 {
    pub parsed_sf2: Arc<parsed::SF2>,
}

#[pymethods]
impl ParsedSF2 {
    #[getter]
    fn info(&self) -> SF2Info {
        SF2Info {
            info: Arc::clone(&self.parsed_sf2.info),
        }
    }

    #[getter]
    fn sdta(&self) -> SF2sdta {
        SF2sdta {
            sdta: Arc::clone(&self.parsed_sf2.sdta),
        }
    }

    #[getter]
    fn pdta(&self) -> SF2pdta {
        SF2pdta {
            pdta: Arc::clone(&self.parsed_sf2.pdta),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.parsed_sf2))
    }
}

#[pyclass(module = "sf2")]
pub struct SF2Info {
    info: Arc<parsed::info::SF2Info>,
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
    sdta: Arc<parsed::sdta::SF2sdta>,
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
    pdta: Arc<parsed::pdta::SF2pdta>,
}

#[pymethods]
impl SF2pdta {
    #[getter]
    fn phdr(&self) -> Vec<SFPresetHeader> {
        let mut phdr = Vec::new();
        for phdr_ in self.pdta.phdr.clone() {
            phdr.push(SFPresetHeader {
                sf_preset_header: phdr_,
            });
        }
        phdr
    }

    #[getter]
    fn pbag(&self) -> Vec<SFBag> {
        let mut pbag = Vec::new();
        for pbag_ in self.pdta.pbag.clone() {
            pbag.push(SFBag { sf_bag: pbag_ });
        }
        pbag
    }

    #[getter]
    fn pmod(&self) -> Vec<SFMod> {
        let mut pmod = Vec::new();
        for pmod_ in self.pdta.pmod.clone() {
            pmod.push(SFMod { sf_mod: pmod_ });
        }
        pmod
    }

    #[getter]
    fn pgen(&self) -> Vec<SFGen> {
        let mut pgen = Vec::new();
        for pgen_ in self.pdta.pgen.clone() {
            pgen.push(SFGen { sf_gen: pgen_ });
        }
        pgen
    }

    #[getter]
    fn inst(&self) -> Vec<SFInstHeader> {
        let mut inst = Vec::new();
        for inst_ in self.pdta.inst.clone() {
            inst.push(SFInstHeader {
                sf_inst_header: inst_,
            });
        }
        inst
    }

    #[getter]
    fn ibag(&self) -> Vec<SFBag> {
        let mut ibag = Vec::new();
        for ibag_ in self.pdta.ibag.clone() {
            ibag.push(SFBag { sf_bag: ibag_ });
        }
        ibag
    }

    #[getter]
    fn imod(&self) -> Vec<SFMod> {
        let mut imod = Vec::new();
        for imod_ in self.pdta.imod.clone() {
            imod.push(SFMod { sf_mod: imod_ });
        }
        imod
    }

    #[getter]
    fn igen(&self) -> Vec<SFGen> {
        let mut igen = Vec::new();
        for igen_ in self.pdta.igen.clone() {
            igen.push(SFGen { sf_gen: igen_ });
        }
        igen
    }

    #[getter]
    fn shdr(&self) -> Vec<SFSampleHeader> {
        let mut shdr = Vec::new();
        for shdr_ in self.pdta.shdr.clone() {
            shdr.push(SFSampleHeader {
                sf_sample_header: shdr_,
            });
        }
        shdr
    }
}

#[pyclass(module = "sf2")]
pub struct SFBag {
    pub sf_bag: Arc<parsed::pdta::sf_bag::SFBag>,
}

#[pymethods]
impl SFBag {
    #[getter]
    fn gen_index(&self) -> u16 {
        self.sf_bag.gen_index
    }
    #[getter]
    fn mod_index(&self) -> u16 {
        self.sf_bag.mod_index
    }
}

#[pyclass(module = "sf2")]
pub struct SFGen {
    pub sf_gen: Arc<parsed::pdta::sf_gen::SFGen>,
}

#[pymethods]
impl SFGen {
    #[getter]
    fn gen_oper(&self) -> u16 {
        self.sf_gen.gen_oper
    }
    #[getter]
    fn gen_amount(&self) -> i16 {
        self.sf_gen.gen_amount
    }
}

#[pyclass(module = "sf2")]
pub struct SFInstHeader {
    pub sf_inst_header: Arc<parsed::pdta::sf_inst_header::SFInstHeader>,
}

#[pymethods]
impl SFInstHeader {
    #[getter]
    fn name(&self) -> String {
        self.sf_inst_header.name.clone()
    }
    #[getter]
    fn bag_index(&self) -> u16 {
        self.sf_inst_header.bag_index
    }
}

#[pyclass(module = "sf2")]
pub struct SFMod {
    pub sf_mod: Arc<parsed::pdta::sf_mod::SFMod>,
}

#[pymethods]
impl SFMod {
    #[getter]
    fn src_oper(&self) -> u16 {
        self.sf_mod.src_oper
    }
    #[getter]
    fn dest_oper(&self) -> u16 {
        self.sf_mod.dest_oper
    }
    #[getter]
    fn mod_amount(&self) -> i16 {
        self.sf_mod.mod_amount
    }
    #[getter]
    fn amt_src_oper(&self) -> u16 {
        self.sf_mod.amt_src_oper
    }
    #[getter]
    fn mod_trans_oper(&self) -> u16 {
        self.sf_mod.mod_trans_oper
    }
}

#[pyclass(module = "sf2")]
pub struct SFPresetHeader {
    pub sf_preset_header: Arc<parsed::pdta::sf_preset_header::SFPresetHeader>,
}

#[pymethods]
impl SFPresetHeader {
    #[getter]
    fn name(&self) -> String {
        self.sf_preset_header.name.clone()
    }
    #[getter]
    fn presento(&self) -> u16 {
        self.sf_preset_header.presento
    }
    #[getter]
    fn bank(&self) -> u16 {
        self.sf_preset_header.bank
    }
    #[getter]
    fn bag_index(&self) -> u16 {
        self.sf_preset_header.bag_index
    }
    #[getter]
    fn library(&self) -> u32 {
        self.sf_preset_header.library
    }
    #[getter]
    fn morph(&self) -> u32 {
        self.sf_preset_header.morph
    }
}
#[pyclass(module = "sf2")]
pub struct SFSampleHeader {
    pub sf_sample_header: Arc<parsed::pdta::sf_sample_header::SFSampleHeader>,
}

#[pymethods]
impl SFSampleHeader {
    #[getter]
    fn name(&self) -> String {
        self.sf_sample_header.name.clone()
    }
    #[getter]
    fn start(&self) -> u32 {
        self.sf_sample_header.start
    }
    #[getter]
    fn end(&self) -> u32 {
        self.sf_sample_header.end
    }
    #[getter]
    fn loopstart(&self) -> u32 {
        self.sf_sample_header.loopstart
    }
    #[getter]
    fn loopend(&self) -> u32 {
        self.sf_sample_header.loopend
    }
    #[getter]
    fn sample_rate(&self) -> u32 {
        self.sf_sample_header.sample_rate
    }
    #[getter]
    fn original_key(&self) -> u8 {
        self.sf_sample_header.original_key
    }
    #[getter]
    fn correction(&self) -> i8 {
        self.sf_sample_header.correction
    }
    #[getter]
    fn sample_link(&self) -> u16 {
        self.sf_sample_header.sample_link
    }
    #[getter]
    fn typee(&self) -> u16 {
        self.sf_sample_header.typee
    }
}
#[pymodule]
pub fn sf2(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // own
    m.add_wrapped(wrap_pyfunction!(read_sf2))?;
    m.add_class::<SF2>()?;
    m.add_class::<SF2Preset>()?;
    m.add_class::<SF2Generator>()?;
    m.add_class::<SF2Instrument>()?;
    m.add_class::<SF2Sample>()?;

    // parsed
    m.add_wrapped(wrap_pyfunction!(read_parsed_sf2))?;
    m.add_class::<ParsedSF2>()?;
    m.add_class::<SF2Info>()?;
    m.add_class::<SF2sdta>()?;
    m.add_class::<SF2pdta>()?;
    m.add_class::<SFBag>()?;
    m.add_class::<SFGen>()?;
    m.add_class::<SFInstHeader>()?;
    m.add_class::<SFMod>()?;
    m.add_class::<SFPresetHeader>()?;
    m.add_class::<SFSampleHeader>()?;

    Ok(())
}
