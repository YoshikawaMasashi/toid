use std::sync::Arc;

pub enum SampleType {
    Monoral,
    Right,
    Left,
    LinkSample,
}

impl SampleType {
    pub fn from_flg(flg: u16) -> Option<SampleType> {
        match flg {
            1 => Some(SampleType::Monoral),
            2 => Some(SampleType::Right),
            4 => Some(SampleType::Left),
            8 => Some(SampleType::LinkSample),
            _ => None,
        }
    }
}

pub struct Sample {
    pub sample_access: Arc<Vec<i16>>,
    pub name: String,
    pub start: u32,
    pub end: u32,
    pub loopstart: u32,
    pub loopend: u32,
    pub sample_rate: u32,
    pub original_key: u8,
    pub correction: i8,
    pub sample_link: Option<Arc<Sample>>,
    pub typee: SampleType,
}

impl Sample {
    pub fn get_sample(&self, start: usize, end: usize) -> Vec<i16> {
        let mut sample = Vec::new();
        sample.resize(end - start, 0);

        sample
    }
}
