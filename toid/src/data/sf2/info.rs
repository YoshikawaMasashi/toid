use std::fmt;

use nom::number::streaming::le_u16;
use nom::IResult;

use super::super::riff::{RiffChank, RiffData};

pub struct SF2Info {
    ifil_major: u16,
    ifil_minor: u16,
    isng: String,
    inam: String,
    irom: Option<String>,
    iver_major: Option<u16>,
    iver_minor: Option<u16>,
    icrd: Option<String>,
    ieng: Option<String>,
    iprd: Option<String>,
    icop: Option<String>,
    icmt: Option<String>,
    isft: Option<String>,
}

impl fmt::Display for SF2Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "***SF2Info***\n")?;
        write!(f, "ifil: {} {}\n", self.ifil_major, self.ifil_minor)?;
        write!(f, "isng: {}\n", self.isng)?;
        write!(f, "INAM: {}\n", self.inam)?;
        if let Some(irom) = &self.irom {
            write!(f, "irom: {}\n", irom)?;
        }
        if let Some(iver_major) = self.iver_major {
            if let Some(iver_minor) = self.iver_minor {
                write!(f, "iver: {} {}\n", iver_major, iver_minor)?;
            }
        }
        if let Some(icrd) = &self.icrd {
            write!(f, "ICRD: {}\n", icrd)?;
        }
        if let Some(ieng) = &self.ieng {
            write!(f, "IENG: {}\n", ieng)?;
        }
        if let Some(iprd) = &self.iprd {
            write!(f, "IPRD: {}\n", iprd)?;
        }
        if let Some(icop) = &self.icop {
            write!(f, "ICOP: {}\n", icop)?;
        }
        if let Some(icmt) = &self.icmt {
            write!(f, "ICMT: {}\n", icmt)?;
        }
        if let Some(isft) = &self.isft {
            write!(f, "ISFT: {}\n", isft)?;
        }

        Ok(())
    }
}

fn parse_sfversion(i: &[u8]) -> IResult<&[u8], (u16, u16)> {
    let (i, major) = le_u16(i)?;
    let (i, minor) = le_u16(i)?;
    Ok((i, (major, minor)))
}

pub fn convert_chank_to_sf2info(chank: &RiffChank) -> Result<SF2Info, String> {
    let mut ifil_major: Option<u16> = None;
    let mut ifil_minor: Option<u16> = None;
    let mut isng: Option<String> = None;
    let mut inam: Option<String> = None;
    let mut irom: Option<String> = None;
    let mut iver_major: Option<u16> = None;
    let mut iver_minor: Option<u16> = None;
    let mut icrd: Option<String> = None;
    let mut ieng: Option<String> = None;
    let mut iprd: Option<String> = None;
    let mut icop: Option<String> = None;
    let mut icmt: Option<String> = None;
    let mut isft: Option<String> = None;

    if let Some(chank_type) = &chank.chank_type {
        if chank_type == "INFO" && chank.id == "LIST" {
            if let RiffData::Chanks(subchanks) = &chank.data {
                for subchank in subchanks {
                    if let RiffData::Data(data_in_subchank) = &subchank.data {
                        match subchank.id.as_str() {
                            "ifil" => {
                                let i = data_in_subchank;
                                let (_i, (ifil_major_, ifil_minor_)) =
                                    parse_sfversion(i).expect("Invalid ifil");
                                ifil_major = Some(ifil_major_);
                                ifil_minor = Some(ifil_minor_);
                            }
                            "isng" => {
                                isng = Some(
                                    String::from_utf8(data_in_subchank.to_vec())
                                        .expect("Invalid isng"),
                                );
                            }
                            "INAM" => {
                                inam = Some(
                                    String::from_utf8(data_in_subchank.to_vec())
                                        .expect("Invalid INAM"),
                                );
                            }
                            "irom" => {
                                irom = Some(
                                    String::from_utf8(data_in_subchank.to_vec())
                                        .expect("Invalid irom"),
                                );
                            }
                            "iver" => {
                                let i = data_in_subchank;
                                let (_i, (iver_major_, iver_minor_)) =
                                    parse_sfversion(i).expect("Invalid iver");
                                iver_major = Some(iver_major_);
                                iver_minor = Some(iver_minor_);
                            }
                            "ICRD" => {
                                icrd = Some(
                                    String::from_utf8(data_in_subchank.to_vec())
                                        .expect("Invalid ICRD"),
                                );
                            }
                            "IENG" => {
                                ieng = Some(
                                    String::from_utf8(data_in_subchank.to_vec())
                                        .expect("Invalid IENG"),
                                );
                            }
                            "IPRD" => {
                                iprd = Some(
                                    String::from_utf8(data_in_subchank.to_vec())
                                        .expect("Invalid IPRD"),
                                );
                            }
                            "ICOP" => {
                                icop = Some(
                                    String::from_utf8(data_in_subchank.to_vec())
                                        .expect("Invalid ICOP"),
                                );
                            }
                            "ICMT" => {
                                icmt = Some(
                                    String::from_utf8(data_in_subchank.to_vec())
                                        .expect("Invalid ICMT"),
                                );
                            }
                            "ISFT" => {
                                isft = Some(
                                    String::from_utf8(data_in_subchank.to_vec())
                                        .expect("Invalid ISFT"),
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(SF2Info {
        ifil_major: ifil_major.unwrap(),
        ifil_minor: ifil_minor.unwrap(),
        isng: isng.unwrap(),
        inam: inam.unwrap(),
        irom,
        iver_major,
        iver_minor,
        icrd,
        ieng,
        iprd,
        icop,
        icmt,
        isft,
    })
}
