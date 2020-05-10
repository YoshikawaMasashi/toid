use nom::number::streaming::{le_i16, le_u16, le_u32};
use nom::IResult;

use super::super::super::riff::{RiffChank, RiffData};

pub struct FormatChunk {
    pub format: i16,
    pub channels: u16,
    pub samplerate: u32,
    pub bytepersec: u32,
    pub blockalign: u16,
    pub bitswidth: u16,
}

impl std::fmt::Display for FormatChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "***FormatChunk***\n")?;

        Ok(())
    }
}

fn parse_format_chunk(i: &[u8]) -> IResult<&[u8], FormatChunk> {
    let (i, format) = le_i16(i)?;
    let (i, channels) = le_u16(i)?;
    let (i, samplerate) = le_u32(i)?;
    let (i, bytepersec) = le_u32(i)?;
    let (i, blockalign) = le_u16(i)?;
    let (i, bitswidth) = le_u16(i)?;

    Ok((
        i,
        FormatChunk {
            format,
            channels,
            samplerate,
            bytepersec,
            blockalign,
            bitswidth,
        },
    ))
}

pub fn convert_chank_to_format_chunk(chank: &RiffChank) -> Result<FormatChunk, String> {
    if chank.chank_type == None && chank.size == 16 {
        if let RiffData::Data(data) = &chank.data {
            let i: &[u8] = data.as_slice();
            return Ok(parse_format_chunk(i).unwrap().1);
        }
    }

    Err("error".to_string())
}
