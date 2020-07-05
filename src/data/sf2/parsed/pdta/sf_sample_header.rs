use nom::bytes::complete::take;
use nom::multi::many_m_n;
use nom::number::streaming::{le_i8, le_u16, le_u32, le_u8};
use nom::IResult;
use std::sync::Arc;

pub struct SFSampleHeader {
    pub name: String,
    pub start: u32,
    pub end: u32,
    pub loopstart: u32,
    pub loopend: u32,
    pub sample_rate: u32,
    pub original_key: u8,
    pub correction: i8,
    pub sample_link: u16,
    pub typee: u16,
}

pub fn parse_sf_sample_headers(
    i: &[u8],
    preset_num: usize,
) -> IResult<&[u8], Vec<Arc<SFSampleHeader>>> {
    many_m_n(preset_num, preset_num, parse_sf_sample_header)(i)
}

fn parse_sf_sample_header(i: &[u8]) -> IResult<&[u8], Arc<SFSampleHeader>> {
    let (i, name) = take(20u8)(i)?;
    let name = if let Some(x) = name.iter().position(|&x| x == 0) {
        name.split_at(x).0
    } else {
        name
    };
    let name = String::from_utf8(name.to_vec())
        .map_err(|_| nom::Err::Error((i, nom::error::ErrorKind::NoneOf)))?;
    let (i, start) = le_u32(i)?;
    let (i, end) = le_u32(i)?;
    let (i, loopstart) = le_u32(i)?;
    let (i, loopend) = le_u32(i)?;
    let (i, sample_rate) = le_u32(i)?;
    let (i, original_key) = le_u8(i)?;
    let (i, correction) = le_i8(i)?;
    let (i, sample_link) = le_u16(i)?;
    let (i, typee) = le_u16(i)?;

    Ok((
        i,
        Arc::new(SFSampleHeader {
            name,
            start,
            end,
            loopstart,
            loopend,
            sample_rate,
            original_key,
            correction,
            sample_link,
            typee,
        }),
    ))
}
