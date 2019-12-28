use nom::bytes::complete::take;
use nom::multi::many_m_n;
use nom::number::streaming::le_u16;
use nom::IResult;
use std::sync::Arc;

pub struct SFInstHeader {
    pub name: String,
    pub bag_index: u16,
}

pub fn parse_sf_inst_headers(
    i: &[u8],
    preset_num: usize,
) -> IResult<&[u8], Vec<Arc<SFInstHeader>>> {
    many_m_n(preset_num, preset_num, parse_sf_inst_header)(i)
}

fn parse_sf_inst_header(i: &[u8]) -> IResult<&[u8], Arc<SFInstHeader>> {
    let (i, name) = take(20u8)(i)?;
    let name = String::from_utf8(name.to_vec()).unwrap();
    let (i, bag_index) = le_u16(i)?;
    Ok((i, Arc::new(SFInstHeader { name, bag_index })))
}
