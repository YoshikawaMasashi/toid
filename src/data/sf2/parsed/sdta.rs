use std::fmt;
use std::sync::Arc;

use nom::multi::many_m_n;
use nom::number::streaming::le_i16;
use nom::IResult;

use super::super::super::riff::{RiffChunk, RiffData};

pub struct SF2sdta {
    pub smpl: Arc<Vec<i16>>,
}

impl fmt::Display for SF2sdta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "***SF2sdta***\n")?;
        write!(
            f,
            "smpl: {} {} {} ... length {} \n",
            self.smpl.get(0).ok_or(fmt::Error {})?,
            self.smpl.get(1).ok_or(fmt::Error {})?,
            self.smpl.get(2).ok_or(fmt::Error {})?,
            self.smpl.len()
        )?;

        Ok(())
    }
}

fn parse_smpl(i: &[u8], num: usize) -> IResult<&[u8], Vec<i16>> {
    let (_, smpl) = many_m_n(num, num, le_i16)(i)?;
    let smpl = smpl;
    Ok((i, (smpl)))
}

pub fn convert_chunk_to_sf2sdta(chunk: &RiffChunk) -> Result<SF2sdta, String> {
    let mut smpl: Option<Vec<i16>> = None;

    if let Some(chunk_type) = &chunk.chunk_type {
        if chunk_type == "sdta" && chunk.id == "LIST" {
            if let RiffData::Chunks(subchunks) = &chunk.data {
                for subchunk in subchunks {
                    if let RiffData::Data(data_in_subchunk) = &subchunk.data {
                        match subchunk.id.as_str() {
                            "smpl" => {
                                let (_, smpl_) = parse_smpl(data_in_subchunk, subchunk.size / 2)
                                    .expect("Invalid smpl");
                                smpl = Some(smpl_);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(SF2sdta {
        smpl: Arc::new(smpl.expect("Failed to parse smpl")),
    })
}
