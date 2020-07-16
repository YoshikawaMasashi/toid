pub mod info;
pub mod pdta;
pub mod sdta;

use std::fmt;
use std::sync::Arc;

use super::super::riff::{RiffChunk, RiffData};
use info::{convert_chunk_to_sf2info, SF2Info};
use pdta::{convert_chunk_to_sf2pdta, SF2pdta};
use sdta::{convert_chunk_to_sf2sdta, SF2sdta};

pub struct SF2 {
    pub info: Arc<SF2Info>,
    pub sdta: Arc<SF2sdta>,
    pub pdta: Arc<SF2pdta>,
}

impl SF2 {
    pub fn parse(i: &[u8]) -> Result<Self, String> {
        let chunk = RiffChunk::parse(i)?;
        let sf2 = convert_chunk_to_sf2(&chunk);
        sf2
    }
}

impl fmt::Display for SF2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SF2\n")?;
        write!(f, "{}", self.info)?;
        write!(f, "{}", self.sdta)?;
        write!(f, "{}", self.pdta)?;

        Ok(())
    }
}

fn convert_chunk_to_sf2(chunk: &RiffChunk) -> Result<SF2, String> {
    let mut info: Option<SF2Info> = None;
    let mut sdta: Option<SF2sdta> = None;
    let mut pdta: Option<SF2pdta> = None;

    if let Some(chunk_type) = &chunk.chunk_type {
        if chunk_type == "sfbk" && chunk.id == "RIFF" {
            if let RiffData::Chunks(subchunks) = &chunk.data {
                for subchunk in subchunks {
                    if let Some(subchunk_type) = &subchunk.chunk_type {
                        match subchunk_type.as_str() {
                            "INFO" => {
                                info = Some(convert_chunk_to_sf2info(&subchunk)?);
                            }
                            "sdta" => {
                                sdta = Some(convert_chunk_to_sf2sdta(&subchunk)?);
                            }
                            "pdta" => {
                                pdta = Some(convert_chunk_to_sf2pdta(&subchunk)?);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    let info = match info {
        Some(info) => info,
        None => return Err("Failed to parse info".to_string()),
    };
    let sdta = match sdta {
        Some(sdta) => sdta,
        None => return Err("Failed to parse sdta".to_string()),
    };
    let pdta = match pdta {
        Some(pdta) => pdta,
        None => return Err("Failed to parse pdta".to_string()),
    };

    let info = Arc::new(info);
    let sdta = Arc::new(sdta);
    let pdta = Arc::new(pdta);

    Ok(SF2 { info, sdta, pdta })
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::io::Read;

    #[test]
    fn test_parse() {
        let paths = ["toid-sample-resource/sf2/florestan-subset.sf2"];
        for path in paths.iter() {
            let mut f = fs::File::open(path).map_err(|_| "file open error").unwrap();
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)
                .map_err(|_| "read error")
                .unwrap();
            let buffer = buffer.as_slice();

            SF2::parse(buffer).unwrap();
        }
    }
}
