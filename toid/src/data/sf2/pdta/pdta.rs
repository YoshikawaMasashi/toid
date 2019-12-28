use std::boxed::Box;
use std::fmt;

use nom::multi::many_m_n;
use nom::number::streaming::le_i16;
use nom::IResult;

use super::super::super::riff::{RiffChank, RiffData};
use super::preset_header::{parse_preset_headers, SFPresetHeader};

pub struct SF2pdta {
    phdr: Vec<SFPresetHeader>,
}

impl fmt::Display for SF2pdta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "***SF2pdta***\n")?;
        write!(f, "phdr length: {}\n", self.phdr.len())?;

        Ok(())
    }
}

pub fn convert_chank_to_sf2pdta(chank: &RiffChank) -> Result<SF2pdta, String> {
    let mut phdr: Option<Vec<SFPresetHeader>> = None;

    if let Some(chank_type) = &chank.chank_type {
        if chank_type == "pdta" && chank.id == "LIST" {
            if let RiffData::Chanks(subchanks) = &chank.data {
                for subchank in subchanks {
                    if let RiffData::Data(data_in_subchank) = &subchank.data {
                        match subchank.id.as_str() {
                            "phdr" => {
                                let (_, phdr_) =
                                    parse_preset_headers(data_in_subchank, subchank.size / 38 - 1)
                                        .expect("Invalid phdr");
                                phdr = Some(phdr_);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(SF2pdta {
        phdr: phdr.expect("Failed to parse phdr"),
    })
}
