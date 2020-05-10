pub mod info;
pub mod pdta;
pub mod sdta;

use std::fmt;
use std::sync::Arc;

use super::super::riff::{RiffChank, RiffData};
use info::{convert_chank_to_sf2info, SF2Info};
use pdta::{convert_chank_to_sf2pdta, SF2pdta};
use sdta::{convert_chank_to_sf2sdta, SF2sdta};

pub struct SF2 {
    pub info: Arc<SF2Info>,
    pub sdta: Arc<SF2sdta>,
    pub pdta: Arc<SF2pdta>,
}

impl SF2 {
    pub fn parse(i: &[u8]) -> Result<Self, String> {
        let chank = RiffChank::parse(i)?;
        let sf2 = convert_chank_to_sf2(&chank);
        sf2
    }
}

impl fmt::Display for SF2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SF2\n")?;
        write!(f, "{}", self.info)?;
        write!(f, "{}", self.sdta)?;
        write!(f, "{}", self.pdta)?;

        Ok(())
    }
}

fn convert_chank_to_sf2(chank: &RiffChank) -> Result<SF2, String> {
    let mut info: Option<SF2Info> = None;
    let mut sdta: Option<SF2sdta> = None;
    let mut pdta: Option<SF2pdta> = None;

    if let Some(chank_type) = &chank.chank_type {
        if chank_type == "sfbk" && chank.id == "RIFF" {
            if let RiffData::Chanks(subchanks) = &chank.data {
                for subchank in subchanks {
                    if let Some(subchank_type) = &subchank.chank_type {
                        match subchank_type.as_str() {
                            "INFO" => {
                                info = Some(convert_chank_to_sf2info(&subchank)?);
                            }
                            "sdta" => {
                                sdta = Some(convert_chank_to_sf2sdta(&subchank)?);
                            }
                            "pdta" => {
                                pdta = Some(convert_chank_to_sf2pdta(&subchank)?);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    let info = match info {
        Some(info) => info,
        None => return Err("Failed to parse info".to_string()),
    };
    let sdta = match sdta {
        Some(sdta) => sdta,
        None => return Err("Failed to parse sdta".to_string()),
    };
    let pdta = match pdta {
        Some(pdta) => pdta,
        None => return Err("Failed to parse pdta".to_string()),
    };

    let info = Arc::new(info);
    let sdta = Arc::new(sdta);
    let pdta = Arc::new(pdta);

    Ok(SF2 { info, sdta, pdta })
}
