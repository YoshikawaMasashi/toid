use nom::multi::many_m_n;
use nom::number::streaming::le_u16;
use nom::IResult;
use std::sync::Arc;

pub struct SFGen {
    pub gen_per: u16,
    pub gen_amount: u16,
}

pub fn parse_sf_gens(i: &[u8], preset_num: usize) -> IResult<&[u8], Vec<Arc<SFGen>>> {
    many_m_n(preset_num, preset_num, parse_sf_gen)(i)
}

fn parse_sf_gen(i: &[u8]) -> IResult<&[u8], Arc<SFGen>> {
    let (i, gen_per) = le_u16(i)?;
    let (i, gen_amount) = le_u16(i)?;
    Ok((
        i,
        Arc::new(SFGen {
            gen_per,
            gen_amount,
        }),
    ))
}
