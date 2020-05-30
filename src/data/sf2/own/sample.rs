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
    pub sample_access: Arc<Vec<f32>>,
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
    pub fn get_sample(&self, key: u8, idx: usize) -> Result<f32, String> {
        let pitch_shift =
            (key as i16 - self.original_key as i16) as f32 + (self.correction as f32) / 100.0;
        let freq_shift = f32::powf(2.0, pitch_shift / 12.0);
        let sample_link_idx = self.calculate_idx_of_sample_access(idx as f32 * freq_shift);
        self.sample_for_float_sample_link_idx(sample_link_idx)
    }

    pub fn get_samples(&self, key: u8, start: usize, end: usize) -> Result<Vec<f32>, String> {
        let mut sample = Vec::new();
        sample.resize(end - start, 0.0);

        let pitch_shift =
            (key as i16 - self.original_key as i16) as f32 + (self.correction as f32) / 100.0;
        let freq_shift = f32::powf(2.0, pitch_shift / 12.0);
        let freq_shift = freq_shift * self.sample_rate as f32 / 44100.0;

        for idx in start..end {
            let sample_link_idx = self.calculate_idx_of_sample_access(idx as f32 * freq_shift);
            sample.insert(
                idx - start,
                self.sample_for_float_sample_link_idx(sample_link_idx)?,
            );
        }

        Ok(sample)
    }

    fn calculate_idx_of_sample_access(&self, idx: f32) -> f32 {
        if idx < (self.loopstart - self.start) as f32 {
            self.start as f32 + idx
        } else {
            self.loopstart as f32
                + (idx - ((self.loopstart - self.start) as f32))
                    % ((self.loopend - self.loopstart) as f32)
        }
    }

    fn sample_for_float_sample_link_idx(&self, idx: f32) -> Result<f32, String> {
        let floor_idx = idx.floor() as usize;
        let ceil_idx = floor_idx + 1;
        let ratio = 1.0 - (idx % 1.0);

        let floor_sample = *self.sample_access.get(floor_idx).ok_or("get faild")? as f32;
        let ceil_sample = *self.sample_access.get(ceil_idx).ok_or("get faild")? as f32;
        Ok(floor_sample * ratio + ceil_sample * (1.0 - ratio))
    }
}
