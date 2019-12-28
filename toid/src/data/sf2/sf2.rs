use std::fmt;

use super::super::riff::{RiffChank, RiffData};
use super::info::{convert_chank_to_sf2info, SF2Info};

pub struct SF2 {
    info: SF2Info,
}

impl SF2 {
    pub fn parse(i: &[u8]) -> Self {
        let chank = RiffChank::parse(i);
        convert_chank_to_sf2(&chank).expect("Failed to conver to sf2")
    }
}

impl fmt::Display for SF2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SF2\n")?;
        self.info.fmt(f)
    }
}

fn convert_chank_to_sf2(chank: &RiffChank) -> Result<SF2, String> {
    let mut info: Option<SF2Info> = None;

    if let Some(chank_type) = &chank.chank_type {
        if chank_type == "sfbk" && chank.id == "RIFF" {
            if let RiffData::Chanks(subchanks) = &chank.data {
                for subchank in subchanks {
                    if let Some(subchank_type) = &subchank.chank_type {
                        match subchank_type.as_str() {
                            "INFO" => {
                                info = Some(
                                    convert_chank_to_sf2info(&subchank)
                                        .expect("invalid INFO chank"),
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    if let Some(info) = info {
        Ok(SF2 { info })
    } else {
        Err(String::from("Invalid SF2 chank"))
    }
}
