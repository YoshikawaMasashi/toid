use nom::bytes::complete::take;
use nom::multi::many_m_n;
use nom::number::streaming::{le_u16, le_u32};
use nom::IResult;

pub struct SFPresetHeader {
    name: String,
    presento: u16,
    bank: u16,
    bagIndex: u16,
    library: u32,
    genre: u32,
    morph: u32,
}

pub fn parse_preset_headers(i: &[u8], preset_num: usize) -> IResult<&[u8], Vec<SFPresetHeader>> {
    many_m_n(preset_num, preset_num, parse_preset_header)(i)
}

fn parse_preset_header(i: &[u8]) -> IResult<&[u8], SFPresetHeader> {
    let (i, name) = take(20u8)(i)?;
    let name = String::from_utf8(name.to_vec()).unwrap();
    let (i, presento) = le_u16(i)?;
    let (i, bank) = le_u16(i)?;
    let (i, bagIndex) = le_u16(i)?;
    let (i, library) = le_u32(i)?;
    let (i, genre) = le_u32(i)?;
    let (i, morph) = le_u32(i)?;
    Ok((
        i,
        SFPresetHeader {
            name,
            presento,
            bank,
            bagIndex,
            library,
            genre,
            morph,
        },
    ))
}
