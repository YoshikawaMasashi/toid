use std::boxed::Box;
use std::fmt;
use std::sync::Arc;

use nom::multi::many_m_n;
use nom::number::streaming::le_i16;
use nom::IResult;

use super::super::super::riff::{RiffChank, RiffData};

pub struct SF2sdta {
    pub smpl: Arc<Vec<i16>>,
}

impl fmt::Display for SF2sdta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "***SF2sdta***\n")?;
        write!(
            f,
            "smpl: {} {} {} ... length {} \n",
            self.smpl.get(0).unwrap(),
            self.smpl.get(1).unwrap(),
            self.smpl.get(2).unwrap(),
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

pub fn convert_chank_to_sf2sdta(chank: &RiffChank) -> Result<SF2sdta, String> {
    let mut smpl: Option<Vec<i16>> = None;

    if let Some(chank_type) = &chank.chank_type {
        if chank_type == "sdta" && chank.id == "LIST" {
            if let RiffData::Chanks(subchanks) = &chank.data {
                for subchank in subchanks {
                    if let RiffData::Data(data_in_subchank) = &subchank.data {
                        match subchank.id.as_str() {
                            "smpl" => {
                                let (_, smpl_) = parse_smpl(data_in_subchank, subchank.size / 2)
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
