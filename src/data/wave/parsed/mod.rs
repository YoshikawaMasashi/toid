mod data;
mod fmt;

use std::sync::Arc;

use super::super::riff::{RiffChank, RiffData};
use data::{convert_chank_to_data_chunk, DataChunk};
use fmt::{convert_chank_to_format_chunk, FormatChunk};

pub struct Wave {
    pub format: Arc<FormatChunk>,
    pub data: Arc<DataChunk>,
}

impl Wave {
    pub fn parse(i: &[u8]) -> Result<Self, String> {
        let chunk = RiffChank::parse(i)?;
        Self::convert_from_chunk(&chunk)
    }

    fn convert_from_chunk(chank: &RiffChank) -> Result<Wave, String> {
        let mut format: Option<FormatChunk> = None;
        let mut data: Option<DataChunk> = None;

        if let Some(chank_type) = &chank.chank_type {
            if chank_type == "WAVE" && chank.id == "RIFF" {
                match &chank.data {
                    RiffData::Chanks(subchanks) => {
                        for subchank in subchanks {
                            if let Some(subchank_type) = &subchank.chank_type {
                                match subchank_type.as_str() {
                                    _ => {}
                                }
                            } else {
                                match subchank.id.as_str() {
                                    "fmt " => {
                                        format =
                                            Some(convert_chank_to_format_chunk(subchank).unwrap());
                                    }
                                    "data" => {
                                        data = Some(convert_chank_to_data_chunk(subchank).unwrap());
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let format = match format {
            Some(format) => format,
            None => return Err("Failed to parse format".to_string()),
        };
        let data = match data {
            Some(data) => data,
            None => return Err("Failed to parse fodatarmat".to_string()),
        };

        let format = Arc::new(format);
        let data = Arc::new(data);

        Ok(Wave { format, data })
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
