use std::fmt;
use std::sync::Arc;

use super::super::super::riff::{RiffChank, RiffData};
use super::sf_bag::{parse_sf_bags, SFBag};
use super::sf_gen::{parse_sf_gens, SFGen};
use super::sf_inst_header::{parse_sf_inst_headers, SFInstHeader};
use super::sf_mod::{parse_sf_mods, SFMod};
use super::sf_preset_header::{parse_sf_preset_headers, SFPresetHeader};
use super::sf_sample_header::{parse_sf_sample_headers, SFSampleHeader};

pub struct SF2pdta {
    phdr: Vec<Arc<SFPresetHeader>>,
    pbag: Vec<Arc<SFBag>>,
    pmod: Vec<Arc<SFMod>>,
    pgen: Vec<Arc<SFGen>>,
    inst: Vec<Arc<SFInstHeader>>,
    ibag: Vec<Arc<SFBag>>,
    imod: Vec<Arc<SFMod>>,
    igen: Vec<Arc<SFGen>>,
    shdr: Vec<Arc<SFSampleHeader>>,
}

impl fmt::Display for SF2pdta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "***SF2pdta***\n")?;
        write!(f, "phdr length: {}\n", self.phdr.len())?;
        write!(f, "pbag length: {}\n", self.pbag.len())?;
        write!(f, "pmod length: {}\n", self.pmod.len())?;
        write!(f, "pgen length: {}\n", self.pgen.len())?;
        write!(f, "inst length: {}\n", self.inst.len())?;
        write!(f, "ibag length: {}\n", self.ibag.len())?;
        write!(f, "imod length: {}\n", self.imod.len())?;
        write!(f, "igen length: {}\n", self.igen.len())?;
        write!(f, "shdr length: {}\n", self.shdr.len())?;

        Ok(())
    }
}

pub fn convert_chank_to_sf2pdta(chank: &RiffChank) -> Result<SF2pdta, String> {
    let mut phdr: Option<Vec<Arc<SFPresetHeader>>> = None;
    let mut pbag: Option<Vec<Arc<SFBag>>> = None;
    let mut pmod: Option<Vec<Arc<SFMod>>> = None;
    let mut pgen: Option<Vec<Arc<SFGen>>> = None;
    let mut inst: Option<Vec<Arc<SFInstHeader>>> = None;
    let mut ibag: Option<Vec<Arc<SFBag>>> = None;
    let mut imod: Option<Vec<Arc<SFMod>>> = None;
    let mut igen: Option<Vec<Arc<SFGen>>> = None;
    let mut shdr: Option<Vec<Arc<SFSampleHeader>>> = None;

    if let Some(chank_type) = &chank.chank_type {
        if chank_type == "pdta" && chank.id == "LIST" {
            if let RiffData::Chanks(subchanks) = &chank.data {
                for subchank in subchanks {
                    if let RiffData::Data(data_in_subchank) = &subchank.data {
                        match subchank.id.as_str() {
                            "phdr" => {
                                let (_, phdr_) = parse_sf_preset_headers(
                                    data_in_subchank,
                                    subchank.size / 38 - 1,
                                )
                                .expect("Invalid phdr");
                                phdr = Some(phdr_);
                            }
                            "pbag" => {
                                let (_, pbag_) =
                                    parse_sf_bags(data_in_subchank, subchank.size / 4 - 1)
                                        .expect("Invalid pbag");
                                pbag = Some(pbag_);
                            }
                            "pmod" => {
                                let (_, pmod_) =
                                    parse_sf_mods(data_in_subchank, subchank.size / 10 - 1)
                                        .expect("Invalid pmod");
                                pmod = Some(pmod_);
                            }
                            "pgen" => {
                                let (_, pgen_) =
                                    parse_sf_gens(data_in_subchank, subchank.size / 4 - 1)
                                        .expect("Invalid pgen");
                                pgen = Some(pgen_);
                            }
                            "inst" => {
                                let (_, inst_) =
                                    parse_sf_inst_headers(data_in_subchank, subchank.size / 22 - 1)
                                        .expect("Invalid inst");
                                inst = Some(inst_);
                            }
                            "ibag" => {
                                let (_, ibag_) =
                                    parse_sf_bags(data_in_subchank, subchank.size / 4 - 1)
                                        .expect("Invalid ibag");
                                ibag = Some(ibag_);
                            }
                            "imod" => {
                                let (_, imod_) =
                                    parse_sf_mods(data_in_subchank, subchank.size / 10 - 1)
                                        .expect("Invalid imod");
                                imod = Some(imod_);
                            }
                            "igen" => {
                                let (_, igen_) =
                                    parse_sf_gens(data_in_subchank, subchank.size / 4 - 1)
                                        .expect("Invalid igen");
                                igen = Some(igen_);
                            }
                            "shdr" => {
                                let (_, shdr_) = parse_sf_sample_headers(
                                    data_in_subchank,
                                    subchank.size / 46 - 1,
                                )
                                .expect("Invalid shdr");
                                shdr = Some(shdr_);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(SF2pdta {
        phdr: phdr.expect("Failed to parse phdr"),
        pbag: pbag.expect("Failed to parse pbag"),
        pmod: pmod.expect("Failed to parse pmod"),
        pgen: pgen.expect("Failed to parse pgen"),
        inst: inst.expect("Failed to parse inst"),
        ibag: ibag.expect("Failed to parse ibag"),
        imod: imod.expect("Failed to parse imod"),
        igen: igen.expect("Failed to parse igen"),
        shdr: shdr.expect("Failed to parse shdr"),
    })
}