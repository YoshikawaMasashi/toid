use nom::number::streaming::{le_i16, le_i24};
use nom::IResult;

use super::parsed;

pub enum Data {
    Monoral(Vec<f32>),
    Stereo((Vec<f32>, Vec<f32>)),
}

pub struct Wave {
    pub data: Data,
    pub sample_num: usize,
    pub sample_rate: f32,
}

impl Wave {
    pub fn parse(i: &[u8]) -> Result<Self, String> {
        let parsed_wave = parsed::Wave::parse(i)?;
        Self::parsed_wave_to_own_wave(parsed_wave)
    }

    pub fn get_samples(&self, start: usize, end: usize) -> Result<(Vec<f32>, Vec<f32>), String> {
        let mut left_sample = Vec::new();
        let mut right_sample = Vec::new();
        left_sample.resize(end - start, 0.0);
        right_sample.resize(end - start, 0.0);

        let start = std::cmp::min(start, self.sample_num);
        let end = std::cmp::min(end, self.sample_num);

        match &self.data {
            Data::Monoral(data) => {
                for idx in start..end {
                    left_sample.insert(idx - start, data[idx]);
                    right_sample.insert(idx - start, data[idx]);
                }
            }
            Data::Stereo((left_data, right_data)) => {
                for idx in start..end {
                    left_sample.insert(idx - start, left_data[idx]);
                    right_sample.insert(idx - start, right_data[idx]);
                }
            }
        };

        Ok((left_sample, right_sample))
    }

    fn parsed_wave_to_own_wave(parsed_wave: parsed::Wave) -> Result<Wave, String> {
        match parsed_wave.format.channels {
            1 => {
                let data = Self::parse_monoral_data(
                    parsed_wave.data.data.as_slice(),
                    parsed_wave.data.data.len() / (parsed_wave.format.bitswidth as usize / 8),
                    parsed_wave.format.bitswidth as usize,
                )
                .unwrap()
                .1;
                Ok(Wave {
                    data,
                    sample_num: parsed_wave.data.data.len()
                        / (parsed_wave.format.bitswidth as usize / 8),
                    sample_rate: parsed_wave.format.samplerate as f32,
                })
            }
            2 => {
                let data = Self::parse_stereo_data(
                    parsed_wave.data.data.as_slice(),
                    parsed_wave.data.data.len() / (parsed_wave.format.bitswidth as usize / 8) / 2,
                    parsed_wave.format.bitswidth as usize,
                )
                .unwrap()
                .1;
                Ok(Wave {
                    data,
                    sample_num: parsed_wave.data.data.len()
                        / (parsed_wave.format.bitswidth as usize / 8)
                        / 2,
                    sample_rate: parsed_wave.format.samplerate as f32,
                })
            }
            _ => Err("invalid channel".to_string()),
        }
    }

    fn parse_monoral_data(i: &[u8], sample_num: usize, bit_num: usize) -> IResult<&[u8], Data> {
        let mut data = vec![];
        data.resize(sample_num, 0.0);
        let mut i = i;
        match bit_num {
            16 => {
                for sample_idx in 0..sample_num {
                    let ret = le_i16(i)?;
                    i = ret.0;
                    data[sample_idx] = ret.1 as f32 / i16::MAX as f32;
                }
            }
            24 => {
                for sample_idx in 0..sample_num {
                    let ret = le_i24(i)?;
                    i = ret.0;
                    data[sample_idx] = ret.1 as f32 / (i16::MAX as i32 * u8::MAX as i32) as f32;
                }
            }
            _ => {
                return Err(nom::Err::Error((i, nom::error::ErrorKind::NoneOf)));
            }
        }
        Ok((i, Data::Monoral(data)))
    }

    fn parse_stereo_data(i: &[u8], sample_num: usize, bit_num: usize) -> IResult<&[u8], Data> {
        let mut left_data = vec![];
        let mut right_data = vec![];
        left_data.resize(sample_num, 0.0);
        right_data.resize(sample_num, 0.0);
        let mut i = i;
        match bit_num {
            16 => {
                for sample_idx in 0..sample_num {
                    let ret = le_i16(i)?;
                    i = ret.0;
                    left_data[sample_idx] = ret.1 as f32 / i16::MAX as f32;
                    let ret = le_i16(i)?;
                    i = ret.0;
                    right_data[sample_idx] = ret.1 as f32 / i16::MAX as f32;
                }
            }
            24 => {
                for sample_idx in 0..sample_num {
                    let ret = le_i24(i)?;
                    i = ret.0;
                    left_data[sample_idx] =
                        ret.1 as f32 / (i16::MAX as i32 * u8::MAX as i32) as f32;
                    let ret = le_i24(i)?;
                    i = ret.0;
                    right_data[sample_idx] =
                        ret.1 as f32 / (i16::MAX as i32 * u8::MAX as i32) as f32;
                }
            }
            _ => {
                return Err(nom::Err::Error((i, nom::error::ErrorKind::NoneOf)));
            }
        }
        Ok((i, Data::Stereo((left_data, right_data))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::io::Read;

    #[test]
    fn test_parse() {
        let paths = [
            "toid-sample-resource/drums/0_hihat_closed.wav",
            "toid-sample-resource/drums/0_snare_drum.wav",
            "toid-sample-resource/drums/3_kick_drum.wav",
        ];
        for path in paths.iter() {
            let mut f = fs::File::open(path).map_err(|_| "file open error").unwrap();
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)
                .map_err(|_| "read error")
                .unwrap();
            let buffer = buffer.as_slice();

            Wave::parse(buffer).unwrap();
        }
    }
}
