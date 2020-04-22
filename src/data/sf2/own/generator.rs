use std::sync::Arc;

use super::instrument::Instrument;
use super::sample::Sample;

pub struct Range {
    pub min: u8,
    pub max: u8,
}

pub struct PresetGenerator {
    pub generator: Generator,
    pub instrument: Option<Arc<Instrument>>,
}

impl PresetGenerator {
    pub fn new() -> Self {
        PresetGenerator {
            generator: Generator::new(),
            instrument: None,
        }
    }

    pub fn set_instrument(&mut self, instrument: Arc<Instrument>) {
        self.instrument = Some(Arc::clone(&instrument));
    }

    pub fn set_oper(&mut self, generator: GeneratorEnum, amount: i16) {
        self.generator.set_oper(generator, amount);
    }
}

pub struct InstrumentGenerator {
    pub generator: Generator,
    pub sample: Option<Arc<Sample>>,
}

impl InstrumentGenerator {
    pub fn new() -> Self {
        InstrumentGenerator {
            generator: Generator::new(),
            sample: None,
        }
    }

    pub fn set_sample(&mut self, sample: Arc<Sample>) {
        self.sample = Some(Arc::clone(&sample));
    }

    pub fn set_oper(&mut self, generator: GeneratorEnum, amount: i16) {
        self.generator.set_oper(generator, amount);
    }
}

