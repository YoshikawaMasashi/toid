use nom::bytes::complete::take;
use nom::multi::many_m_n;
use nom::number::streaming::{le_u16, le_u32};
use nom::IResult;
use std::sync::Arc;

pub struct SFPresetHeader {
    pub name: String,
    pub presento: u16,
    pub bank: u16,
    pub bag_index: u16,
    pub library: u32,
    pub genre: u32,
    pub morph: u32,
}

pub fn parse_sf_preset_headers(
    i: &[u8],
    preset_num: usize,
) -> IResult<&[u8], Vec<Arc<SFPresetHeader>>> {
    many_m_n(preset_num, preset_num, parse_sf_preset_header)(i)
}

fn parse_sf_preset_header(i: &[u8]) -> IResult<&[u8], Arc<SFPresetHeader>> {
    let (i, name) = take(20u8)(i)?;
    let name = String::from_utf8(name.to_vec())
        .map_err(|_| nom::Err::Error((i, nom::error::ErrorKind::NoneOf)))?;
    let (i, presento) = le_u16(i)?;
    let (i, bank) = le_u16(i)?;
    let (i, bag_index) = le_u16(i)?;
    let (i, library) = le_u32(i)?;
    let (i, genre) = le_u32(i)?;
    let (i, morph) = le_u32(i)?;
    Ok((
        i,
        Arc::new(SFPresetHeader {
            name,
            presento,
            bank,
            bag_index,
            library,
            genre,
            morph,
        }),
    ))
}
