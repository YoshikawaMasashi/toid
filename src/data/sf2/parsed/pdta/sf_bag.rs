use nom::multi::many_m_n;
use nom::number::streaming::le_u16;
use nom::IResult;
use std::sync::Arc;

pub struct SFBag {
    pub gen_index: u16,
    pub mod_index: u16,
}

pub fn parse_sf_bags(i: &[u8], preset_num: usize) -> IResult<&[u8], Vec<Arc<SFBag>>> {
    many_m_n(preset_num, preset_num, parse_sf_bag)(i)
}

fn parse_sf_bag(i: &[u8]) -> IResult<&[u8], Arc<SFBag>> {
    let (i, gen_index) = le_u16(i)?;
    let (i, mod_index) = le_u16(i)?;
    Ok((
        i,
        Arc::new(SFBag {
            gen_index,
            mod_index,
        }),
    ))
}