pub struct Generator {
    pub start_addrs_offset: u16,
    pub end_addrs_offset: u16,
    pub startloop_addrs_offset: u16,
    pub endloop_addrs_offset: u16,
    pub start_addrs_coarse_offset: u16,
    pub mod_lfo_to_pitch: f32,
    pub vib_lfo_to_pitch: f32,
    pub mod_env_to_pitch: f32,
    pub initial_filter_fc: f32,
    pub initial_filter_q: f32,
    pub mod_lfo_to_filter_fc: f32,
    pub mod_env_to_filter_fc: f32,
    pub end_addrs_coarse_offset: u16,
    pub mod_lfo_to_volume: f32,
    // unused1: Option<()>,
    pub chorus_effects_send: f32,
    pub reverb_effects_send: f32,
    pub pan: f32,
    // unused2: Option<()>,
    // unused3: Option<()>,
    // unused4: Option<()>,
    pub delay_mod_lfo: f32,
    pub freq_mod_lfo: f32,
    pub delay_vib_lfo: f32,
    pub freq_vib_lfo: f32,
    pub delay_mod_env: f32,
    pub attack_mod_env: f32,
    pub hold_mod_env: f32,
    pub decay_mod_env: f32,
    pub sustain_mod_env: f32,
    pub release_mod_env: f32,
    pub keynum_to_mod_env_hold: f32,
    pub keynum_to_mod_env_decay: f32,
    pub delay_vol_env: f32,
    pub attack_vol_env: f32,
    pub hold_vol_env: f32,
    pub decay_vol_env: f32,
    pub sustain_vol_env: f32,
    pub release_vol_env: f32,
    pub keynum_to_vol_env_hold: f32,
    pub keynum_to_vol_env_decay: f32,
    // instrument: Option<usize>,
    // reserved1: Option<()>,
    pub key_range: Range,
    pub vel_range: Range,
    pub startloop_addrs_coarse_offset: u16,
    pub keynum: Option<u8>,
    pub velocity: Option<u8>,
    pub initial_attenuation: f32,
    // reserved2: Option<()>,
    pub endloop_addrs_coarse_offset: u16,
    pub coarse_tune: f32,
    pub fine_tune: i16,
    // sample_id: Option<usize>,
    pub sample_modes: u8,
    // reserved3: Option<()>,
    pub scale_tuning: u16,
    pub exclusive_class: Option<u8>,
    pub overriding_root_key: Option<u8>,
    // unused5: Option<()>,
    // end_oper: Option<()>,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            start_addrs_offset: 0,
            end_addrs_offset: 0,
            startloop_addrs_offset: 0,
            endloop_addrs_offset: 0,
            start_addrs_coarse_offset: 0,
            mod_lfo_to_pitch: 0.0,
            vib_lfo_to_pitch: 0.0,
            mod_env_to_pitch: 0.0,
            initial_filter_fc: 19912.70,
            initial_filter_q: 0.0,
            mod_lfo_to_filter_fc: 0.0,
            mod_env_to_filter_fc: 0.0,
            end_addrs_coarse_offset: 0,
            mod_lfo_to_volume: 0.0,
            // unused1: None,
            chorus_effects_send: 0.0,
            reverb_effects_send: 0.0,
            pan: 0.0,
            // unused2: None,
            // unused3: None,
            // unused4: None,
            delay_mod_lfo: 0.001,
            freq_mod_lfo: 8.2,
            delay_vib_lfo: 0.001,
            freq_vib_lfo: 8.2,
            delay_mod_env: 0.001,
            attack_mod_env: 0.001,
            hold_mod_env: 0.001,
            decay_mod_env: 0.001,
            sustain_mod_env: 0.0,
            release_mod_env: 0.001,
            keynum_to_mod_env_hold: 0.0,
            keynum_to_mod_env_decay: 0.0,
            delay_vol_env: 0.001,
            attack_vol_env: 0.001,
            hold_vol_env: 0.001,
            decay_vol_env: 0.001,
            sustain_vol_env: 0.0,
            release_vol_env: 0.001,
            keynum_to_vol_env_hold: 0.0,
            keynum_to_vol_env_decay: 0.0,
            // reserved1: None,
            key_range: Range { min: 0, max: 127 },
            vel_range: Range { min: 0, max: 127 },
            startloop_addrs_coarse_offset: 0,
            keynum: None,
            velocity: None,
            initial_attenuation: 0.0,
            // reserved2: None,
            endloop_addrs_coarse_offset: 0,
            coarse_tune: 0.0,
            fine_tune: 0,
            // sample_id: None,
            sample_modes: 0,
            // reserved3: None,
            scale_tuning: 100,
            exclusive_class: None,
            overriding_root_key: None,
            // unused5: None,
            // end_oper: None,
        }
    }

    pub fn set_oper(&mut self, generator: GeneratorEnum, amount: i16) {
        match generator {
            GeneratorEnum::StartAddrsOffset => {
                self.start_addrs_offset = amount as u16;
            }
            GeneratorEnum::EndAddrsOffset => {
                self.end_addrs_offset = amount as u16;
            }
            GeneratorEnum::StartloopAddrsOffset => {
                self.startloop_addrs_offset = amount as u16;
            }
            GeneratorEnum::EndloopAddrsOffset => {
                self.endloop_addrs_offset = amount as u16;
            }
            GeneratorEnum::StartAddrsCoarseOffset => {
                self.start_addrs_coarse_offset = amount as u16;
            }
            GeneratorEnum::ModLfoToPitch => {
                self.mod_lfo_to_pitch = (amount as f32) / 100.0;
            }
            GeneratorEnum::VibLfoToPitch => {
                self.vib_lfo_to_pitch = (amount as f32) / 100.0;
            }
            GeneratorEnum::ModEnvToPitch => {
                self.mod_env_to_pitch = (amount as f32) / 100.0;
            }
            GeneratorEnum::InitialFilterFc => {
                self.initial_filter_fc = 8.176 * f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::InitialFilterQ => {
                self.initial_filter_q = (amount as f32) / 10.0;
            }
            GeneratorEnum::ModLfoToFilterFc => {
                self.mod_lfo_to_filter_fc = (amount as f32) / 100.0;
            }
            GeneratorEnum::ModEnvToFilterFc => {
                self.mod_env_to_filter_fc = (amount as f32) / 100.0;
            }
            GeneratorEnum::EndAddrsCoarseOffset => {
                self.end_addrs_coarse_offset = amount as u16;
            }
            GeneratorEnum::ModLfoToVolume => {
                self.mod_lfo_to_volume = (amount as f32) / 10.0;
            }
            GeneratorEnum::Unused1 => {}
            GeneratorEnum::ChorusEffectsSend => {
                self.chorus_effects_send = (amount as f32) / 10.0;
            }
            GeneratorEnum::ReverbEffectsSend => {
                self.reverb_effects_send = (amount as f32) / 10.0;
            }
            GeneratorEnum::Pan => {
                self.pan = (amount as f32) / 10.0;
            }
            GeneratorEnum::Unused2 => {}
            GeneratorEnum::Unused3 => {}
            GeneratorEnum::Unused4 => {}
            GeneratorEnum::DelayModLFO => {
                self.delay_mod_lfo = f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::FreqModLFO => {
                self.freq_mod_lfo = 8.176 * f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::DelayVibLFO => {
                self.delay_vib_lfo = f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::FreqVibLFO => {
                self.freq_vib_lfo = 8.176 * f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::DelayModEnv => {
                self.delay_mod_env = f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::AttackModEnv => {
                self.attack_mod_env = f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::HoldModEnv => {
                self.hold_mod_env = f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::DecayModEnv => {
                self.decay_mod_env = f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::SustainModEnv => {
                self.sustain_mod_env = (amount as f32) / 10.0;
            }
            GeneratorEnum::ReleaseModEnv => {
                self.release_mod_env = f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::KeynumToModEnvHold => {
                self.keynum_to_mod_env_hold = (amount as f32) / 100.0;
            }
            GeneratorEnum::KeynumToModEnvDecay => {
                self.keynum_to_mod_env_decay = (amount as f32) / 100.0;
            }
            GeneratorEnum::DelayVolEnv => {
                self.delay_vol_env = f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::AttackVolEnv => {
                self.attack_vol_env = f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::HoldVolEnv => {
                self.hold_vol_env = f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::DecayVolEnv => {
                self.decay_vol_env = f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::SustainVolEnv => {
                self.sustain_vol_env = (amount as f32) / 10.0;
            }
            GeneratorEnum::ReleaseVolEnv => {
                self.release_vol_env = f32::powf(2.0, (amount as f32) / 1200.0);
            }
            GeneratorEnum::KeynumToVolEnvHold => {
                self.keynum_to_vol_env_hold = (amount as f32) / 100.0;
            }
            GeneratorEnum::KeynumToVolEnvDecay => {
                self.keynum_to_vol_env_decay = (amount as f32) / 100.0;
            }
            GeneratorEnum::Instrument => {
                // self.instrument = Some(amount as usize);
            }
            GeneratorEnum::Reserved1 => {}
            GeneratorEnum::KeyRange => {
                self.key_range = Range {
                    min: (amount & 0x00FF) as u8,
                    max: (amount >> 8) as u8,
                };
            }
            GeneratorEnum::VelRange => {
                self.vel_range = Range {
                    min: (amount & 0x00FF) as u8,
                    max: (amount >> 8) as u8,
                };
            }
            GeneratorEnum::StartloopAddrsCoarseOffset => {
                self.startloop_addrs_coarse_offset = amount as u16;
            }
            GeneratorEnum::Keynum => {
                self.keynum = Some(amount as u8);
            }
            GeneratorEnum::Velocity => {
                self.velocity = Some(amount as u8);
            }
            GeneratorEnum::InitialAttenuation => {
                self.initial_attenuation = (amount as f32) / 10.0;
            }
            GeneratorEnum::Reserved2 => {}
            GeneratorEnum::EndloopAddrsCoarseOffset => {
                self.endloop_addrs_coarse_offset = amount as u16;
            }
            GeneratorEnum::CoarseTune => {
                self.coarse_tune = (amount as f32) / 10.0;
            }
            GeneratorEnum::FineTune => {
                self.fine_tune = amount as i16;
            }
            GeneratorEnum::SampleID => {
                // self.sample_id = Some(amount as usize);
            }
            GeneratorEnum::SampleModes => {
                self.sample_modes = amount as u8;
            }
            GeneratorEnum::Reserved3 => {}
            GeneratorEnum::ScaleTuning => {
                self.scale_tuning = amount as u16;
            }
            GeneratorEnum::ExclusiveClass => {
                self.exclusive_class = Some(amount as u8);
            }
            GeneratorEnum::OverridingRootKey => {
                self.overriding_root_key = Some(amount as u8);
            }
            GeneratorEnum::Unused5 => {}
            GeneratorEnum::EndOper => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GeneratorEnum {
    StartAddrsOffset,
    EndAddrsOffset,
    StartloopAddrsOffset,
    EndloopAddrsOffset,
    StartAddrsCoarseOffset,
    ModLfoToPitch,
    VibLfoToPitch,
    ModEnvToPitch,
    InitialFilterFc,
    InitialFilterQ,
    ModLfoToFilterFc,
    ModEnvToFilterFc,
    EndAddrsCoarseOffset,
    ModLfoToVolume,
    Unused1,
    ChorusEffectsSend,
    ReverbEffectsSend,
    Pan,
    Unused2,
    Unused3,
    Unused4,
    DelayModLFO,
    FreqModLFO,
    DelayVibLFO,
    FreqVibLFO,
    DelayModEnv,
    AttackModEnv,
    HoldModEnv,
    DecayModEnv,
    SustainModEnv,
    ReleaseModEnv,
    KeynumToModEnvHold,
    KeynumToModEnvDecay,
    DelayVolEnv,
    AttackVolEnv,
    HoldVolEnv,
    DecayVolEnv,
    SustainVolEnv,
    ReleaseVolEnv,
    KeynumToVolEnvHold,
    KeynumToVolEnvDecay,
    Instrument,
    Reserved1,
    KeyRange,
    VelRange,
    StartloopAddrsCoarseOffset,
    Keynum,
    Velocity,
    InitialAttenuation,
    Reserved2,
    EndloopAddrsCoarseOffset,
    CoarseTune,
    FineTune,
    SampleID,
    SampleModes,
    Reserved3,
    ScaleTuning,
    ExclusiveClass,
    OverridingRootKey,
    Unused5,
    EndOper,
}

impl GeneratorEnum {
    pub fn from_id(id: u16) -> Option<GeneratorEnum> {
        match id {
            0 => Some(GeneratorEnum::StartAddrsOffset),
            1 => Some(GeneratorEnum::EndAddrsOffset),
            2 => Some(GeneratorEnum::StartloopAddrsOffset),
            3 => Some(GeneratorEnum::EndloopAddrsOffset),
            4 => Some(GeneratorEnum::StartAddrsCoarseOffset),
            5 => Some(GeneratorEnum::ModLfoToPitch),
            6 => Some(GeneratorEnum::VibLfoToPitch),
            7 => Some(GeneratorEnum::ModEnvToPitch),
            8 => Some(GeneratorEnum::InitialFilterFc),
            9 => Some(GeneratorEnum::InitialFilterQ),
            10 => Some(GeneratorEnum::ModLfoToFilterFc),
            11 => Some(GeneratorEnum::ModEnvToFilterFc),
            12 => Some(GeneratorEnum::EndAddrsCoarseOffset),
            13 => Some(GeneratorEnum::ModLfoToVolume),
            14 => Some(GeneratorEnum::Unused1),
            15 => Some(GeneratorEnum::ChorusEffectsSend),
            16 => Some(GeneratorEnum::ReverbEffectsSend),
            17 => Some(GeneratorEnum::Pan),
            18 => Some(GeneratorEnum::Unused2),
            19 => Some(GeneratorEnum::Unused3),
            20 => Some(GeneratorEnum::Unused4),
            21 => Some(GeneratorEnum::DelayModLFO),
            22 => Some(GeneratorEnum::FreqModLFO),
            23 => Some(GeneratorEnum::DelayVibLFO),
            24 => Some(GeneratorEnum::FreqVibLFO),
            25 => Some(GeneratorEnum::DelayModEnv),
            26 => Some(GeneratorEnum::AttackModEnv),
            27 => Some(GeneratorEnum::HoldModEnv),
            28 => Some(GeneratorEnum::DecayModEnv),
            29 => Some(GeneratorEnum::SustainModEnv),
            30 => Some(GeneratorEnum::ReleaseModEnv),
            31 => Some(GeneratorEnum::KeynumToModEnvHold),
            32 => Some(GeneratorEnum::KeynumToModEnvDecay),
            33 => Some(GeneratorEnum::DelayVolEnv),
            34 => Some(GeneratorEnum::AttackVolEnv),
            35 => Some(GeneratorEnum::HoldVolEnv),
            36 => Some(GeneratorEnum::DecayVolEnv),
            37 => Some(GeneratorEnum::SustainVolEnv),
            38 => Some(GeneratorEnum::ReleaseVolEnv),
            39 => Some(GeneratorEnum::KeynumToVolEnvHold),
            40 => Some(GeneratorEnum::KeynumToVolEnvDecay),
            41 => Some(GeneratorEnum::Instrument),
            42 => Some(GeneratorEnum::Reserved1),
            43 => Some(GeneratorEnum::KeyRange),
            44 => Some(GeneratorEnum::VelRange),
            45 => Some(GeneratorEnum::StartloopAddrsCoarseOffset),
            46 => Some(GeneratorEnum::Keynum),
            47 => Some(GeneratorEnum::Velocity),
            48 => Some(GeneratorEnum::InitialAttenuation),
            49 => Some(GeneratorEnum::Reserved2),
            50 => Some(GeneratorEnum::EndloopAddrsCoarseOffset),
            51 => Some(GeneratorEnum::CoarseTune),
            52 => Some(GeneratorEnum::FineTune),
            53 => Some(GeneratorEnum::SampleID),
            54 => Some(GeneratorEnum::SampleModes),
            55 => Some(GeneratorEnum::Reserved3),
            56 => Some(GeneratorEnum::ScaleTuning),
            57 => Some(GeneratorEnum::ExclusiveClass),
            58 => Some(GeneratorEnum::OverridingRootKey),
            59 => Some(GeneratorEnum::Unused5),
            60 => Some(GeneratorEnum::EndOper),
            _ => None,
        }
    }
}
