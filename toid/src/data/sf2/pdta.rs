use std::boxed::Box;
use std::fmt;

use nom::multi::many_m_n;
use nom::number::streaming::le_i16;
use nom::IResult;

use super::super::riff::{RiffChank, RiffData};

pub struct SF2pdta {}

impl fmt::Display for SF2pdta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "***SF2pdta***\n")?;

        Ok(())
    }
}

pub fn convert_chank_to_sf2pdta(chank: &RiffChank) -> Result<SF2pdta, String> {
    Ok(SF2pdta {})
}
