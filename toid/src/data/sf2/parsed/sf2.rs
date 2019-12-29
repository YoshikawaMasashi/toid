use std::fmt;
use std::sync::Arc;

use super::super::super::riff::{RiffChank, RiffData};
use super::info::{convert_chank_to_sf2info, SF2Info};
use super::pdta::{convert_chank_to_sf2pdta, SF2pdta};
use super::sdta::{convert_chank_to_sf2sdta, SF2sdta};

pub struct SF2 {
    pub info: Arc<SF2Info>,
    pub sdta: Arc<SF2sdta>,
    pub pdta: Arc<SF2pdta>,
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
                                info = Some(
                                    convert_chank_to_sf2info(&subchank)
                                        .expect("invalid INFO chank"),
                                );
                            }
                            "sdta" => {
                                sdta = Some(
                                    convert_chank_to_sf2sdta(&subchank)
                                        .expect("invalid sdta chank"),
                                );
                            }
                            "pdta" => {
                                pdta = Some(
                                    convert_chank_to_sf2pdta(&subchank)
                                        .expect("invalid pdta chank"),
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    let info = info.expect("Failed to parse info");
    let sdta = sdta.expect("Failed to parse sdta");
    let pdta = pdta.expect("Failed to parse pdta");

    let info = Arc::new(info);
    let sdta = Arc::new(sdta);
    let pdta = Arc::new(pdta);

    Ok(SF2 { info, sdta, pdta })
}
