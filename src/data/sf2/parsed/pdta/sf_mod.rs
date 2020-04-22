use nom::multi::many_m_n;
use nom::number::streaming::{le_i16, le_u16};
use nom::IResult;
use std::sync::Arc;

pub struct SFMod {
    pub src_oper: u16,
    pub dest_oper: u16,
    pub mod_amount: i16,
    pub amt_src_oper: u16,
    pub mod_trans_oper: u16,
}

pub fn parse_sf_mods(i: &[u8], preset_num: usize) -> IResult<&[u8], Vec<Arc<SFMod>>> {
    many_m_n(preset_num, preset_num, parse_sf_mod)(i)
}

fn parse_sf_mod(i: &[u8]) -> IResult<&[u8], Arc<SFMod>> {
    let (i, src_oper) = le_u16(i)?;
    let (i, dest_oper) = le_u16(i)?;
    let (i, mod_amount) = le_i16(i)?;
    let (i, amt_src_oper) = le_u16(i)?;
    let (i, mod_trans_oper) = le_u16(i)?;
    Ok((
        i,
        Arc::new(SFMod {
            src_oper,
            dest_oper,
            mod_amount,
            amt_src_oper,
            mod_trans_oper,
        }),
    ))
}
