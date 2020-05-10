mod data;
mod fmt;

use std::sync::Arc;

use super::super::riff::{RiffChunk, RiffData};
use data::{convert_chunk_to_data_chunk, DataChunk};
use fmt::{convert_chunk_to_format_chunk, FormatChunk};

pub struct Wave {
    pub format: Arc<FormatChunk>,
    pub data: Arc<DataChunk>,
}

impl Wave {
    pub fn parse(i: &[u8]) -> Result<Self, String> {
        let chunk = RiffChunk::parse(i)?;
        Self::convert_from_chunk(&chunk)
    }

    fn convert_from_chunk(chunk: &RiffChunk) -> Result<Wave, String> {
        let mut format: Option<FormatChunk> = None;
        let mut data: Option<DataChunk> = None;

        if let Some(chunk_type) = &chunk.chunk_type {
            if chunk_type == "WAVE" && chunk.id == "RIFF" {
                match &chunk.data {
                    RiffData::Chunks(subchunks) => {
                        for subchunk in subchunks {
                            if let Some(subchunk_type) = &subchunk.chunk_type {
                                match subchunk_type.as_str() {
                                    _ => {}
                                }
                            } else {
                                match subchunk.id.as_str() {
                                    "fmt " => {
                                        format =
                                            Some(convert_chunk_to_format_chunk(subchunk).unwrap());
                                    }
                                    "data" => {
                                        data = Some(convert_chunk_to_data_chunk(subchunk).unwrap());
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
