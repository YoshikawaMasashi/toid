use std::sync::Arc;

use super::super::sf2;
use super::generator::{Generator, GeneratorEnum};
use super::instrument::Instrument;
use super::preset::Preset;
use super::sample::{Sample, SampleType};

pub struct SF2 {
    presets: Vec<Arc<Preset>>,
}

impl SF2 {
    pub fn new() -> Self {
        SF2 {
            presets: Vec::new(),
        }
    }

    pub fn add_preset(&mut self, preset: Arc<Preset>) {
        self.presets.push(preset);
    }

    pub fn parse(i: &[u8]) -> Self {
        let parsed_sf2 = sf2::SF2::parse(i);
        parsed_sf2_to_own_sf2(parsed_sf2)
    }
}

fn parsed_sf2_to_own_sf2(parsed_sf2: sf2::SF2) -> SF2 {
    let mut own_sf2 = SF2::new();
    let sample_access = Arc::clone(&parsed_sf2.sdta.smpl);

    let mut samples = Vec::new();
    for sample_header in parsed_sf2.pdta.shdr.iter() {
        let sample = Sample {
            sample_access: Arc::clone(&sample_access),
            name: sample_header.name.clone(),
            start: sample_header.start,
            end: sample_header.end,
            loopstart: sample_header.loopstart,
            loopend: sample_header.end,
            sample_rate: sample_header.sample_rate,
            original_key: sample_header.original_key,
            correction: sample_header.correction,
            sample_link: None,
            typee: SampleType::from_flg(sample_header.typee).unwrap(),
        };
        let sample = Arc::new(sample);
        samples.push(sample);
    }

    let mut inst_gen_info_sections = Vec::new();
    for ibag in parsed_sf2.pdta.ibag.iter() {
        inst_gen_info_sections.push(ibag.gen_index as usize);
    }
    inst_gen_info_sections.push(parsed_sf2.pdta.igen.len());

    let mut inst_generators = Vec::new();
    for inst_gen_idx in 0..parsed_sf2.pdta.ibag.len() {
        let inst_gen_info_start = inst_gen_info_sections.get(inst_gen_idx).unwrap();
        let inst_gen_info_end = inst_gen_info_sections.get(inst_gen_idx + 1).unwrap();

        let mut generator = Generator::new();

        for inst_gen_info_idx in *inst_gen_info_start..*inst_gen_info_end {
            let inst_gen_info = parsed_sf2.pdta.igen.get(inst_gen_info_idx).unwrap();
            let gen_oper = GeneratorEnum::from_id(inst_gen_info.gen_oper).unwrap();
            let gen_amount = inst_gen_info.gen_amount;

            generator.set_oper(gen_oper, gen_amount);
            if let GeneratorEnum::SampleID = gen_oper{
                let sample_idx = gen_amount as usize;
                generator.set_sample(Arc::clone(samples.get(sample_idx).unwrap()));
            }
        }

        let generator = Arc::new(generator);
        inst_generators.push(generator);
    }

    let mut inst_gen_sections = Vec::new();
    for inst in parsed_sf2.pdta.inst.iter() {
        inst_gen_sections.push(inst.bag_index as usize);
    }
    inst_gen_sections.push(parsed_sf2.pdta.ibag.len());

    let mut instruments = Vec::new();
    for (inst_idx, inst) in parsed_sf2.pdta.inst.iter().enumerate() {
        let inst_gen_start = inst_gen_sections.get(inst_idx).unwrap();
        let inst_gen_end = inst_gen_sections.get(inst_idx + 1).unwrap();

        let mut instrument = Instrument::new();
        instrument.set_name(inst.name.clone());
        for inst_gen_idx in *inst_gen_start..*inst_gen_end {
            instrument.add_generator(Arc::clone(inst_generators.get(inst_gen_idx).unwrap()));
        }
        instruments.push(Arc::new(instrument));
    }

    let mut preset_gen_info_sections = Vec::new();
    for pbag in parsed_sf2.pdta.pbag.iter() {
        preset_gen_info_sections.push(pbag.gen_index as usize);
    }
    preset_gen_info_sections.push(parsed_sf2.pdta.pgen.len());

    let mut preset_generators = Vec::new();
    for preset_gen_idx in 0..parsed_sf2.pdta.pbag.len() {
        let preset_gen_info_start = preset_gen_info_sections.get(preset_gen_idx).unwrap();
        let preset_gen_info_end = preset_gen_info_sections.get(preset_gen_idx + 1).unwrap();

        let mut generator = Generator::new();

        for preset_gen_info_idx in *preset_gen_info_start..*preset_gen_info_end {
            let preset_gen_info = parsed_sf2.pdta.igen.get(preset_gen_info_idx).unwrap();
            let gen_oper = GeneratorEnum::from_id(preset_gen_info.gen_oper).unwrap();
            let gen_amount = preset_gen_info.gen_amount;

            generator.set_oper(gen_oper, gen_amount);
            if let GeneratorEnum::Instrument = gen_oper {
                let instrument_idx = gen_amount as usize;
                generator.set_instrument(Arc::clone(instruments.get(instrument_idx).unwrap()));
            }
        }

        let generator = Arc::new(generator);
        preset_generators.push(generator);
    }

    let mut preset_gen_sections = Vec::new();
    for phdr in parsed_sf2.pdta.phdr.iter() {
        preset_gen_sections.push(phdr.bag_index as usize);
    }
    preset_gen_sections.push(parsed_sf2.pdta.pbag.len());

    for (preset_idx, phdr) in parsed_sf2.pdta.phdr.iter().enumerate() {
        let preset_gen_start = preset_gen_sections.get(preset_idx).unwrap();
        let preset_gen_end = preset_gen_sections.get(preset_idx + 1).unwrap();

        let mut preset = Preset::new();
        preset.set_name(phdr.name.clone());
        for preset_gen_idx in *preset_gen_start..*preset_gen_end {
            preset.add_generator(Arc::clone(preset_generators.get(preset_gen_idx).unwrap()));
        }
        own_sf2.add_preset(Arc::new(preset));
    }

    own_sf2
}
